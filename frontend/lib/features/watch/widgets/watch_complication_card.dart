import 'package:flutter/material.dart';
import '../models/watch_complication.dart';

class WatchComplicationCard extends StatelessWidget {
  final WatchComplication complication;
  final VoidCallback? onUpdate;
  final VoidCallback? onRemove;
  final VoidCallback? onConfigure;

  const WatchComplicationCard({
    super.key,
    required this.complication,
    this.onUpdate,
    this.onRemove,
    this.onConfigure,
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
                  child: Center(
                    child: Text(
                      complication.type.iconEmoji,
                      style: const TextStyle(fontSize: 24),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        complication.title,
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        complication.type.shortName,
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
            const SizedBox(height: 16),
            Row(
              children: [
                Expanded(
                  child: _buildValueDisplay(context),
                ),
                if (complication.trend != null) ...[
                  const SizedBox(width: 12),
                  _buildTrendIndicator(context),
                ],
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Icon(
                  Icons.access_time,
                  size: 16,
                  color: theme.colorScheme.onSurfaceVariant,
                ),
                const SizedBox(width: 4),
                Text(
                  'Updated ${_formatLastUpdated()}',
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: theme.colorScheme.onSurfaceVariant,
                  ),
                ),
                const Spacer(),
                Text(
                  '${complication.refreshIntervalSeconds}s refresh',
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: theme.colorScheme.onSurfaceVariant,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                FilledButton.icon(
                  onPressed: onUpdate,
                  icon: const Icon(Icons.refresh, size: 18),
                  label: const Text('Update'),
                  style: FilledButton.styleFrom(
                    minimumSize: const Size(0, 36),
                  ),
                ),
                const SizedBox(width: 8),
                OutlinedButton.icon(
                  onPressed: onConfigure,
                  icon: const Icon(Icons.settings, size: 18),
                  label: const Text('Configure'),
                  style: OutlinedButton.styleFrom(
                    minimumSize: const Size(0, 36),
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: onRemove,
                  icon: const Icon(Icons.delete_outline),
                  color: theme.colorScheme.error,
                  tooltip: 'Remove complication',
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
    final isEnabled = complication.enabled;
    
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: isEnabled
            ? theme.colorScheme.primaryContainer
            : theme.colorScheme.surfaceVariant,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            isEnabled ? Icons.check_circle : Icons.pause_circle_outline,
            size: 16,
            color: isEnabled
                ? theme.colorScheme.onPrimaryContainer
                : theme.colorScheme.onSurfaceVariant,
          ),
          const SizedBox(width: 4),
          Text(
            isEnabled ? 'Active' : 'Paused',
            style: theme.textTheme.labelSmall?.copyWith(
              color: isEnabled
                  ? theme.colorScheme.onPrimaryContainer
                  : theme.colorScheme.onSurfaceVariant,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildValueDisplay(BuildContext context) {
    final theme = Theme.of(context);
    
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: theme.colorScheme.surfaceVariant.withOpacity(0.5),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          Text(
            complication.value,
            style: theme.textTheme.headlineMedium?.copyWith(
              fontWeight: FontWeight.bold,
              color: theme.colorScheme.onSurface,
            ),
          ),
          if (complication.unit?.isNotEmpty == true) ...[
            const SizedBox(width: 4),
            Text(
              complication.unit!,
              style: theme.textTheme.bodyLarge?.copyWith(
                color: theme.colorScheme.onSurfaceVariant,
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildTrendIndicator(BuildContext context) {
    final theme = Theme.of(context);
    final trend = complication.trend!;
    
    Color trendColor;
    switch (trend) {
      case WatchComplicationTrend.up:
        trendColor = Colors.green;
        break;
      case WatchComplicationTrend.down:
        trendColor = Colors.red;
        break;
      case WatchComplicationTrend.stable:
        trendColor = theme.colorScheme.onSurfaceVariant;
        break;
    }
    
    return Container(
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: trendColor.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            trend.symbol,
            style: TextStyle(
              fontSize: 16,
              color: trendColor,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(width: 4),
          Text(
            trend.emoji,
            style: const TextStyle(fontSize: 16),
          ),
        ],
      ),
    );
  }

  String _formatLastUpdated() {
    if (complication.lastUpdated == null) {
      return 'Never';
    }
    
    final now = DateTime.now();
    final difference = now.difference(complication.lastUpdated!);
    
    if (difference.inSeconds < 60) {
      return 'just now';
    } else if (difference.inMinutes < 60) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inHours < 24) {
      return '${difference.inHours}h ago';
    } else {
      return '${difference.inDays}d ago';
    }
  }
}