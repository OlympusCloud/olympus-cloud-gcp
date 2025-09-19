import 'package:flutter/material.dart';

/// Adaptive layout that adjusts based on screen size and platform
class AdaptiveLayout extends StatelessWidget {
  final Widget child;
  final double mobileBreakpoint;
  final double tabletBreakpoint;
  final double desktopBreakpoint;

  const AdaptiveLayout({
    super.key,
    required this.child,
    this.mobileBreakpoint = 600,
    this.tabletBreakpoint = 900,
    this.desktopBreakpoint = 1200,
  });

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final width = constraints.maxWidth;
        
        // Determine device type
        DeviceType deviceType;
        if (width < mobileBreakpoint) {
          deviceType = DeviceType.mobile;
        } else if (width < tabletBreakpoint) {
          deviceType = DeviceType.tablet;
        } else {
          deviceType = DeviceType.desktop;
        }

        return AdaptiveLayoutScope(
          deviceType: deviceType,
          screenWidth: width,
          child: child,
        );
      },
    );
  }
}

/// Provides adaptive layout information to child widgets
class AdaptiveLayoutScope extends InheritedWidget {
  final DeviceType deviceType;
  final double screenWidth;

  const AdaptiveLayoutScope({
    super.key,
    required this.deviceType,
    required this.screenWidth,
    required super.child,
  });

  static AdaptiveLayoutScope? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<AdaptiveLayoutScope>();
  }

  @override
  bool updateShouldNotify(AdaptiveLayoutScope oldWidget) {
    return deviceType != oldWidget.deviceType ||
           screenWidth != oldWidget.screenWidth;
  }
}

/// Device type enumeration
enum DeviceType {
  mobile,
  tablet,
  desktop,
}

/// Extension methods for device type
extension DeviceTypeExtension on DeviceType {
  bool get isMobile => this == DeviceType.mobile;
  bool get isTablet => this == DeviceType.tablet;
  bool get isDesktop => this == DeviceType.desktop;
  
  bool get isMobileOrTablet => isMobile || isTablet;
  bool get isTabletOrDesktop => isTablet || isDesktop;
}

/// Helper class to get layout information from context
class AdaptiveLayoutHelper {
  static DeviceType getDeviceType(BuildContext context) {
    final scope = AdaptiveLayoutScope.of(context);
    return scope?.deviceType ?? DeviceType.mobile;
  }

  static double getScreenWidth(BuildContext context) {
    final scope = AdaptiveLayoutScope.of(context);
    return scope?.screenWidth ?? MediaQuery.of(context).size.width;
  }

  static bool isMobile(BuildContext context) {
    return getDeviceType(context).isMobile;
  }

  static bool isTablet(BuildContext context) {
    return getDeviceType(context).isTablet;
  }

  static bool isDesktop(BuildContext context) {
    return getDeviceType(context).isDesktop;
  }

  static bool isMobileOrTablet(BuildContext context) {
    return getDeviceType(context).isMobileOrTablet;
  }

  static bool isTabletOrDesktop(BuildContext context) {
    return getDeviceType(context).isTabletOrDesktop;
  }

  /// Get appropriate padding based on device type
  static EdgeInsets getScreenPadding(BuildContext context) {
    final deviceType = getDeviceType(context);
    
    switch (deviceType) {
      case DeviceType.mobile:
        return const EdgeInsets.all(16);
      case DeviceType.tablet:
        return const EdgeInsets.all(24);
      case DeviceType.desktop:
        return const EdgeInsets.all(32);
    }
  }

  /// Get appropriate column count for grids
  static int getGridColumnCount(BuildContext context, {
    int mobileColumns = 1,
    int tabletColumns = 2,
    int desktopColumns = 3,
  }) {
    final deviceType = getDeviceType(context);
    
    switch (deviceType) {
      case DeviceType.mobile:
        return mobileColumns;
      case DeviceType.tablet:
        return tabletColumns;
      case DeviceType.desktop:
        return desktopColumns;
    }
  }

  /// Get appropriate maximum width for content
  static double getContentMaxWidth(BuildContext context) {
    final deviceType = getDeviceType(context);
    
    switch (deviceType) {
      case DeviceType.mobile:
        return double.infinity;
      case DeviceType.tablet:
        return 768;
      case DeviceType.desktop:
        return 1200;
    }
  }

  /// Get appropriate cross axis count for responsive grids
  static int getResponsiveCrossAxisCount(
    BuildContext context,
    double itemWidth,
  ) {
    final screenWidth = getScreenWidth(context);
    final padding = getScreenPadding(context);
    final availableWidth = screenWidth - padding.horizontal;
    
    return (availableWidth / itemWidth).floor().clamp(1, 6);
  }
}