import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/orders_provider.dart';
import '../models/order.dart';
import '../../../../shared/presentation/widgets/loading_widgets.dart';
import '../../../../shared/presentation/widgets/natural_language_bar.dart';
import '../../../../core/theme/app_theme.dart';

/// Order management screen showing list of orders with filtering and actions
class OrderManagementScreen extends ConsumerStatefulWidget {
  const OrderManagementScreen({super.key});

  @override
  ConsumerState<OrderManagementScreen> createState() => _OrderManagementScreenState();
}

class _OrderManagementScreenState extends ConsumerState<OrderManagementScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  String _searchQuery = '';
  OrderStatus? _selectedStatus;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final ordersAsync = ref.watch(ordersProvider);
    final activeOrders = ref.watch(activeOrdersProvider);
    final completedOrders = ref.watch(completedOrdersProvider);
    final todaysOrders = ref.watch(todaysOrdersProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Order Management'),
        elevation: 0,
        backgroundColor: theme.colorScheme.surface,
        foregroundColor: theme.colorScheme.onSurface,
        bottom: TabBar(
          controller: _tabController,
          indicatorColor: theme.colorScheme.primary,
          labelColor: theme.colorScheme.primary,
          unselectedLabelColor: theme.colorScheme.onSurface.withValues(alpha: 0.6),
          tabs: [
            Tab(
              text: 'Active (${activeOrders.length})',
              icon: const Icon(Icons.access_time),
            ),
            Tab(
              text: 'Today (${todaysOrders.length})',
              icon: const Icon(Icons.today),
            ),
            Tab(
              text: 'Completed (${completedOrders.length})',
              icon: const Icon(Icons.check_circle),
            ),
            Tab(
              text: 'All Orders',
              icon: const Icon(Icons.list),
            ),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: _showFilterDialog,
          ),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () => ref.read(ordersProvider.notifier).refresh(),
          ),
        ],
      ),
      body: Column(
        children: [
          // Search and Natural Language Bar
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: theme.colorScheme.surface,
              boxShadow: [
                BoxShadow(
                  color: theme.shadowColor.withValues(alpha: 0.1),
                  blurRadius: 4,
                  offset: const Offset(0, 2),
                ),
              ],
            ),
            child: Column(
              children: [
                // Search Bar
                TextField(
                  decoration: InputDecoration(
                    hintText: 'Search orders by number, customer, or item...',
                    prefixIcon: const Icon(Icons.search),
                    suffixIcon: _searchQuery.isNotEmpty
                        ? IconButton(
                            icon: const Icon(Icons.clear),
                            onPressed: () {
                              setState(() => _searchQuery = '');
                              ref.read(ordersProvider.notifier).loadOrders();
                            },
                          )
                        : null,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                      borderSide: BorderSide.none,
                    ),
                    filled: true,
                    fillColor: theme.colorScheme.surfaceContainerHighest,
                  ),
                  onChanged: (value) {
                    setState(() => _searchQuery = value);
                    if (value.isNotEmpty) {
                      ref.read(ordersProvider.notifier).searchOrders(value);
                    } else {
                      ref.read(ordersProvider.notifier).loadOrders();
                    }
                  },
                ),
                const SizedBox(height: 12),
                // Natural Language Bar
                NaturalLanguageBar(
                  onCommand: _handleNaturalLanguageCommand,
                  suggestions: const [
                    'Show pending orders',
                    'Find orders from today',
                    'Show high priority orders',
                    'Create new order',
                  ],
                ),
              ],
            ),
          ),
          // Order List
          Expanded(
            child: TabBarView(
              controller: _tabController,
              children: [
                _buildOrderList(activeOrders, 'No active orders'),
                _buildOrderList(todaysOrders, 'No orders today'),
                _buildOrderList(completedOrders, 'No completed orders'),
                _buildAllOrdersList(ordersAsync),
              ],
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _createNewOrder,
        icon: const Icon(Icons.add),
        label: const Text('New Order'),
      ),
    );
  }

  Widget _buildAllOrdersList(AsyncValue<List<Order>> ordersAsync) {
    return ordersAsync.when(
      data: (orders) => _buildOrderList(orders, 'No orders found'),
      loading: () => const Center(child: LoadingSpinner()),
      error: (error, stackTrace) => Center(
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
              'Failed to load orders',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: 8),
            Text(
              error.toString(),
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () => ref.read(ordersProvider.notifier).refresh(),
              child: const Text('Try Again'),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildOrderList(List<Order> orders, String emptyMessage) {
    if (orders.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.receipt_long_outlined,
              size: 64,
              color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5),
            ),
            const SizedBox(height: 16),
            Text(
              emptyMessage,
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
              ),
            ),
          ],
        ),
      );
    }

    return RefreshIndicator(
      onRefresh: () => ref.read(ordersProvider.notifier).refresh(),
      child: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: orders.length,
        itemBuilder: (context, index) {
          final order = orders[index];
          return _buildOrderCard(order);
        },
      ),
    );
  }

  Widget _buildOrderCard(Order order) {
    final theme = Theme.of(context);
    final statusColor = _getStatusColor(order.status);
    final priorityColor = _getPriorityColor(order.priority);

    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 2,
      child: InkWell(
        onTap: () => _showOrderDetails(order),
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header with order number and status
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'Order ${order.orderNumber}',
                    style: theme.textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  Row(
                    children: [
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: priorityColor.withValues(alpha: 0.1),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(color: priorityColor),
                        ),
                        child: Text(
                          order.priority.name.toUpperCase(),
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: priorityColor,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: statusColor.withValues(alpha: 0.1),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(color: statusColor),
                        ),
                        child: Text(
                          order.status.name.toUpperCase(),
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: statusColor,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 8),
              // Customer and table info
              if (order.customer != null || order.tableNumber != null)
                Row(
                  children: [
                    if (order.customer != null) ...[
                      Icon(
                        Icons.person,
                        size: 16,
                        color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                      ),
                      const SizedBox(width: 4),
                      Text(
                        order.customer!.name,
                        style: theme.textTheme.bodyMedium?.copyWith(
                          color: theme.colorScheme.onSurface.withValues(alpha: 0.8),
                        ),
                      ),
                      if (order.tableNumber != null) const SizedBox(width: 16),
                    ],
                    if (order.tableNumber != null) ...[
                      Icon(
                        Icons.table_restaurant,
                        size: 16,
                        color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                      ),
                      const SizedBox(width: 4),
                      Text(
                        'Table ${order.tableNumber}',
                        style: theme.textTheme.bodyMedium?.copyWith(
                          color: theme.colorScheme.onSurface.withValues(alpha: 0.8),
                        ),
                      ),
                    ],
                  ],
                ),
              const SizedBox(height: 8),
              // Items summary
              Text(
                '${order.items.length} items • \$${order.total.toStringAsFixed(2)}',
                style: theme.textTheme.bodyLarge?.copyWith(
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 4),
              // Time and estimated completion
              Row(
                children: [
                  Icon(
                    Icons.access_time,
                    size: 14,
                    color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                  ),
                  const SizedBox(width: 4),
                  Text(
                    _formatOrderTime(order.createdAt),
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                    ),
                  ),
                  if (order.estimatedCompletionTime != null) ...[
                    const SizedBox(width: 16),
                    Icon(
                      Icons.schedule,
                      size: 14,
                      color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                    ),
                    const SizedBox(width: 4),
                    Text(
                      'ETA: ${_formatOrderTime(order.estimatedCompletionTime!)}',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                      ),
                    ),
                  ],
                ],
              ),
              // Quick actions
              const SizedBox(height: 12),
              Row(
                children: [
                  Expanded(
                    child: _buildQuickActionButton(
                      'View',
                      Icons.visibility,
                      () => _showOrderDetails(order),
                    ),
                  ),
                  const SizedBox(width: 8),
                  if (order.status != OrderStatus.completed &&
                      order.status != OrderStatus.cancelled)
                    Expanded(
                      child: _buildQuickActionButton(
                        _getNextStatusAction(order.status),
                        _getNextStatusIcon(order.status),
                        () => _updateOrderStatus(order),
                      ),
                    ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildQuickActionButton(String label, IconData icon, VoidCallback onPressed) {
    final theme = Theme.of(context);
    return OutlinedButton.icon(
      onPressed: onPressed,
      icon: Icon(icon, size: 16),
      label: Text(label),
      style: OutlinedButton.styleFrom(
        padding: const EdgeInsets.symmetric(vertical: 8),
        side: BorderSide(color: theme.colorScheme.outline),
      ),
    );
  }

  Color _getStatusColor(OrderStatus status) {
    final theme = Theme.of(context);
    switch (status) {
      case OrderStatus.pending:
        return theme.colorScheme.secondary;
      case OrderStatus.confirmed:
        return Colors.blue;
      case OrderStatus.preparing:
        return Colors.orange;
      case OrderStatus.ready:
        return Colors.green;
      case OrderStatus.completed:
        return theme.colorScheme.primary;
      case OrderStatus.cancelled:
        return theme.colorScheme.error;
    }
  }

  Color _getPriorityColor(OrderPriority priority) {
    switch (priority) {
      case OrderPriority.low:
        return Colors.grey;
      case OrderPriority.normal:
        return Colors.blue;
      case OrderPriority.high:
        return Colors.orange;
      case OrderPriority.urgent:
        return Colors.red;
    }
  }

  String _getNextStatusAction(OrderStatus status) {
    switch (status) {
      case OrderStatus.pending:
        return 'Confirm';
      case OrderStatus.confirmed:
        return 'Prepare';
      case OrderStatus.preparing:
        return 'Ready';
      case OrderStatus.ready:
        return 'Complete';
      case OrderStatus.completed:
      case OrderStatus.cancelled:
        return 'View';
    }
  }

  IconData _getNextStatusIcon(OrderStatus status) {
    switch (status) {
      case OrderStatus.pending:
        return Icons.check_circle;
      case OrderStatus.confirmed:
        return Icons.restaurant;
      case OrderStatus.preparing:
        return Icons.done;
      case OrderStatus.ready:
        return Icons.check_circle_outline;
      case OrderStatus.completed:
      case OrderStatus.cancelled:
        return Icons.visibility;
    }
  }

  String _formatOrderTime(DateTime dateTime) {
    final now = DateTime.now();
    final difference = now.difference(dateTime);

    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inMinutes < 60) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inHours < 24) {
      return '${difference.inHours}h ago';
    } else {
      return '${dateTime.day}/${dateTime.month}/${dateTime.year}';
    }
  }

  void _showOrderDetails(Order order) {
    // TODO: Navigate to order details screen
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Order ${order.orderNumber}'),
        content: Text('Order details screen not implemented yet'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  void _updateOrderStatus(Order order) {
    final nextStatus = _getNextStatus(order.status);
    if (nextStatus != null) {
      ref.read(ordersProvider.notifier).updateOrderStatus(order.id, nextStatus);
    }
  }

  OrderStatus? _getNextStatus(OrderStatus currentStatus) {
    switch (currentStatus) {
      case OrderStatus.pending:
        return OrderStatus.confirmed;
      case OrderStatus.confirmed:
        return OrderStatus.preparing;
      case OrderStatus.preparing:
        return OrderStatus.ready;
      case OrderStatus.ready:
        return OrderStatus.completed;
      case OrderStatus.completed:
      case OrderStatus.cancelled:
        return null;
    }
  }

  void _createNewOrder() {
    // TODO: Navigate to create order screen
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Create New Order'),
        content: const Text('Create order screen not implemented yet'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  void _showFilterDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Filter Orders'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              title: const Text('All Statuses'),
              leading: Radio<OrderStatus?>(
                value: null,
                groupValue: _selectedStatus,
                onChanged: (value) {
                  setState(() => _selectedStatus = value);
                  Navigator.of(context).pop();
                  _applyFilters();
                },
              ),
            ),
            ...OrderStatus.values.map((status) => ListTile(
              title: Text(status.name),
              leading: Radio<OrderStatus?>(
                value: status,
                groupValue: _selectedStatus,
                onChanged: (value) {
                  setState(() => _selectedStatus = value);
                  Navigator.of(context).pop();
                  _applyFilters();
                },
              ),
            )),
          ],
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

  void _applyFilters() {
    if (_selectedStatus != null) {
      ref.read(ordersProvider.notifier).filterOrdersByStatus([_selectedStatus!]);
    } else {
      ref.read(ordersProvider.notifier).loadOrders();
    }
  }

  void _handleNaturalLanguageCommand(String command) {
    final lowerCommand = command.toLowerCase();
    
    if (lowerCommand.contains('pending')) {
      _tabController.animateTo(0);
      ref.read(ordersProvider.notifier).filterOrdersByStatus([OrderStatus.pending]);
    } else if (lowerCommand.contains('today')) {
      _tabController.animateTo(1);
    } else if (lowerCommand.contains('completed')) {
      _tabController.animateTo(2);
    } else if (lowerCommand.contains('high priority')) {
      // Filter by high priority orders (would need additional implementation)
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Showing high priority orders')),
      );
    } else if (lowerCommand.contains('create') || lowerCommand.contains('new order')) {
      _createNewOrder();
    } else {
      // Try searching for the command
      setState(() => _searchQuery = command);
      ref.read(ordersProvider.notifier).searchOrders(command);
    }
  }
}