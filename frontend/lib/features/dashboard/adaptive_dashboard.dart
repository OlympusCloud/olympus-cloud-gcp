import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/branding/branding_provider.dart';
import '../../core/branding/industry_branding.dart';
import '../../shared/widgets/industry_widgets.dart';
import 'industry_dashboards.dart';

class AdaptiveDashboard extends ConsumerWidget {
  const AdaptiveDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final industry = ref.watch(currentIndustryProvider);
    
    // Return industry-specific dashboard
    switch (industry) {
      case IndustryType.restaurant:
        return const RestaurantDashboard();
      case IndustryType.retail:
        return const RetailDashboard();
      case IndustryType.salon:
        return const SalonDashboard();
      case IndustryType.events:
        return const EventsDashboard();
      case IndustryType.hospitality:
        return const HospitalityDashboard();
      case IndustryType.other:
        return const GenericDashboard();
    }
  }
}

/// Events-specific dashboard for Events Master
class EventsDashboard extends ConsumerWidget {
  const EventsDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Events Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Event overview cards
            Row(
              children: [
                Expanded(
                  child: _buildEventCard(
                    'Active Events',
                    '5',
                    'This Month',
                    Icons.event,
                    branding.primaryColor,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildEventCard(
                    'Total Revenue',
                    '\$12,500',
                    'This Month',
                    Icons.attach_money,
                    Colors.green,
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 16),
            
            Row(
              children: [
                Expanded(
                  child: _buildEventCard(
                    'Upcoming',
                    '3 Events',
                    'Next 7 Days',
                    Icons.upcoming,
                    Colors.orange,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildEventCard(
                    'Attendees',
                    '245',
                    'Total',
                    Icons.people,
                    branding.secondaryColor,
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Upcoming events
            IndustryCard(
              title: 'Upcoming Events',
              child: Column(
                children: [
                  _buildEventItem(
                    'Corporate Annual Meeting',
                    'Sep 25, 2025',
                    '150 attendees',
                    'confirmed',
                  ),
                  const Divider(),
                  _buildEventItem(
                    'Wedding Reception',
                    'Oct 2, 2025',
                    '80 attendees',
                    'tentative',
                  ),
                  const Divider(),
                  _buildEventItem(
                    'Product Launch',
                    'Oct 10, 2025',
                    '200 attendees',
                    'confirmed',
                  ),
                ],
              ),
            ),
            
            const SizedBox(height: 24),
            
            // Quick actions
            Text(
              'Quick Actions',
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            
            GridView.count(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              crossAxisCount: 3,
              mainAxisSpacing: 16,
              crossAxisSpacing: 16,
              childAspectRatio: 1.2,
              children: [
                _buildActionButton('New Event', Icons.add_circle, () {}),
                _buildActionButton('Calendar', Icons.calendar_month, () {}),
                _buildActionButton('Venues', Icons.location_on, () {}),
                _buildActionButton('Vendors', Icons.handshake, () {}),
                _buildActionButton('Reports', Icons.analytics, () {}),
                _buildActionButton('Settings', Icons.settings, () {}),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEventCard(String title, String value, String subtitle, IconData icon, Color color) {
    return IndustryCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, color: color, size: 24),
              const Spacer(),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            value,
            style: const TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            title,
            style: const TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
          ),
          Text(
            subtitle,
            style: TextStyle(
              fontSize: 10,
              color: Colors.grey.shade600,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildEventItem(String name, String date, String attendees, String status) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
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
                const SizedBox(height: 4),
                Text(
                  date,
                  style: const TextStyle(fontSize: 12),
                ),
                Text(
                  attendees,
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey.shade600,
                  ),
                ),
              ],
            ),
          ),
          IndustryStatusIndicator(
            status: status,
            showLabel: true,
            size: 8,
          ),
        ],
      ),
    );
  }

  Widget _buildActionButton(String label, IconData icon, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.shade300),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 32),
            const SizedBox(height: 8),
            Text(
              label,
              style: const TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }
}

/// Hospitality dashboard for Hotel Haven
class HospitalityDashboard extends ConsumerWidget {
  const HospitalityDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Hotel Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Room status overview
            Row(
              children: [
                Expanded(
                  child: _buildRoomCard('Occupied', '45/60', Colors.blue),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildRoomCard('Available', '12/60', Colors.green),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildRoomCard('Maintenance', '3/60', Colors.orange),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Today's arrivals and departures
            IndustryCard(
              title: 'Today\'s Activity',
              child: Column(
                children: [
                  _buildGuestActivity('Check-ins', '8 guests', Icons.login, Colors.green),
                  const Divider(),
                  _buildGuestActivity('Check-outs', '12 guests', Icons.logout, Colors.blue),
                  const Divider(),
                  _buildGuestActivity('Reservations', '15 bookings', Icons.book_online, Colors.orange),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildRoomCard(String status, String count, Color color) {
    return IndustryCard(
      child: Column(
        children: [
          Text(
            count,
            style: TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: color,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            status,
            style: const TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildGuestActivity(String title, String count, IconData icon, Color color) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(icon, color: color, size: 24),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              title,
              style: const TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
          Text(
            count,
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: color,
            ),
          ),
        ],
      ),
    );
  }
}

/// Generic dashboard for other industries
class GenericDashboard extends ConsumerWidget {
  const GenericDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Business Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Generic business metrics
            Row(
              children: [
                Expanded(
                  child: _buildMetricCard('Revenue', '\$5,230', '+15.2%', Icons.trending_up),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildMetricCard('Customers', '156', '+8.1%', Icons.people),
                ),
              ],
            ),
            
            const SizedBox(height: 16),
            
            Row(
              children: [
                Expanded(
                  child: _buildMetricCard('Orders', '89', '+12.5%', Icons.shopping_cart),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildMetricCard('Growth', '23%', '+2.3%', Icons.show_chart),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Quick actions
            IndustryCard(
              title: 'Quick Actions',
              child: GridView.count(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                crossAxisCount: 3,
                mainAxisSpacing: 12,
                crossAxisSpacing: 12,
                childAspectRatio: 1.2,
                children: [
                  _buildQuickAction('New Order', Icons.add_shopping_cart),
                  _buildQuickAction('Customers', Icons.people),
                  _buildQuickAction('Inventory', Icons.inventory),
                  _buildQuickAction('Reports', Icons.analytics),
                  _buildQuickAction('Settings', Icons.settings),
                  _buildQuickAction('Help', Icons.help),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricCard(String title, String value, String change, IconData icon) {
    return IndustryCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, size: 24),
              const Spacer(),
              Text(
                change,
                style: TextStyle(
                  fontSize: 12,
                  color: change.startsWith('+') ? Colors.green : Colors.red,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            value,
            style: const TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            title,
            style: TextStyle(
              fontSize: 12,
              color: Colors.grey.shade600,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildQuickAction(String label, IconData icon) {
    return InkWell(
      onTap: () {},
      borderRadius: BorderRadius.circular(8),
      child: Container(
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.shade300),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 24),
            const SizedBox(height: 4),
            Text(
              label,
              style: const TextStyle(fontSize: 10),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }
}