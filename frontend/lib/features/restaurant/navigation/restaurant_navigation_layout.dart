import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/branding/industry_branding.dart';
import '../../../core/branding/branding_provider.dart';
import '../../../core/auth/auth_controller.dart';

/// Restaurant-specific navigation menu items
enum RestaurantNavItem {
  dashboard('Dashboard', Icons.dashboard, '/restaurant/dashboard'),
  kitchen('Kitchen Display', Icons.restaurant, '/restaurant/kitchen'),
  tables('Table Management', Icons.table_restaurant, '/restaurant/tables'),
  orders('Orders', Icons.receipt_long, '/restaurant/orders'),
  menu('Menu Management', Icons.restaurant_menu, '/restaurant/menu'),
  inventory('Inventory', Icons.inventory, '/restaurant/inventory'),
  analytics('Analytics', Icons.analytics, '/restaurant/analytics'),
  settings('Settings', Icons.settings, '/restaurant/settings');

  const RestaurantNavItem(this.label, this.icon, this.route);
  
  final String label;
  final IconData icon;
  final String route;
}

/// Main navigation layout for Restaurant Revolution
class RestaurantNavigationLayout extends ConsumerStatefulWidget {
  final Widget child;
  final String currentRoute;

  const RestaurantNavigationLayout({
    super.key,
    required this.child,
    required this.currentRoute,
  });

  @override
  ConsumerState<RestaurantNavigationLayout> createState() => _RestaurantNavigationLayoutState();
}

class _RestaurantNavigationLayoutState extends ConsumerState<RestaurantNavigationLayout> {
  bool _isRailExpanded = true;
  
  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    final isTablet = size.width > 800 && size.width <= 1200;
    final isMobile = size.width <= 800;

    if (isMobile) {
      return _buildMobileLayout(context);
    } else if (isTablet) {
      return _buildTabletLayout(context);
    } else {
      return _buildDesktopLayout(context);
    }
  }

  Widget _buildDesktopLayout(BuildContext context) {
    final branding = ref.watch(brandingProvider);
    final theme = Theme.of(context);

    return Scaffold(
      body: Row(
        children: [
          // Sidebar navigation
          AnimatedContainer(
            duration: const Duration(milliseconds: 200),
            width: _isRailExpanded ? 280 : 80,
            child: Container(
              decoration: BoxDecoration(
                color: theme.colorScheme.surface,
                border: Border(
                  right: BorderSide(
                    color: theme.dividerColor,
                    width: 1,
                  ),
                ),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withAlpha(13),
                    blurRadius: 10,
                    offset: const Offset(2, 0),
                  ),
                ],
              ),
              child: Column(
                children: [
                  // Header
                  _buildSidebarHeader(branding, theme),
                  
                  // Navigation items
                  Expanded(
                    child: _buildNavigationItems(theme, isExpanded: _isRailExpanded),
                  ),
                  
                  // Footer
                  _buildSidebarFooter(theme),
                ],
              ),
            ),
          ),
          
          // Main content
          Expanded(child: widget.child),
        ],
      ),
    );
  }

  Widget _buildTabletLayout(BuildContext context) {
    final theme = Theme.of(context);

    return Scaffold(
      body: Row(
        children: [
          // Compact sidebar
          NavigationRail(
            selectedIndex: _getSelectedIndex(),
            onDestinationSelected: (index) {
              _navigateToItem(RestaurantNavItem.values[index]);
            },
            labelType: NavigationRailLabelType.selected,
            backgroundColor: theme.colorScheme.surface,
            destinations: RestaurantNavItem.values.map((item) {
              return NavigationRailDestination(
                icon: Icon(item.icon),
                selectedIcon: Icon(item.icon),
                label: Text(item.label),
              );
            }).toList(),
            leading: _buildCompactHeader(),
            trailing: _buildCompactFooter(),
          ),
          
          // Divider
          VerticalDivider(
            thickness: 1,
            width: 1,
            color: theme.dividerColor,
          ),
          
          // Main content
          Expanded(child: widget.child),
        ],
      ),
    );
  }

  Widget _buildMobileLayout(BuildContext context) {
    return Scaffold(
      body: widget.child,
      bottomNavigationBar: _buildBottomNavigation(),
      drawer: _buildMobileDrawer(),
    );
  }

  Widget _buildSidebarHeader(IndustryBranding branding, ThemeData theme) {
    return Container(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          Row(
            children: [
              if (branding.logoPath != null) ...[
                Image.asset(
                  branding.logoPath!,
                  height: 40,
                  width: 40,
                ),
                if (_isRailExpanded) const SizedBox(width: 12),
              ],
              if (_isRailExpanded) ...[
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        branding.name,
                        style: theme.textTheme.titleLarge?.copyWith(
                          fontWeight: FontWeight.bold,
                          color: branding.primaryColor,
                        ),
                      ),
                      Text(
                        branding.tagline,
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: theme.colorScheme.onSurface.withAlpha(178),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
              IconButton(
                onPressed: () {
                  setState(() {
                    _isRailExpanded = !_isRailExpanded;
                  });
                },
                icon: Icon(
                  _isRailExpanded ? Icons.menu_open : Icons.menu,
                  color: branding.primaryColor,
                ),
              ),
            ],
          ),
          
          if (_isRailExpanded) ...[
            const SizedBox(height: 16),
            
            // Quick stats card
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: branding.primaryColor.withAlpha(25),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(
                  color: branding.primaryColor.withAlpha(51),
                ),
              ),
              child: Row(
                children: [
                  Icon(
                    Icons.trending_up,
                    color: branding.primaryColor,
                    size: 20,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Today\'s Revenue',
                          style: theme.textTheme.labelSmall,
                        ),
                        Text(
                          '\$2,847.50', // TODO: Connect to real data
                          style: theme.textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                            color: branding.primaryColor,
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildNavigationItems(ThemeData theme, {required bool isExpanded}) {
    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      itemCount: RestaurantNavItem.values.length,
      itemBuilder: (context, index) {
        final item = RestaurantNavItem.values[index];
        final isSelected = widget.currentRoute.startsWith(item.route);
        final branding = ref.watch(brandingProvider);

        return Padding(
          padding: const EdgeInsets.only(bottom: 4),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 200),
            child: ListTile(
              selected: isSelected,
              selectedTileColor: branding.primaryColor.withAlpha(25),
              selectedColor: branding.primaryColor,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
              contentPadding: EdgeInsets.symmetric(
                horizontal: isExpanded ? 16 : 12,
                vertical: 4,
              ),
              leading: Icon(
                item.icon,
                size: 24,
                color: isSelected 
                    ? branding.primaryColor 
                    : theme.colorScheme.onSurface.withAlpha(178),
              ),
              title: isExpanded 
                  ? Text(
                      item.label,
                      style: theme.textTheme.bodyMedium?.copyWith(
                        fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
                        color: isSelected 
                            ? branding.primaryColor 
                            : theme.colorScheme.onSurface,
                      ),
                    )
                  : null,
              onTap: () => _navigateToItem(item),
            ),
          ),
        );
      },
    );
  }

  Widget _buildSidebarFooter(ThemeData theme) {
    final user = ref.watch(currentUserSafeProvider);
    final branding = ref.watch(brandingProvider);

    return Container(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          if (_isRailExpanded && user != null) ...[
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: theme.colorScheme.surfaceContainerHighest.withAlpha(76),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                children: [
                  CircleAvatar(
                    radius: 16,
                    backgroundColor: branding.primaryColor,
                    child: Text(
                      user.firstName.isNotEmpty ? user.firstName[0].toUpperCase() : 'U',
                      style: const TextStyle(
                        color: Colors.white,
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
                          '${user.firstName} ${user.lastName}',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        Text(
                          user.email,
                          style: theme.textTheme.bodySmall?.copyWith(
                            color: theme.colorScheme.onSurface.withAlpha(178),
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ),
                  IconButton(
                    onPressed: () async {
                      await ref.read(authControllerProvider.notifier).logout();
                    },
                    icon: const Icon(Icons.logout, size: 20),
                    tooltip: 'Logout',
                  ),
                ],
              ),
            ),
          ] else ...[
            IconButton(
              onPressed: () async {
                await ref.read(authControllerProvider.notifier).logout();
              },
              icon: const Icon(Icons.logout),
              tooltip: 'Logout',
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildCompactHeader() {
    final branding = ref.watch(brandingProvider);
    
    return Padding(
      padding: const EdgeInsets.all(8),
      child: Container(
        decoration: BoxDecoration(
          color: branding.primaryColor,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Padding(
          padding: const EdgeInsets.all(8),
          child: Icon(
            Icons.restaurant_menu,
            color: Colors.white,
            size: 24,
          ),
        ),
      ),
    );
  }

  Widget _buildCompactFooter() {
    return Padding(
      padding: const EdgeInsets.all(8),
      child: IconButton(
        onPressed: () async {
          await ref.read(authControllerProvider.notifier).logout();
        },
        icon: const Icon(Icons.logout),
        tooltip: 'Logout',
      ),
    );
  }

  Widget _buildBottomNavigation() {
    final theme = Theme.of(context);
    final branding = ref.watch(brandingProvider);
    
    // Show only the most important items in bottom nav
    final bottomNavItems = [
      RestaurantNavItem.dashboard,
      RestaurantNavItem.kitchen,
      RestaurantNavItem.tables,
      RestaurantNavItem.orders,
      RestaurantNavItem.analytics,
    ];

    return BottomNavigationBar(
      type: BottomNavigationBarType.fixed,
      selectedItemColor: branding.primaryColor,
      unselectedItemColor: theme.colorScheme.onSurface.withAlpha(153),
      currentIndex: _getSelectedBottomNavIndex(bottomNavItems),
      onTap: (index) => _navigateToItem(bottomNavItems[index]),
      items: bottomNavItems.map((item) {
        return BottomNavigationBarItem(
          icon: Icon(item.icon),
          label: item.label.split(' ').first, // Use first word only
        );
      }).toList(),
    );
  }

  Widget _buildMobileDrawer() {
    final theme = Theme.of(context);
    final branding = ref.watch(brandingProvider);
    final user = ref.watch(currentUserSafeProvider);

    return Drawer(
      child: Column(
        children: [
          // Drawer header
          DrawerHeader(
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  branding.primaryColor,
                  branding.secondaryColor,
                ],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    if (branding.logoPath != null) ...[
                      Image.asset(
                        branding.logoPath!,
                        height: 40,
                        width: 40,
                      ),
                      const SizedBox(width: 12),
                    ],
                    Expanded(
                      child: Text(
                        branding.name,
                        style: theme.textTheme.titleLarge?.copyWith(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                Text(
                  branding.tagline,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: Colors.white.withAlpha(230),
                  ),
                ),
                const Spacer(),
                if (user != null) ...[
                  Row(
                    children: [
                      CircleAvatar(
                        radius: 16,
                        backgroundColor: Colors.white.withOpacity(0.2),
                        child: Text(
                          user.firstName.isNotEmpty ? user.firstName[0].toUpperCase() : 'U',
                          style: const TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          '${user.firstName} ${user.lastName}',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: Colors.white,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                    ],
                  ),
                ],
              ],
            ),
          ),
          
          // Navigation items
          Expanded(
            child: ListView(
              padding: EdgeInsets.zero,
              children: RestaurantNavItem.values.map((item) {
                final isSelected = widget.currentRoute.startsWith(item.route);
                
                return ListTile(
                  selected: isSelected,
                  selectedTileColor: branding.primaryColor.withAlpha(25),
                  leading: Icon(
                    item.icon,
                    color: isSelected 
                        ? branding.primaryColor 
                        : theme.colorScheme.onSurface.withAlpha(178),
                  ),
                  title: Text(
                    item.label,
                    style: TextStyle(
                      color: isSelected 
                          ? branding.primaryColor 
                          : theme.colorScheme.onSurface,
                      fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
                    ),
                  ),
                  onTap: () {
                    Navigator.of(context).pop(); // Close drawer
                    _navigateToItem(item);
                  },
                );
              }).toList(),
            ),
          ),
          
          // Logout
          Padding(
            padding: const EdgeInsets.all(16),
            child: ElevatedButton.icon(
              onPressed: () async {
                Navigator.of(context).pop(); // Close drawer
                await ref.read(authControllerProvider.notifier).logout();
              },
              icon: const Icon(Icons.logout),
              label: const Text('Logout'),
              style: ElevatedButton.styleFrom(
                backgroundColor: theme.colorScheme.errorContainer,
                foregroundColor: theme.colorScheme.onErrorContainer,
              ),
            ),
          ),
        ],
      ),
    );
  }

  int _getSelectedIndex() {
    for (int i = 0; i < RestaurantNavItem.values.length; i++) {
      if (widget.currentRoute.startsWith(RestaurantNavItem.values[i].route)) {
        return i;
      }
    }
    return 0;
  }

  int _getSelectedBottomNavIndex(List<RestaurantNavItem> items) {
    for (int i = 0; i < items.length; i++) {
      if (widget.currentRoute.startsWith(items[i].route)) {
        return i;
      }
    }
    return 0;
  }

  void _navigateToItem(RestaurantNavItem item) {
    context.go(item.route);
  }
}