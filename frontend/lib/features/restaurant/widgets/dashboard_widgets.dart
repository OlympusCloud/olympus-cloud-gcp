import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';

import '../../../core/network/services/restaurant_api_service.dart';
import '../../../core/auth/auth_controller.dart';

/// Kitchen Display System widget showing active orders
class KitchenDisplayWidget extends ConsumerStatefulWidget {
  const KitchenDisplayWidget({
    super.key,
    this.height = 300,
  });
  
  final double height;

  @override
  ConsumerState<KitchenDisplayWidget> createState() => _KitchenDisplayWidgetState();
}

class _KitchenDisplayWidgetState extends ConsumerState<KitchenDisplayWidget> {
  List<KitchenOrder> _orders = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadOrders();
  }

  Future<void> _loadOrders() async {
    try {
      setState(() {
        _isLoading = true;
        _error = null;
      });

      final tenantId = ref.read(currentTenantProvider);
      if (tenantId == null) {
        setState(() {
          _error = 'No tenant ID available';
          _isLoading = false;
        });
        return;
      }

      final restaurantApi = ref.read(restaurantApiServiceProvider);
      final orders = await restaurantApi.getKitchenOrders(
        tenantId: tenantId,
        locationId: 'default', // TODO: Get from location provider
      );

      setState(() {
        _orders = orders;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Card(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Icon(
                  Icons.restaurant_menu,
                  color: theme.colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'Kitchen Orders',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: _loadOrders,
                  icon: const Icon(Icons.refresh),
                ),
              ],
            ),
          ),
          Expanded(
            child: _buildContent(theme),
          ),
        ],
      ),
    );
  }

  Widget _buildContent(ThemeData theme) {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_error != null) {
      return Center(
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
              'Failed to load orders',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 8),
            Text(
              _error!,
              style: theme.textTheme.bodySmall,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadOrders,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_orders.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.check_circle_outline,
              size: 48,
              color: theme.colorScheme.primary,
            ),
            const SizedBox(height: 16),
            Text(
              'All caught up!',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 8),
            Text(
              'No pending orders in the kitchen',
              style: theme.textTheme.bodySmall,
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      itemCount: _orders.length,
      itemBuilder: (context, index) {
        final order = _orders[index];
        return _buildOrderCard(order, theme);
      },
    );
  }

  Widget _buildOrderCard(KitchenOrder order, ThemeData theme) {
    final urgencyColor = _getUrgencyColor(order.waitTimeMinutes, theme);
    
    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            // Wait time indicator
            Container(
              width: 4,
              height: 60,
              decoration: BoxDecoration(
                color: urgencyColor,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            
            const SizedBox(width: 12),
            
            // Order details
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        'Order #${order.id.substring(0, 8)}',
                        style: theme.textTheme.titleSmall?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const Spacer(),
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: _getStatusColor(order.status, theme),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          order.status.toUpperCase(),
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ],
                  ),
                  
                  const SizedBox(height: 4),
                  
                  Text(
                    '${order.items.length} items • \$${order.totalAmount.toStringAsFixed(2)}',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: theme.colorScheme.onSurface.withOpacity(0.7),
                    ),
                  ),
                  
                  const SizedBox(height: 8),
                  
                  // Items preview
                  Text(
                    order.items.map((item) => item['name'] ?? 'Item').take(3).join(', '),
                    style: theme.textTheme.bodyMedium,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ),
            ),
            
            const SizedBox(width: 12),
            
            // Wait time
            Column(
              children: [
                Text(
                  '${order.waitTimeMinutes}m',
                  style: theme.textTheme.titleLarge?.copyWith(
                    color: urgencyColor,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Text(
                  'wait',
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: theme.colorScheme.onSurface.withOpacity(0.7),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Color _getUrgencyColor(int waitTimeMinutes, ThemeData theme) {
    if (waitTimeMinutes >= 30) return Colors.red;
    if (waitTimeMinutes >= 20) return Colors.orange;
    if (waitTimeMinutes >= 10) return Colors.yellow.shade700;
    return theme.colorScheme.primary;
  }

  Color _getStatusColor(String status, ThemeData theme) {
    switch (status.toLowerCase()) {
      case 'pending':
        return Colors.orange;
      case 'preparing':
        return Colors.blue;
      case 'ready':
        return Colors.green;
      case 'completed':
        return theme.colorScheme.primary;
      default:
        return Colors.grey;
    }
  }
}

/// Table status overview widget
class TableStatusWidget extends ConsumerStatefulWidget {
  const TableStatusWidget({
    super.key,
    this.height = 250,
  });
  
  final double height;

  @override
  ConsumerState<TableStatusWidget> createState() => _TableStatusWidgetState();
}

class _TableStatusWidgetState extends ConsumerState<TableStatusWidget> {
  TableStatusSummary? _tableStatus;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadTableStatus();
  }

  Future<void> _loadTableStatus() async {
    try {
      setState(() {
        _isLoading = true;
        _error = null;
      });

      final tenantId = ref.read(currentTenantProvider);
      if (tenantId == null) {
        setState(() {
          _error = 'No tenant ID available';
          _isLoading = false;
        });
        return;
      }

      final restaurantApi = ref.read(restaurantApiServiceProvider);
      final status = await restaurantApi.getTableStatus(
        tenantId: tenantId,
        locationId: 'default', // TODO: Get from location provider
      );

      setState(() {
        _tableStatus = status;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Card(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Icon(
                  Icons.table_restaurant,
                  color: theme.colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'Table Status',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: _loadTableStatus,
                  icon: const Icon(Icons.refresh),
                ),
              ],
            ),
          ),
          Expanded(
            child: _buildContent(theme),
          ),
        ],
      ),
    );
  }

  Widget _buildContent(ThemeData theme) {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_error != null) {
      return Center(
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
              'Failed to load table status',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadTableStatus,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    final tableStatus = _tableStatus!;
    final total = tableStatus.available + 
                  tableStatus.occupied + 
                  tableStatus.reserved + 
                  tableStatus.needsCleaning;

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      child: Column(
        children: [
          // Status summary cards
          Row(
            children: [
              Expanded(
                child: _buildStatusCard(
                  'Available',
                  tableStatus.available,
                  total,
                  Colors.green,
                  theme,
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: _buildStatusCard(
                  'Occupied',
                  tableStatus.occupied,
                  total,
                  Colors.red,
                  theme,
                ),
              ),
            ],
          ),
          
          const SizedBox(height: 8),
          
          Row(
            children: [
              Expanded(
                child: _buildStatusCard(
                  'Reserved',
                  tableStatus.reserved,
                  total,
                  Colors.orange,
                  theme,
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: _buildStatusCard(
                  'Cleaning',
                  tableStatus.needsCleaning,
                  total,
                  Colors.blue,
                  theme,
                ),
              ),
            ],
          ),
          
          const SizedBox(height: 16),
          
          // Pie chart
          Expanded(
            child: PieChart(
              PieChartData(
                sectionsSpace: 0,
                centerSpaceRadius: 40,
                sections: [
                  PieChartSectionData(
                    color: Colors.green,
                    value: tableStatus.available.toDouble(),
                    title: '${tableStatus.available}',
                    radius: 50,
                    titleStyle: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  PieChartSectionData(
                    color: Colors.red,
                    value: tableStatus.occupied.toDouble(),
                    title: '${tableStatus.occupied}',
                    radius: 50,
                    titleStyle: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  PieChartSectionData(
                    color: Colors.orange,
                    value: tableStatus.reserved.toDouble(),
                    title: '${tableStatus.reserved}',
                    radius: 50,
                    titleStyle: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  PieChartSectionData(
                    color: Colors.blue,
                    value: tableStatus.needsCleaning.toDouble(),
                    title: '${tableStatus.needsCleaning}',
                    radius: 50,
                    titleStyle: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
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

  Widget _buildStatusCard(
    String label,
    int count,
    int total,
    Color color,
    ThemeData theme,
  ) {
    final percentage = total > 0 ? (count / total * 100).round() : 0;
    
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Column(
        children: [
          Text(
            count.toString(),
            style: theme.textTheme.headlineSmall?.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
            ),
          ),
          Text(
            label,
            style: theme.textTheme.labelMedium?.copyWith(
              color: color,
            ),
          ),
          Text(
            '$percentage%',
            style: theme.textTheme.labelSmall?.copyWith(
              color: theme.colorScheme.onSurface.withOpacity(0.7),
            ),
          ),
        ],
      ),
    );
  }
}