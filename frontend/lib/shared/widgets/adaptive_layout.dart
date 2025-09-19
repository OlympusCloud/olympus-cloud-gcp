import 'package:flutter/material.dart';

/// Adaptive layout wrapper that provides responsive design helpers
class AdaptiveLayout extends StatelessWidget {
  final Widget child;
  final EdgeInsets? padding;
  final double? maxWidth;

  const AdaptiveLayout({
    super.key,
    required this.child,
    this.padding,
    this.maxWidth,
  });

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final effectiveMaxWidth = maxWidth ?? _getMaxWidth(screenWidth);
    final effectivePadding = padding ?? _getPadding(screenWidth);

    return Container(
      width: double.infinity,
      padding: effectivePadding,
      child: Center(
        child: Container(
          constraints: BoxConstraints(maxWidth: effectiveMaxWidth),
          child: child,
        ),
      ),
    );
  }

  double _getMaxWidth(double screenWidth) {
    if (screenWidth > 1200) return 1200; // Desktop
    if (screenWidth > 800) return screenWidth * 0.9; // Tablet
    return double.infinity; // Mobile
  }

  EdgeInsets _getPadding(double screenWidth) {
    if (screenWidth > 1200) return const EdgeInsets.all(24); // Desktop
    if (screenWidth > 800) return const EdgeInsets.all(16); // Tablet
    return const EdgeInsets.all(12); // Mobile
  }
}

/// Helper class for responsive design utilities
class ResponsiveHelper {
  static bool isMobile(BuildContext context) {
    return MediaQuery.of(context).size.width <= 800;
  }

  static bool isTablet(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    return width > 800 && width <= 1200;
  }

  static bool isDesktop(BuildContext context) {
    return MediaQuery.of(context).size.width > 1200;
  }

  static int getGridColumns(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    if (width > 1200) return 4; // Desktop
    if (width > 800) return 3; // Tablet
    return 1; // Mobile
  }

  static double getCardAspectRatio(BuildContext context) {
    if (isDesktop(context)) return 1.2;
    if (isTablet(context)) return 1.1;
    return 1.5; // Mobile
  }
}