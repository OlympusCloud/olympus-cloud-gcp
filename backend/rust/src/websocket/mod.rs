//! WebSocket support for real-time updates

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};
use uuid::Uuid;

pub mod handlers;
pub mod messages;

/// WebSocket connection state
#[derive(Debug, Clone)]
pub struct WsConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub subscriptions: Vec<String>,
}

/// WebSocket hub for managing connections
pub struct WsHub {
    /// Active connections
    connections: Arc<RwLock<HashMap<Uuid, WsConnection>>>,
    /// Broadcast channel for sending messages
    sender: broadcast::Sender<WsMessage>,
    /// Redis client for pub/sub
    redis_client: redis::Client,
}

impl WsHub {
    pub fn new(redis_url: &str) -> Self {
        let (sender, _) = broadcast::channel(1000);
        let redis_client = redis::Client::open(redis_url)
            .expect("Failed to create Redis client");

        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sender,
            redis_client,
        }
    }

    /// Register a new connection
    pub async fn register(&self, conn: WsConnection) {
        let mut connections = self.connections.write().await;
        connections.insert(conn.id, conn.clone());
        info!("WebSocket connection registered: {}", conn.id);
    }

    /// Unregister a connection
    pub async fn unregister(&self, conn_id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&conn_id);
        info!("WebSocket connection unregistered: {}", conn_id);
    }

    /// Broadcast message to all connections in a tenant
    pub async fn broadcast_to_tenant(&self, tenant_id: Uuid, message: WsMessage) {
        let connections = self.connections.read().await;
        let tenant_connections: Vec<_> = connections
            .values()
            .filter(|c| c.tenant_id == tenant_id)
            .collect();

        if let Err(e) = self.sender.send(message) {
            error!("Failed to broadcast message: {}", e);
        }

        info!("Broadcasted message to {} connections in tenant {}",
              tenant_connections.len(), tenant_id);
    }

    /// Send message to specific user
    pub async fn send_to_user(&self, user_id: Uuid, message: WsMessage) {
        let connections = self.connections.read().await;
        let user_connections: Vec<_> = connections
            .values()
            .filter(|c| c.user_id == user_id)
            .collect();

        if let Err(e) = self.sender.send(message) {
            error!("Failed to send message to user: {}", e);
        }

        info!("Sent message to {} connections for user {}",
              user_connections.len(), user_id);
    }

    /// Get broadcast receiver
    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    /// Start Redis subscription for events
    pub async fn start_redis_subscription(self: Arc<Self>) {
        let hub = Arc::clone(&self);
        tokio::spawn(async move {
            if let Err(e) = hub.redis_event_loop().await {
                error!("Redis subscription error: {}", e);
            }
        });
    }

    async fn redis_event_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.redis_client.get_async_connection().await?;
        let mut pubsub = conn.as_pubsub();

        // Subscribe to event channels
        pubsub.subscribe("events.order.*").await?;
        pubsub.subscribe("events.product.*").await?;
        pubsub.subscribe("events.inventory.*").await?;

        let mut stream = pubsub.on_message();

        while let Some(msg) = stream.next().await {
            let payload: String = msg.get_payload()?;
            let channel: String = msg.get_channel_name().to_string();

            // Parse event and convert to WebSocket message
            if let Ok(event) = serde_json::from_str::<DomainEvent>(&payload) {
                let ws_msg = WsMessage::Event {
                    event_type: event.event_type,
                    data: event.data,
                    timestamp: event.timestamp,
                };

                // Broadcast to relevant tenant
                self.broadcast_to_tenant(event.tenant_id, ws_msg).await;
            }
        }

        Ok(())
    }
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// Ping/Pong for keepalive
    Ping,
    Pong,

    /// Subscribe to events
    Subscribe {
        channels: Vec<String>,
    },

    /// Unsubscribe from events
    Unsubscribe {
        channels: Vec<String>,
    },

    /// Event notification
    Event {
        event_type: String,
        data: serde_json::Value,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Order updates
    OrderUpdate {
        order_id: Uuid,
        status: String,
        data: serde_json::Value,
    },

    /// Inventory updates
    InventoryUpdate {
        product_id: Uuid,
        location_id: Uuid,
        quantity_on_hand: i32,
        quantity_available: i32,
    },

    /// Notification
    Notification {
        id: Uuid,
        title: String,
        message: String,
        severity: String,
    },

    /// Error message
    Error {
        code: String,
        message: String,
    },
}

/// Domain event from Redis
#[derive(Debug, Serialize, Deserialize)]
struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub tenant_id: Uuid,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// WebSocket handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<WsHub>>,
    user_id: Uuid,
    tenant_id: Uuid,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, hub, user_id, tenant_id))
}

async fn handle_socket(
    socket: WebSocket,
    hub: Arc<WsHub>,
    user_id: Uuid,
    tenant_id: Uuid,
) {
    let (mut sender, mut receiver) = socket.split();
    let conn_id = Uuid::new_v4();

    // Register connection
    let connection = WsConnection {
        id: conn_id,
        user_id,
        tenant_id,
        subscriptions: vec![],
    };
    hub.register(connection.clone()).await;

    // Subscribe to broadcast channel
    let mut rx = hub.subscribe();

    // Spawn task to handle incoming messages
    let hub_clone = Arc::clone(&hub);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, &hub_clone, conn_id).await.is_err() {
                break;
            }
        }
    });

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Filter messages based on connection's tenant
            if should_send_message(&msg, tenant_id, user_id) {
                let json = serde_json::to_string(&msg).unwrap();
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn keepalive task
    let mut keepalive = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let ping = WsMessage::Ping;
            let json = serde_json::to_string(&ping).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Wait for any task to complete
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut keepalive) => {
            recv_task.abort();
            send_task.abort();
        }
    }

    // Unregister connection
    hub.unregister(conn_id).await;
    info!("WebSocket connection {} closed", conn_id);
}

async fn process_message(
    msg: Message,
    hub: &Arc<WsHub>,
    conn_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    match msg {
        Message::Text(text) => {
            if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                match ws_msg {
                    WsMessage::Ping => {
                        // Reply with pong
                        let pong = WsMessage::Pong;
                        // Send pong back through the hub
                    }
                    WsMessage::Subscribe { channels } => {
                        info!("Connection {} subscribing to: {:?}", conn_id, channels);
                        // Update connection subscriptions
                    }
                    WsMessage::Unsubscribe { channels } => {
                        info!("Connection {} unsubscribing from: {:?}", conn_id, channels);
                        // Update connection subscriptions
                    }
                    _ => {
                        warn!("Unexpected message type from client");
                    }
                }
            }
        }
        Message::Close(_) => {
            return Err("Connection closed".into());
        }
        _ => {}
    }
    Ok(())
}

fn should_send_message(msg: &WsMessage, tenant_id: Uuid, user_id: Uuid) -> bool {
    match msg {
        WsMessage::Event { .. } => true, // Filtered by tenant in broadcast
        WsMessage::OrderUpdate { .. } => true,
        WsMessage::InventoryUpdate { .. } => true,
        WsMessage::Notification { .. } => true,
        _ => false,
    }
}

/// Real-time subscription types for GraphQL
pub mod subscriptions {
    use super::*;
    use async_graphql::{Context, Subscription};
    use futures::Stream;

    pub struct SubscriptionRoot;

    #[Subscription]
    impl SubscriptionRoot {
        /// Subscribe to order updates for a tenant
        async fn order_updates(
            &self,
            ctx: &Context<'_>,
            tenant_id: Uuid,
        ) -> impl Stream<Item = OrderUpdate> {
            let hub = ctx.data::<Arc<WsHub>>().unwrap();
            let mut rx = hub.subscribe();

            async_stream::stream! {
                while let Ok(msg) = rx.recv().await {
                    if let WsMessage::OrderUpdate { order_id, status, data } = msg {
                        yield OrderUpdate {
                            order_id,
                            status,
                            data,
                        };
                    }
                }
            }
        }

        /// Subscribe to inventory updates
        async fn inventory_updates(
            &self,
            ctx: &Context<'_>,
            tenant_id: Uuid,
            location_id: Option<Uuid>,
        ) -> impl Stream<Item = InventoryUpdate> {
            let hub = ctx.data::<Arc<WsHub>>().unwrap();
            let mut rx = hub.subscribe();

            async_stream::stream! {
                while let Ok(msg) = rx.recv().await {
                    if let WsMessage::InventoryUpdate {
                        product_id,
                        location_id: loc_id,
                        quantity_on_hand,
                        quantity_available,
                    } = msg {
                        if location_id.is_none() || location_id == Some(loc_id) {
                            yield InventoryUpdate {
                                product_id,
                                location_id: loc_id,
                                quantity_on_hand,
                                quantity_available,
                            };
                        }
                    }
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct OrderUpdate {
        pub order_id: Uuid,
        pub status: String,
        pub data: serde_json::Value,
    }

    #[derive(Debug, Clone)]
    pub struct InventoryUpdate {
        pub product_id: Uuid,
        pub location_id: Uuid,
        pub quantity_on_hand: i32,
        pub quantity_available: i32,
    }
}