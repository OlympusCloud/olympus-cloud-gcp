import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/widgets/dashboard_card.dart';
import '../../../core/widgets/metric_card.dart';
import '../../../core/widgets/chart_card.dart';

class EventDashboard extends ConsumerWidget {
  const EventDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Event Master Dashboard',
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
                  icon: Icons.event,
                  label: 'Create Event',
                  color: Colors.deepPurple,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.people,
                  label: 'Manage Attendees',
                  color: Colors.blue,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.location_on,
                  label: 'Venue Setup',
                  color: Colors.orange,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.schedule,
                  label: 'Schedule',
                  color: Colors.green,
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
                  title: 'Active Events',
                  value: '8',
                  subtitle: '3 this weekend',
                  icon: Icons.event,
                  color: Colors.deepPurple,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Total Attendees',
                  value: '2,450',
                  subtitle: '+12% vs last month',
                  icon: Icons.people,
                  color: Colors.blue,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          Row(
            children: [
              Expanded(
                child: MetricCard(
                  title: 'Revenue',
                  value: '\$45,600',
                  subtitle: '+8% this month',
                  icon: Icons.attach_money,
                  color: Colors.green,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Venues',
                  value: '12',
                  subtitle: '2 new partnerships',
                  icon: Icons.location_on,
                  color: Colors.orange,
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
                  title: 'Event Bookings',
                  subtitle: 'Last 6 months',
                  chartType: ChartType.line,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ChartCard(
                  title: 'Event Types',
                  subtitle: 'Current season',
                  chartType: ChartType.pie,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Event-specific features
          DashboardCard(
            title: 'Upcoming Events',
            child: Column(
              children: [
                _buildEventTile(
                  name: 'Tech Conference 2024',
                  type: 'Corporate',
                  date: 'Mar 15, 2024',
                  attendees: '450',
                  status: 'Confirmed',
                  statusColor: Colors.green,
                ),
                _buildEventTile(
                  name: 'Wedding Reception',
                  type: 'Wedding',
                  date: 'Mar 18, 2024',
                  attendees: '120',
                  status: 'Planning',
                  statusColor: Colors.orange,
                ),
                _buildEventTile(
                  name: 'Music Festival',
                  type: 'Entertainment',
                  date: 'Mar 22, 2024',
                  attendees: '1,200',
                  status: 'Setup',
                  statusColor: Colors.blue,
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          DashboardCard(
            title: 'Venue Performance',
            child: Column(
              children: [
                _buildVenueTile(
                  name: 'Grand Ballroom',
                  events: '5 events',
                  capacity: '500 guests',
                  utilization: '85%',
                ),
                _buildVenueTile(
                  name: 'Conference Center',
                  events: '8 events',
                  capacity: '300 guests',
                  utilization: '92%',
                ),
                _buildVenueTile(
                  name: 'Outdoor Pavilion',
                  events: '3 events',
                  capacity: '800 guests',
                  utilization: '60%',
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          DashboardCard(
            title: 'Recent Activity',
            child: Column(
              children: [
                _buildActivityTile(
                  activity: 'New booking: Corporate Retreat',
                  time: '2 hours ago',
                  icon: Icons.event_available,
                  color: Colors.green,
                ),
                _buildActivityTile(
                  activity: 'Payment received: Wedding Reception',
                  time: '4 hours ago',
                  icon: Icons.payment,
                  color: Colors.blue,
                ),
                _buildActivityTile(
                  activity: 'Venue confirmed: Music Festival',
                  time: '6 hours ago',
                  icon: Icons.location_on,
                  color: Colors.orange,
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
          color: color.withAlpha((255 * 0.1).round()),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: color.withAlpha((255 * 0.3).round())),
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

  Widget _buildEventTile({
    required String name,
    required String type,
    required String date,
    required String attendees,
    required String status,
    required Color statusColor,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            width: 4,
            height: 50,
            decoration: BoxDecoration(
              color: Colors.deepPurple,
              borderRadius: BorderRadius.circular(2),
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
                  '$type • $date',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 12,
                  ),
                ),
                Text(
                  '$attendees attendees',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 11,
                  ),
                ),
              ],
            ),
          ),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              color: statusColor.withAlpha((255 * 0.1).round()),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Text(
              status,
              style: TextStyle(
                color: statusColor,
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildVenueTile({
    required String name,
    required String events,
    required String capacity,
    required String utilization,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: Colors.orange.withAlpha((255 * 0.1).round()),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Icon(
              Icons.location_on,
              color: Colors.orange,
              size: 20,
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
                  '$events • $capacity',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          Text(
            utilization,
            style: const TextStyle(
              fontWeight: FontWeight.w600,
              color: Colors.green,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildActivityTile({
    required String activity,
    required String time,
    required IconData icon,
    required Color color,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(6),
            decoration: BoxDecoration(
              color: color.withAlpha((255 * 0.1).round()),
              borderRadius: BorderRadius.circular(6),
            ),
            child: Icon(
              icon,
              color: color,
              size: 16,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              activity,
              style: const TextStyle(
                fontSize: 13,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
          Text(
            time,
            style: TextStyle(
              color: Colors.grey[600],
              fontSize: 12,
            ),
          ),
        ],
      ),
    );
  }
}