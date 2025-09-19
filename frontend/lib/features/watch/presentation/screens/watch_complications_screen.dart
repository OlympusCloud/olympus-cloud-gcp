import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../models/watch_complication.dart';
import '../../providers/watch_providers.dart';
import '../../widgets/watch_complication_card.dart';
import '../../../../shared/widgets/loading_widgets.dart';

class WatchComplicationsScreen extends ConsumerStatefulWidget {
  const WatchComplicationsScreen({super.key});

  @override
  ConsumerState<WatchComplicationsScreen> createState() => _WatchComplicationsScreenState();
}

class _WatchComplicationsScreenState extends ConsumerState<WatchComplicationsScreen> {
  bool _isSessionActive = false;

  @override
  Widget build(BuildContext context) {
    final complications = ref.watch(watchComplicationsNotifierProvider);
    final connectedDevices = ref.watch(connectedWatchDevicesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Watch Complications'),
        actions: [
          PopupMenuButton(
            icon: const Icon(Icons.more_vert),
            itemBuilder: (context) => [
              PopupMenuItem(
                onTap: _toggleComplicationSession,
                child: Row(
                  children: [
                    Icon(_isSessionActive ? Icons.stop : Icons.play_arrow),
                    const SizedBox(width: 8),
                    Text(_isSessionActive ? 'Stop Updates' : 'Start Updates'),
                  ],
                ),
              ),
              const PopupMenuItem(
                value: 'refresh',
                child: Row(
                  children: [
                    Icon(Icons.refresh),
                    SizedBox(width: 8),
                    Text('Refresh All'),
                  ],
                ),
              ),
            ],
            onSelected: (value) {
              if (value == 'refresh') {
                _refreshAllComplications();
              }
            },
          ),
        ],
      ),
      body: connectedDevices.when(
        data: (devices) {
          if (devices.isEmpty) {
            return _buildNoDevicesState();
          }
          return _buildComplicationsList(complications);
        },
        loading: () => const Center(child: LoadingSpinner()),
        error: (error, stack) => _buildErrorState(error),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _showAddComplicationDialog,
        icon: const Icon(Icons.add),
        label: const Text('Add Complication'),
      ),
    );
  }

  Widget _buildComplicationsList(Map<WatchComplicationType, WatchComplication> complications) {
    if (complications.isEmpty) {
      return _buildEmptyState();
    }

    final sortedComplications = complications.values.toList()
      ..sort((a, b) => a.type.displayName.compareTo(b.type.displayName));

    return RefreshIndicator(
      onRefresh: _refreshAllComplications,
      child: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: sortedComplications.length,
        itemBuilder: (context, index) {
          final complication = sortedComplications[index];
          return Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: WatchComplicationCard(
              complication: complication,
              onUpdate: () => _updateComplication(complication),
              onRemove: () => _removeComplication(complication),
              onConfigure: () => _configureComplication(complication),
            ),
          );
        },
      ),
    );
  }

  Widget _buildNoDevicesState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.watch_off_outlined,
            size: 64,
            color: Theme.of(context).colorScheme.outline,
          ),
          const SizedBox(height: 16),
          Text(
            'No Connected Watch Devices',
            style: Theme.of(context).textTheme.headlineSmall,
          ),
          const SizedBox(height: 8),
          Text(
            'Connect a watch device to manage complications.',
            style: Theme.of(context).textTheme.bodyMedium,
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: () => Navigator.pushNamed(context, '/watch/devices'),
            icon: const Icon(Icons.watch),
            label: const Text('Manage Devices'),
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.widgets_outlined,
            size: 64,
            color: Theme.of(context).colorScheme.outline,
          ),
          const SizedBox(height: 16),
          Text(
            'No Complications Added',
            style: Theme.of(context).textTheme.headlineSmall,
          ),
          const SizedBox(height: 8),
          Text(
            'Add complications to display business metrics on your watch.',
            style: Theme.of(context).textTheme.bodyMedium,
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: _showAddComplicationDialog,
            icon: const Icon(Icons.add),
            label: const Text('Add Complication'),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorState(Object error) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.error_outline,
            size: 64,
            color: Theme.of(context).colorScheme.error,
          ),
          const SizedBox(height: 16),
          Text(
            'Error Loading Complications',
            style: Theme.of(context).textTheme.headlineSmall,
          ),
          const SizedBox(height: 8),
          Text(
            error.toString(),
            style: Theme.of(context).textTheme.bodyMedium,
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: () => ref.refresh(watchComplicationsNotifierProvider),
            icon: const Icon(Icons.refresh),
            label: const Text('Try Again'),
          ),
        ],
      ),
    );
  }

  Future<void> _toggleComplicationSession() async {
    final notifier = ref.read(watchComplicationsNotifierProvider.notifier);
    
    if (_isSessionActive) {
      final success = await notifier.stopComplicationSession();
      if (success) {
        setState(() => _isSessionActive = false);
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
              content: Text('Stopped real-time updates'),
              backgroundColor: Colors.orange,
            ),
          );
        }
      }
    } else {
      final complications = ref.read(watchComplicationsNotifierProvider);
      final types = complications.keys.toList();
      
      if (types.isNotEmpty) {
        final success = await notifier.startComplicationSession(types);
        if (success) {
          setState(() => _isSessionActive = true);
          if (mounted) {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Started real-time updates'),
                backgroundColor: Colors.green,
              ),
            );
          }
        }
      }
    }
  }

  Future<void> _refreshAllComplications() async {
    final notifier = ref.read(watchComplicationsNotifierProvider.notifier);
    final complications = ref.read(watchComplicationsNotifierProvider);
    
    for (final complication in complications.values) {
      await _updateComplicationData(complication);
    }
  }

  Future<void> _updateComplication(WatchComplication complication) async {
    await _updateComplicationData(complication);
  }

  Future<void> _updateComplicationData(WatchComplication complication) async {
    final notifier = ref.read(watchComplicationsNotifierProvider.notifier);
    
    // Update with mock data based on type
    switch (complication.type) {
      case WatchComplicationType.dailySales:
        await notifier.updateDailySales(1250.00, trend: WatchComplicationTrend.up);
        break;
      case WatchComplicationType.orderCount:
        await notifier.updateOrderCount(23, trend: WatchComplicationTrend.up);
        break;
      case WatchComplicationType.currentCustomers:
        await notifier.updateCurrentCustomers(8);
        break;
      case WatchComplicationType.inventoryAlerts:
        await notifier.updateInventoryAlerts(2);
        break;
      case WatchComplicationType.staffStatus:
        await notifier.updateStaffStatus(3, 4);
        break;
      case WatchComplicationType.nextAppointment:
        await notifier.updateNextAppointment(DateTime.now().add(const Duration(minutes: 15)));
        break;
      default:
        // For other types, just update the timestamp
        await notifier.updateComplication(complication.copyWith(
          lastUpdated: DateTime.now(),
        ));
        break;
    }
  }

  void _removeComplication(WatchComplication complication) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Remove Complication'),
        content: Text('Remove "${complication.title}" from your watch?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () {
              Navigator.of(context).pop();
              // TODO: Implement remove complication
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: Text('Removed "${complication.title}"'),
                  backgroundColor: Colors.orange,
                ),
              );
            },
            child: const Text('Remove'),
          ),
        ],
      ),
    );
  }

  void _configureComplication(WatchComplication complication) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Configure ${complication.title}'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.timer),
              title: const Text('Refresh Interval'),
              subtitle: Text('${complication.refreshIntervalSeconds}s'),
              onTap: () {
                // TODO: Implement refresh interval configuration
              },
            ),
            SwitchListTile(
              secondary: const Icon(Icons.auto_awesome),
              title: const Text('Auto Update'),
              subtitle: const Text('Update automatically'),
              value: complication.enabled,
              onChanged: (value) {
                // TODO: Implement auto update toggle
              },
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Save'),
          ),
        ],
      ),
    );
  }

  void _showAddComplicationDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Add Complication'),
        content: SizedBox(
          width: double.maxFinite,
          child: ListView(
            shrinkWrap: true,
            children: WatchComplicationType.values.map((type) {
              final existing = ref.read(watchComplicationsNotifierProvider);
              final alreadyAdded = existing.containsKey(type);
              
              return ListTile(
                leading: Text(
                  type.iconEmoji,
                  style: const TextStyle(fontSize: 24),
                ),
                title: Text(type.displayName),
                subtitle: Text(type.shortName),
                enabled: !alreadyAdded,
                trailing: alreadyAdded
                    ? const Icon(Icons.check, color: Colors.green)
                    : null,
                onTap: alreadyAdded ? null : () => _addComplication(type),
              );
            }).toList(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }

  Future<void> _addComplication(WatchComplicationType type) async {
    Navigator.of(context).pop();
    
    final complication = WatchComplication(
      id: type.name,
      title: type.displayName,
      type: type,
      value: '...',
      refreshIntervalSeconds: type.defaultRefreshInterval,
      lastUpdated: DateTime.now(),
    );
    
    final notifier = ref.read(watchComplicationsNotifierProvider.notifier);
    final success = await notifier.updateComplication(complication);
    
    if (success && mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Added "${type.displayName}" complication'),
          backgroundColor: Colors.green,
        ),
      );
      
      // Update with initial data
      await _updateComplicationData(complication);
    }
  }
}