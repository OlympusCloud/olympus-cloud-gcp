import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';

import '../../../core/network/services/restaurant_api_service.dart';
import '../../../core/auth/auth_controller.dart';

/// Service metrics widget showing wait times, prep times, etc.
class ServiceMetricsWidget extends ConsumerStatefulWidget {
  const ServiceMetricsWidget({
    super.key,
    this.height = 200,
  });
  
  final double height;

  @override
  ConsumerState<ServiceMetricsWidget> createState() => _ServiceMetricsWidgetState();
}

class _ServiceMetricsWidgetState extends ConsumerState<ServiceMetricsWidget> {
  RestaurantAnalytics? _analytics;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadAnalytics();
  }

  Future<void> _loadAnalytics() async {
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
      final analytics = await restaurantApi.getAnalytics(
        tenantId: tenantId,
      );

      setState(() {
        _analytics = analytics;
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
                  Icons.speed,
                  color: theme.colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'Service Metrics',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: _loadAnalytics,
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
              'Failed to load metrics',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadAnalytics,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    final analytics = _analytics!;

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      child: Row(
        children: [
          Expanded(
            child: _buildMetricCard(
              'Avg Wait Time',
              '${analytics.averageWaitTime.toStringAsFixed(1)}m',
              Icons.access_time,
              _getPerformanceColor(analytics.averageWaitTime, 15, true),
              theme,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: _buildMetricCard(
              'Prep Time',
              '${analytics.averagePrepTime.toStringAsFixed(1)}m',
              Icons.restaurant,
              _getPerformanceColor(analytics.averagePrepTime, 20, true),
              theme,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: _buildMetricCard(
              'Order Accuracy',
              '${(analytics.orderAccuracyRate * 100).toStringAsFixed(1)}%',
              Icons.check_circle,
              _getPerformanceColor(analytics.orderAccuracyRate, 0.95, false),
              theme,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMetricCard(
    String label,
    String value,
    IconData icon,
    Color color,
    ThemeData theme,
  ) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            icon,
            size: 32,
            color: color,
          ),
          const SizedBox(height: 8),
          Text(
            value,
            style: theme.textTheme.headlineSmall?.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            label,
            style: theme.textTheme.labelMedium?.copyWith(
              color: theme.colorScheme.onSurface.withOpacity(0.7),
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Color _getPerformanceColor(double value, double threshold, bool lowerIsBetter) {
    if (lowerIsBetter) {
      if (value <= threshold * 0.8) return Colors.green;
      if (value <= threshold) return Colors.orange;
      return Colors.red;
    } else {
      if (value >= threshold) return Colors.green;
      if (value >= threshold * 0.8) return Colors.orange;
      return Colors.red;
    }
  }
}

/// Revenue analytics widget
class RevenueAnalyticsWidget extends ConsumerStatefulWidget {
  const RevenueAnalyticsWidget({
    super.key,
    this.height = 300,
  });
  
  final double height;

  @override
  ConsumerState<RevenueAnalyticsWidget> createState() => _RevenueAnalyticsWidgetState();
}

class _RevenueAnalyticsWidgetState extends ConsumerState<RevenueAnalyticsWidget> {
  RestaurantAnalytics? _analytics;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadAnalytics();
  }

  Future<void> _loadAnalytics() async {
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
      final analytics = await restaurantApi.getAnalytics(
        tenantId: tenantId,
      );

      setState(() {
        _analytics = analytics;
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
                  Icons.attach_money,
                  color: theme.colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'Revenue Analytics',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: _loadAnalytics,
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
              'Failed to load revenue data',
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadAnalytics,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    final analytics = _analytics!;

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      child: Column(
        children: [
          // Revenue metrics
          Row(
            children: [
              Expanded(
                child: _buildRevenueCard(
                  'Per Table',
                  '\$${analytics.revenuePerTable.toStringAsFixed(2)}',
                  Icons.table_restaurant,
                  theme,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildRevenueCard(
                  'Per Seat',
                  '\$${analytics.revenuePerSeat.toStringAsFixed(2)}',
                  Icons.event_seat,
                  theme,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildRevenueCard(
                  'Avg Check',
                  '\$${analytics.averageCheckSize.toStringAsFixed(2)}',
                  Icons.receipt,
                  theme,
                ),
              ),
            ],
          ),
          
          const SizedBox(height: 16),
          
          // Efficiency metrics
          Row(
            children: [
              Expanded(
                child: _buildEfficiencyCard(
                  'Kitchen Efficiency',
                  '${(analytics.kitchenEfficiency * 100).toStringAsFixed(1)}%',
                  analytics.kitchenEfficiency,
                  theme,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildEfficiencyCard(
                  'Server Efficiency',
                  '${(analytics.serverEfficiency * 100).toStringAsFixed(1)}%',
                  analytics.serverEfficiency,
                  theme,
                ),
              ),
            ],
          ),
          
          if (analytics.topMenuItems.isNotEmpty) ...[
            const SizedBox(height: 16),
            
            // Top menu items
            Align(
              alignment: Alignment.centerLeft,
              child: Text(
                'Top Menu Items',
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            
            const SizedBox(height: 8),
            
            Expanded(
              child: ListView.builder(
                itemCount: analytics.topMenuItems.take(3).length,
                itemBuilder: (context, index) {
                  final item = analytics.topMenuItems[index];
                  return ListTile(
                    dense: true,
                    leading: CircleAvatar(
                      radius: 12,
                      backgroundColor: theme.colorScheme.primary,
                      child: Text(
                        '${index + 1}',
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                    title: Text(
                      item['name'] ?? 'Unknown Item',
                      style: theme.textTheme.bodyMedium,
                    ),
                    trailing: Text(
                      '${item['count'] ?? 0} sold',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurface.withOpacity(0.7),
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildRevenueCard(
    String label,
    String value,
    IconData icon,
    ThemeData theme,
  ) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: theme.colorScheme.primaryContainer.withOpacity(0.3),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        children: [
          Icon(
            icon,
            size: 24,
            color: theme.colorScheme.primary,
          ),
          const SizedBox(height: 8),
          Text(
            value,
            style: theme.textTheme.titleMedium?.copyWith(
              color: theme.colorScheme.primary,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            label,
            style: theme.textTheme.labelSmall?.copyWith(
              color: theme.colorScheme.onSurface.withOpacity(0.7),
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Widget _buildEfficiencyCard(
    String label,
    String value,
    double efficiency,
    ThemeData theme,
  ) {
    final color = _getEfficiencyColor(efficiency);
    
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Column(
        children: [
          CircularProgressIndicator(
            value: efficiency,
            backgroundColor: color.withOpacity(0.2),
            valueColor: AlwaysStoppedAnimation(color),
            strokeWidth: 6,
          ),
          const SizedBox(height: 8),
          Text(
            value,
            style: theme.textTheme.titleMedium?.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            label,
            style: theme.textTheme.labelSmall?.copyWith(
              color: theme.colorScheme.onSurface.withOpacity(0.7),
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Color _getEfficiencyColor(double efficiency) {
    if (efficiency >= 0.9) return Colors.green;
    if (efficiency >= 0.7) return Colors.orange;
    return Colors.red;
  }
}

/// Quick stats overview widget
class QuickStatsWidget extends ConsumerStatefulWidget {
  const QuickStatsWidget({
    super.key,
    this.height = 120,
  });
  
  final double height;

  @override
  ConsumerState<QuickStatsWidget> createState() => _QuickStatsWidgetState();
}

class _QuickStatsWidgetState extends ConsumerState<QuickStatsWidget> {
  RestaurantAnalytics? _analytics;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadAnalytics();
  }

  Future<void> _loadAnalytics() async {
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
      final analytics = await restaurantApi.getAnalytics(
        tenantId: tenantId,
      );

      setState(() {
        _analytics = analytics;
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

    if (_isLoading) {
      return Card(
        child: Container(
          height: widget.height,
          child: const Center(child: CircularProgressIndicator()),
        ),
      );
    }

    if (_error != null) {
      return Card(
        child: Container(
          height: widget.height,
          padding: const EdgeInsets.all(16),
          child: Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  Icons.error_outline,
                  color: theme.colorScheme.error,
                ),
                const SizedBox(height: 8),
                Text(
                  'Failed to load stats',
                  style: theme.textTheme.bodySmall,
                ),
              ],
            ),
          ),
        ),
      );
    }

    final analytics = _analytics!;

    return Card(
      child: Container(
        height: widget.height,
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            Expanded(
              child: _buildQuickStat(
                'Table Utilization',
                '${(analytics.tableUtilizationRate * 100).toStringAsFixed(1)}%',
                Icons.table_view,
                theme,
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: _buildQuickStat(
                'Avg Dining Time',
                '${analytics.averageDiningDuration.toStringAsFixed(0)}m',
                Icons.schedule,
                theme,
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: _buildQuickStat(
                'Table Turnover',
                '${analytics.tableTransferRate.toStringAsFixed(1)}x',
                Icons.sync,
                theme,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildQuickStat(
    String label,
    String value,
    IconData icon,
    ThemeData theme,
  ) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Icon(
          icon,
          size: 24,
          color: theme.colorScheme.primary,
        ),
        const SizedBox(height: 8),
        Text(
          value,
          style: theme.textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.bold,
            color: theme.colorScheme.primary,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: theme.textTheme.labelMedium?.copyWith(
            color: theme.colorScheme.onSurface.withOpacity(0.7),
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}