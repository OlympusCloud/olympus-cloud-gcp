import 'package:freezed_annotation/freezed_annotation.dart';

part 'watch_notification.freezed.dart';
part 'watch_notification.g.dart';

@freezed
class WatchNotification with _$WatchNotification {
  const factory WatchNotification({
    required String id,
    required String title,
    required String body,
    required WatchNotificationType type,
    required WatchNotificationPriority priority,
    DateTime? scheduledTime,
    String? actionUrl,
    List<WatchNotificationAction>? actions,
    Map<String, dynamic>? metadata,
    @Default(false) bool sent,
    @Default(false) bool acknowledged,
    DateTime? sentAt,
    DateTime? acknowledgedAt,
  }) = _WatchNotification;

  factory WatchNotification.fromJson(Map<String, dynamic> json) =>
      _$WatchNotificationFromJson(json);
}

@freezed
class WatchNotificationAction with _$WatchNotificationAction {
  const factory WatchNotificationAction({
    required String id,
    required String title,
    required WatchNotificationActionType type,
    String? url,
    Map<String, dynamic>? payload,
  }) = _WatchNotificationAction;

  factory WatchNotificationAction.fromJson(Map<String, dynamic> json) =>
      _$WatchNotificationActionFromJson(json);
}

enum WatchNotificationType {
  @JsonValue('order_received')
  orderReceived,
  @JsonValue('payment_completed')
  paymentCompleted,
  @JsonValue('inventory_low')
  inventoryLow,
  @JsonValue('shift_reminder')
  shiftReminder,
  @JsonValue('appointment_reminder')
  appointmentReminder,
  @JsonValue('system_alert')
  systemAlert,
  @JsonValue('marketing')
  marketing,
  @JsonValue('general')
  general,
}

enum WatchNotificationPriority {
  @JsonValue('low')
  low,
  @JsonValue('normal')
  normal,
  @JsonValue('high')
  high,
  @JsonValue('urgent')
  urgent,
}

enum WatchNotificationActionType {
  @JsonValue('acknowledge')
  acknowledge,
  @JsonValue('view_details')
  viewDetails,
  @JsonValue('quick_reply')
  quickReply,
  @JsonValue('mark_complete')
  markComplete,
  @JsonValue('snooze')
  snooze,
  @JsonValue('custom')
  custom,
}

extension WatchNotificationTypeExtension on WatchNotificationType {
  String get displayName {
    switch (this) {
      case WatchNotificationType.orderReceived:
        return 'New Order';
      case WatchNotificationType.paymentCompleted:
        return 'Payment Completed';
      case WatchNotificationType.inventoryLow:
        return 'Low Inventory';
      case WatchNotificationType.shiftReminder:
        return 'Shift Reminder';
      case WatchNotificationType.appointmentReminder:
        return 'Appointment Reminder';
      case WatchNotificationType.systemAlert:
        return 'System Alert';
      case WatchNotificationType.marketing:
        return 'Marketing';
      case WatchNotificationType.general:
        return 'General';
    }
  }

  String get iconEmoji {
    switch (this) {
      case WatchNotificationType.orderReceived:
        return '🛒';
      case WatchNotificationType.paymentCompleted:
        return '💳';
      case WatchNotificationType.inventoryLow:
        return '📦';
      case WatchNotificationType.shiftReminder:
        return '⏰';
      case WatchNotificationType.appointmentReminder:
        return '📅';
      case WatchNotificationType.systemAlert:
        return '⚠️';
      case WatchNotificationType.marketing:
        return '📢';
      case WatchNotificationType.general:
        return 'ℹ️';
    }
  }

  bool get requiresImmediate {
    switch (this) {
      case WatchNotificationType.orderReceived:
      case WatchNotificationType.systemAlert:
        return true;
      default:
        return false;
    }
  }
}

extension WatchNotificationPriorityExtension on WatchNotificationPriority {
  String get displayName {
    switch (this) {
      case WatchNotificationPriority.low:
        return 'Low';
      case WatchNotificationPriority.normal:
        return 'Normal';
      case WatchNotificationPriority.high:
        return 'High';
      case WatchNotificationPriority.urgent:
        return 'Urgent';
    }
  }

  int get level {
    switch (this) {
      case WatchNotificationPriority.low:
        return 1;
      case WatchNotificationPriority.normal:
        return 2;
      case WatchNotificationPriority.high:
        return 3;
      case WatchNotificationPriority.urgent:
        return 4;
    }
  }

  bool get shouldVibrate => level >= 3;
  bool get shouldPlaySound => level >= 2;
}

extension WatchNotificationActionTypeExtension on WatchNotificationActionType {
  String get displayName {
    switch (this) {
      case WatchNotificationActionType.acknowledge:
        return 'OK';
      case WatchNotificationActionType.viewDetails:
        return 'View';
      case WatchNotificationActionType.quickReply:
        return 'Reply';
      case WatchNotificationActionType.markComplete:
        return 'Complete';
      case WatchNotificationActionType.snooze:
        return 'Snooze';
      case WatchNotificationActionType.custom:
        return 'Action';
    }
  }
}