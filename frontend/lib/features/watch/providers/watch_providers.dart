import 'package:riverpod_annotation/riverpod_annotation.dart';
import '../models/watch_device.dart';
import '../models/watch_notification.dart';
import '../models/watch_complication.dart';
import '../services/watch_service.dart';
import '../services/watch_notification_service.dart';

part 'watch_providers.g.dart';

/// Provider for the WatchService instance
@riverpod
WatchService watchService(WatchServiceRef ref) {
  return WatchService.instance;
}

/// Provider for the WatchNotificationService instance
@riverpod
WatchNotificationService watchNotificationService(WatchNotificationServiceRef ref) {
  return WatchNotificationService.instance;
}

/// Provider for connected watch devices stream
@riverpod
Stream<List<WatchDevice>> connectedWatchDevices(ConnectedWatchDevicesRef ref) {
  final watchService = ref.watch(watchServiceProvider);
  return watchService.devicesStream;
}

/// Provider for watch notifications stream
@riverpod
Stream<WatchNotification> watchNotifications(WatchNotificationsRef ref) {
  final watchService = ref.watch(watchServiceProvider);
  return watchService.notificationStream;
}

/// Provider for watch complications stream
@riverpod
Stream<WatchComplication> watchComplications(WatchComplicationsRef ref) {
  final watchService = ref.watch(watchServiceProvider);
  return watchService.complicationStream;
}

/// Provider for watch availability
@riverpod
Future<bool> watchAvailability(WatchAvailabilityRef ref) async {
  final watchService = ref.watch(watchServiceProvider);
  return await watchService.isAvailable();
}

/// Provider for device capabilities
@riverpod
Future<WatchDeviceCapabilities?> watchDeviceCapabilities(
  WatchDeviceCapabilitiesRef ref,
  String deviceId,
) async {
  final watchService = ref.watch(watchServiceProvider);
  return await watchService.getDeviceCapabilities(deviceId);
}

/// State notifier for managing watch devices
@riverpod
class WatchDevicesNotifier extends _$WatchDevicesNotifier {
  @override
  Future<List<WatchDevice>> build() async {
    final watchService = ref.watch(watchServiceProvider);
    
    // Listen to device stream updates
    ref.listen(connectedWatchDevicesProvider, (previous, next) {
      next.whenData((devices) {
        if (state.hasValue && state.value != devices) {
          state = AsyncValue.data(devices);
        }
      });
    });
    
    return await watchService.scanForDevices();
  }

  /// Scan for available devices
  Future<void> scanForDevices() async {
    state = const AsyncValue.loading();
    
    try {
      final watchService = ref.read(watchServiceProvider);
      final devices = await watchService.scanForDevices();
      state = AsyncValue.data(devices);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Connect to a device
  Future<bool> connectToDevice(String deviceId) async {
    try {
      final watchService = ref.read(watchServiceProvider);
      final success = await watchService.connectToDevice(deviceId);
      
      if (success) {
        await scanForDevices(); // Refresh device list
      }
      
      return success;
    } catch (e) {
      return false;
    }
  }

  /// Disconnect from a device
  Future<bool> disconnectFromDevice(String deviceId) async {
    try {
      final watchService = ref.read(watchServiceProvider);
      final success = await watchService.disconnectFromDevice(deviceId);
      
      if (success) {
        await scanForDevices(); // Refresh device list
      }
      
      return success;
    } catch (e) {
      return false;
    }
  }
}

/// State notifier for managing watch complications
@riverpod
class WatchComplicationsNotifier extends _$WatchComplicationsNotifier {
  @override
  Map<WatchComplicationType, WatchComplication> build() {
    // Listen to complication stream updates
    ref.listen(watchComplicationsProvider, (previous, next) {
      next.whenData((complication) {
        state = {
          ...state,
          complication.type: complication,
        };
      });
    });
    
    return {};
  }

  /// Update a specific complication
  Future<bool> updateComplication(WatchComplication complication) async {
    try {
      final watchService = ref.read(watchServiceProvider);
      final success = await watchService.updateComplication(complication);
      
      if (success) {
        state = {
          ...state,
          complication.type: complication.copyWith(
            lastUpdated: DateTime.now(),
          ),
        };
      }
      
      return success;
    } catch (e) {
      return false;
    }
  }

  /// Update daily sales complication
  Future<bool> updateDailySales(double amount, {WatchComplicationTrend? trend}) async {
    final complication = WatchComplication(
      id: 'daily_sales',
      title: 'Daily Sales',
      type: WatchComplicationType.dailySales,
      value: '\$${amount.toStringAsFixed(0)}',
      unit: '',
      trend: trend,
      lastUpdated: DateTime.now(),
    );
    
    return await updateComplication(complication);
  }

  /// Update order count complication
  Future<bool> updateOrderCount(int count, {WatchComplicationTrend? trend}) async {
    final complication = WatchComplication(
      id: 'order_count',
      title: 'Orders Today',
      type: WatchComplicationType.orderCount,
      value: count.toString(),
      trend: trend,
      lastUpdated: DateTime.now(),
    );
    
    return await updateComplication(complication);
  }

  /// Update current customers complication
  Future<bool> updateCurrentCustomers(int count) async {
    final complication = WatchComplication(
      id: 'current_customers',
      title: 'Current Customers',
      type: WatchComplicationType.currentCustomers,
      value: count.toString(),
      lastUpdated: DateTime.now(),
      refreshIntervalSeconds: 60, // Update every minute
    );
    
    return await updateComplication(complication);
  }

  /// Update inventory alerts complication
  Future<bool> updateInventoryAlerts(int alertCount) async {
    final complication = WatchComplication(
      id: 'inventory_alerts',
      title: 'Inventory Alerts',
      type: WatchComplicationType.inventoryAlerts,
      value: alertCount > 0 ? alertCount.toString() : '✓',
      lastUpdated: DateTime.now(),
    );
    
    return await updateComplication(complication);
  }

  /// Update staff status complication
  Future<bool> updateStaffStatus(int activeStaff, int totalStaff) async {
    final complication = WatchComplication(
      id: 'staff_status',
      title: 'Staff Status',
      type: WatchComplicationType.staffStatus,
      value: '$activeStaff/$totalStaff',
      lastUpdated: DateTime.now(),
    );
    
    return await updateComplication(complication);
  }

  /// Update next appointment complication
  Future<bool> updateNextAppointment(DateTime? nextAppointment) async {
    String value = 'None';
    
    if (nextAppointment != null) {
      final now = DateTime.now();
      final difference = nextAppointment.difference(now);
      
      if (difference.inMinutes < 60) {
        value = '${difference.inMinutes}m';
      } else if (difference.inHours < 24) {
        value = '${difference.inHours}h';
      } else {
        value = '${difference.inDays}d';
      }
    }
    
    final complication = WatchComplication(
      id: 'next_appointment',
      title: 'Next Appointment',
      type: WatchComplicationType.nextAppointment,
      value: value,
      lastUpdated: DateTime.now(),
      refreshIntervalSeconds: 30, // Update every 30 seconds
    );
    
    return await updateComplication(complication);
  }

  /// Start complication session for real-time updates
  Future<bool> startComplicationSession(List<WatchComplicationType> types) async {
    try {
      final watchService = ref.read(watchServiceProvider);
      return await watchService.startComplicationSession(types);
    } catch (e) {
      return false;
    }
  }

  /// Stop complication session
  Future<bool> stopComplicationSession() async {
    try {
      final watchService = ref.read(watchServiceProvider);
      return await watchService.stopComplicationSession();
    } catch (e) {
      return false;
    }
  }
}

/// State notifier for managing watch notifications
@riverpod
class WatchNotificationsNotifier extends _$WatchNotificationsNotifier {
  @override
  List<WatchNotification> build() {
    // Listen to notification stream updates
    ref.listen(watchNotificationsProvider, (previous, next) {
      next.whenData((notification) {
        state = [...state, notification];
      });
    });
    
    return [];
  }

  /// Send order notification
  Future<bool> sendOrderNotification({
    required String orderId,
    required String customerName,
    required double amount,
    String? specialInstructions,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendOrderNotification(
      orderId: orderId,
      customerName: customerName,
      amount: amount,
      specialInstructions: specialInstructions,
    );
  }

  /// Send payment notification
  Future<bool> sendPaymentNotification({
    required String orderId,
    required double amount,
    required String paymentMethod,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendPaymentNotification(
      orderId: orderId,
      amount: amount,
      paymentMethod: paymentMethod,
    );
  }

  /// Send inventory alert
  Future<bool> sendInventoryAlert({
    required String itemName,
    required int currentStock,
    required int minimumStock,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendInventoryAlert(
      itemName: itemName,
      currentStock: currentStock,
      minimumStock: minimumStock,
    );
  }

  /// Send shift reminder
  Future<bool> sendShiftReminder({
    required String employeeName,
    required DateTime shiftStart,
    required String location,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendShiftReminder(
      employeeName: employeeName,
      shiftStart: shiftStart,
      location: location,
    );
  }

  /// Send appointment reminder
  Future<bool> sendAppointmentReminder({
    required String customerName,
    required DateTime appointmentTime,
    required String service,
    String? notes,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendAppointmentReminder(
      customerName: customerName,
      appointmentTime: appointmentTime,
      service: service,
      notes: notes,
    );
  }

  /// Send system alert
  Future<bool> sendSystemAlert({
    required String title,
    required String message,
    WatchNotificationPriority priority = WatchNotificationPriority.high,
    String? actionUrl,
  }) async {
    final notificationService = ref.read(watchNotificationServiceProvider);
    return await notificationService.sendSystemAlert(
      title: title,
      message: message,
      priority: priority,
      actionUrl: actionUrl,
    );
  }

  /// Clear notifications history
  void clearNotifications() {
    state = [];
  }
}