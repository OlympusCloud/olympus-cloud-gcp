import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/widgets/dashboard_card.dart';
import '../../../core/widgets/metric_card.dart';
import '../../../core/widgets/chart_card.dart';

class SalonDashboard extends ConsumerWidget {
  const SalonDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Salon Luxe Dashboard',
            style: Theme.of(context).textTheme.headlineMedium?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 24),
          
          // Quick Actions
          DashboardCard(
            title: 'Quick Actions',
            child: Wrap(
              spacing: 12,
              runSpacing: 12,
              children: [
                _buildQuickAction(
                  context,
                  icon: Icons.event_available,
                  label: 'Book Appointment',
                  color: Colors.pink,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.content_cut,
                  label: 'New Service',
                  color: Colors.purple,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.people,
                  label: 'Client Management',
                  color: Colors.indigo,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.inventory,
                  label: 'Product Inventory',
                  color: Colors.teal,
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          // Metrics Row
          Row(
            children: [
              Expanded(
                child: MetricCard(
                  title: 'Today\'s Appointments',
                  value: '12',
                  subtitle: '+3 from yesterday',
                  icon: Icons.calendar_today,
                  color: Colors.pink,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Revenue',
                  value: '\$2,450',
                  subtitle: '+15% this week',
                  icon: Icons.attach_money,
                  color: Colors.green,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          Row(
            children: [
              Expanded(
                child: MetricCard(
                  title: 'Active Clients',
                  value: '89',
                  subtitle: '+5 new this week',
                  icon: Icons.people,
                  color: Colors.blue,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Services',
                  value: '24',
                  subtitle: '8 completed today',
                  icon: Icons.content_cut,
                  color: Colors.purple,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Charts Row
          Row(
            children: [
              Expanded(
                child: ChartCard(
                  title: 'Booking Trends',
                  subtitle: 'Last 30 days',
                  chartType: ChartType.line,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ChartCard(
                  title: 'Service Distribution',
                  subtitle: 'This month',
                  chartType: ChartType.pie,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Salon-specific features
          DashboardCard(
            title: 'Today\'s Schedule',
            child: Column(
              children: [
                _buildAppointmentTile(
                  client: 'Sarah Johnson',
                  service: 'Hair Cut & Color',
                  time: '10:00 AM',
                  stylist: 'Emma',
                ),
                _buildAppointmentTile(
                  client: 'Maria Garcia',
                  service: 'Manicure & Pedicure',
                  time: '11:30 AM',
                  stylist: 'Sophia',
                ),
                _buildAppointmentTile(
                  client: 'Lisa Chen',
                  service: 'Facial Treatment',
                  time: '2:00 PM',
                  stylist: 'Olivia',
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          DashboardCard(
            title: 'Stylist Performance',
            child: Column(
              children: [
                _buildStylistTile(
                  name: 'Emma Rodriguez',
                  services: '8 services',
                  rating: '4.9',
                  revenue: '\$480',
                ),
                _buildStylistTile(
                  name: 'Sophia Williams',
                  services: '6 services',
                  rating: '4.8',
                  revenue: '\$360',
                ),
                _buildStylistTile(
                  name: 'Olivia Brown',
                  services: '5 services',
                  rating: '4.7',
                  revenue: '\$320',
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildQuickAction(
    BuildContext context, {
    required IconData icon,
    required String label,
    required Color color,
  }) {
    return InkWell(
      onTap: () {
        // TODO: Implement navigation
      },
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: color.withAlpha(25),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: color.withAlpha(77)),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, color: color, size: 20),
            const SizedBox(width: 8),
            Text(
              label,
              style: TextStyle(
                color: color,
                fontWeight: FontWeight.w600,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAppointmentTile({
    required String client,
    required String service,
    required String time,
    required String stylist,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            width: 4,
            height: 40,
            decoration: BoxDecoration(
              color: Colors.pink,
              borderRadius: BorderRadius.circular(2),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  client,
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                  ),
                ),
                Text(
                  '$service • $stylist',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          Text(
            time,
            style: const TextStyle(
              fontWeight: FontWeight.w500,
              color: Colors.pink,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStylistTile({
    required String name,
    required String services,
    required String rating,
    required String revenue,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          CircleAvatar(
            radius: 20,
            backgroundColor: Colors.pink.withAlpha(51),
            child: Text(
              name.split(' ').map((e) => e[0]).join(),
              style: const TextStyle(
                color: Colors.pink,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  name,
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                  ),
                ),
                Text(
                  '$services • ⭐ $rating',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          Text(
            revenue,
            style: const TextStyle(
              fontWeight: FontWeight.w600,
              color: Colors.green,
            ),
          ),
        ],
      ),
    );
  }
}