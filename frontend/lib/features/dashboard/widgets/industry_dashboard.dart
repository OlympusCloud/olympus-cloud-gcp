import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/branding/industry_branding.dart';
import '../../../core/branding/branding_provider.dart';
import '../../restaurant/widgets/restaurant_dashboard.dart';
import '../../retail/widgets/retail_dashboard.dart';
import '../../salon/widgets/salon_dashboard.dart';
import '../../event/widgets/event_dashboard.dart';
import '../../hospitality/widgets/hospitality_dashboard.dart';
import 'default_dashboard.dart';
import 'package:frontend/core/auth/auth_controller.dart';
import 'package:frontend/core/platform/responsive_layout.dart';
import 'package:frontend/core/auth/user_extensions.dart';

/// Industry-specific dashboard widget that adapts based on current branding
class IndustryDashboard extends ConsumerWidget {
  const IndustryDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    
    return switch (branding.industryType) {
      'restaurant' => const RestaurantDashboard(),
      'retail' => const RetailDashboard(),
      'salon' => const SalonDashboard(),
      'event' => const EventDashboard(),
      'hospitality' => const HospitalityDashboard(),
      _ => const DefaultDashboard(),
    };
  }
}

/// Base dashboard widget that provides common structure for all industries
abstract class BaseDashboard extends ConsumerWidget {
  const BaseDashboard({super.key});

  /// Build industry-specific header
  Widget buildHeader(BuildContext context, WidgetRef ref, IndustryBranding branding);
  
  /// Build industry-specific quick stats
  Widget buildQuickStats(BuildContext context, WidgetRef ref, IndustryBranding branding);
  
  /// Build industry-specific main content
  Widget buildMainContent(BuildContext context, WidgetRef ref, IndustryBranding branding);
  
  /// Build industry-specific quick actions
  Widget buildQuickActions(BuildContext context, WidgetRef ref, IndustryBranding branding);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    final user = ref.watch(currentUserProvider);

    return Scaffold(
      appBar: AppBar(
        title: Text('${branding.name} Dashboard'),
        actions: [
          IconButton(
            icon: const Icon(Icons.notifications),
            onPressed: () {},
          ),
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: CircleAvatar(
              child: Text(user?.initials ?? 'A'),
            ),
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Welcome back, ${user?.displayName}!',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: 24),
            _buildMetricGrid(context, branding),
            const SizedBox(height: 24),
            _buildCharts(context, branding),
            const SizedBox(height: 24),
            _buildActionableInsights(context, branding),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricGrid(BuildContext context, IndustryBranding branding) {
    return GridView.count(
      crossAxisCount: ResponsiveLayout.isMobile(context) ? 2 : 4,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      crossAxisSpacing: 16,
      mainAxisSpacing: 16,
      children: [
        _MetricCard(
          title: 'Sales',
          value: '\$12,450',
          icon: Icons.trending_up,
          color: branding.primaryColor,
        ),
        _MetricCard(
          title: 'Customers',
          value: '1,234',
          icon: Icons.people,
          color: Colors.orange,
        ),
        _MetricCard(
          title: 'Orders',
          value: '567',
          icon: Icons.receipt,
          color: Colors.green,
        ),
        _MetricCard(
          title: 'Satisfaction',
          value: '98%',
          icon: Icons.sentiment_satisfied,
          color: Colors.purple,
        ),
      ],
    );
  }

  Widget _buildCharts(BuildContext context, IndustryBranding branding) {
    return LayoutBuilder(
      builder: (context, constraints) {
        if (constraints.maxWidth < 600) {
          return Column(
            children: [
              _SalesChart(branding: branding),
              const SizedBox(height: 24),
              _CategoryPieChart(branding: branding),
            ],
          );
        }
        return Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              flex: 2,
              child: _SalesChart(branding: branding),
            ),
            const SizedBox(width: 24),
            Expanded(
              flex: 1,
              child: _CategoryPieChart(branding: branding),
            ),
          ],
        );
      },
    );
  }

  Widget _buildActionableInsights(
      BuildContext context, IndustryBranding branding) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Actionable Insights',
                style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 16),
            _InsightTile(
              icon: Icons.inventory,
              title: 'Low Stock Alert',
              subtitle: 'Product "Espresso Beans" is running low.',
              color: Colors.red.withAlpha(51),
            ),
            const Divider(),
            _InsightTile(
              icon: Icons.star,
              title: 'Top Performing Product',
              subtitle: '"Iced Latte" is your best seller this week.',
              color: Colors.green.withAlpha(51),
            ),
            const Divider(),
            _InsightTile(
              icon: Icons.access_time,
              title: 'Peak Hours',
              subtitle: 'Your busiest time is 10 AM - 12 PM.',
              color: Colors.blue.withAlpha(51),
            ),
          ],
        ),
      ),
    );
  }
}

class _MetricCard extends StatelessWidget {
  final String title;
  final String value;
  final IconData icon;
  final Color color;

  const _MetricCard({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(title, style: Theme.of(context).textTheme.titleMedium),
                Icon(icon, color: color),
              ],
            ),
            Text(value, style: Theme.of(context).textTheme.headlineSmall),
          ],
        ),
      ),
    );
  }
}

class _SalesChart extends StatelessWidget {
  final IndustryBranding branding;
  const _SalesChart({required this.branding});

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Sales Over Time',
                style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 24),
            SizedBox(
              height: 200,
              child: LineChart(
                LineChartData(
                  gridData: const FlGridData(show: false),
                  titlesData: const FlTitlesData(show: false),
                  borderData: FlBorderData(show: false),
                  lineBarsData: [
                    LineChartBarData(
                      spots: const [
                        FlSpot(0, 3),
                        FlSpot(1, 4),
                        FlSpot(2, 3.5),
                        FlSpot(3, 5),
                        FlSpot(4, 4),
                        FlSpot(5, 6),
                        FlSpot(6, 5.5),
                      ],
                      isCurved: true,
                      color: branding.primaryColor,
                      barWidth: 4,
                      isStrokeCapRound: true,
                      dotData: const FlDotData(show: false),
                      belowBarData: BarAreaData(
                        show: true,
                        color: branding.primaryColor.withAlpha(51),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _CategoryPieChart extends StatelessWidget {
  final IndustryBranding branding;
  const _CategoryPieChart({required this.branding});

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Sales by Category',
                style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 24),
            SizedBox(
              height: 200,
              child: PieChart(
                PieChartData(
                  sections: [
                    PieChartSectionData(
                        color: branding.primaryColor,
                        value: 40,
                        title: '40%',
                        radius: 50),
                    PieChartSectionData(
                        color: branding.primaryColor.withAlpha(153),
                        value: 30,
                        title: '30%',
                        radius: 50),
                    PieChartSectionData(
                        color: branding.primaryColor.withAlpha(102),
                        value: 15,
                        title: '15%',
                        radius: 50),
                    PieChartSectionData(
                        color: branding.primaryColor.withAlpha(51),
                        value: 15,
                        title: '15%',
                        radius: 50),
                  ],
                  sectionsSpace: 2,
                  centerSpaceRadius: 40,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _InsightTile extends StatelessWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final Color color;

  const _InsightTile({
    required this.icon,
    required this.title,
    required this.subtitle,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: CircleAvatar(
        backgroundColor: color,
        child: Icon(icon, color: Theme.of(context).colorScheme.onPrimary),
      ),
      title: Text(title),
      subtitle: Text(subtitle),
      trailing: const Icon(Icons.arrow_forward_ios),
      onTap: () {},
    );
  }
}