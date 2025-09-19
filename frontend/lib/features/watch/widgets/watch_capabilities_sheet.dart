import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/watch_device.dart';
import '../providers/watch_providers.dart';

class WatchCapabilitiesSheet extends ConsumerWidget {
  final WatchDevice device;

  const WatchCapabilitiesSheet({
    super.key,
    required this.device,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final capabilitiesAsync = ref.watch(watchDeviceCapabilitiesProvider(device.id));

    return DraggableScrollableSheet(
      initialChildSize: 0.7,
      minChildSize: 0.5,
      maxChildSize: 0.9,
      expand: false,
      builder: (context, scrollController) {
        return Container(
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
          ),
          child: Column(
            children: [
              Container(
                width: 40,
                height: 4,
                margin: const EdgeInsets.symmetric(vertical: 8),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.onSurfaceVariant.withOpacity(0.4),
                  borderRadius: BorderRadius.circular(2),
                ),
              ),
              Expanded(
                child: capabilitiesAsync.when(
                  data: (capabilities) => _buildCapabilitiesContent(
                    context,
                    scrollController,
                    capabilities,
                  ),
                  loading: () => const Center(child: CircularProgressIndicator()),
                  error: (error, stack) => _buildErrorContent(context, error),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildCapabilitiesContent(
    BuildContext context,
    ScrollController scrollController,
    WatchDeviceCapabilities? capabilities,
  ) {
    final theme = Theme.of(context);

    return CustomScrollView(
      controller: scrollController,
      slivers: [
        SliverToBoxAdapter(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(24, 0, 24, 16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Container(
                      width: 56,
                      height: 56,
                      decoration: BoxDecoration(
                        color: theme.colorScheme.primaryContainer,
                        borderRadius: BorderRadius.circular(16),
                      ),
                      child: Icon(
                        _getDeviceIcon(),
                        color: theme.colorScheme.onPrimaryContainer,
                        size: 28,
                      ),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            device.name,
                            style: theme.textTheme.headlineSmall?.copyWith(
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          Text(
                            device.type.displayName,
                            style: theme.textTheme.bodyLarge?.copyWith(
                              color: theme.colorScheme.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 24),
                Text(
                  'Device Information',
                  style: theme.textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 12),
              ],
            ),
          ),
        ),
        SliverToBoxAdapter(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: _buildDeviceInfo(context),
          ),
        ),
        if (capabilities != null) ...[
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 24, 24, 16),
              child: Text(
                'Capabilities',
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ),
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 24),
              child: _buildCapabilitiesList(context, capabilities),
            ),
          ),
        ] else ...[
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 24, 24, 16),
              child: Card(
                child: Padding(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      Icon(
                        Icons.info_outline,
                        color: theme.colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          'Capabilities information is not available for this device.',
                          style: theme.textTheme.bodyMedium,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
        const SliverToBoxAdapter(
          child: SizedBox(height: 24),
        ),
      ],
    );
  }

  Widget _buildDeviceInfo(BuildContext context) {
    final theme = Theme.of(context);
    
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            _buildInfoRow(
              context,
              'Status',
              device.status.displayName,
              _getStatusIcon(),
              _getStatusColor(theme),
            ),
            if (device.model != null) ...[
              const Divider(),
              _buildInfoRow(context, 'Model', device.model!, Icons.phone_android),
            ],
            if (device.osVersion != null) ...[
              const Divider(),
              _buildInfoRow(context, 'OS Version', device.osVersion!, Icons.memory),
            ],
            if (device.lastConnected != null) ...[
              const Divider(),
              _buildInfoRow(
                context,
                'Last Connected',
                _formatLastConnected(device.lastConnected!),
                Icons.access_time,
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildInfoRow(
    BuildContext context,
    String label,
    String value,
    IconData icon, [
    Color? iconColor,
  ]) {
    final theme = Theme.of(context);
    
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          Icon(
            icon,
            size: 20,
            color: iconColor ?? theme.colorScheme.onSurfaceVariant,
          ),
          const SizedBox(width: 12),
          Text(
            label,
            style: theme.textTheme.bodyMedium?.copyWith(
              color: theme.colorScheme.onSurfaceVariant,
            ),
          ),
          const Spacer(),
          Text(
            value,
            style: theme.textTheme.bodyMedium?.copyWith(
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCapabilitiesList(BuildContext context, WatchDeviceCapabilities capabilities) {
    final capabilityItems = [
      _CapabilityItem('Notifications', capabilities.supportsNotifications, Icons.notifications),
      _CapabilityItem('Heart Rate', capabilities.supportsHeartRate, Icons.favorite),
      _CapabilityItem('GPS', capabilities.supportsGPS, Icons.location_on),
      _CapabilityItem('Cellular', capabilities.supportsCellular, Icons.signal_cellular_4_bar),
      _CapabilityItem('Payments', capabilities.supportsPayments, Icons.payment),
      _CapabilityItem('Apps', capabilities.supportsApps, Icons.apps),
      _CapabilityItem('Complications', capabilities.supportsComplication, Icons.widgets),
      _CapabilityItem('Haptic Feedback', capabilities.supportsHapticFeedback, Icons.vibration),
      _CapabilityItem('Microphone', capabilities.supportsMicrophone, Icons.mic),
      _CapabilityItem('Speaker', capabilities.supportsSpeaker, Icons.volume_up),
    ];

    return Card(
      child: Column(
        children: capabilityItems
            .map((item) => _buildCapabilityItem(context, item))
            .expand((widget) => [widget, if (widget != capabilityItems.last) const Divider(height: 1)])
            .toList(),
      ),
    );
  }

  Widget _buildCapabilityItem(BuildContext context, _CapabilityItem item) {
    final theme = Theme.of(context);
    
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      child: Row(
        children: [
          Icon(
            item.icon,
            size: 24,
            color: item.isSupported
                ? theme.colorScheme.primary
                : theme.colorScheme.onSurfaceVariant.withOpacity(0.5),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Text(
              item.label,
              style: theme.textTheme.bodyMedium?.copyWith(
                color: item.isSupported
                    ? theme.colorScheme.onSurface
                    : theme.colorScheme.onSurfaceVariant.withOpacity(0.7),
              ),
            ),
          ),
          Icon(
            item.isSupported ? Icons.check_circle : Icons.cancel,
            size: 20,
            color: item.isSupported
                ? theme.colorScheme.primary
                : theme.colorScheme.onSurfaceVariant.withOpacity(0.5),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorContent(BuildContext context, Object error) {
    final theme = Theme.of(context);
    
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 48,
              color: theme.colorScheme.error,
            ),
            const SizedBox(height: 16),
            Text(
              'Error Loading Capabilities',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 8),
            Text(
              error.toString(),
              style: theme.textTheme.bodyMedium?.copyWith(
                color: theme.colorScheme.onSurfaceVariant,
              ),
              textAlign: TextAlign.center,
            ),
          ],
        ),
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

  IconData _getStatusIcon() {
    switch (device.status) {
      case WatchConnectionStatus.connected:
        return Icons.check_circle;
      case WatchConnectionStatus.connecting:
      case WatchConnectionStatus.pairing:
        return Icons.sync;
      case WatchConnectionStatus.error:
        return Icons.error;
      case WatchConnectionStatus.disconnected:
      default:
        return Icons.radio_button_unchecked;
    }
  }

  Color _getStatusColor(ThemeData theme) {
    switch (device.status) {
      case WatchConnectionStatus.connected:
        return theme.colorScheme.primary;
      case WatchConnectionStatus.connecting:
      case WatchConnectionStatus.pairing:
        return theme.colorScheme.secondary;
      case WatchConnectionStatus.error:
        return theme.colorScheme.error;
      case WatchConnectionStatus.disconnected:
      default:
        return theme.colorScheme.onSurfaceVariant;
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

class _CapabilityItem {
  final String label;
  final bool isSupported;
  final IconData icon;

  const _CapabilityItem(this.label, this.isSupported, this.icon);
}