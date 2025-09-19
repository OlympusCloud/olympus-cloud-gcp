import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/branding/industry_branding.dart';
import '../../dashboard/widgets/industry_dashboard.dart';

/// Restaurant Revolution dashboard with restaurant-specific features
class RestaurantDashboard extends BaseDashboard {
  const RestaurantDashboard({super.key});

  @override
  Widget buildHeader(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    final theme = Theme.of(context);
    final greeting = _getTimeBasedGreeting();
    
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            branding.lightColorScheme.primary,
            branding.lightColorScheme.secondary,
          ],
        ),
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: Colors.white.withAlpha(51),
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: Icon(
                      Icons.restaurant_menu,
                      color: branding.lightColorScheme.onPrimary,
                      size: 32,
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          branding.brandName,
                          style: theme.textTheme.headlineSmall?.copyWith(
                            color: branding.lightColorScheme.onPrimary,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        Text(
                          'Your Restaurant Command Center',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: branding.lightColorScheme.onPrimary.withAlpha(230),
                          ),
                        ),
                      ],
                    ),
                  ),
                  IconButton(
                    onPressed: () => _showQuickActions(context),
                    icon: Icon(
                      Icons.more_vert,
                      color: branding.lightColorScheme.onPrimary,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 24),
              Text(
                '$greeting, Manager',
                style: theme.textTheme.titleLarge?.copyWith(
                  color: branding.lightColorScheme.onPrimary,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                _getCurrentServicePeriod(),
                style: theme.textTheme.bodyLarge?.copyWith(
                  color: branding.lightColorScheme.onPrimary.withAlpha(230),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  @override
  Widget buildQuickStats(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        buildSectionHeader(
          context,
          title: 'Today\'s Performance',
          subtitle: 'Real-time restaurant metrics',
        ),
        GridView.count(
          crossAxisCount: 2,
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          crossAxisSpacing: 12,
          mainAxisSpacing: 12,
          childAspectRatio: 1.2,
          children: [
            buildStatCard(
              context,
              icon: Icons.table_restaurant,
              title: 'Active Tables',
              value: '18/25',
              subtitle: '72% occupancy',
              color: branding.lightColorScheme.secondary,
              onTap: () => context.push('/tables'),
            ),
            buildStatCard(
              context,
              icon: Icons.receipt_long,
              title: 'Orders Today',
              value: '127',
              subtitle: '+12% vs yesterday',
              color: branding.lightColorScheme.primary,
              onTap: () => context.push('/orders'),
            ),
            buildStatCard(
              context,
              icon: Icons.attach_money,
              title: 'Revenue',
              value: '\$3,847',
              subtitle: 'Target: \$4,200',
              color: const Color(0xFF10B981),
              onTap: () => context.push('/analytics'),
            ),
            buildStatCard(
              context,
              icon: Icons.schedule,
              title: 'Avg. Wait Time',
              value: '12 min',
              subtitle: 'Within target',
              color: const Color(0xFFEAB308),
              onTap: () => context.push('/kitchen'),
            ),
          ],
        ),
      ],
    );
  }

  @override
  Widget buildMainContent(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        // Recent Orders Section
        buildSectionHeader(
          context,
          title: 'Recent Orders',
          subtitle: 'Latest customer orders',
          action: TextButton(
            onPressed: () => context.push('/orders'),
            child: const Text('View All'),
          ),
        ),
        _buildRecentOrdersList(context, branding),
        
        const SizedBox(height: 24),
        
        // Kitchen Status Section
        buildSectionHeader(
          context,
          title: 'Kitchen Status',
          subtitle: 'Current order preparation',
          action: TextButton(
            onPressed: () => context.push('/kitchen'),
            child: const Text('Kitchen Display'),
          ),
        ),
        _buildKitchenStatus(context, branding),
        
        const SizedBox(height: 24),
        
        // Staff Performance Section
        buildSectionHeader(
          context,
          title: 'Staff Performance',
          subtitle: 'Today\'s top performers',
        ),
        _buildStaffPerformance(context, branding),
      ],
    );
  }

  @override
  Widget buildQuickActions(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        buildSectionHeader(
          context,
          title: 'Quick Actions',
          subtitle: 'Common restaurant tasks',
        ),
        GridView.count(
          crossAxisCount: 4,
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          crossAxisSpacing: 12,
          mainAxisSpacing: 12,
          childAspectRatio: 0.8,
          children: [
            buildActionButton(
              context,
              icon: Icons.add_circle_outline,
              label: 'New Order',
              color: branding.lightColorScheme.primary,
              onTap: () => _createNewOrder(context),
            ),
            buildActionButton(
              context,
              icon: Icons.book_online,
              label: 'Reservations',
              color: branding.lightColorScheme.secondary,
              onTap: () => context.push('/reservations'),
            ),
            buildActionButton(
              context,
              icon: Icons.menu_book,
              label: 'Menu',
              color: branding.lightColorScheme.tertiary,
              onTap: () => context.push('/menu'),
            ),
            buildActionButton(
              context,
              icon: Icons.delivery_dining,
              label: 'Delivery',
              color: const Color(0xFF10B981),
              onTap: () => context.push('/delivery'),
            ),
            buildActionButton(
              context,
              icon: Icons.people,
              label: 'Staff',
              color: const Color(0xFF8B5CF6),
              onTap: () => context.push('/staff'),
            ),
            buildActionButton(
              context,
              icon: Icons.inventory_2,
              label: 'Inventory',
              color: const Color(0xFFEAB308),
              onTap: () => context.push('/inventory'),
            ),
            buildActionButton(
              context,
              icon: Icons.campaign,
              label: 'Promotions',
              color: const Color(0xFFEF4444),
              onTap: () => context.push('/promotions'),
            ),
            buildActionButton(
              context,
              icon: Icons.settings,
              label: 'Settings',
              color: const Color(0xFF6B7280),
              onTap: () => context.push('/settings'),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildRecentOrdersList(BuildContext context, IndustryBranding branding) {
    final orders = [
      {'id': '#1247', 'table': 'Table 12', 'items': '2 items', 'total': '\$47.50', 'status': 'Preparing'},
      {'id': '#1246', 'table': 'Table 8', 'items': '4 items', 'total': '\$89.25', 'status': 'Ready'},
      {'id': '#1245', 'table': 'Takeout', 'items': '1 item', 'total': '\$15.99', 'status': 'Complete'},
    ];

    return Card(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: orders.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final order = orders[index];
          final status = order['status'] as String;
          final statusColor = _getStatusColor(status, branding);
          
          return ListTile(
            leading: CircleAvatar(
              backgroundColor: statusColor.withAlpha(25),
              child: Text(
                order['id']!.replaceAll('#', ''),
                style: TextStyle(
                  color: statusColor,
                  fontWeight: FontWeight.bold,
                  fontSize: 12,
                ),
              ),
            ),
            title: Text(order['table']!),
            subtitle: Text(order['items']!),
            trailing: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  order['total']!,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                  decoration: BoxDecoration(
                    color: statusColor.withAlpha(25),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    status,
                    style: TextStyle(
                      color: statusColor,
                      fontSize: 10,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ],
            ),
            onTap: () => _viewOrderDetails(context, order['id']!),
          );
        },
      ),
    );
  }

  Widget _buildKitchenStatus(BuildContext context, IndustryBranding branding) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            Expanded(
              child: _buildKitchenMetric(
                context,
                'Orders in Queue',
                '8',
                Icons.queue,
                branding.lightColorScheme.primary,
              ),
            ),
            Container(
              width: 1,
              height: 60,
              color: Theme.of(context).colorScheme.outline.withAlpha(77),
            ),
            Expanded(
              child: _buildKitchenMetric(
                context,
                'Avg. Prep Time',
                '14 min',
                Icons.timer,
                branding.lightColorScheme.secondary,
              ),
            ),
            Container(
              width: 1,
              height: 60,
              color: Theme.of(context).colorScheme.outline.withAlpha(77),
            ),
            Expanded(
              child: _buildKitchenMetric(
                context,
                'Delayed Orders',
                '2',
                Icons.warning,
                const Color(0xFFEF4444),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildKitchenMetric(
    BuildContext context,
    String label,
    String value,
    IconData icon,
    Color color,
  ) {
    return Column(
      children: [
        Icon(icon, color: color, size: 24),
        const SizedBox(height: 8),
        Text(
          value,
          style: Theme.of(context).textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.bold,
            color: color,
          ),
        ),
        Text(
          label,
          style: Theme.of(context).textTheme.bodySmall,
          textAlign: TextAlign.center,
        ),
      ],
    );
  }

  Widget _buildStaffPerformance(BuildContext context, IndustryBranding branding) {
    final staff = [
      {'name': 'Sarah M.', 'role': 'Server', 'sales': '\$847', 'rating': 4.9},
      {'name': 'Mike R.', 'role': 'Server', 'sales': '\$723', 'rating': 4.7},
      {'name': 'Elena K.', 'role': 'Bartender', 'sales': '\$542', 'rating': 4.8},
    ];

    return Card(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: staff.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final member = staff[index];
          
          return ListTile(
            leading: CircleAvatar(
              backgroundColor: branding.lightColorScheme.primary.withAlpha(25),
              child: Text(
                (member['name'] as String).split(' ').map((n) => n[0]).join(),
                style: TextStyle(
                  color: branding.lightColorScheme.primary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            title: Text(member['name'] as String),
            subtitle: Text(member['role'] as String),
            trailing: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  member['sales'] as String,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(
                      Icons.star,
                      size: 16,
                      color: const Color(0xFFEAB308),
                    ),
                    Text(
                      (member['rating'] as double).toString(),
                      style: const TextStyle(fontSize: 12),
                    ),
                  ],
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  
  Widget buildActionButton(
    BuildContext context, {
    required IconData icon,
    required String label,
    required Color color,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: color.withAlpha(25),
              borderRadius: BorderRadius.circular(12),
            ),
            child: Icon(icon, color: color, size: 24),
          ),
          const SizedBox(height: 8),
          Text(
            label,
            style: Theme.of(context).textTheme.bodySmall,
            textAlign: TextAlign.center,
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }

  
  Widget buildSectionHeader(
    BuildContext context, {
    required String title,
    String? subtitle,
    Widget? action,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12, top: 24),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
              ),
              if (subtitle != null)
                Text(
                  subtitle,
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
            ],
          ),
          if (action != null) action,
        ],
      ),
    );
  }

  
  Widget buildStatCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String value,
    String? subtitle,
    required Color color,
    VoidCallback? onTap,
  }) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    title,
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                  Icon(icon, color: color, size: 24),
                ],
              ),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    value,
                    style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                          fontWeight: FontWeight.bold,
                          color: color,
                        ),
                  ),
                  if (subtitle != null)
                    Text(
                      subtitle,
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  String _getTimeBasedGreeting() {
    final hour = DateTime.now().hour;
    if (hour < 12) return 'Good Morning';
    if (hour < 17) return 'Good Afternoon';
    return 'Good Evening';
  }

  String _getCurrentServicePeriod() {
    final hour = DateTime.now().hour;
    if (hour >= 6 && hour < 11) return 'Breakfast Service';
    if (hour >= 11 && hour < 17) return 'Lunch Service';
    if (hour >= 17 && hour < 22) return 'Dinner Service';
    return 'Late Night Service';
  }

  Color _getStatusColor(String status, IndustryBranding branding) {
    switch (status.toLowerCase()) {
      case 'preparing':
        return branding.lightColorScheme.secondary;
      case 'ready':
        return const Color(0xFF10B981);
      case 'complete':
        return const Color(0xFF6B7280);
      default:
        return branding.lightColorScheme.primary;
    }
  }

  void _showQuickActions(BuildContext context) {
    showModalBottomSheet(
      context: context,
      builder: (context) => Container(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.notifications),
              title: const Text('Notifications'),
              onTap: () => Navigator.pop(context),
            ),
            ListTile(
              leading: const Icon(Icons.help),
              title: const Text('Help & Support'),
              onTap: () => Navigator.pop(context),
            ),
            ListTile(
              leading: const Icon(Icons.exit_to_app),
              title: const Text('Sign Out'),
              onTap: () => Navigator.pop(context),
            ),
          ],
        ),
      ),
    );
  }

  void _createNewOrder(BuildContext context) {
    // TODO: Implement new order creation
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('New Order feature coming soon!')),
    );
  }

  void _viewOrderDetails(BuildContext context, String orderId) {
    // TODO: Implement order details view
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Viewing order $orderId')),
    );
  }
}