import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import '../models/watch_device.dart';
import '../models/watch_notification.dart';
import '../models/watch_complication.dart';

/// Service for managing watch device connections and communication
class WatchService {
  static const _channel = MethodChannel('com.olympus.watch');
  
  static WatchService? _instance;
  static WatchService get instance => _instance ??= WatchService._();
  
  WatchService._() {
    _initialize();
  }

  final StreamController<List<WatchDevice>> _devicesController = 
      StreamController<List<WatchDevice>>.broadcast();
  final StreamController<WatchNotification> _notificationController = 
      StreamController<WatchNotification>.broadcast();
  final StreamController<WatchComplication> _complicationController = 
      StreamController<WatchComplication>.broadcast();

  List<WatchDevice> _connectedDevices = [];
  bool _isInitialized = false;

  // Streams
  Stream<List<WatchDevice>> get devicesStream => _devicesController.stream;
  Stream<WatchNotification> get notificationStream => _notificationController.stream;
  Stream<WatchComplication> get complicationStream => _complicationController.stream;

  // Getters
  List<WatchDevice> get connectedDevices => List.from(_connectedDevices);
  bool get hasConnectedDevices => _connectedDevices.isNotEmpty;
  bool get isInitialized => _isInitialized;

  /// Initialize the watch service
  Future<void> _initialize() async {
    try {
      _channel.setMethodCallHandler(_handleMethodCall);
      await _channel.invokeMethod('initialize');
      await _scanForDevices();
      _isInitialized = true;
    } catch (e) {
      debugPrint('Failed to initialize watch service: $e');
    }
  }

  /// Handle method calls from native platform
  Future<dynamic> _handleMethodCall(MethodCall call) async {
    switch (call.method) {
      case 'onDeviceConnected':
        _handleDeviceConnected(call.arguments);
        break;
      case 'onDeviceDisconnected':
        _handleDeviceDisconnected(call.arguments);
        break;
      case 'onNotificationDelivered':
        _handleNotificationDelivered(call.arguments);
        break;
      case 'onNotificationInteraction':
        _handleNotificationInteraction(call.arguments);
        break;
      case 'onComplicationUpdated':
        _handleComplicationUpdated(call.arguments);
        break;
      default:
        debugPrint('Unknown method call: ${call.method}');
    }
  }

  /// Scan for available watch devices
  Future<List<WatchDevice>> scanForDevices() async {
    try {
      final result = await _channel.invokeMethod('scanForDevices');
      final devices = (result as List)
          .map((json) => WatchDevice.fromJson(Map<String, dynamic>.from(json)))
          .toList();
      
      _connectedDevices = devices.where((d) => d.status.isConnected).toList();
      _devicesController.add(_connectedDevices);
      
      return devices;
    } catch (e) {
      debugPrint('Failed to scan for devices: $e');
      return [];
    }
  }

  /// Connect to a specific watch device
  Future<bool> connectToDevice(String deviceId) async {
    try {
      final success = await _channel.invokeMethod('connectToDevice', {
        'deviceId': deviceId,
      });
      
      if (success == true) {
        await _scanForDevices(); // Refresh device list
      }
      
      return success == true;
    } catch (e) {
      debugPrint('Failed to connect to device: $e');
      return false;
    }
  }

  /// Disconnect from a watch device
  Future<bool> disconnectFromDevice(String deviceId) async {
    try {
      final success = await _channel.invokeMethod('disconnectFromDevice', {
        'deviceId': deviceId,
      });
      
      if (success == true) {
        _connectedDevices.removeWhere((d) => d.id == deviceId);
        _devicesController.add(_connectedDevices);
      }
      
      return success == true;
    } catch (e) {
      debugPrint('Failed to disconnect from device: $e');
      return false;
    }
  }

  /// Send notification to connected watch devices
  Future<bool> sendNotification(WatchNotification notification) async {
    if (!hasConnectedDevices) {
      debugPrint('No connected watch devices to send notification');
      return false;
    }

    try {
      final success = await _channel.invokeMethod('sendNotification', {
        'notification': notification.toJson(),
        'deviceIds': _connectedDevices.map((d) => d.id).toList(),
      });
      
      return success == true;
    } catch (e) {
      debugPrint('Failed to send notification: $e');
      return false;
    }
  }

  /// Update complication data on connected watch devices
  Future<bool> updateComplication(WatchComplication complication) async {
    if (!hasConnectedDevices) {
      debugPrint('No connected watch devices to update complication');
      return false;
    }

    try {
      final success = await _channel.invokeMethod('updateComplication', {
        'complication': complication.toJson(),
        'deviceIds': _connectedDevices.map((d) => d.id).toList(),
      });
      
      return success == true;
    } catch (e) {
      debugPrint('Failed to update complication: $e');
      return false;
    }
  }

  /// Get device capabilities for a specific device
  Future<WatchDeviceCapabilities?> getDeviceCapabilities(String deviceId) async {
    try {
      final result = await _channel.invokeMethod('getDeviceCapabilities', {
        'deviceId': deviceId,
      });
      
      if (result != null) {
        return WatchDeviceCapabilities.fromJson(Map<String, dynamic>.from(result));
      }
      
      return null;
    } catch (e) {
      debugPrint('Failed to get device capabilities: $e');
      return null;
    }
  }

  /// Start a complication update session for real-time data
  Future<bool> startComplicationSession(List<WatchComplicationType> types) async {
    try {
      final success = await _channel.invokeMethod('startComplicationSession', {
        'types': types.map((t) => t.name).toList(),
      });
      
      return success == true;
    } catch (e) {
      debugPrint('Failed to start complication session: $e');
      return false;
    }
  }

  /// Stop the complication update session
  Future<bool> stopComplicationSession() async {
    try {
      final success = await _channel.invokeMethod('stopComplicationSession');
      return success == true;
    } catch (e) {
      debugPrint('Failed to stop complication session: $e');
      return false;
    }
  }

  /// Check if watch integration is available on this platform
  Future<bool> isAvailable() async {
    try {
      final available = await _channel.invokeMethod('isAvailable');
      return available == true;
    } catch (e) {
      debugPrint('Watch integration not available: $e');
      return false;
    }
  }

  // Private event handlers
  void _handleDeviceConnected(dynamic arguments) {
    try {
      final device = WatchDevice.fromJson(Map<String, dynamic>.from(arguments));
      final existingIndex = _connectedDevices.indexWhere((d) => d.id == device.id);
      
      if (existingIndex >= 0) {
        _connectedDevices[existingIndex] = device;
      } else {
        _connectedDevices.add(device);
      }
      
      _devicesController.add(_connectedDevices);
    } catch (e) {
      debugPrint('Failed to handle device connected: $e');
    }
  }

  void _handleDeviceDisconnected(dynamic arguments) {
    try {
      final deviceId = arguments['deviceId'] as String;
      _connectedDevices.removeWhere((d) => d.id == deviceId);
      _devicesController.add(_connectedDevices);
    } catch (e) {
      debugPrint('Failed to handle device disconnected: $e');
    }
  }

  void _handleNotificationDelivered(dynamic arguments) {
    try {
      final notification = WatchNotification.fromJson(Map<String, dynamic>.from(arguments));
      _notificationController.add(notification);
    } catch (e) {
      debugPrint('Failed to handle notification delivered: $e');
    }
  }

  void _handleNotificationInteraction(dynamic arguments) {
    try {
      final notification = WatchNotification.fromJson(Map<String, dynamic>.from(arguments));
      _notificationController.add(notification);
    } catch (e) {
      debugPrint('Failed to handle notification interaction: $e');
    }
  }

  void _handleComplicationUpdated(dynamic arguments) {
    try {
      final complication = WatchComplication.fromJson(Map<String, dynamic>.from(arguments));
      _complicationController.add(complication);
    } catch (e) {
      debugPrint('Failed to handle complication updated: $e');
    }
  }

  /// Dispose resources
  void dispose() {
    _devicesController.close();
    _notificationController.close();
    _complicationController.close();
  }
}