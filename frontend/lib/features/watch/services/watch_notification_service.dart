import 'dart:async';
import 'package:flutter/foundation.dart';
import '../models/watch_notification.dart';
import '../models/watch_device.dart';
import 'watch_service.dart';

/// Service for managing business-specific watch notifications
class WatchNotificationService {
  static WatchNotificationService? _instance;
  static WatchNotificationService get instance => _instance ??= WatchNotificationService._();
  
  WatchNotificationService._();

  final WatchService _watchService = WatchService.instance;
  final List<WatchNotification> _pendingNotifications = [];
  final List<WatchNotification> _sentNotifications = [];

  Timer? _retryTimer;

  /// Send an order received notification
  Future<bool> sendOrderNotification({
    required String orderId,
    required String customerName,
    required double amount,
    String? specialInstructions,
  }) async {
    final notification = WatchNotification(
      id: 'order_$orderId',
      title: 'New Order Received',
      body: '$customerName - \$${amount.toStringAsFixed(2)}',
      type: WatchNotificationType.orderReceived,
      priority: WatchNotificationPriority.high,
      actionUrl: '/orders/$orderId',
      actions: [
        const WatchNotificationAction(
          id: 'view_order',
          title: 'View Order',
          type: WatchNotificationActionType.viewDetails,
          url: '/orders/details',
        ),
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
      ],
      metadata: {
        'orderId': orderId,
        'customerName': customerName,
        'amount': amount,
        'specialInstructions': specialInstructions,
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send a payment completed notification
  Future<bool> sendPaymentNotification({
    required String orderId,
    required double amount,
    required String paymentMethod,
  }) async {
    final notification = WatchNotification(
      id: 'payment_$orderId',
      title: 'Payment Received',
      body: '\$${amount.toStringAsFixed(2)} via $paymentMethod',
      type: WatchNotificationType.paymentCompleted,
      priority: WatchNotificationPriority.normal,
      actions: [
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
      ],
      metadata: {
        'orderId': orderId,
        'amount': amount,
        'paymentMethod': paymentMethod,
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send an inventory low notification
  Future<bool> sendInventoryAlert({
    required String itemName,
    required int currentStock,
    required int minimumStock,
  }) async {
    final notification = WatchNotification(
      id: 'inventory_${itemName.replaceAll(' ', '_')}',
      title: 'Low Inventory Alert',
      body: '$itemName: $currentStock left (min: $minimumStock)',
      type: WatchNotificationType.inventoryLow,
      priority: WatchNotificationPriority.high,
      actions: [
        const WatchNotificationAction(
          id: 'view_inventory',
          title: 'View Inventory',
          type: WatchNotificationActionType.viewDetails,
          url: '/inventory',
        ),
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
      ],
      metadata: {
        'itemName': itemName,
        'currentStock': currentStock,
        'minimumStock': minimumStock,
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send a shift reminder notification
  Future<bool> sendShiftReminder({
    required String employeeName,
    required DateTime shiftStart,
    required String location,
  }) async {
    final timeUntilShift = shiftStart.difference(DateTime.now());
    final minutesUntilShift = timeUntilShift.inMinutes;

    final notification = WatchNotification(
      id: 'shift_${employeeName.replaceAll(' ', '_')}_${shiftStart.millisecondsSinceEpoch}',
      title: 'Shift Reminder',
      body: '$employeeName\'s shift starts in $minutesUntilShift minutes at $location',
      type: WatchNotificationType.shiftReminder,
      priority: WatchNotificationPriority.normal,
      scheduledTime: shiftStart.subtract(const Duration(minutes: 15)),
      actions: [
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
        const WatchNotificationAction(
          id: 'snooze',
          title: 'Snooze 5m',
          type: WatchNotificationActionType.snooze,
          payload: {'snoozeMinutes': 5},
        ),
      ],
      metadata: {
        'employeeName': employeeName,
        'shiftStart': shiftStart.toIso8601String(),
        'location': location,
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send an appointment reminder notification
  Future<bool> sendAppointmentReminder({
    required String customerName,
    required DateTime appointmentTime,
    required String service,
    String? notes,
  }) async {
    final timeUntilAppointment = appointmentTime.difference(DateTime.now());
    final minutesUntilAppointment = timeUntilAppointment.inMinutes;

    final notification = WatchNotification(
      id: 'appointment_${customerName.replaceAll(' ', '_')}_${appointmentTime.millisecondsSinceEpoch}',
      title: 'Upcoming Appointment',
      body: '$customerName - $service in $minutesUntilAppointment minutes',
      type: WatchNotificationType.appointmentReminder,
      priority: WatchNotificationPriority.normal,
      scheduledTime: appointmentTime.subtract(const Duration(minutes: 10)),
      actions: [
        const WatchNotificationAction(
          id: 'view_appointment',
          title: 'View Details',
          type: WatchNotificationActionType.viewDetails,
          url: '/appointments',
        ),
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
      ],
      metadata: {
        'customerName': customerName,
        'appointmentTime': appointmentTime.toIso8601String(),
        'service': service,
        'notes': notes,
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send a system alert notification
  Future<bool> sendSystemAlert({
    required String title,
    required String message,
    WatchNotificationPriority priority = WatchNotificationPriority.high,
    String? actionUrl,
  }) async {
    final notification = WatchNotification(
      id: 'system_${DateTime.now().millisecondsSinceEpoch}',
      title: title,
      body: message,
      type: WatchNotificationType.systemAlert,
      priority: priority,
      actionUrl: actionUrl,
      actions: [
        const WatchNotificationAction(
          id: 'acknowledge',
          title: 'OK',
          type: WatchNotificationActionType.acknowledge,
        ),
        if (actionUrl != null)
          const WatchNotificationAction(
            id: 'view_details',
            title: 'View Details',
            type: WatchNotificationActionType.viewDetails,
          ),
      ],
      metadata: {
        'alertType': 'system',
        'timestamp': DateTime.now().toIso8601String(),
      },
    );

    return await _sendNotificationWithRetry(notification);
  }

  /// Send a custom notification
  Future<bool> sendCustomNotification(WatchNotification notification) async {
    return await _sendNotificationWithRetry(notification);
  }

  /// Send notification with retry logic
  Future<bool> _sendNotificationWithRetry(WatchNotification notification) async {
    try {
      // Check if we have connected devices
      if (!_watchService.hasConnectedDevices) {
        debugPrint('No connected watch devices, queuing notification: ${notification.id}');
        _pendingNotifications.add(notification);
        _startRetryTimer();
        return false;
      }

      // Send the notification
      final success = await _watchService.sendNotification(notification);
      
      if (success) {
        _sentNotifications.add(notification.copyWith(
          sent: true,
          sentAt: DateTime.now(),
        ));
        debugPrint('Notification sent successfully: ${notification.id}');
        return true;
      } else {
        debugPrint('Failed to send notification, queuing for retry: ${notification.id}');
        _pendingNotifications.add(notification);
        _startRetryTimer();
        return false;
      }
    } catch (e) {
      debugPrint('Error sending notification: $e');
      _pendingNotifications.add(notification);
      _startRetryTimer();
      return false;
    }
  }

  /// Start retry timer for pending notifications
  void _startRetryTimer() {
    _retryTimer?.cancel();
    _retryTimer = Timer.periodic(const Duration(seconds: 30), (timer) {
      _retryPendingNotifications();
    });
  }

  /// Retry sending pending notifications
  Future<void> _retryPendingNotifications() async {
    if (_pendingNotifications.isEmpty) {
      _retryTimer?.cancel();
      return;
    }

    if (!_watchService.hasConnectedDevices) {
      return;
    }

    final notificationsToRetry = List<WatchNotification>.from(_pendingNotifications);
    _pendingNotifications.clear();

    for (final notification in notificationsToRetry) {
      final success = await _watchService.sendNotification(notification);
      
      if (success) {
        _sentNotifications.add(notification.copyWith(
          sent: true,
          sentAt: DateTime.now(),
        ));
        debugPrint('Retry notification sent successfully: ${notification.id}');
      } else {
        _pendingNotifications.add(notification);
      }
    }

    if (_pendingNotifications.isEmpty) {
      _retryTimer?.cancel();
    }
  }

  /// Get pending notifications
  List<WatchNotification> get pendingNotifications => List.from(_pendingNotifications);

  /// Get sent notifications
  List<WatchNotification> get sentNotifications => List.from(_sentNotifications);

  /// Clear sent notifications history
  void clearSentNotifications() {
    _sentNotifications.clear();
  }

  /// Cancel pending notification
  bool cancelPendingNotification(String notificationId) {
    final removed = _pendingNotifications.removeWhere((n) => n.id == notificationId);
    return removed > 0;
  }

  /// Dispose resources
  void dispose() {
    _retryTimer?.cancel();
    _pendingNotifications.clear();
    _sentNotifications.clear();
  }
}