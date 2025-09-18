import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/watch_device.dart';
import '../providers/watch_providers.dart';
import '../../widgets/watch_device_card.dart';
import '../../widgets/watch_capabilities_sheet.dart';
import '../../../../shared/widgets/loading_widgets.dart';

class WatchDevicesScreen extends ConsumerStatefulWidget {
  const WatchDevicesScreen({super.key});

  @override
  ConsumerState<WatchDevicesScreen> createState() => _WatchDevicesScreenState();
}

class _WatchDevicesScreenState extends ConsumerState<WatchDevicesScreen> {
  bool _isScanning = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _checkWatchAvailability();
    });
  }

  Future<void> _checkWatchAvailability() async {
    final isAvailable = await ref.read(watchAvailabilityProvider.future);
    if (!isAvailable && mounted) {
      _showUnsupportedDialog();
    }
  }

  void _showUnsupportedDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Watch Integration Unavailable'),
        content: const Text(
          'Watch integration is not available on this platform or device. '
          'Please check if your device supports watch connectivity.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('OK'),
          ),
        ],
      ),
    );
  }

  Future<void> _scanForDevices() async {
    setState(() => _isScanning = true);
    
    try {
      await ref.read(watchDevicesNotifierProvider.notifier).scanForDevices();
    } finally {
      if (mounted) {
        setState(() => _isScanning = false);
      }
    }
  }

  Future<void> _connectToDevice(WatchDevice device) async {
    final messenger = ScaffoldMessenger.of(context);
    
    try {
      final success = await ref
          .read(watchDevicesNotifierProvider.notifier)
          .connectToDevice(device.id);
      
      if (success) {
        messenger.showSnackBar(
          SnackBar(
            content: Text('Connected to ${device.name}'),
            backgroundColor: Colors.green,
          ),
        );
      } else {
        messenger.showSnackBar(
          SnackBar(
            content: Text('Failed to connect to ${device.name}'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } catch (e) {
      messenger.showSnackBar(
        SnackBar(
          content: Text('Error connecting to ${device.name}: $e'),
          backgroundColor: Colors.red,
        ),
      );
    }
  }

  Future<void> _disconnectFromDevice(WatchDevice device) async {
    final messenger = ScaffoldMessenger.of(context);
    
    try {
      final success = await ref
          .read(watchDevicesNotifierProvider.notifier)
          .disconnectFromDevice(device.id);
      
      if (success) {
        messenger.showSnackBar(
          SnackBar(
            content: Text('Disconnected from ${device.name}'),
            backgroundColor: Colors.orange,
          ),
        );
      } else {
        messenger.showSnackBar(
          SnackBar(
            content: Text('Failed to disconnect from ${device.name}'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } catch (e) {
      messenger.showSnackBar(
        SnackBar(
          content: Text('Error disconnecting from ${device.name}: $e'),
          backgroundColor: Colors.red,
        ),
      );
    }
  }

  void _showDeviceCapabilities(WatchDevice device) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => WatchCapabilitiesSheet(device: device),
    );
  }

  @override
  Widget build(BuildContext context) {
    final devicesAsync = ref.watch(watchDevicesNotifierProvider);
    final connectedDevicesAsync = ref.watch(connectedWatchDevicesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Watch Devices'),
        actions: [
          IconButton(
            onPressed: _isScanning ? null : _scanForDevices,
            icon: _isScanning
                ? const SizedBox(
                    width: 24,
                    height: 24,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const Icon(Icons.refresh),
            tooltip: 'Scan for devices',
          ),
        ],
      ),
      body: devicesAsync.when(
        data: (devices) => _buildDevicesList(devices, connectedDevicesAsync),
        loading: () => const Center(child: LoadingSpinner()),
        error: (error, stack) => _buildErrorState(error),
      ),
    );
  }

  Widget _buildDevicesList(
    List<WatchDevice> devices,
    AsyncValue<List<WatchDevice>> connectedDevicesAsync,
  ) {
    if (devices.isEmpty) {
      return _buildEmptyState();
    }

    final connectedDevices = connectedDevicesAsync.value ?? [];
    final availableDevices = devices.where((d) => !d.status.isConnected).toList();

    return RefreshIndicator(
      onRefresh: _scanForDevices,
      child: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          if (connectedDevices.isNotEmpty) ...[
            Text(
              'Connected Devices',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            ...connectedDevices.map(
              (device) => Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: WatchDeviceCard(
                  device: device,
                  onConnect: () => _connectToDevice(device),
                  onDisconnect: () => _disconnectFromDevice(device),
                  onShowCapabilities: () => _showDeviceCapabilities(device),
                ),
              ),
            ),
            const SizedBox(height: 24),
          ],
          if (availableDevices.isNotEmpty) ...[
            Text(
              'Available Devices',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            ...availableDevices.map(
              (device) => Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: WatchDeviceCard(
                  device: device,
                  onConnect: () => _connectToDevice(device),
                  onDisconnect: () => _disconnectFromDevice(device),
                  onShowCapabilities: () => _showDeviceCapabilities(device),
                ),
              ),
            ),
          ],
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
            Icons.watch_outlined,
            size: 64,
            color: Theme.of(context).colorScheme.outline,
          ),
          const SizedBox(height: 16),
          Text(
            'No Watch Devices Found',
            style: Theme.of(context).textTheme.headlineSmall,
          ),
          const SizedBox(height: 8),
          Text(
            'Make sure your watch is nearby and in pairing mode.',
            style: Theme.of(context).textTheme.bodyMedium,
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: _isScanning ? null : _scanForDevices,
            icon: _isScanning
                ? const SizedBox(
                    width: 16,
                    height: 16,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const Icon(Icons.search),
            label: Text(_isScanning ? 'Scanning...' : 'Scan for Devices'),
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
            'Error Loading Devices',
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
            onPressed: _scanForDevices,
            icon: const Icon(Icons.refresh),
            label: const Text('Try Again'),
          ),
        ],
      ),
    );
  }
}