import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';

/// Advanced analytics dashboard with interactive charts and metrics
class AnalyticsDashboard extends ConsumerStatefulWidget {
  const AnalyticsDashboard({super.key});

  @override
  ConsumerState<AnalyticsDashboard> createState() => _AnalyticsDashboardState();
}

class _AnalyticsDashboardState extends ConsumerState<AnalyticsDashboard>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  DateTimeRange _selectedDateRange = DateTimeRange(
    start: DateTime.now().subtract(const Duration(days: 30)),
    end: DateTime.now(),
  );

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

    return Scaffold(
      appBar: AppBar(
        title: const Text('Analytics Dashboard'),
        elevation: 0,
        backgroundColor: theme.colorScheme.surface,
        foregroundColor: theme.colorScheme.onSurface,
        actions: [
          IconButton(
            icon: const Icon(Icons.date_range),
            onPressed: _selectDateRange,
            tooltip: 'Select Date Range',
          ),
          IconButton(
            icon: const Icon(Icons.download),
            onPressed: _exportData,
            tooltip: 'Export Data',
          ),
          PopupMenuButton<String>(
            icon: const Icon(Icons.more_vert),
            onSelected: _handleMenuAction,
            itemBuilder: (context) => [
              const PopupMenuItem(
                value: 'refresh',
                child: Row(
                  children: [
                    Icon(Icons.refresh),
                    SizedBox(width: 8),
                    Text('Refresh Data'),
                  ],
                ),
              ),
              const PopupMenuItem(
                value: 'customize',
                child: Row(
                  children: [
                    Icon(Icons.settings),
                    SizedBox(width: 8),
                    Text('Customize Dashboard'),
                  ],
                ),
              ),
            ],
          ),
        ],
        bottom: TabBar(
          controller: _tabController,
          indicatorColor: theme.colorScheme.primary,
          labelColor: theme.colorScheme.primary,
          unselectedLabelColor: theme.colorScheme.onSurface.withValues(alpha: 0.6),
          tabs: const [
            Tab(text: 'Overview', icon: Icon(Icons.dashboard)),
            Tab(text: 'Sales', icon: Icon(Icons.trending_up)),
            Tab(text: 'Customers', icon: Icon(Icons.people)),
            Tab(text: 'Products', icon: Icon(Icons.inventory)),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildOverviewTab(),
          _buildSalesTab(),
          _buildCustomersTab(),
          _buildProductsTab(),
        ],
      ),
    );
  }

  Widget _buildOverviewTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Key Metrics Cards
          _buildKeyMetricsSection(),
          const SizedBox(height: 24),
          
          // Revenue Chart
          _buildSectionTitle('Revenue Trends'),
          const SizedBox(height: 16),
          _buildRevenueChart(),
          const SizedBox(height: 24),
          
          // Performance Indicators
          _buildSectionTitle('Performance Indicators'),
          const SizedBox(height: 16),
          _buildPerformanceIndicators(),
          const SizedBox(height: 24),
          
          // Recent Activity
          _buildSectionTitle('Recent Activity'),
          const SizedBox(height: 16),
          _buildRecentActivityList(),
        ],
      ),
    );
  }

  Widget _buildKeyMetricsSection() {
    return LayoutBuilder(
      builder: (context, constraints) {
        final isDesktop = constraints.maxWidth > 800;
        final crossAxisCount = isDesktop ? 4 : 2;
        
        return GridView.count(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          crossAxisCount: crossAxisCount,
          crossAxisSpacing: 16,
          mainAxisSpacing: 16,
          childAspectRatio: 1.5,
          children: [
            _buildMetricCard(
              'Total Revenue',
              '\$124,590',
              '+12.5%',
              Icons.attach_money,
              Colors.green,
              Colors.green.withValues(alpha: 0.1),
            ),
            _buildMetricCard(
              'Orders',
              '2,847',
              '+8.2%',
              Icons.shopping_cart,
              Colors.blue,
              Colors.blue.withValues(alpha: 0.1),
            ),
            _buildMetricCard(
              'Customers',
              '1,429',
              '+15.7%',
              Icons.people,
              Colors.purple,
              Colors.purple.withValues(alpha: 0.1),
            ),
            _buildMetricCard(
              'Avg Order Value',
              '\$87.45',
              '-2.1%',
              Icons.analytics,
              Colors.orange,
              Colors.orange.withValues(alpha: 0.1),
            ),
          ],
        );
      },
    );
  }

  Widget _buildMetricCard(
    String title,
    String value,
    String change,
    IconData icon,
    Color accentColor,
    Color backgroundColor,
  ) {
    final isPositive = change.startsWith('+');
    final changeColor = isPositive ? Colors.green : Colors.red;
    final changeIcon = isPositive ? Icons.trending_up : Icons.trending_down;

    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: backgroundColor,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Icon(icon, color: accentColor, size: 20),
                ),
                Row(
                  children: [
                    Icon(changeIcon, color: changeColor, size: 16),
                    const SizedBox(width: 4),
                    Text(
                      change,
                      style: TextStyle(
                        color: changeColor,
                        fontWeight: FontWeight.bold,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ],
            ),
            const Spacer(),
            Text(
              title,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
              ),
            ),
            const SizedBox(height: 4),
            Text(
              value,
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSectionTitle(String title) {
    return Text(
      title,
      style: Theme.of(context).textTheme.titleLarge?.copyWith(
        fontWeight: FontWeight.bold,
      ),
    );
  }

  Widget _buildRevenueChart() {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: SizedBox(
          height: 300,
          child: LineChart(
            LineChartData(
              lineBarsData: [
                LineChartBarData(
                  spots: _generateRevenueData(),
                  isCurved: true,
                  color: Theme.of(context).colorScheme.primary,
                  barWidth: 3,
                  dotData: const FlDotData(show: false),
                  belowBarData: BarAreaData(
                    show: true,
                    color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.1),
                  ),
                ),
              ],
              titlesData: FlTitlesData(
                leftTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 60,
                    getTitlesWidget: (value, meta) {
                      return Text(
                        '\$${(value / 1000).toStringAsFixed(0)}K',
                        style: Theme.of(context).textTheme.bodySmall,
                      );
                    },
                  ),
                ),
                bottomTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 30,
                    getTitlesWidget: (value, meta) {
                      final date = DateTime.now().subtract(Duration(days: (30 - value).toInt()));
                      return Text(
                        '${date.day}/${date.month}',
                        style: Theme.of(context).textTheme.bodySmall,
                      );
                    },
                  ),
                ),
                rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
              ),
              gridData: FlGridData(
                show: true,
                drawHorizontalLine: true,
                drawVerticalLine: false,
                horizontalInterval: 5000,
                getDrawingHorizontalLine: (value) {
                  return FlLine(
                    color: Theme.of(context).colorScheme.outline.withValues(alpha: 0.2),
                    strokeWidth: 1,
                  );
                },
              ),
              borderData: FlBorderData(show: false),
              lineTouchData: LineTouchData(
                touchTooltipData: LineTouchTooltipData(
                  getTooltipColor: (touchedSpot) => Theme.of(context).colorScheme.surface,
                  getTooltipItems: (touchedSpots) {
                    return touchedSpots.map((spot) {
                      return LineTooltipItem(
                        '\$${spot.y.toStringAsFixed(0)}',
                        TextStyle(
                          color: Theme.of(context).colorScheme.onSurface,
                          fontWeight: FontWeight.bold,
                        ),
                      );
                    }).toList();
                  },
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildPerformanceIndicators() {
    return Row(
      children: [
        Expanded(
          child: _buildPerformanceCard(
            'Conversion Rate',
            '3.2%',
            0.32,
            Colors.green,
          ),
        ),
        const SizedBox(width: 16),
        Expanded(
          child: _buildPerformanceCard(
            'Customer Satisfaction',
            '4.8/5',
            0.96,
            Colors.blue,
          ),
        ),
        const SizedBox(width: 16),
        Expanded(
          child: _buildPerformanceCard(
            'Return Rate',
            '2.1%',
            0.21,
            Colors.orange,
          ),
        ),
      ],
    );
  }

  Widget _buildPerformanceCard(
    String title,
    String value,
    double progress,
    Color color,
  ) {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              value,
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
                color: color,
              ),
            ),
            const SizedBox(height: 12),
            LinearProgressIndicator(
              value: progress,
              color: color,
              backgroundColor: color.withValues(alpha: 0.2),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildRecentActivityList() {
    final activities = [
      ('New order #2847 received', '2 minutes ago', Icons.shopping_cart, Colors.green),
      ('Customer John Doe registered', '15 minutes ago', Icons.person_add, Colors.blue),
      ('Product "Premium Coffee" low stock', '1 hour ago', Icons.warning, Colors.orange),
      ('Payment of \$245.67 processed', '2 hours ago', Icons.payment, Colors.green),
      ('Review submitted for "Latte"', '3 hours ago', Icons.star, Colors.purple),
    ];

    return Card(
      elevation: 2,
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: activities.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final (message, time, icon, color) = activities[index];
          return ListTile(
            leading: Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: color.withValues(alpha: 0.1),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(icon, color: color, size: 20),
            ),
            title: Text(message),
            subtitle: Text(time),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              // Handle activity tap
            },
          );
        },
      ),
    );
  }

  Widget _buildSalesTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildSectionTitle('Sales Performance'),
          const SizedBox(height: 16),
          
          // Sales metrics
          Row(
            children: [
              Expanded(
                child: _buildMetricCard(
                  'Daily Sales',
                  '\$4,234',
                  '+5.2%',
                  Icons.today,
                  Colors.blue,
                  Colors.blue.withValues(alpha: 0.1),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: _buildMetricCard(
                  'Weekly Sales',
                  '\$28,456',
                  '+12.8%',
                  Icons.date_range,
                  Colors.green,
                  Colors.green.withValues(alpha: 0.1),
                ),
              ),
            ],
          ),
          const SizedBox(height: 24),
          
          // Sales by category chart
          _buildSectionTitle('Sales by Category'),
          const SizedBox(height: 16),
          _buildSalesByCategoryChart(),
          const SizedBox(height: 24),
          
          // Top selling products
          _buildSectionTitle('Top Selling Products'),
          const SizedBox(height: 16),
          _buildTopProductsList(),
        ],
      ),
    );
  }

  Widget _buildSalesByCategoryChart() {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: SizedBox(
          height: 300,
          child: PieChart(
            PieChartData(
              sections: [
                PieChartSectionData(
                  value: 35,
                  title: 'Coffee\n35%',
                  color: Colors.brown,
                  radius: 100,
                  titleStyle: const TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                PieChartSectionData(
                  value: 25,
                  title: 'Pastries\n25%',
                  color: Colors.orange,
                  radius: 100,
                  titleStyle: const TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                PieChartSectionData(
                  value: 20,
                  title: 'Sandwiches\n20%',
                  color: Colors.green,
                  radius: 100,
                  titleStyle: const TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                PieChartSectionData(
                  value: 20,
                  title: 'Beverages\n20%',
                  color: Colors.blue,
                  radius: 100,
                  titleStyle: const TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
              ],
              sectionsSpace: 2,
              centerSpaceRadius: 40,
              startDegreeOffset: -90,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTopProductsList() {
    final products = [
      ('Cappuccino', '\$4.50', '234 sold', 0.8),
      ('Croissant', '\$3.25', '189 sold', 0.65),
      ('Latte', '\$4.75', '167 sold', 0.6),
      ('Americano', '\$3.00', '145 sold', 0.5),
      ('Muffin', '\$2.75', '123 sold', 0.42),
    ];

    return Card(
      elevation: 2,
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: products.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final (name, price, sold, progress) = products[index];
          return ListTile(
            leading: CircleAvatar(
              backgroundColor: Theme.of(context).colorScheme.primary.withValues(alpha: 0.1),
              child: Text(
                name[0],
                style: TextStyle(
                  color: Theme.of(context).colorScheme.primary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            title: Text(name),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('$price • $sold'),
                const SizedBox(height: 4),
                LinearProgressIndicator(
                  value: progress,
                  backgroundColor: Theme.of(context).colorScheme.outline.withValues(alpha: 0.2),
                ),
              ],
            ),
            isThreeLine: true,
          );
        },
      ),
    );
  }

  Widget _buildCustomersTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildSectionTitle('Customer Insights'),
          const SizedBox(height: 16),
          
          // Customer metrics
          Row(
            children: [
              Expanded(
                child: _buildMetricCard(
                  'New Customers',
                  '127',
                  '+18.3%',
                  Icons.person_add,
                  Colors.blue,
                  Colors.blue.withValues(alpha: 0.1),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: _buildMetricCard(
                  'Returning Customers',
                  '892',
                  '+7.1%',
                  Icons.repeat,
                  Colors.green,
                  Colors.green.withValues(alpha: 0.1),
                ),
              ),
            ],
          ),
          const SizedBox(height: 24),
          
          // Customer satisfaction chart
          _buildSectionTitle('Customer Satisfaction Trends'),
          const SizedBox(height: 16),
          _buildCustomerSatisfactionChart(),
          const SizedBox(height: 24),
          
          // Customer segments
          _buildSectionTitle('Customer Segments'),
          const SizedBox(height: 16),
          _buildCustomerSegments(),
        ],
      ),
    );
  }

  Widget _buildCustomerSatisfactionChart() {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: SizedBox(
          height: 300,
          child: BarChart(
            BarChartData(
              barGroups: _generateSatisfactionData(),
              titlesData: FlTitlesData(
                leftTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 40,
                    getTitlesWidget: (value, meta) {
                      return Text(
                        '${value.toInt()}',
                        style: Theme.of(context).textTheme.bodySmall,
                      );
                    },
                  ),
                ),
                bottomTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 30,
                    getTitlesWidget: (value, meta) {
                      const labels = ['1⭐', '2⭐', '3⭐', '4⭐', '5⭐'];
                      return Text(
                        labels[value.toInt()],
                        style: Theme.of(context).textTheme.bodySmall,
                      );
                    },
                  ),
                ),
                rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
              ),
              gridData: const FlGridData(show: false),
              borderData: FlBorderData(show: false),
              maxY: 100,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildCustomerSegments() {
    final segments = [
      ('VIP Customers', '124', Colors.purple, 0.15),
      ('Regular Customers', '567', Colors.blue, 0.68),
      ('New Customers', '143', Colors.green, 0.17),
    ];

    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: segments.map((segment) {
            final (name, count, color, percentage) = segment;
            return Padding(
              padding: const EdgeInsets.only(bottom: 16),
              child: Row(
                children: [
                  Container(
                    width: 12,
                    height: 12,
                    decoration: BoxDecoration(
                      color: color,
                      borderRadius: BorderRadius.circular(6),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Text(
                              name,
                              style: Theme.of(context).textTheme.titleMedium,
                            ),
                            Text(
                              count,
                              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 4),
                        LinearProgressIndicator(
                          value: percentage,
                          color: color,
                          backgroundColor: color.withValues(alpha: 0.2),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            );
          }).toList(),
        ),
      ),
    );
  }

  Widget _buildProductsTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildSectionTitle('Product Performance'),
          const SizedBox(height: 16),
          
          // Product metrics
          Row(
            children: [
              Expanded(
                child: _buildMetricCard(
                  'Total Products',
                  '156',
                  '+12',
                  Icons.inventory,
                  Colors.blue,
                  Colors.blue.withValues(alpha: 0.1),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: _buildMetricCard(
                  'Low Stock',
                  '8',
                  '-3',
                  Icons.warning,
                  Colors.orange,
                  Colors.orange.withValues(alpha: 0.1),
                ),
              ),
            ],
          ),
          const SizedBox(height: 24),
          
          // Stock levels chart
          _buildSectionTitle('Stock Levels Overview'),
          const SizedBox(height: 16),
          _buildStockLevelsChart(),
          const SizedBox(height: 24),
          
          // Category performance
          _buildSectionTitle('Category Performance'),
          const SizedBox(height: 16),
          _buildCategoryPerformance(),
        ],
      ),
    );
  }

  Widget _buildStockLevelsChart() {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: SizedBox(
          height: 300,
          child: BarChart(
            BarChartData(
              barGroups: _generateStockData(),
              titlesData: FlTitlesData(
                leftTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 40,
                    getTitlesWidget: (value, meta) {
                      return Text(
                        '${value.toInt()}',
                        style: Theme.of(context).textTheme.bodySmall,
                      );
                    },
                  ),
                ),
                bottomTitles: AxisTitles(
                  sideTitles: SideTitles(
                    showTitles: true,
                    reservedSize: 40,
                    getTitlesWidget: (value, meta) {
                      const categories = ['Coffee', 'Pastries', 'Sandwiches', 'Beverages', 'Snacks'];
                      return Padding(
                        padding: const EdgeInsets.only(top: 8),
                        child: Text(
                          categories[value.toInt()],
                          style: Theme.of(context).textTheme.bodySmall,
                          textAlign: TextAlign.center,
                        ),
                      );
                    },
                  ),
                ),
                rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
              ),
              gridData: const FlGridData(show: false),
              borderData: FlBorderData(show: false),
              maxY: 200,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildCategoryPerformance() {
    final categories = [
      ('Coffee', '89%', Colors.brown, 0.89),
      ('Pastries', '67%', Colors.orange, 0.67),
      ('Sandwiches', '78%', Colors.green, 0.78),
      ('Beverages', '92%', Colors.blue, 0.92),
      ('Snacks', '45%', Colors.purple, 0.45),
    ];

    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: categories.map((category) {
            final (name, performance, color, percentage) = category;
            return Padding(
              padding: const EdgeInsets.only(bottom: 16),
              child: Row(
                children: [
                  SizedBox(
                    width: 100,
                    child: Text(
                      name,
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                  ),
                  Expanded(
                    child: LinearProgressIndicator(
                      value: percentage,
                      color: color,
                      backgroundColor: color.withValues(alpha: 0.2),
                    ),
                  ),
                  const SizedBox(width: 12),
                  SizedBox(
                    width: 50,
                    child: Text(
                      performance,
                      style: Theme.of(context).textTheme.titleMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                        color: color,
                      ),
                      textAlign: TextAlign.end,
                    ),
                  ),
                ],
              ),
            );
          }).toList(),
        ),
      ),
    );
  }

  // Helper methods for chart data generation
  List<FlSpot> _generateRevenueData() {
    return List.generate(30, (index) {
      final baseValue = 15000 + (index * 200);
      final randomVariation = (index % 3 == 0) ? 2000 : (index % 5 == 0) ? -1000 : 500;
      return FlSpot(index.toDouble(), (baseValue + randomVariation).toDouble());
    });
  }

  List<BarChartGroupData> _generateSatisfactionData() {
    return [
      BarChartGroupData(x: 0, barRods: [BarChartRodData(toY: 5, color: Colors.red)]),
      BarChartGroupData(x: 1, barRods: [BarChartRodData(toY: 12, color: Colors.orange)]),
      BarChartGroupData(x: 2, barRods: [BarChartRodData(toY: 28, color: Colors.yellow)]),
      BarChartGroupData(x: 3, barRods: [BarChartRodData(toY: 45, color: Colors.lightGreen)]),
      BarChartGroupData(x: 4, barRods: [BarChartRodData(toY: 78, color: Colors.green)]),
    ];
  }

  List<BarChartGroupData> _generateStockData() {
    return [
      BarChartGroupData(x: 0, barRods: [BarChartRodData(toY: 125, color: Colors.brown)]),
      BarChartGroupData(x: 1, barRods: [BarChartRodData(toY: 89, color: Colors.orange)]),
      BarChartGroupData(x: 2, barRods: [BarChartRodData(toY: 156, color: Colors.green)]),
      BarChartGroupData(x: 3, barRods: [BarChartRodData(toY: 178, color: Colors.blue)]),
      BarChartGroupData(x: 4, barRods: [BarChartRodData(toY: 67, color: Colors.purple)]),
    ];
  }

  void _selectDateRange() async {
    final pickedRange = await showDateRangePicker(
      context: context,
      firstDate: DateTime.now().subtract(const Duration(days: 365)),
      lastDate: DateTime.now(),
      initialDateRange: _selectedDateRange,
    );

    if (pickedRange != null) {
      setState(() {
        _selectedDateRange = pickedRange;
      });
      // Refresh data with new date range
    }
  }

  void _exportData() {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Exporting analytics data...'),
        duration: Duration(seconds: 2),
      ),
    );
  }

  void _handleMenuAction(String action) {
    switch (action) {
      case 'refresh':
        // Refresh all data
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Refreshing data...')),
        );
        break;
      case 'customize':
        // Show customization options
        _showCustomizationDialog();
        break;
    }
  }

  void _showCustomizationDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Customize Dashboard'),
        content: const Text('Dashboard customization options will be available here'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Save'),
          ),
        ],
      ),
    );
  }
}