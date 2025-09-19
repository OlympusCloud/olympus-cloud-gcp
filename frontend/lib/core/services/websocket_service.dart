import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:web_socket_channel/io.dart';
import 'dart:convert';
import 'dart:async';
import '../constants/app_constants.dart';
import 'storage_service.dart';

/// Service for managing WebSocket connections for real-time features
class WebSocketService {
  static WebSocketChannel? _channel;
  static StreamSubscription? _subscription;
  static Timer? _heartbeatTimer;
  static Timer? _reconnectTimer;
  static bool _isConnected = false;
  static bool _shouldReconnect = true;
  static int _reconnectAttempts = 0;
  static const int _maxReconnectAttempts = 5;
  static const Duration _heartbeatInterval = Duration(seconds: 30);
  static const Duration _reconnectDelay = Duration(seconds: 5);

  // Stream controllers for different message types
  static final StreamController<Map<String, dynamic>> _orderUpdatesController =
      StreamController<Map<String, dynamic>>.broadcast();
  static final StreamController<Map<String, dynamic>> _inventoryUpdatesController =
      StreamController<Map<String, dynamic>>.broadcast();
  static final StreamController<Map<String, dynamic>> _notificationsController =
      StreamController<Map<String, dynamic>>.broadcast();
  static final StreamController<ConnectionStatus> _connectionStatusController =
      StreamController<ConnectionStatus>.broadcast();

  /// Initialize WebSocket connection
  static Future<void> connect() async {
    if (_isConnected) return;

    try {
      final token = StorageService.getUserData<String>(AppConstants.accessTokenKey);
      if (token == null || token.isEmpty) {
        throw Exception('No authentication token available');
      }

      final wsUrl = AppConstants.websocketUrl;
      final uri = Uri.parse('$wsUrl?token=$token');

      _channel = IOWebSocketChannel.connect(uri);
      _isConnected = true;
      _reconnectAttempts = 0;
      
      _connectionStatusController.add(ConnectionStatus.connected);
      
      // Listen to incoming messages
      _subscription = _channel!.stream.listen(
        _handleMessage,
        onError: _handleError,
        onDone: _handleDisconnection,
      );

      // Start heartbeat
      _startHeartbeat();
    } catch (e) {
      _isConnected = false;
      _connectionStatusController.add(ConnectionStatus.disconnected);
      _scheduleReconnect();
    }
  }

  /// Disconnect WebSocket
  static Future<void> disconnect() async {
    _shouldReconnect = false;
    _stopHeartbeat();
    _stopReconnectTimer();
    
    await _subscription?.cancel();
    await _channel?.sink.close();
    
    _isConnected = false;
    _connectionStatusController.add(ConnectionStatus.disconnected);
  }

  /// Send a message through WebSocket
  static void sendMessage(Map<String, dynamic> message) {
    if (_isConnected && _channel != null) {
      final jsonMessage = json.encode(message);
      _channel!.sink.add(jsonMessage);
    }
  }

  /// Handle incoming messages
  static void _handleMessage(dynamic data) {
    try {
      final Map<String, dynamic> message = json.decode(data);
      final String type = message['type'] ?? '';

      switch (type) {
        case 'order_update':
          _orderUpdatesController.add(message['data']);
          break;
        case 'inventory_update':
          _inventoryUpdatesController.add(message['data']);
          break;
        case 'notification':
          _notificationsController.add(message['data']);
          break;
        case 'pong':
          // Heartbeat response - connection is alive
          break;
        default:
          // print('Unknown message type: $type');
      }
    } catch (e) {
      // print('Error parsing WebSocket message: $e');
    }
  }

  /// Handle WebSocket errors
  static void _handleError(dynamic error) {
    // print('WebSocket error: $error');
    _isConnected = false;
    _connectionStatusController.add(ConnectionStatus.error);
    _scheduleReconnect();
  }

  /// Handle WebSocket disconnection
  static void _handleDisconnection() {
    // print('WebSocket disconnected');
    _isConnected = false;
    _connectionStatusController.add(ConnectionStatus.disconnected);
    _stopHeartbeat();
    
    if (_shouldReconnect) {
      _scheduleReconnect();
    }
  }

  /// Start heartbeat to keep connection alive
  static void _startHeartbeat() {
    _heartbeatTimer = Timer.periodic(_heartbeatInterval, (timer) {
      if (_isConnected) {
        sendMessage({'type': 'ping'});
      }
    });
  }

  /// Stop heartbeat timer
  static void _stopHeartbeat() {
    _heartbeatTimer?.cancel();
    _heartbeatTimer = null;
  }

  /// Schedule reconnection attempt
  static void _scheduleReconnect() {
    if (!_shouldReconnect || _reconnectAttempts >= _maxReconnectAttempts) {
      return;
    }

    _reconnectAttempts++;
    _connectionStatusController.add(ConnectionStatus.reconnecting);
    
    _reconnectTimer = Timer(_reconnectDelay, () {
      // print('Attempting to reconnect... (attempt $_reconnectAttempts)');
      connect();
    });
  }

  /// Stop reconnect timer
  static void _stopReconnectTimer() {
    _reconnectTimer?.cancel();
    _reconnectTimer = null;
  }

  /// Subscribe to order updates
  static Stream<Map<String, dynamic>> get orderUpdates => _orderUpdatesController.stream;

  /// Subscribe to inventory updates
  static Stream<Map<String, dynamic>> get inventoryUpdates => _inventoryUpdatesController.stream;

  /// Subscribe to notifications
  static Stream<Map<String, dynamic>> get notifications => _notificationsController.stream;

  /// Subscribe to connection status changes
  static Stream<ConnectionStatus> get connectionStatus => _connectionStatusController.stream;

  /// Check if WebSocket is connected
  static bool get isConnected => _isConnected;

  /// Subscribe to specific order updates
  static void subscribeToOrder(String orderId) {
    sendMessage({
      'type': 'subscribe',
      'channel': 'order',
      'id': orderId,
    });
  }

  /// Unsubscribe from specific order updates
  static void unsubscribeFromOrder(String orderId) {
    sendMessage({
      'type': 'unsubscribe',
      'channel': 'order',
      'id': orderId,
    });
  }

  /// Subscribe to inventory updates for a location
  static void subscribeToInventory(String locationId) {
    sendMessage({
      'type': 'subscribe',
      'channel': 'inventory',
      'location_id': locationId,
    });
  }

  /// Subscribe to user notifications
  static void subscribeToNotifications(String userId) {
    sendMessage({
      'type': 'subscribe',
      'channel': 'notifications',
      'user_id': userId,
    });
  }

  /// Dispose all resources
  static Future<void> dispose() async {
    await disconnect();
    await _orderUpdatesController.close();
    await _inventoryUpdatesController.close();
    await _notificationsController.close();
    await _connectionStatusController.close();
  }
}

/// WebSocket connection status
enum ConnectionStatus {
  connected,
  disconnected,
  reconnecting,
  error,
}

/// Extension to get human-readable status
extension ConnectionStatusExtension on ConnectionStatus {
  String get displayName {
    switch (this) {
      case ConnectionStatus.connected:
        return 'Connected';
      case ConnectionStatus.disconnected:
        return 'Disconnected';
      case ConnectionStatus.reconnecting:
        return 'Reconnecting...';
      case ConnectionStatus.error:
        return 'Connection Error';
    }
  }

  bool get isConnected => this == ConnectionStatus.connected;
}