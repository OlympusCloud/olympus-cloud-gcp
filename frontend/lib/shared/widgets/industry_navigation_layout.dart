import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/branding/branding_provider.dart';
import '../../shared/widgets/adaptive_layout.dart';

/// Navigation item definition
class NavigationItem {
  final String route;
  final String label;
  final IconData icon;
  final IconData? selectedIcon;
  final bool enabled;
  final String? badge;

  const NavigationItem({
    required this.route,
    required this.label,
    required this.icon,
    this.selectedIcon,
    this.enabled = true,
    this.badge,
  });
}

/// Industry-specific navigation layout
class IndustryNavigationLayout extends ConsumerStatefulWidget {
  final Widget child;
  final String currentPath;

  const IndustryNavigationLayout({
    required this.child,
    required this.currentPath,
    super.key,
  });

  @override
  ConsumerState<IndustryNavigationLayout> createState() => _IndustryNavigationLayoutState();
}

class _IndustryNavigationLayoutState extends ConsumerState<IndustryNavigationLayout> {
  @override
  Widget build(BuildContext context) {
    final branding = ref.watch(brandingProvider);
    final size = MediaQuery.of(context).size;
    final isDesktop = size.width > 1200;
    final isTablet = size.width > 800 && size.width <= 1200;
    final isMobile = size.width <= 800;

    if (isDesktop) {
      return _buildDesktopLayout(context, branding);
    } else if (isTablet) {
      return _buildTabletLayout(context, branding);
    } else {
      return _buildMobileLayout(context, branding);
    }
  }

  /// Desktop layout with permanent navigation rail
  Widget _buildDesktopLayout(BuildContext context, branding) {
    final navigationItems = _getNavigationItems(branding.industryType);
    final theme = Theme.of(context);

    return AdaptiveLayout(
      child: Scaffold(
        body: Row(
          children: [
            // Navigation Rail
            Container(
              width: 280,
              decoration: BoxDecoration(
                color: theme.colorScheme.surface,
                border: Border(
                  right: BorderSide(
                    color: theme.colorScheme.outline.withOpacity(0.2),
                  ),
                ),
              ),
              child: Column(
                children: [
                  // Brand header
                  _buildBrandHeader(context, branding, true),
                  
                  // Navigation items
                  Expanded(
                    child: ListView(
                      padding: const EdgeInsets.symmetric(vertical: 8),
                      children: navigationItems.map((item) {
                        return _buildNavigationTile(context, item, true);
                      }).toList(),
                    ),
                  ),
                  
                  // User profile section
                  _buildUserSection(context, true),
                ],
              ),
            ),
            
            // Main content
            Expanded(
              child: widget.child,
            ),
          ],
        ),
      ),
    );
  }

  /// Tablet layout with collapsible navigation rail
  Widget _buildTabletLayout(BuildContext context, branding) {
    final navigationItems = _getNavigationItems(branding.industryType);
    final theme = Theme.of(context);

    return AdaptiveLayout(
      child: Scaffold(
        body: Row(
          children: [
            // Compact Navigation Rail
            Container(
              width: 80,
              decoration: BoxDecoration(
                color: theme.colorScheme.surface,
                border: Border(
                  right: BorderSide(
                    color: theme.colorScheme.outline.withOpacity(0.2),
                  ),
                ),
              ),
              child: Column(
                children: [
                  // Compact brand header
                  _buildBrandHeader(context, branding, false),
                  
                  // Navigation icons
                  Expanded(
                    child: ListView(
                      padding: const EdgeInsets.symmetric(vertical: 8),
                      children: navigationItems.map((item) {
                        return _buildNavigationTile(context, item, false);
                      }).toList(),
                    ),
                  ),
                  
                  // Compact user section
                  _buildUserSection(context, false),
                ],
              ),
            ),
            
            // Main content
            Expanded(
              child: widget.child,
            ),
          ],
        ),
      ),
    );
  }

  /// Mobile layout with bottom navigation
  Widget _buildMobileLayout(BuildContext context, branding) {
    final navigationItems = _getNavigationItems(branding.industryType);
    final theme = Theme.of(context);
    final currentIndex = _getCurrentIndex(navigationItems);

    return AdaptiveLayout(
      child: Scaffold(
        appBar: AppBar(
          title: _buildBrandHeader(context, branding, true),
          backgroundColor: theme.colorScheme.surface,
          elevation: 0,
          actions: [
            IconButton(
              onPressed: () {
                // TODO: Show notifications
              },
              icon: Badge(
                label: const Text('3'),
                child: const Icon(Icons.notifications_outlined),
              ),
            ),
            IconButton(
              onPressed: () {
                // TODO: Show user menu
              },
              icon: const Icon(Icons.account_circle_outlined),
            ),
          ],
        ),
        body: widget.child,
        bottomNavigationBar: BottomNavigationBar(
          currentIndex: currentIndex,
          onTap: (index) {
            if (index < navigationItems.length) {
              context.go(navigationItems[index].route);
            }
          },
          type: BottomNavigationBarType.fixed,
          items: navigationItems.take(5).map((item) {
            return BottomNavigationBarItem(
              icon: Icon(item.icon),
              activeIcon: Icon(item.selectedIcon ?? item.icon),
              label: item.label,
            );
          }).toList(),
        ),
      ),
    );
  }

  Widget _buildBrandHeader(BuildContext context, branding, bool showLabel) {
    final theme = Theme.of(context);
    
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          // Brand icon/logo
          Container(
            width: 40,
            height: 40,
            decoration: BoxDecoration(
              color: branding.getPrimaryColor(theme.brightness),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Icon(
              _getBrandIcon(branding.industryType),
              color: Colors.white,
              size: 24,
            ),
          ),
          
          if (showLabel) ...[
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    branding.brandName,
                    style: theme.textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.bold,
                      color: branding.getPrimaryColor(theme.brightness),
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                  Text(
                    'Dashboard',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: theme.colorScheme.onSurface.withOpacity(0.6),
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

  Widget _buildNavigationTile(BuildContext context, NavigationItem item, bool showLabel) {
    final theme = Theme.of(context);
    final isSelected = widget.currentPath.startsWith(item.route);
    final branding = ref.watch(brandingProvider);
    
    return Container(
      margin: EdgeInsets.symmetric(
        horizontal: showLabel ? 8 : 4,
        vertical: 2,
      ),
      child: ListTile(
        selected: isSelected,
        enabled: item.enabled,
        onTap: item.enabled ? () => context.go(item.route) : null,
        leading: Badge(
          label: item.badge != null ? Text(item.badge!) : null,
          isLabelVisible: item.badge != null,
          child: Icon(
            isSelected ? (item.selectedIcon ?? item.icon) : item.icon,
            color: isSelected 
                ? branding.getPrimaryColor(theme.brightness)
                : theme.colorScheme.onSurface.withOpacity(0.6),
          ),
        ),
        title: showLabel ? Text(
          item.label,
          style: theme.textTheme.bodyMedium?.copyWith(
            color: isSelected 
                ? branding.getPrimaryColor(theme.brightness)
                : theme.colorScheme.onSurface.withOpacity(0.8),
            fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
          ),
        ) : null,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(showLabel ? 12 : 8),
        ),
        selectedTileColor: branding.getPrimaryColor(theme.brightness).withOpacity(0.1),
        dense: !showLabel,
        visualDensity: showLabel ? null : VisualDensity.compact,
      ),
    );
  }

  Widget _buildUserSection(BuildContext context, bool showLabel) {
    final theme = Theme.of(context);
    
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          CircleAvatar(
            radius: 20,
            backgroundColor: theme.colorScheme.primary,
            child: const Icon(
              Icons.person,
              color: Colors.white,
              size: 20,
            ),
          ),
          
          if (showLabel) ...[
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'John Doe',
                    style: theme.textTheme.bodyMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  Text(
                    'Manager',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: theme.colorScheme.onSurface.withOpacity(0.6),
                    ),
                  ),
                ],
              ),
            ),
            IconButton(
              onPressed: () {
                // TODO: Show user menu
              },
              icon: const Icon(Icons.more_vert),
            ),
          ],
        ],
      ),
    );
  }

  List<NavigationItem> _getNavigationItems(String industryType) {
    switch (industryType) {
      case 'restaurant':
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/menu',
            label: 'Menu',
            icon: Icons.restaurant_menu_outlined,
            selectedIcon: Icons.restaurant_menu,
          ),
          NavigationItem(
            route: '/tables',
            label: 'Tables',
            icon: Icons.table_restaurant_outlined,
            selectedIcon: Icons.table_restaurant,
          ),
          NavigationItem(
            route: '/kitchen',
            label: 'Kitchen',
            icon: Icons.restaurant_outlined,
            selectedIcon: Icons.restaurant,
          ),
          NavigationItem(
            route: '/orders',
            label: 'Orders',
            icon: Icons.receipt_long_outlined,
            selectedIcon: Icons.receipt_long,
            badge: '12',
          ),
          NavigationItem(
            route: '/delivery',
            label: 'Delivery',
            icon: Icons.delivery_dining_outlined,
            selectedIcon: Icons.delivery_dining,
          ),
          NavigationItem(
            route: '/reservations',
            label: 'Reservations',
            icon: Icons.book_online_outlined,
            selectedIcon: Icons.book_online,
          ),
          NavigationItem(
            route: '/staff',
            label: 'Staff',
            icon: Icons.people_outlined,
            selectedIcon: Icons.people,
          ),
          NavigationItem(
            route: '/analytics',
            label: 'Analytics',
            icon: Icons.analytics_outlined,
            selectedIcon: Icons.analytics,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];

      case 'retail':
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/catalog',
            label: 'Catalog',
            icon: Icons.inventory_2_outlined,
            selectedIcon: Icons.inventory_2,
          ),
          NavigationItem(
            route: '/pos',
            label: 'POS',
            icon: Icons.point_of_sale_outlined,
            selectedIcon: Icons.point_of_sale,
          ),
          NavigationItem(
            route: '/inventory',
            label: 'Inventory',
            icon: Icons.warehouse_outlined,
            selectedIcon: Icons.warehouse,
          ),
          NavigationItem(
            route: '/customers',
            label: 'Customers',
            icon: Icons.people_outlined,
            selectedIcon: Icons.people,
          ),
          NavigationItem(
            route: '/promotions',
            label: 'Promotions',
            icon: Icons.local_offer_outlined,
            selectedIcon: Icons.local_offer,
          ),
          NavigationItem(
            route: '/reports',
            label: 'Reports',
            icon: Icons.assessment_outlined,
            selectedIcon: Icons.assessment,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];

      case 'salon':
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/appointments',
            label: 'Appointments',
            icon: Icons.schedule_outlined,
            selectedIcon: Icons.schedule,
            badge: '5',
          ),
          NavigationItem(
            route: '/services',
            label: 'Services',
            icon: Icons.content_cut_outlined,
            selectedIcon: Icons.content_cut,
          ),
          NavigationItem(
            route: '/clients',
            label: 'Clients',
            icon: Icons.person_outlined,
            selectedIcon: Icons.person,
          ),
          NavigationItem(
            route: '/staff',
            label: 'Staff',
            icon: Icons.people_outlined,
            selectedIcon: Icons.people,
          ),
          NavigationItem(
            route: '/calendar',
            label: 'Calendar',
            icon: Icons.calendar_today_outlined,
            selectedIcon: Icons.calendar_today,
          ),
          NavigationItem(
            route: '/products',
            label: 'Products',
            icon: Icons.shopping_bag_outlined,
            selectedIcon: Icons.shopping_bag,
          ),
          NavigationItem(
            route: '/analytics',
            label: 'Analytics',
            icon: Icons.insights_outlined,
            selectedIcon: Icons.insights,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];

      case 'event':
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/events',
            label: 'Events',
            icon: Icons.event_outlined,
            selectedIcon: Icons.event,
          ),
          NavigationItem(
            route: '/venues',
            label: 'Venues',
            icon: Icons.location_on_outlined,
            selectedIcon: Icons.location_on,
          ),
          NavigationItem(
            route: '/tickets',
            label: 'Tickets',
            icon: Icons.confirmation_number_outlined,
            selectedIcon: Icons.confirmation_number,
          ),
          NavigationItem(
            route: '/attendees',
            label: 'Attendees',
            icon: Icons.group_outlined,
            selectedIcon: Icons.group,
          ),
          NavigationItem(
            route: '/vendors',
            label: 'Vendors',
            icon: Icons.business_outlined,
            selectedIcon: Icons.business,
          ),
          NavigationItem(
            route: '/timeline',
            label: 'Timeline',
            icon: Icons.timeline_outlined,
            selectedIcon: Icons.timeline,
          ),
          NavigationItem(
            route: '/budget',
            label: 'Budget',
            icon: Icons.account_balance_wallet_outlined,
            selectedIcon: Icons.account_balance_wallet,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];

      case 'hospitality':
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/rooms',
            label: 'Rooms',
            icon: Icons.hotel_outlined,
            selectedIcon: Icons.hotel,
          ),
          NavigationItem(
            route: '/reservations',
            label: 'Reservations',
            icon: Icons.book_online_outlined,
            selectedIcon: Icons.book_online,
          ),
          NavigationItem(
            route: '/guests',
            label: 'Guests',
            icon: Icons.people_outlined,
            selectedIcon: Icons.people,
          ),
          NavigationItem(
            route: '/housekeeping',
            label: 'Housekeeping',
            icon: Icons.cleaning_services_outlined,
            selectedIcon: Icons.cleaning_services,
          ),
          NavigationItem(
            route: '/concierge',
            label: 'Concierge',
            icon: Icons.concierge_outlined,
            selectedIcon: Icons.concierge,
          ),
          NavigationItem(
            route: '/billing',
            label: 'Billing',
            icon: Icons.receipt_outlined,
            selectedIcon: Icons.receipt,
          ),
          NavigationItem(
            route: '/analytics',
            label: 'Analytics',
            icon: Icons.insights_outlined,
            selectedIcon: Icons.insights,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];

      default:
        return const [
          NavigationItem(
            route: '/dashboard',
            label: 'Dashboard',
            icon: Icons.dashboard_outlined,
            selectedIcon: Icons.dashboard,
          ),
          NavigationItem(
            route: '/analytics',
            label: 'Analytics',
            icon: Icons.analytics_outlined,
            selectedIcon: Icons.analytics,
          ),
          NavigationItem(
            route: '/customers',
            label: 'Customers',
            icon: Icons.people_outlined,
            selectedIcon: Icons.people,
          ),
          NavigationItem(
            route: '/orders',
            label: 'Orders',
            icon: Icons.receipt_long_outlined,
            selectedIcon: Icons.receipt_long,
          ),
          NavigationItem(
            route: '/inventory',
            label: 'Inventory',
            icon: Icons.inventory_outlined,
            selectedIcon: Icons.inventory,
          ),
          NavigationItem(
            route: '/reports',
            label: 'Reports',
            icon: Icons.assessment_outlined,
            selectedIcon: Icons.assessment,
          ),
          NavigationItem(
            route: '/settings',
            label: 'Settings',
            icon: Icons.settings_outlined,
            selectedIcon: Icons.settings,
          ),
        ];
    }
  }

  int _getCurrentIndex(List<NavigationItem> items) {
    for (int i = 0; i < items.length; i++) {
      if (widget.currentPath.startsWith(items[i].route)) {
        return i;
      }
    }
    return 0;
  }

  IconData _getBrandIcon(String industryType) {
    switch (industryType) {
      case 'restaurant':
        return Icons.restaurant_menu;
      case 'retail':
        return Icons.shopping_bag;
      case 'salon':
        return Icons.content_cut;
      case 'event':
        return Icons.celebration;
      case 'hospitality':
        return Icons.hotel;
      default:
        return Icons.business;
    }
  }
}