import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../shared/presentation/widgets/adaptive_layout.dart';
import '../../../../shared/presentation/widgets/responsive_navigation.dart';
import '../../../../shared/presentation/widgets/natural_language_bar.dart';

/// Main dashboard screen with adaptive navigation and natural language interface
class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({super.key});

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen> {
  int _selectedIndex = 0;

  final List<NavigationDestination> _destinations = [
    const NavigationDestination(
      icon: Icon(Icons.dashboard_outlined),
      selectedIcon: Icon(Icons.dashboard),
      label: 'Overview',
    ),
    const NavigationDestination(
      icon: Icon(Icons.receipt_long_outlined),
      selectedIcon: Icon(Icons.receipt_long),
      label: 'Orders',
    ),
    const NavigationDestination(
      icon: Icon(Icons.inventory_2_outlined),
      selectedIcon: Icon(Icons.inventory_2),
      label: 'Inventory',
    ),
    const NavigationDestination(
      icon: Icon(Icons.people_outlined),
      selectedIcon: Icon(Icons.people),
      label: 'Customers',
    ),
    const NavigationDestination(
      icon: Icon(Icons.analytics_outlined),
      selectedIcon: Icon(Icons.analytics),
      label: 'Analytics',
    ),
  ];

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return AdaptiveLayout(
      child: Scaffold(
        appBar: _buildAppBar(theme),
        body: Column(
          children: [
            // Natural Language Interface
            const NaturalLanguageBar(),
            
            // Main Content
            Expanded(
              child: _buildContent(theme),
            ),
          ],
        ),
        bottomNavigationBar: ResponsiveNavigation(
          selectedIndex: _selectedIndex,
          destinations: _destinations,
          onDestinationSelected: (index) {
            setState(() {
              _selectedIndex = index;
            });
          },
        ),
      ),
    );
  }

  PreferredSizeWidget _buildAppBar(ThemeData theme) {
    return AppBar(
      title: Row(
        children: [
          // Business Logo/Icon
          Container(
            width: 32,
            height: 32,
            decoration: BoxDecoration(
              color: theme.colorScheme.primary.withAlpha(25),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Icon(
              Icons.business,
              size: 20,
              color: theme.colorScheme.primary,
            ),
          ),
          
          const SizedBox(width: 12),
          
          // Business Name
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'My Business', // TODO: Get from user data
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.w600,
                ),
              ),
              Text(
                'Restaurant', // TODO: Get from user data
                style: theme.textTheme.bodySmall?.copyWith(
                  color: theme.colorScheme.onSurface.withAlpha(153),
                ),
              ),
            ],
          ),
        ],
      ),
      actions: [
        // Notifications
        IconButton(
          onPressed: () {
            // TODO: Show notifications
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Notifications not yet implemented'),
              ),
            );
          },
          icon: Badge(
            label: const Text('3'),
            child: const Icon(Icons.notifications_outlined),
          ),
        ),
        
        // Profile Menu
        PopupMenuButton(
          icon: CircleAvatar(
            radius: 16,
            backgroundColor: theme.colorScheme.primary,
            child: Text(
              'JD', // TODO: Get user initials
              style: TextStyle(
                color: theme.colorScheme.onPrimary,
                fontSize: 12,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          itemBuilder: (context) => <PopupMenuEntry<String>>[
            const PopupMenuItem(
              value: 'profile',
              child: ListTile(
                leading: Icon(Icons.person_outlined),
                title: Text('Profile'),
                contentPadding: EdgeInsets.zero,
              ),
            ),
            const PopupMenuItem(
              value: 'settings',
              child: ListTile(
                leading: Icon(Icons.settings_outlined),
                title: Text('Settings'),
                contentPadding: EdgeInsets.zero,
              ),
            ),
            const PopupMenuItem(
              value: 'help',
              child: ListTile(
                leading: Icon(Icons.help_outline),
                title: Text('Help'),
                contentPadding: EdgeInsets.zero,
              ),
            ),
            const PopupMenuDivider(),
            const PopupMenuItem(
              value: 'logout',
              child: ListTile(
                leading: Icon(Icons.logout),
                title: Text('Sign Out'),
                contentPadding: EdgeInsets.zero,
              ),
            ),
          ],
          onSelected: (value) {
            // TODO: Handle menu selections
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text('$value selected'),
              ),
            );
          },
        ),
        
        const SizedBox(width: 8),
      ],
    );
  }

  Widget _buildContent(ThemeData theme) {
    switch (_selectedIndex) {
      case 0:
        return _buildOverview(theme);
      case 1:
        return _buildOrders(theme);
      case 2:
        return _buildInventory(theme);
      case 3:
        return _buildCustomers(theme);
      case 4:
        return _buildAnalytics(theme);
      default:
        return _buildOverview(theme);
    }
  }

  Widget _buildOverview(ThemeData theme) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Welcome Section
          Card(
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.wb_sunny_outlined,
                        color: theme.colorScheme.primary,
                        size: 24,
                      ),
                      const SizedBox(width: 8),
                      Text(
                        'Good morning, John!',
                        style: theme.textTheme.headlineSmall?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    "Here's what's happening with your business today",
                    style: theme.textTheme.bodyLarge?.copyWith(
                      color: theme.colorScheme.onSurface.withAlpha(178),
                    ),
                  ),
                ],
              ),
            ),
          ),
          
          const SizedBox(height: 16),
          
          // Quick Stats
          _buildQuickStats(theme),
          
          const SizedBox(height: 16),
          
          // Quick Actions
          _buildQuickActions(theme),
          
          const SizedBox(height: 16),
          
          // Recent Activity
          _buildRecentActivity(theme),
        ],
      ),
    );
  }

  Widget _buildQuickStats(ThemeData theme) {
    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      childAspectRatio: 1.5,
      crossAxisSpacing: 16,
      mainAxisSpacing: 16,
      children: [
        _buildStatCard(
          theme,
          icon: Icons.attach_money,
          title: 'Today\'s Revenue',
          value: '\$2,847',
          trend: '+12%',
          isPositive: true,
        ),
        _buildStatCard(
          theme,
          icon: Icons.receipt_long,
          title: 'Orders',
          value: '47',
          trend: '+8%',
          isPositive: true,
        ),
        _buildStatCard(
          theme,
          icon: Icons.people,
          title: 'Customers',
          value: '156',
          trend: '+5%',
          isPositive: true,
        ),
        _buildStatCard(
          theme,
          icon: Icons.trending_up,
          title: 'Avg Order',
          value: '\$60.57',
          trend: '+3%',
          isPositive: true,
        ),
      ],
    );
  }

  Widget _buildStatCard(
    ThemeData theme, {
    required IconData icon,
    required String title,
    required String value,
    required String trend,
    required bool isPositive,
  }) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  icon,
                  color: theme.colorScheme.primary,
                  size: 20,
                ),
                const Spacer(),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: (isPositive 
                        ? Colors.green 
                        : Colors.red).withAlpha(25),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    trend,
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: isPositive ? Colors.green : Colors.red,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              value,
              style: theme.textTheme.headlineMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            Text(
              title,
              style: theme.textTheme.bodyMedium?.copyWith(
                color: theme.colorScheme.onSurface.withAlpha(153),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildQuickActions(ThemeData theme) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Quick Actions',
          style: theme.textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 12),
        Row(
          children: [
            Expanded(
              child: _buildActionCard(
                theme,
                icon: Icons.add_shopping_cart,
                title: 'New Order',
                subtitle: 'Create a new order',
                onTap: () {},
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _buildActionCard(
                theme,
                icon: Icons.inventory_2,
                title: 'Update Inventory',
                subtitle: 'Manage stock levels',
                onTap: () {},
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildActionCard(
    ThemeData theme, {
    required IconData icon,
    required String title,
    required String subtitle,
    required VoidCallback onTap,
  }) {
    return Card(
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: theme.colorScheme.primary.withAlpha(25),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Icon(
                  icon,
                  color: theme.colorScheme.primary,
                  size: 24,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                title,
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.w600,
                ),
                textAlign: TextAlign.center,
              ),
              Text(
                subtitle,
                style: theme.textTheme.bodySmall?.copyWith(
                  color: theme.colorScheme.onSurface.withAlpha(153),
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildRecentActivity(ThemeData theme) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Recent Activity',
          style: theme.textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Column(
            children: [
              _buildActivityItem(
                theme,
                icon: Icons.shopping_cart,
                title: 'New order #1234',
                subtitle: '2 minutes ago',
                amount: '\$45.67',
              ),
              const Divider(height: 1),
              _buildActivityItem(
                theme,
                icon: Icons.inventory,
                title: 'Inventory updated',
                subtitle: '15 minutes ago',
                amount: null,
              ),
              const Divider(height: 1),
              _buildActivityItem(
                theme,
                icon: Icons.person_add,
                title: 'New customer registered',
                subtitle: '1 hour ago',
                amount: null,
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildActivityItem(
    ThemeData theme, {
    required IconData icon,
    required String title,
    required String subtitle,
    String? amount,
  }) {
    return ListTile(
      leading: Container(
        padding: const EdgeInsets.all(8),
        decoration: BoxDecoration(
          color: theme.colorScheme.primary.withAlpha(25),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Icon(
          icon,
          color: theme.colorScheme.primary,
          size: 20,
        ),
      ),
      title: Text(title),
      subtitle: Text(subtitle),
      trailing: amount != null
          ? Text(
              amount,
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.w600,
                color: theme.colorScheme.primary,
              ),
            )
          : null,
    );
  }

  Widget _buildOrders(ThemeData theme) {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.receipt_long, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('Orders feature coming soon'),
        ],
      ),
    );
  }

  Widget _buildInventory(ThemeData theme) {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.inventory_2, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('Inventory feature coming soon'),
        ],
      ),
    );
  }

  Widget _buildCustomers(ThemeData theme) {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.people, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('Customers feature coming soon'),
        ],
      ),
    );
  }

  Widget _buildAnalytics(ThemeData theme) {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.analytics, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('Analytics feature coming soon'),
        ],
      ),
    );
  }
}