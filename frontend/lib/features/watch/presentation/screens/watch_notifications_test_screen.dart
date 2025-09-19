import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../models/watch_notification.dart';
import '../../providers/watch_providers.dart';
import '../../../../shared/widgets/custom_form_fields.dart';

class WatchNotificationsTestScreen extends ConsumerStatefulWidget {
  const WatchNotificationsTestScreen({super.key});

  @override
  ConsumerState<WatchNotificationsTestScreen> createState() => _WatchNotificationsTestScreenState();
}

class _WatchNotificationsTestScreenState extends ConsumerState<WatchNotificationsTestScreen> {
  final _formKey = GlobalKey<FormState>();
  final _customerNameController = TextEditingController();
  final _amountController = TextEditingController();
  final _itemNameController = TextEditingController();
  final _employeeNameController = TextEditingController();
  final _serviceController = TextEditingController();
  final _titleController = TextEditingController();
  final _messageController = TextEditingController();

  @override
  void dispose() {
    _customerNameController.dispose();
    _amountController.dispose();
    _itemNameController.dispose();
    _employeeNameController.dispose();
    _serviceController.dispose();
    _titleController.dispose();
    _messageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final connectedDevices = ref.watch(connectedWatchDevicesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Test Watch Notifications'),
      ),
      body: connectedDevices.when(
        data: (devices) {
          if (devices.isEmpty) {
            return _buildNoDevicesState();
          }
          return _buildNotificationTests();
        },
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (error, stack) => _buildErrorState(error),
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
            'Connect a watch device to test notifications.',
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

  Widget _buildNotificationTests() {
    return Form(
      key: _formKey,
      child: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '🛒 Order Notification',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _customerNameController,
                    label: 'Customer Name',
                    hintText: 'Enter customer name',
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter customer name';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _amountController,
                    label: 'Order Amount',
                    hintText: 'Enter order amount',
                    keyboardType: TextInputType.number,
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter order amount';
                      }
                      final amount = double.tryParse(value!);
                      if (amount == null || amount <= 0) {
                        return 'Please enter a valid amount';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _sendOrderNotification,
                      icon: const Icon(Icons.shopping_cart),
                      label: const Text('Send Order Notification'),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '📦 Inventory Alert',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _itemNameController,
                    label: 'Item Name',
                    hintText: 'Enter item name',
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter item name';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _sendInventoryAlert,
                      icon: const Icon(Icons.inventory),
                      label: const Text('Send Inventory Alert'),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '⏰ Shift Reminder',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _employeeNameController,
                    label: 'Employee Name',
                    hintText: 'Enter employee name',
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter employee name';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _sendShiftReminder,
                      icon: const Icon(Icons.schedule),
                      label: const Text('Send Shift Reminder'),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '📅 Appointment Reminder',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _serviceController,
                    label: 'Service Name',
                    hintText: 'Enter service name',
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter service name';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _sendAppointmentReminder,
                      icon: const Icon(Icons.event),
                      label: const Text('Send Appointment Reminder'),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '⚠️ System Alert',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _titleController,
                    label: 'Alert Title',
                    hintText: 'Enter alert title',
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter alert title';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 12),
                  CustomTextField(
                    controller: _messageController,
                    label: 'Alert Message',
                    hintText: 'Enter alert message',
                    maxLines: 3,
                    validator: (value) {
                      if (value?.isEmpty ?? true) {
                        return 'Please enter alert message';
                      }
                      return null;
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: FilledButton.icon(
                      onPressed: _sendSystemAlert,
                      icon: const Icon(Icons.warning),
                      label: const Text('Send System Alert'),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '🎯 Quick Tests',
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 12),
                  SizedBox(
                    width: double.infinity,
                    child: OutlinedButton.icon(
                      onPressed: _sendPaymentNotification,
                      icon: const Icon(Icons.payment),
                      label: const Text('Send Payment Notification'),
                    ),
                  ),
                ],
              ),
            ),
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
            'Error Loading Watch Status',
            style: Theme.of(context).textTheme.headlineSmall,
          ),
          const SizedBox(height: 8),
          Text(
            error.toString(),
            style: Theme.of(context).textTheme.bodyMedium,
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Future<void> _sendOrderNotification() async {
    if (!_formKey.currentState!.validate()) return;

    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendOrderNotification(
      orderId: 'ORDER_${DateTime.now().millisecondsSinceEpoch}',
      customerName: _customerNameController.text,
      amount: double.parse(_amountController.text),
      specialInstructions: 'Extra sauce, no onions',
    );

    _showResult(success, 'Order notification');
    if (success) {
      _customerNameController.clear();
      _amountController.clear();
    }
  }

  Future<void> _sendInventoryAlert() async {
    if (_itemNameController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please enter an item name'),
          backgroundColor: Colors.red,
        ),
      );
      return;
    }

    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendInventoryAlert(
      itemName: _itemNameController.text,
      currentStock: 3,
      minimumStock: 10,
    );

    _showResult(success, 'Inventory alert');
    if (success) {
      _itemNameController.clear();
    }
  }

  Future<void> _sendShiftReminder() async {
    if (_employeeNameController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please enter an employee name'),
          backgroundColor: Colors.red,
        ),
      );
      return;
    }

    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendShiftReminder(
      employeeName: _employeeNameController.text,
      shiftStart: DateTime.now().add(const Duration(minutes: 15)),
      location: 'Main Store',
    );

    _showResult(success, 'Shift reminder');
    if (success) {
      _employeeNameController.clear();
    }
  }

  Future<void> _sendAppointmentReminder() async {
    if (_serviceController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please enter a service name'),
          backgroundColor: Colors.red,
        ),
      );
      return;
    }

    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendAppointmentReminder(
      customerName: 'Sarah Johnson',
      appointmentTime: DateTime.now().add(const Duration(minutes: 10)),
      service: _serviceController.text,
      notes: 'First-time customer',
    );

    _showResult(success, 'Appointment reminder');
    if (success) {
      _serviceController.clear();
    }
  }

  Future<void> _sendSystemAlert() async {
    if (_titleController.text.isEmpty || _messageController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please enter both title and message'),
          backgroundColor: Colors.red,
        ),
      );
      return;
    }

    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendSystemAlert(
      title: _titleController.text,
      message: _messageController.text,
      priority: WatchNotificationPriority.high,
    );

    _showResult(success, 'System alert');
    if (success) {
      _titleController.clear();
      _messageController.clear();
    }
  }

  Future<void> _sendPaymentNotification() async {
    final notifier = ref.read(watchNotificationsNotifierProvider.notifier);
    final success = await notifier.sendPaymentNotification(
      orderId: 'ORDER_${DateTime.now().millisecondsSinceEpoch}',
      amount: 45.99,
      paymentMethod: 'Credit Card',
    );

    _showResult(success, 'Payment notification');
  }

  void _showResult(bool success, String notificationType) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          success
              ? '$notificationType sent successfully!'
              : 'Failed to send $notificationType',
        ),
        backgroundColor: success ? Colors.green : Colors.red,
      ),
    );
  }
}