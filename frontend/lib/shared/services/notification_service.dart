import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Notification types
enum NotificationType {
  info,
  success,
  warning,
  error,
}

/// Notification model
class AppNotification {
  final String id;
  final String title;
  final String message;
  final NotificationType type;
  final DateTime timestamp;
  final Duration? duration;
  final VoidCallback? onTap;
  final bool persistent;

  AppNotification({
    required this.id,
    required this.title,
    required this.message,
    required this.type,
    DateTime? timestamp,
    this.duration,
    this.onTap,
    this.persistent = false,
  }) : timestamp = timestamp ?? DateTime.now();

  factory AppNotification.info({
    required String title,
    required String message,
    Duration? duration,
    VoidCallback? onTap,
  }) {
    return AppNotification(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      title: title,
      message: message,
      type: NotificationType.info,
      duration: duration ?? const Duration(seconds: 4),
      onTap: onTap,
    );
  }

  factory AppNotification.success({
    required String title,
    required String message,
    Duration? duration,
    VoidCallback? onTap,
  }) {
    return AppNotification(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      title: title,
      message: message,
      type: NotificationType.success,
      duration: duration ?? const Duration(seconds: 3),
      onTap: onTap,
    );
  }

  factory AppNotification.warning({
    required String title,
    required String message,
    Duration? duration,
    VoidCallback? onTap,
  }) {
    return AppNotification(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      title: title,
      message: message,
      type: NotificationType.warning,
      duration: duration ?? const Duration(seconds: 5),
      onTap: onTap,
    );
  }

  factory AppNotification.error({
    required String title,
    required String message,
    Duration? duration,
    VoidCallback? onTap,
  }) {
    return AppNotification(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      title: title,
      message: message,
      type: NotificationType.error,
      duration: duration,
      onTap: onTap,
      persistent: true,
    );
  }
}

/// Notification service provider
final notificationServiceProvider = StateNotifierProvider<NotificationService, List<AppNotification>>((ref) {
  return NotificationService();
});

/// Notification service
class NotificationService extends StateNotifier<List<AppNotification>> {
  NotificationService() : super([]);

  /// Show a notification
  void show(AppNotification notification) {
    state = [...state, notification];
    
    // Auto-remove after duration if not persistent
    if (!notification.persistent && notification.duration != null) {
      Future.delayed(notification.duration!, () {
        remove(notification.id);
      });
    }
  }

  /// Show info notification
  void showInfo(String title, String message, {VoidCallback? onTap}) {
    show(AppNotification.info(title: title, message: message, onTap: onTap));
  }

  /// Show success notification
  void showSuccess(String title, String message, {VoidCallback? onTap}) {
    show(AppNotification.success(title: title, message: message, onTap: onTap));
  }

  /// Show warning notification
  void showWarning(String title, String message, {VoidCallback? onTap}) {
    show(AppNotification.warning(title: title, message: message, onTap: onTap));
  }

  /// Show error notification
  void showError(String title, String message, {VoidCallback? onTap}) {
    show(AppNotification.error(title: title, message: message, onTap: onTap));
  }

  /// Remove a notification
  void remove(String id) {
    state = state.where((notification) => notification.id != id).toList();
  }

  /// Clear all notifications
  void clearAll() {
    state = [];
  }

  /// Clear notifications of specific type
  void clearByType(NotificationType type) {
    state = state.where((notification) => notification.type != type).toList();
  }
}

/// Notification overlay widget
class NotificationOverlay extends ConsumerWidget {
  final Widget child;

  const NotificationOverlay({
    super.key,
    required this.child,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifications = ref.watch(notificationServiceProvider);

    return Stack(
      children: [
        child,
        if (notifications.isNotEmpty)
          Positioned(
            top: MediaQuery.of(context).padding.top + 16,
            left: 16,
            right: 16,
            child: Column(
              children: notifications.map((notification) {
                return NotificationCard(
                  notification: notification,
                  onDismiss: () {
                    ref.read(notificationServiceProvider.notifier).remove(notification.id);
                  },
                );
              }).toList(),
            ),
          ),
      ],
    );
  }
}

/// Individual notification card
class NotificationCard extends StatelessWidget {
  final AppNotification notification;
  final VoidCallback onDismiss;

  const NotificationCard({
    super.key,
    required this.notification,
    required this.onDismiss,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      child: Material(
        elevation: 6,
        borderRadius: BorderRadius.circular(8),
        child: InkWell(
          onTap: notification.onTap,
          borderRadius: BorderRadius.circular(8),
          child: Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(8),
              color: _getBackgroundColor(theme),
              border: Border.all(
                color: _getBorderColor(theme),
                width: 1,
              ),
            ),
            child: Row(
              children: [
                Icon(
                  _getIcon(),
                  color: _getIconColor(theme),
                  size: 24,
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        notification.title,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                          color: _getTextColor(theme),
                        ),
                      ),
                      if (notification.message.isNotEmpty) ...[
                        const SizedBox(height: 4),
                        Text(
                          notification.message,
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: _getTextColor(theme).withOpacity(0.8),
                          ),
                        ),
                      ],
                    ],
                  ),
                ),
                IconButton(
                  onPressed: onDismiss,
                  icon: Icon(
                    Icons.close,
                    size: 20,
                    color: _getTextColor(theme).withOpacity(0.6),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  IconData _getIcon() {
    switch (notification.type) {
      case NotificationType.info:
        return Icons.info_outline;
      case NotificationType.success:
        return Icons.check_circle_outline;
      case NotificationType.warning:
        return Icons.warning_outlined;
      case NotificationType.error:
        return Icons.error_outline;
    }
  }

  Color _getBackgroundColor(ThemeData theme) {
    switch (notification.type) {
      case NotificationType.info:
        return theme.colorScheme.primaryContainer;
      case NotificationType.success:
        return const Color(0xFFE8F5E8);
      case NotificationType.warning:
        return const Color(0xFFFFF3CD);
      case NotificationType.error:
        return const Color(0xFFFDE8E8);
    }
  }

  Color _getBorderColor(ThemeData theme) {
    switch (notification.type) {
      case NotificationType.info:
        return theme.colorScheme.primary;
      case NotificationType.success:
        return const Color(0xFF4CAF50);
      case NotificationType.warning:
        return const Color(0xFFFF9800);
      case NotificationType.error:
        return const Color(0xFFF44336);
    }
  }

  Color _getIconColor(ThemeData theme) {
    return _getBorderColor(theme);
  }

  Color _getTextColor(ThemeData theme) {
    switch (notification.type) {
      case NotificationType.info:
        return theme.colorScheme.onPrimaryContainer;
      case NotificationType.success:
        return const Color(0xFF2E7D32);
      case NotificationType.warning:
        return const Color(0xFFE65100);
      case NotificationType.error:
        return const Color(0xFFC62828);
    }
  }
}