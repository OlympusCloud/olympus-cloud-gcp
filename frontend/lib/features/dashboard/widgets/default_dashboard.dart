import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class DefaultDashboard extends ConsumerWidget {
  const DefaultDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Olympus Cloud Dashboard',
            style: theme.textTheme.headlineMedium?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 24),
          
          // Welcome message
          Card(
            elevation: 0,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Welcome to Olympus Cloud',
                      style: theme.textTheme.titleLarge
                          ?.copyWith(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  Text(
                    'Your all-in-one business management platform',
                    style: theme.textTheme.bodyLarge,
                  ),
                  const SizedBox(height: 16),
                  Wrap(
                    spacing: 12,
                    runSpacing: 12,
                    children: [
                      _buildIndustryChip('Restaurant Revolution', Colors.red),
                      _buildIndustryChip('Retail Edge', Colors.blue),
                      _buildIndustryChip('Salon Luxe', Colors.pink),
                      _buildIndustryChip('Event Master', Colors.purple),
                      _buildIndustryChip('Hotel Haven', Colors.indigo),
                    ],
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          
          // Quick setup actions
          Card(
            elevation: 0,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                children: [
                  _buildSetupTile(
                    icon: Icons.business,
                    title: 'Choose Your Industry',
                    subtitle: 'Select your business type to get started',
                    onTap: () {
                      // TODO: Navigate to industry selection
                    },
                  ),
                  _buildSetupTile(
                    icon: Icons.store,
                    title: 'Setup Your Business',
                    subtitle: 'Add business details and preferences',
                    onTap: () {
                      // TODO: Navigate to business setup
                    },
                  ),
                  _buildSetupTile(
                    icon: Icons.people,
                    title: 'Invite Team Members',
                    subtitle: 'Add staff and manage permissions',
                    onTap: () {
                      // TODO: Navigate to team management
                    },
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          
          // Platform features
          Text(
            'Platform Features',
            style: theme.textTheme.titleLarge?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          
          GridView.count(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            crossAxisCount: 2,
            mainAxisSpacing: 16,
            crossAxisSpacing: 16,
            childAspectRatio: 1.2,
            children: [
              _buildFeatureCard(
                icon: Icons.analytics,
                title: 'Analytics',
                subtitle: 'Business insights and reports',
                color: Colors.blue,
              ),
              _buildFeatureCard(
                icon: Icons.inventory,
                title: 'Inventory',
                subtitle: 'Stock management and tracking',
                color: Colors.green,
              ),
              _buildFeatureCard(
                icon: Icons.people,
                title: 'Customer Management',
                subtitle: 'CRM and customer insights',
                color: Colors.orange,
              ),
              _buildFeatureCard(
                icon: Icons.payment,
                title: 'Payments',
                subtitle: 'Secure payment processing',
                color: Colors.purple,
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Platform stats
          Row(
            children: [
              Expanded(
                child: Card(
                  elevation: 0,
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                  child: Padding(
                    padding: const EdgeInsets.all(20),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text('Businesses Served',
                            style: theme.textTheme.titleLarge
                                ?.copyWith(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        Text(
                          '10,000+',
                          style: theme.textTheme.headlineMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Across all industries',
                          style: theme.textTheme.bodyMedium,
                        ),
                      ],
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Card(
                  elevation: 0,
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                  child: Padding(
                    padding: const EdgeInsets.all(20),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text('Uptime',
                            style: theme.textTheme.titleLarge
                                ?.copyWith(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        Text(
                          '99.9%',
                          style: theme.textTheme.headlineMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Reliable service',
                          style: theme.textTheme.bodyMedium,
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Support and resources
          Card(
            elevation: 0,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                children: [
                  _buildResourceTile(
                    icon: Icons.help,
                    title: 'Help Center',
                    subtitle: 'Find answers and tutorials',
                    onTap: () {
                      // TODO: Open help center
                    },
                  ),
                  _buildResourceTile(
                    icon: Icons.chat,
                    title: 'Live Support',
                    subtitle: '24/7 customer support',
                    onTap: () {
                      // TODO: Open support chat
                    },
                  ),
                  _buildResourceTile(
                    icon: Icons.school,
                    title: 'Training',
                    subtitle: 'Learn how to maximize your ROI',
                    onTap: () {
                      // TODO: Open training resources
                    },
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          
          // Charts section
          Text(
            'Key Metrics',
            style: theme.textTheme.titleLarge?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          
          _buildLineChart(theme),
          const SizedBox(height: 16),
          _buildBarChart(theme),
          const SizedBox(height: 16),
          _buildPieChart(theme),
          const SizedBox(height: 16),
          
          // Recent activities
          _buildRecentActivities(theme),
        ],
      ),
    );
  }

  Widget _buildIndustryChip(String label, Color color) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withAlpha((255 * 0.1).round()),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: color.withAlpha((255 * 0.3).round())),
      ),
      child: Text(
        label,
        style: TextStyle(
          color: color,
          fontSize: 12,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  Widget _buildSetupTile({
    required IconData icon,
    required String title,
    required String subtitle,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 12),
        child: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.blue.withAlpha((255 * 0.1).round()),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Icon(icon, color: Colors.blue, size: 24),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      fontWeight: FontWeight.w600,
                      fontSize: 16,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    subtitle,
                    style: TextStyle(
                      color: Colors.grey[600],
                      fontSize: 14,
                    ),
                  ),
                ],
              ),
            ),
            Icon(
              Icons.arrow_forward_ios,
              size: 16,
              color: Colors.grey[400],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildFeatureCard({
    required IconData icon,
    required String title,
    required String subtitle,
    required Color color,
  }) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: InkWell(
        onTap: () {
          // TODO: Navigate to feature
        },
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: color.withAlpha((255 * 0.1).round()),
                  borderRadius: BorderRadius.circular(16),
                ),
                child: Icon(icon, color: color, size: 32),
              ),
              const SizedBox(height: 12),
              Text(
                title,
                style: const TextStyle(
                  fontWeight: FontWeight.w600,
                  fontSize: 16,
                ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 4),
              Text(
                subtitle,
                style: TextStyle(
                  color: Colors.grey[600],
                  fontSize: 12,
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildResourceTile({
    required IconData icon,
    required String title,
    required String subtitle,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 8),
        child: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: Colors.grey.withAlpha((255 * 0.1).round()),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(icon, color: Colors.grey[700], size: 20),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                    ),
                  ),
                  Text(
                    subtitle,
                    style: TextStyle(
                      color: Colors.grey[600],
                      fontSize: 12,
                    ),
                  ),
                ],
              ),
            ),
            Icon(
              Icons.arrow_forward_ios,
              size: 14,
              color: Colors.grey[400],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildLineChart(ThemeData theme) {
    final spots = [
      FlSpot(0, 3),
      FlSpot(1, 1),
      FlSpot(2, 4),
      FlSpot(3, 2),
      FlSpot(4, 5),
    ];

    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Sales Overview',
                style: theme.textTheme.titleLarge
                    ?.copyWith(fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: LineChart(
                LineChartData(
                  gridData: const FlGridData(show: false),
                  titlesData: const FlTitlesData(show: false),
                  borderData: FlBorderData(show: false),
                  lineBarsData: [
                    LineChartBarData(
                      spots: spots,
                      isCurved: true,
                      color: Colors.blue,
                      barWidth: 4,
                      isStrokeCapRound: true,
                      dotData: const FlDotData(show: false),
                      belowBarData: BarAreaData(
                        show: true,
                        color: Colors.blue.withAlpha(51),
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

  Widget _buildBarChart(ThemeData theme) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Key Metrics',
              style: theme.textTheme.titleLarge
                  ?.copyWith(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: BarChart(
                BarChartData(
                  alignment: BarChartAlignment.spaceAround,
                  gridData: const FlGridData(show: false),
                  titlesData: FlTitlesData(
                    leftTitles: const AxisTitles(),
                    topTitles: const AxisTitles(),
                    rightTitles: const AxisTitles(),
                    bottomTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: (double value, TitleMeta meta) {
                          final style = TextStyle(
                            color: theme.colorScheme.onSurface,
                            fontWeight: FontWeight.bold,
                            fontSize: 14,
                          );
                          String text;
                          switch (value.toInt()) {
                            case 0:
                              text = 'Revenue';
                              break;
                            case 1:
                              text = 'Users';
                              break;
                            case 2:
                              text = 'Orders';
                              break;
                            default:
                              text = '';
                              break;
                          }
                          return SideTitleWidget(
                            space: 4.0,
                            child: Text(text, style: style),
                          );
                        },
                        reservedSize: 38,
                      ),
                    ),
                  ),
                  borderData: FlBorderData(show: false),
                  barGroups: [
                    _makeGroupData(0, 5, barColor: Colors.blue),
                    _makeGroupData(1, 6.5, barColor: Colors.green),
                    _makeGroupData(2, 5, barColor: Colors.orange),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  BarChartGroupData _makeGroupData(int x, double y,
      {Color barColor = Colors.blue, double width = 22}) {
    return BarChartGroupData(
      x: x,
      barRods: [
        BarChartRodData(
          toY: y,
          color: barColor,
          width: width,
          borderRadius: BorderRadius.zero,
        ),
      ],
    );
  }

  Widget _buildPieChart(ThemeData theme) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Sales by Category',
              style: theme.textTheme.titleLarge
                  ?.copyWith(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: PieChart(
                PieChartData(
                  sections: [
                    PieChartSectionData(
                        color: Colors.blue, value: 40, title: '40%', radius: 50),
                    PieChartSectionData(
                        color: Colors.green, value: 30, title: '30%', radius: 50),
                    PieChartSectionData(
                        color: Colors.orange, value: 15, title: '15%', radius: 50),
                    PieChartSectionData(
                        color: Colors.red, value: 15, title: '15%', radius: 50),
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

  Widget _buildRecentActivities(ThemeData theme) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Recent Activities',
              style: theme.textTheme.titleLarge
                  ?.copyWith(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: ListView(
                children: [
                  _buildActivityItem(theme, Icons.shopping_cart,
                      'New order placed', '10m ago', Colors.blue),
                  _buildActivityItem(theme, Icons.person_add,
                      'New user registered', '1h ago', Colors.green),
                  _buildActivityItem(theme, Icons.payment,
                      'Payment processed', '3h ago', Colors.orange),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildActivityItem(ThemeData theme, IconData icon, String text,
      String time, Color color) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: color.withAlpha(25),
              shape: BoxShape.circle,
            ),
            child: Icon(icon, color: color, size: 20),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(text, style: theme.textTheme.bodyMedium),
                Text(
                  time,
                  style: theme.textTheme.bodySmall
                      ?.copyWith(color: theme.colorScheme.onSurface.withAlpha(153)),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}