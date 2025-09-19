import 'package:flutter/material.dart';
import '../models/watch_device.dart';

class WatchDeviceCard extends StatelessWidget {
  final WatchDevice device;
  final VoidCallback? onConnect;
  final VoidCallback? onDisconnect;
  final VoidCallback? onShowCapabilities;

  const WatchDeviceCard({
    super.key,
    required this.device,
    this.onConnect,
    this.onDisconnect,
    this.onShowCapabilities,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  width: 48,
                  height: 48,
                  decoration: BoxDecoration(
                    color: theme.colorScheme.primaryContainer,
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Icon(
                    _getDeviceIcon(),
                    color: theme.colorScheme.onPrimaryContainer,
                    size: 24,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        device.name,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        device.type.displayName,
                        style: theme.textTheme.bodyMedium?.copyWith(
                          color: theme.colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
                _buildStatusChip(context),
              ],
            ),
            if (device.model != null || device.osVersion != null) ...[
              const SizedBox(height: 12),
              Row(
                children: [
                  if (device.model != null) ...[
                    Icon(
                      Icons.info_outline,
                      size: 16,
                      color: theme.colorScheme.onSurfaceVariant,
                    ),
                    const SizedBox(width: 4),
                    Text(
                      device.model!,
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurfaceVariant,
                      ),
                    ),
                  ],
                  if (device.model != null && device.osVersion != null)
                    Text(
                      ' • ',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurfaceVariant,
                      ),
                    ),
                  if (device.osVersion != null) ...[
                    Text(
                      'OS ${device.osVersion!}',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurfaceVariant,
                      ),
                    ),
                  ],
                ],
              ),
            ],
            if (device.lastConnected != null) ...[
              const SizedBox(height: 8),
              Row(
                children: [
                  Icon(
                    Icons.access_time,
                    size: 16,
                    color: theme.colorScheme.onSurfaceVariant,
                  ),
                  const SizedBox(width: 4),
                  Text(
                    'Last connected: ${_formatLastConnected(device.lastConnected!)}',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: theme.colorScheme.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
            ],
            const SizedBox(height: 16),
            Row(
              children: [
                if (device.status.isConnected) ...[
                  FilledButton.icon(
                    onPressed: onDisconnect,
                    icon: const Icon(Icons.link_off, size: 18),
                    label: const Text('Disconnect'),
                    style: FilledButton.styleFrom(
                      backgroundColor: theme.colorScheme.error,
                      foregroundColor: theme.colorScheme.onError,
                    ),
                  ),
                ] else ...[
                  FilledButton.icon(
                    onPressed: device.status.isConnecting ? null : onConnect,
                    icon: device.status.isConnecting
                        ? const SizedBox(
                            width: 18,
                            height: 18,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          )
                        : const Icon(Icons.link, size: 18),
                    label: Text(device.status.isConnecting ? 'Connecting...' : 'Connect'),
                  ),
                ],
                const SizedBox(width: 8),
                OutlinedButton.icon(
                  onPressed: onShowCapabilities,
                  icon: const Icon(Icons.settings, size: 18),
                  label: const Text('Capabilities'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusChip(BuildContext context) {
    final theme = Theme.of(context);
    Color backgroundColor;
    Color foregroundColor;
    IconData icon;

    switch (device.status) {
      case WatchConnectionStatus.connected:
        backgroundColor = theme.colorScheme.primaryContainer;
        foregroundColor = theme.colorScheme.onPrimaryContainer;
        icon = Icons.check_circle;
        break;
      case WatchConnectionStatus.connecting:
      case WatchConnectionStatus.pairing:
        backgroundColor = theme.colorScheme.secondaryContainer;
        foregroundColor = theme.colorScheme.onSecondaryContainer;
        icon = Icons.sync;
        break;
      case WatchConnectionStatus.error:
        backgroundColor = theme.colorScheme.errorContainer;
        foregroundColor = theme.colorScheme.onErrorContainer;
        icon = Icons.error;
        break;
      case WatchConnectionStatus.disconnected:
      default:
        backgroundColor = theme.colorScheme.surfaceVariant;
        foregroundColor = theme.colorScheme.onSurfaceVariant;
        icon = Icons.radio_button_unchecked;
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: backgroundColor,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            icon,
            size: 16,
            color: foregroundColor,
          ),
          const SizedBox(width: 4),
          Text(
            device.status.displayName,
            style: theme.textTheme.labelSmall?.copyWith(
              color: foregroundColor,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }

  IconData _getDeviceIcon() {
    switch (device.type) {
      case WatchDeviceType.appleWatch:
        return Icons.watch;
      case WatchDeviceType.wearOS:
        return Icons.watch_outlined;
      case WatchDeviceType.garmin:
        return Icons.fitness_center;
      case WatchDeviceType.fitbit:
        return Icons.monitor_heart;
      case WatchDeviceType.samsungGalaxyWatch:
        return Icons.watch;
      case WatchDeviceType.other:
      default:
        return Icons.watch_outlined;
    }
  }

  String _formatLastConnected(DateTime lastConnected) {
    final now = DateTime.now();
    final difference = now.difference(lastConnected);

    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inMinutes < 60) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inHours < 24) {
      return '${difference.inHours}h ago';
    } else if (difference.inDays < 30) {
      return '${difference.inDays}d ago';
    } else {
      return '${lastConnected.day}/${lastConnected.month}/${lastConnected.year}';
    }
  }
}