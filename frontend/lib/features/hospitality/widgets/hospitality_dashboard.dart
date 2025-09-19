import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/widgets/dashboard_card.dart';
import '../../../core/widgets/metric_card.dart';
import '../../../core/widgets/chart_card.dart';

class HospitalityDashboard extends ConsumerWidget {
  const HospitalityDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Hotel Haven Dashboard',
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
                  icon: Icons.hotel,
                  label: 'Room Booking',
                  color: Colors.indigo,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.receipt_long,
                  label: 'Check-in/out',
                  color: Colors.green,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.room_service,
                  label: 'Room Service',
                  color: Colors.orange,
                ),
                _buildQuickAction(
                  context,
                  icon: Icons.cleaning_services,
                  label: 'Housekeeping',
                  color: Colors.blue,
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
                  title: 'Occupancy Rate',
                  value: '87%',
                  subtitle: '+5% vs last week',
                  icon: Icons.hotel,
                  color: Colors.indigo,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Revenue',
                  value: '\$28,500',
                  subtitle: 'Daily average',
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
                  title: 'Check-ins Today',
                  value: '42',
                  subtitle: '18 pending',
                  icon: Icons.login,
                  color: Colors.blue,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: MetricCard(
                  title: 'Guest Rating',
                  value: '4.8',
                  subtitle: 'Based on 156 reviews',
                  icon: Icons.star,
                  color: Colors.amber,
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
                  title: 'Occupancy Trends',
                  subtitle: 'Last 30 days',
                  chartType: ChartType.line,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ChartCard(
                  title: 'Revenue by Room Type',
                  subtitle: 'This month',
                  chartType: ChartType.pie,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Hotel-specific features
          DashboardCard(
            title: 'Room Status',
            child: Column(
              children: [
                _buildRoomStatusRow(
                  'Standard Rooms',
                  occupied: 45,
                  available: 15,
                  maintenance: 2,
                  color: Colors.blue,
                ),
                _buildRoomStatusRow(
                  'Deluxe Rooms',
                  occupied: 18,
                  available: 8,
                  maintenance: 1,
                  color: Colors.purple,
                ),
                _buildRoomStatusRow(
                  'Suites',
                  occupied: 8,
                  available: 4,
                  maintenance: 0,
                  color: Colors.amber,
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          DashboardCard(
            title: 'Today\'s Arrivals',
            child: Column(
              children: [
                _buildGuestTile(
                  name: 'John Smith',
                  room: 'Deluxe Suite 304',
                  checkIn: '2:00 PM',
                  nights: '3 nights',
                  status: 'VIP',
                ),
                _buildGuestTile(
                  name: 'Maria Rodriguez',
                  room: 'Standard 215',
                  checkIn: '3:30 PM',
                  nights: '2 nights',
                  status: 'Business',
                ),
                _buildGuestTile(
                  name: 'David Johnson',
                  room: 'Deluxe 412',
                  checkIn: '4:15 PM',
                  nights: '1 night',
                  status: 'Regular',
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          
          DashboardCard(
            title: 'Service Requests',
            child: Column(
              children: [
                _buildServiceTile(
                  request: 'Extra towels',
                  room: 'Room 208',
                  time: '10 min ago',
                  priority: 'Low',
                  priorityColor: Colors.green,
                ),
                _buildServiceTile(
                  request: 'Air conditioning issue',
                  room: 'Room 315',
                  time: '25 min ago',
                  priority: 'High',
                  priorityColor: Colors.red,
                ),
                _buildServiceTile(
                  request: 'Room service order',
                  room: 'Suite 501',
                  time: '1 hour ago',
                  priority: 'Medium',
                  priorityColor: Colors.orange,
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

  Widget _buildRoomStatusRow(
    String roomType, {
    required int occupied,
    required int available,
    required int maintenance,
    required Color color,
  }) {
    final total = occupied + available + maintenance;
    
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                roomType,
                style: const TextStyle(
                  fontWeight: FontWeight.w600,
                  fontSize: 14,
                ),
              ),
              Text(
                '$occupied/$total occupied',
                style: TextStyle(
                  color: Colors.grey[600],
                  fontSize: 12,
                ),
              ),
            ],
          ),
          const SizedBox(height: 4),
          LinearProgressIndicator(
            value: occupied / total,
            backgroundColor: Colors.grey[200],
            valueColor: AlwaysStoppedAnimation<Color>(color),
          ),
          const SizedBox(height: 4),
          Row(
            children: [
              _buildStatusIndicator('Occupied', occupied, color),
              const SizedBox(width: 16),
              _buildStatusIndicator('Available', available, Colors.green),
              const SizedBox(width: 16),
              _buildStatusIndicator('Maintenance', maintenance, Colors.orange),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildStatusIndicator(String label, int count, Color color) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 8,
          height: 8,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
          ),
        ),
        const SizedBox(width: 4),
        Text(
          '$label ($count)',
          style: TextStyle(
            fontSize: 11,
            color: Colors.grey[600],
          ),
        ),
      ],
    );
  }

  Widget _buildGuestTile({
    required String name,
    required String room,
    required String checkIn,
    required String nights,
    required String status,
  }) {
    Color statusColor = status == 'VIP' 
        ? Colors.amber 
        : status == 'Business' 
            ? Colors.blue 
            : Colors.grey;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            width: 4,
            height: 50,
            decoration: BoxDecoration(
              color: Colors.indigo,
              borderRadius: BorderRadius.circular(2),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Text(
                      name,
                      style: const TextStyle(
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                      ),
                    ),
                    const SizedBox(width: 8),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                      decoration: BoxDecoration(
                        color: statusColor.withAlpha((255 * 0.1).round()),
                        borderRadius: BorderRadius.circular(4),
                      ),
                      child: Text(
                        status,
                        style: TextStyle(
                          color: statusColor,
                          fontSize: 10,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ),
                  ],
                ),
                Text(
                  '$room • $nights',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          Text(
            checkIn,
            style: const TextStyle(
              fontWeight: FontWeight.w500,
              color: Colors.indigo,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildServiceTile({
    required String request,
    required String room,
    required String time,
    required String priority,
    required Color priorityColor,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(6),
            decoration: BoxDecoration(
              color: priorityColor.withAlpha((255 * 0.1).round()),
              borderRadius: BorderRadius.circular(6),
            ),
            child: Icon(
              Icons.room_service,
              color: priorityColor,
              size: 16,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  request,
                  style: const TextStyle(
                    fontSize: 13,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                Text(
                  '$room • $priority priority',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 11,
                  ),
                ),
              ],
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