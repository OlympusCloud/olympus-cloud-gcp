import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../shared/widgets/industry_widgets.dart';
import '../../core/branding/branding_provider.dart';
import '../../core/branding/industry_branding.dart';

/// Restaurant-specific dashboard for Restaurant Revolution
class RestaurantDashboard extends ConsumerWidget {
  const RestaurantDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Restaurant Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Quick stats row
            Row(
              children: [
                Expanded(
                  child: _buildStatCard(
                    context,
                    'Tables',
                    '12/15',
                    'occupied',
                    Icons.table_restaurant,
                    branding,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildStatCard(
                    context,
                    'Orders',
                    '23',
                    'active',
                    Icons.receipt_long,
                    branding,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            
            Row(
              children: [
                Expanded(
                  child: _buildStatCard(
                    context,
                    'Revenue',
                    '\$2,450',
                    'completed',
                    Icons.attach_money,
                    branding,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildStatCard(
                    context,
                    'Wait Time',
                    '12 min',
                    'pending',
                    Icons.timer,
                    branding,
                  ),
                ),
              ],
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
            
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: [
                _buildActionButton('Take Order', Icons.add_shopping_cart, () {}),
                _buildActionButton('Seat Guests', Icons.event_seat, () {}),
                _buildActionButton('Kitchen Display', Icons.kitchen, () {}),
                _buildActionButton('Reservations', Icons.book_online, () {}),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Active orders
            IndustryCard(
              title: 'Active Orders',
              child: Column(
                children: [
                  _buildOrderItem('Table 5', 'Pasta Carbonara, Wine', '\$45.00', 'preparing'),
                  const Divider(),
                  _buildOrderItem('Table 2', 'Steak, Salad', '\$72.00', 'ready'),
                  const Divider(),
                  _buildOrderItem('Takeout #123', 'Pizza, Soda', '\$28.00', 'completed'),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatCard(
    BuildContext context,
    String title,
    String value,
    String status,
    IconData icon,
    IndustryBranding branding,
  ) {
    return IndustryCard(
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: branding.primaryColor.withOpacity(0.1),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Icon(
              icon,
              color: branding.primaryColor,
              size: 24,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: TextStyle(
                    fontSize: 12,
                    color: Theme.of(context).textTheme.bodyMedium?.color?.withOpacity(0.7),
                  ),
                ),
                const SizedBox(height: 4),
                Row(
                  children: [
                    Text(
                      value,
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(width: 8),
                    IndustryStatusIndicator(status: status, size: 8),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildActionButton(String label, IconData icon, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        width: 80,
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.shade300),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Column(
          children: [
            Icon(icon, size: 24),
            const SizedBox(height: 8),
            Text(
              label,
              style: const TextStyle(fontSize: 12),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildOrderItem(String table, String items, String total, String status) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  table,
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  items,
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey.shade600,
                  ),
                ),
              ],
            ),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text(
                total,
                style: const TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 14,
                ),
              ),
              const SizedBox(height: 4),
              IndustryStatusIndicator(
                status: status,
                showLabel: true,
                size: 8,
              ),
            ],
          ),
        ],
      ),
    );
  }
}

/// Retail-specific dashboard for Retail Pro
class RetailDashboard extends ConsumerWidget {
  const RetailDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Retail Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Sales overview
            Row(
              children: [
                Expanded(
                  child: _buildMetricCard(
                    'Today\'s Sales',
                    '\$3,250',
                    '+12.5%',
                    Icons.trending_up,
                    Colors.green,
                    branding,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildMetricCard(
                    'Transactions',
                    '47',
                    '+8.2%',
                    Icons.receipt,
                    branding.primaryColor,
                    branding,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            
            Row(
              children: [
                Expanded(
                  child: _buildMetricCard(
                    'Low Stock',
                    '23 items',
                    'Needs attention',
                    Icons.warning,
                    Colors.orange,
                    branding,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildMetricCard(
                    'Customers',
                    '156',
                    'Active',
                    Icons.people,
                    branding.secondaryColor,
                    branding,
                  ),
                ),
              ],
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
                _buildActionTile('New Sale', Icons.point_of_sale, () {}),
                _buildActionTile('Inventory', Icons.inventory, () {}),
                _buildActionTile('Reports', Icons.analytics, () {}),
                _buildActionTile('Products', Icons.category, () {}),
                _buildActionTile('Customers', Icons.people, () {}),
                _buildActionTile('Settings', Icons.settings, () {}),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Recent transactions
            IndustryCard(
              title: 'Recent Transactions',
              child: Column(
                children: [
                  _buildTransactionItem('iPhone 13', '\$799.00', 'completed', '2 min ago'),
                  const Divider(),
                  _buildTransactionItem('Nike Shoes', '\$120.00', 'completed', '15 min ago'),
                  const Divider(),
                  _buildTransactionItem('Coffee Mug', '\$12.99', 'pending', '32 min ago'),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricCard(
    String title,
    String value,
    String subtitle,
    IconData icon,
    Color color,
    IndustryBranding branding,
  ) {
    return IndustryCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, color: color, size: 24),
              const Spacer(),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: color.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  subtitle,
                  style: TextStyle(
                    fontSize: 10,
                    color: color,
                    fontWeight: FontWeight.w600,
                  ),
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

  Widget _buildActionTile(String label, IconData icon, VoidCallback onTap) {
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

  Widget _buildTransactionItem(String product, String amount, String status, String time) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  product,
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  time,
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey.shade600,
                  ),
                ),
              ],
            ),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text(
                amount,
                style: const TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 14,
                ),
              ),
              const SizedBox(height: 4),
              IndustryStatusIndicator(
                status: status,
                showLabel: true,
                size: 8,
              ),
            ],
          ),
        ],
      ),
    );
  }
}

/// Salon-specific dashboard for Salon Suite
class SalonDashboard extends ConsumerWidget {
  const SalonDashboard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    
    return Scaffold(
      appBar: IndustryAppBar(
        title: 'Salon Dashboard',
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Today's schedule overview
            IndustryCard(
              title: 'Today\'s Schedule',
              subtitle: 'Monday, September 19, 2025',
              child: Column(
                children: [
                  _buildAppointmentItem('9:00 AM', 'Sarah Johnson', 'Hair Cut & Style', 'booked'),
                  _buildAppointmentItem('10:30 AM', 'Mike Chen', 'Beard Trim', 'completed'),
                  _buildAppointmentItem('12:00 PM', 'Emma Davis', 'Hair Color', 'booked'),
                  _buildAppointmentItem('2:30 PM', 'Available', '', 'available'),
                ],
              ),
            ),
            
            const SizedBox(height: 16),
            
            // Quick stats
            Row(
              children: [
                Expanded(
                  child: _buildSalonStatCard(
                    'Appointments',
                    '8/12',
                    'Today',
                    Icons.event,
                    branding.primaryColor,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: _buildSalonStatCard(
                    'Revenue',
                    '\$680',
                    'Today',
                    Icons.attach_money,
                    Colors.green,
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 24),
            
            // Services menu
            Text(
              'Services',
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
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
              childAspectRatio: 1.5,
              children: [
                _buildServiceCard('Hair Cut', '\$45', Icons.content_cut),
                _buildServiceCard('Hair Color', '\$120', Icons.palette),
                _buildServiceCard('Facial', '\$80', Icons.face),
                _buildServiceCard('Manicure', '\$35', Icons.back_hand),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAppointmentItem(String time, String client, String service, String status) {
    Color statusColor = Colors.grey;
    if (status == 'booked') statusColor = Colors.blue;
    if (status == 'completed') statusColor = Colors.green;
    if (status == 'available') statusColor = Colors.orange;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            width: 4,
            height: 40,
            decoration: BoxDecoration(
              color: statusColor,
              borderRadius: BorderRadius.circular(2),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  time,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    fontSize: 14,
                  ),
                ),
                if (client.isNotEmpty) ...[
                  Text(
                    client,
                    style: const TextStyle(fontSize: 12),
                  ),
                  if (service.isNotEmpty)
                    Text(
                      service,
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey.shade600,
                      ),
                    ),
                ],
              ],
            ),
          ),
          IndustryStatusIndicator(status: status, size: 8),
        ],
      ),
    );
  }

  Widget _buildSalonStatCard(String title, String value, String subtitle, IconData icon, Color color) {
    return IndustryCard(
      child: Row(
        children: [
          Icon(icon, color: color, size: 32),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  value,
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Text(
                  title,
                  style: const TextStyle(fontSize: 12),
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
          ),
        ],
      ),
    );
  }

  Widget _buildServiceCard(String service, String price, IconData icon) {
    return IndustryCard(
      onTap: () {},
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(icon, size: 32),
          const SizedBox(height: 8),
          Text(
            service,
            style: const TextStyle(
              fontWeight: FontWeight.w600,
              fontSize: 14,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            price,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }
}