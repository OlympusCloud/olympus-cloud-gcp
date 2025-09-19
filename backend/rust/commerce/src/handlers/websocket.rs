// ============================================================================
// OLYMPUS CLOUD - WEBSOCKET HANDLERS FOR REAL-TIME UPDATES
// ============================================================================
// Module: commerce/src/handlers/websocket.rs
// Description: WebSocket handlers for real-time restaurant updates
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::models::restaurant::*;
use crate::services::RestaurantService;
use olympus_shared::integration::AuthContext;

/// WebSocket message types for restaurant updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RestaurantMessage {
    // Connection management
    Subscribe { location_id: Uuid, client_type: ClientType },
    Unsubscribe,
    Ping,
    Pong,

    // Real-time updates
    OrderUpdate(OrderUpdate),
    TableUpdate(TableUpdate),
    KitchenUpdate(KitchenUpdate),
    DashboardUpdate(DashboardUpdate),

    // Error handling
    Error { message: String },
    InvalidMessage { reason: String },
}

/// Type of WebSocket client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientType {
    Dashboard,   // Management dashboard
    Kitchen,     // Kitchen display system
    Server,      // Server/waiter terminals
    Customer,    // Customer-facing displays
}

/// Real-time order update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub order_id: Uuid,
    pub table_id: Option<Uuid>,
    pub status: RestaurantOrderStatus,
    pub kitchen_items: Vec<KitchenDisplayItem>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub update_type: OrderUpdateType,
}

/// Real-time table update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableUpdate {
    pub table_id: Uuid,
    pub table_number: String,
    pub status: TableStatus,
    pub server_id: Option<Uuid>,
    pub current_order_id: Option<Uuid>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Real-time kitchen update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KitchenUpdate {
    pub item_id: Uuid,
    pub order_id: Uuid,
    pub order_number: String,
    pub table_number: Option<String>,
    pub status: KitchenStatus,
    pub estimated_ready_time: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Real-time dashboard metrics update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUpdate {
    pub metrics: RestaurantDashboard,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// WebSocket connection manager for restaurant updates
#[derive(Clone)]
pub struct RestaurantWebSocketManager {
    // Broadcast channel for real-time updates
    order_sender: broadcast::Sender<OrderUpdate>,
    table_sender: broadcast::Sender<TableUpdate>,
    kitchen_sender: broadcast::Sender<KitchenUpdate>,
    dashboard_sender: broadcast::Sender<DashboardUpdate>,
}

impl RestaurantWebSocketManager {
    pub fn new() -> Self {
        let (order_sender, _) = broadcast::channel(1000);
        let (table_sender, _) = broadcast::channel(1000);
        let (kitchen_sender, _) = broadcast::channel(1000);
        let (dashboard_sender, _) = broadcast::channel(1000);

        Self {
            order_sender,
            table_sender,
            kitchen_sender,
            dashboard_sender,
        }
    }

    /// Broadcast order update to all subscribers
    pub async fn broadcast_order_update(&self, update: OrderUpdate) {
        if let Err(e) = self.order_sender.send(update) {
            warn!("Failed to broadcast order update: {}", e);
        }
    }

    /// Broadcast table update to all subscribers
    pub async fn broadcast_table_update(&self, update: TableUpdate) {
        if let Err(e) = self.table_sender.send(update) {
            warn!("Failed to broadcast table update: {}", e);
        }
    }

    /// Broadcast kitchen update to all subscribers
    pub async fn broadcast_kitchen_update(&self, update: KitchenUpdate) {
        if let Err(e) = self.kitchen_sender.send(update) {
            warn!("Failed to broadcast kitchen update: {}", e);
        }
    }

    /// Broadcast dashboard update to all subscribers
    pub async fn broadcast_dashboard_update(&self, update: DashboardUpdate) {
        if let Err(e) = self.dashboard_sender.send(update) {
            warn!("Failed to broadcast dashboard update: {}", e);
        }
    }
}

/// WebSocket route handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
    State((service, ws_manager)): State<(RestaurantService, RestaurantWebSocketManager)>,
) -> Response {
    let location_id = match params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
    {
        Some(id) => id,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Missing location_id parameter".into())
                .unwrap();
        }
    };

    ws.on_upgrade(move |socket| {
        handle_websocket(socket, tenant_id, location_id, service, ws_manager)
    })
}

/// Handle WebSocket connection
async fn handle_websocket(
    mut socket: WebSocket,
    tenant_id: Uuid,
    location_id: Uuid,
    service: RestaurantService,
    ws_manager: RestaurantWebSocketManager,
) {
    info!("WebSocket connection established for tenant {} location {}", tenant_id, location_id);

    let mut client_type: Option<ClientType> = None;
    let mut subscribed = false;

    // Subscribe to broadcast channels
    let mut order_receiver = ws_manager.order_sender.subscribe();
    let mut table_receiver = ws_manager.table_sender.subscribe();
    let mut kitchen_receiver = ws_manager.kitchen_sender.subscribe();
    let mut dashboard_receiver = ws_manager.dashboard_sender.subscribe();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<RestaurantMessage>(&text) {
                            Ok(restaurant_msg) => {
                                if let Err(e) = handle_restaurant_message(
                                    &mut socket,
                                    restaurant_msg,
                                    &mut client_type,
                                    &mut subscribed,
                                    location_id,
                                ).await {
                                    error!("Error handling message: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                warn!("Invalid message format: {}", e);
                                let error_msg = RestaurantMessage::InvalidMessage {
                                    reason: format!("Invalid JSON: {}", e),
                                };
                                if let Err(_) = send_message(&mut socket, error_msg).await {
                                    break;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    None => break,
                    _ => {} // Ignore other message types
                }
            }

            // Handle broadcast updates based on client type
            order_update = order_receiver.recv(), if subscribed && matches!(client_type, Some(ClientType::Dashboard | ClientType::Server)) => {
                if let Ok(update) = order_update {
                    let msg = RestaurantMessage::OrderUpdate(update);
                    if let Err(_) = send_message(&mut socket, msg).await {
                        break;
                    }
                }
            }

            table_update = table_receiver.recv(), if subscribed && matches!(client_type, Some(ClientType::Dashboard | ClientType::Server)) => {
                if let Ok(update) = table_update {
                    let msg = RestaurantMessage::TableUpdate(update);
                    if let Err(_) = send_message(&mut socket, msg).await {
                        break;
                    }
                }
            }

            kitchen_update = kitchen_receiver.recv(), if subscribed && matches!(client_type, Some(ClientType::Kitchen)) => {
                if let Ok(update) = kitchen_update {
                    let msg = RestaurantMessage::KitchenUpdate(update);
                    if let Err(_) = send_message(&mut socket, msg).await {
                        break;
                    }
                }
            }

            dashboard_update = dashboard_receiver.recv(), if subscribed && matches!(client_type, Some(ClientType::Dashboard)) => {
                if let Ok(update) = dashboard_update {
                    let msg = RestaurantMessage::DashboardUpdate(update);
                    if let Err(_) = send_message(&mut socket, msg).await {
                        break;
                    }
                }
            }
        }
    }

    info!("WebSocket connection ended for tenant {} location {}", tenant_id, location_id);
}

/// Handle individual restaurant messages
async fn handle_restaurant_message(
    socket: &mut WebSocket,
    message: RestaurantMessage,
    client_type: &mut Option<ClientType>,
    subscribed: &mut bool,
    location_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match message {
        RestaurantMessage::Subscribe { location_id: sub_location_id, client_type: sub_client_type } => {
            if sub_location_id == location_id {
                *client_type = Some(sub_client_type.clone());
                *subscribed = true;
                info!("Client subscribed as {:?} for location {}", sub_client_type, location_id);

                // Send confirmation
                let response = RestaurantMessage::Pong;
                send_message(socket, response).await?;
            } else {
                let error_msg = RestaurantMessage::Error {
                    message: "Location ID mismatch".to_string(),
                };
                send_message(socket, error_msg).await?;
            }
        }

        RestaurantMessage::Unsubscribe => {
            *subscribed = false;
            *client_type = None;
            info!("Client unsubscribed from location {}", location_id);
        }

        RestaurantMessage::Ping => {
            let response = RestaurantMessage::Pong;
            send_message(socket, response).await?;
        }

        _ => {
            let error_msg = RestaurantMessage::Error {
                message: "Invalid message type for client".to_string(),
            };
            send_message(socket, error_msg).await?;
        }
    }

    Ok(())
}

/// Send a message through WebSocket
async fn send_message(
    socket: &mut WebSocket,
    message: RestaurantMessage,
) -> Result<(), axum::Error> {
    let text = serde_json::to_string(&message)
        .map_err(|e| axum::Error::new(format!("Serialization error: {}", e)))?;

    socket.send(Message::Text(text)).await
}

// Helper function to create WebSocket manager
pub fn create_websocket_manager() -> RestaurantWebSocketManager {
    RestaurantWebSocketManager::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_manager_creation() {
        let manager = RestaurantWebSocketManager::new();
        // Basic test to ensure the manager can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_message_serialization() {
        let message = RestaurantMessage::Ping;
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: RestaurantMessage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            RestaurantMessage::Ping => assert!(true),
            _ => panic!("Message serialization failed"),
        }
    }
}