import 'package:flutter/material.dart';
import 'adaptive_layout.dart';

/// Responsive navigation widget that adapts between bottom navigation and navigation rail
class ResponsiveNavigation extends StatelessWidget {
  final int selectedIndex;
  final List<NavigationDestination> destinations;
  final ValueChanged<int> onDestinationSelected;
  final Widget? leading;
  final Widget? trailing;

  const ResponsiveNavigation({
    super.key,
    required this.selectedIndex,
    required this.destinations,
    required this.onDestinationSelected,
    this.leading,
    this.trailing,
  });

  @override
  Widget build(BuildContext context) {
    final deviceType = AdaptiveLayoutHelper.getDeviceType(context);

    // Use navigation rail for tablet and desktop
    if (deviceType.isTabletOrDesktop) {
      // Return SizedBox.shrink() to hide navigation on larger screens
      return const SizedBox.shrink();
    }

    // Use bottom navigation bar for mobile
    return NavigationBar(
      selectedIndex: selectedIndex,
      onDestinationSelected: onDestinationSelected,
      destinations: destinations,
    );
  }
}

/// Navigation rail for desktop and tablet layouts
class AppNavigationRail extends StatelessWidget {
  final int selectedIndex;
  final List<NavigationDestination> destinations;
  final ValueChanged<int> onDestinationSelected;
  final Widget? leading;
  final Widget? trailing;
  final bool extended;

  const AppNavigationRail({
    super.key,
    required this.selectedIndex,
    required this.destinations,
    required this.onDestinationSelected,
    this.leading,
    this.trailing,
    this.extended = false,
  });

  @override
  Widget build(BuildContext context) {
    return NavigationRail(
      selectedIndex: selectedIndex,
      onDestinationSelected: onDestinationSelected,
      extended: extended,
      leading: leading,
      trailing: trailing,
      destinations: destinations.map((dest) => NavigationRailDestination(
        icon: dest.icon,
        selectedIcon: dest.selectedIcon,
        label: Text(dest.label),
      )).toList(),
    );
  }
}