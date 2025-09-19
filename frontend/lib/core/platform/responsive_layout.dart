import 'package:flutter/material.dart';
import 'platform_info.dart';

/// Responsive breakpoints
class Breakpoints {
  static const double mobile = 600;
  static const double tablet = 900;
  static const double desktop = 1200;
  static const double large = 1600;
}

/// Device screen size categories
enum ScreenSize {
  mobile,
  tablet,
  desktop,
  large,
}

/// Responsive utilities for adaptive layouts
class ResponsiveLayout {
  /// Get current screen size category
  static ScreenSize getScreenSize(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    
    if (width < Breakpoints.mobile) {
      return ScreenSize.mobile;
    } else if (width < Breakpoints.tablet) {
      return ScreenSize.tablet;
    } else if (width < Breakpoints.desktop) {
      return ScreenSize.desktop;
    } else {
      return ScreenSize.large;
    }
  }
  
  /// Check if current screen is mobile
  static bool isMobile(BuildContext context) {
    return getScreenSize(context) == ScreenSize.mobile;
  }
  
  /// Check if current screen is tablet
  static bool isTablet(BuildContext context) {
    return getScreenSize(context) == ScreenSize.tablet;
  }
  
  /// Check if current screen is desktop
  static bool isDesktop(BuildContext context) {
    final size = getScreenSize(context);
    return size == ScreenSize.desktop || size == ScreenSize.large;
  }
  
  /// Get responsive value based on screen size
  static T responsive<T>(
    BuildContext context, {
    required T mobile,
    T? tablet,
    T? desktop,
    T? large,
  }) {
    final screenSize = getScreenSize(context);
    
    switch (screenSize) {
      case ScreenSize.mobile:
        return mobile;
      case ScreenSize.tablet:
        return tablet ?? mobile;
      case ScreenSize.desktop:
        return desktop ?? tablet ?? mobile;
      case ScreenSize.large:
        return large ?? desktop ?? tablet ?? mobile;
    }
  }
  
  /// Get adaptive padding based on screen size
  static EdgeInsets adaptivePadding(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    return responsive(
      context,
      mobile: EdgeInsets.all(uiConstants.compactPadding),
      tablet: EdgeInsets.all(uiConstants.defaultPadding),
      desktop: EdgeInsets.all(uiConstants.largePadding),
    );
  }
  
  /// Get adaptive margin based on screen size
  static EdgeInsets adaptiveMargin(BuildContext context) {
    return responsive(
      context,
      mobile: const EdgeInsets.all(8.0),
      tablet: const EdgeInsets.all(16.0),
      desktop: const EdgeInsets.all(24.0),
    );
  }
  
  /// Get adaptive column count for grid layouts
  static int getGridColumnCount(BuildContext context, {double itemWidth = 250}) {
    final screenWidth = MediaQuery.of(context).size.width;
    final padding = adaptivePadding(context).horizontal;
    final availableWidth = screenWidth - padding;
    
    final columns = (availableWidth / itemWidth).floor();
    return columns.clamp(1, 6);
  }
  
  /// Get adaptive font size multiplier
  static double getFontSizeMultiplier(BuildContext context) {
    return responsive(
      context,
      mobile: 0.9,
      tablet: 1.0,
      desktop: 1.1,
      large: 1.2,
    );
  }
  
  /// Get adaptive icon size
  static double getIconSize(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    return responsive(
      context,
      mobile: uiConstants.iconSize * 0.9,
      tablet: uiConstants.iconSize,
      desktop: uiConstants.iconSize * 1.1,
      large: uiConstants.iconSize * 1.2,
    );
  }
  
  /// Get adaptive list item height
  static double getListItemHeight(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    return responsive(
      context,
      mobile: uiConstants.listItemHeight,
      tablet: uiConstants.listItemHeight * 1.1,
      desktop: uiConstants.listItemHeight * 1.2,
    );
  }
}

/// Responsive widget that builds different layouts for different screen sizes
class ResponsiveWidget extends StatelessWidget {
  final Widget? mobile;
  final Widget? tablet;
  final Widget? desktop;
  final Widget? large;
  final Widget? fallback;
  
  const ResponsiveWidget({
    super.key,
    this.mobile,
    this.tablet,
    this.desktop,
    this.large,
    this.fallback,
  });
  
  @override
  Widget build(BuildContext context) {
    return ResponsiveLayout.responsive(
      context,
      mobile: mobile ?? fallback ?? const SizedBox.shrink(),
      tablet: tablet ?? mobile ?? fallback ?? const SizedBox.shrink(),
      desktop: desktop ?? tablet ?? mobile ?? fallback ?? const SizedBox.shrink(),
      large: large ?? desktop ?? tablet ?? mobile ?? fallback ?? const SizedBox.shrink(),
    );
  }
}

/// Responsive builder that provides screen size information
class ResponsiveBuilder extends StatelessWidget {
  final Widget Function(
    BuildContext context,
    ScreenSize screenSize,
    Widget? child,
  ) builder;
  final Widget? child;
  
  const ResponsiveBuilder({
    super.key,
    required this.builder,
    this.child,
  });
  
  @override
  Widget build(BuildContext context) {
    final screenSize = ResponsiveLayout.getScreenSize(context);
    return builder(context, screenSize, child);
  }
}

/// Adaptive container that adjusts based on platform and screen size
class AdaptiveContainer extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final Color? color;
  final Decoration? decoration;
  final double? width;
  final double? height;
  final AlignmentGeometry? alignment;
  final BoxConstraints? constraints;
  
  const AdaptiveContainer({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.color,
    this.decoration,
    this.width,
    this.height,
    this.alignment,
    this.constraints,
  });
  
  @override
  Widget build(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    final adaptivePadding = padding ?? ResponsiveLayout.adaptivePadding(context);
    final adaptiveMargin = margin ?? ResponsiveLayout.adaptiveMargin(context);
    
    return Container(
      padding: adaptivePadding,
      margin: adaptiveMargin,
      color: color,
      decoration: decoration ?? BoxDecoration(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
      ),
      width: width,
      height: height,
      alignment: alignment,
      constraints: constraints,
      child: child,
    );
  }
}

/// Adaptive card that adjusts elevation and styling based on platform
class AdaptiveCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final Color? color;
  final double? elevation;
  final ShapeBorder? shape;
  final VoidCallback? onTap;
  
  const AdaptiveCard({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.color,
    this.elevation,
    this.shape,
    this.onTap,
  });
  
  @override
  Widget build(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    final adaptiveElevation = elevation ?? uiConstants.cardElevation;
    final adaptiveShape = shape ?? RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(uiConstants.borderRadius),
    );
    
    final card = Card(
      margin: margin ?? ResponsiveLayout.adaptiveMargin(context),
      color: color,
      elevation: adaptiveElevation,
      shape: adaptiveShape,
      child: Padding(
        padding: padding ?? ResponsiveLayout.adaptivePadding(context),
        child: child,
      ),
    );
    
    if (onTap != null) {
      return InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        child: card,
      );
    }
    
    return card;
  }
}

/// Adaptive navigation that switches between different styles based on screen size
class AdaptiveNavigation extends StatelessWidget {
  final List<AdaptiveNavigationItem> items;
  final int selectedIndex;
  final ValueChanged<int>? onItemSelected;
  final Widget? leading;
  final Widget? trailing;
  
  const AdaptiveNavigation({
    super.key,
    required this.items,
    required this.selectedIndex,
    this.onItemSelected,
    this.leading,
    this.trailing,
  });
  
  @override
  Widget build(BuildContext context) {
    return ResponsiveWidget(
      mobile: _buildBottomNavigation(context),
      tablet: _buildBottomNavigation(context),
      desktop: _buildSideNavigation(context),
      large: _buildSideNavigation(context),
    );
  }
  
  Widget _buildBottomNavigation(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    return Container(
      height: uiConstants.navigationBarHeight,
      decoration: BoxDecoration(
        color: Theme.of(context).bottomNavigationBarTheme.backgroundColor,
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(25),
            blurRadius: 4,
            offset: const Offset(0, -2),
          ),
        ],
      ),
      child: Row(
        children: [
          if (leading != null) leading!,
          ...items.asMap().entries.map((entry) {
            final index = entry.key;
            final item = entry.value;
            final isSelected = index == selectedIndex;
            
            return Expanded(
              child: InkWell(
                onTap: () => onItemSelected?.call(index),
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 8),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(
                        isSelected ? item.selectedIcon : item.icon,
                        color: isSelected 
                          ? Theme.of(context).primaryColor
                          : Theme.of(context).unselectedWidgetColor,
                      ),
                      const SizedBox(height: 4),
                      Text(
                        item.label,
                        style: Theme.of(context).textTheme.labelSmall?.copyWith(
                          color: isSelected 
                            ? Theme.of(context).primaryColor
                            : Theme.of(context).unselectedWidgetColor,
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ),
                ),
              ),
            );
          }),
          if (trailing != null) trailing!,
        ],
      ),
    );
  }
  
  Widget _buildSideNavigation(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    return Container(
      width: 250,
      decoration: BoxDecoration(
        color: Theme.of(context).drawerTheme.backgroundColor,
        border: Border(
          right: BorderSide(
            color: Theme.of(context).dividerColor,
            width: 1,
          ),
        ),
      ),
      child: Column(
        children: [
          if (leading != null) 
            Padding(
              padding: ResponsiveLayout.adaptivePadding(context),
              child: leading!,
            ),
          Expanded(
            child: ListView.builder(
              itemCount: items.length,
              itemBuilder: (context, index) {
                final item = items[index];
                final isSelected = index == selectedIndex;
                
                return ListTile(
                  leading: Icon(
                    isSelected ? item.selectedIcon : item.icon,
                    color: isSelected 
                      ? Theme.of(context).primaryColor
                      : Theme.of(context).unselectedWidgetColor,
                  ),
                  title: Text(item.label),
                  selected: isSelected,
                  onTap: () => onItemSelected?.call(index),
                  contentPadding: EdgeInsets.symmetric(
                    horizontal: uiConstants.defaultPadding,
                    vertical: 8,
                  ),
                );
              },
            ),
          ),
          if (trailing != null) 
            Padding(
              padding: ResponsiveLayout.adaptivePadding(context),
              child: trailing!,
            ),
        ],
      ),
    );
  }
}

/// Navigation item for adaptive navigation
class AdaptiveNavigationItem {
  final IconData icon;
  final IconData? selectedIcon;
  final String label;
  final String? tooltip;
  
  const AdaptiveNavigationItem({
    required this.icon,
    this.selectedIcon,
    required this.label,
    this.tooltip,
  });
}