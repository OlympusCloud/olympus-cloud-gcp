import 'package:flutter/material.dart';
import 'adaptive_layout.dart';

/// Responsive form widget that adapts to different screen sizes
class ResponsiveForm extends StatelessWidget {
  final Widget child;
  final double maxWidth;
  final EdgeInsetsGeometry? padding;
  final bool centerOnDesktop;

  const ResponsiveForm({
    super.key,
    required this.child,
    this.maxWidth = 480,
    this.padding,
    this.centerOnDesktop = true,
  });

  @override
  Widget build(BuildContext context) {
    final deviceType = AdaptiveLayoutHelper.getDeviceType(context);
    final screenWidth = AdaptiveLayoutHelper.getScreenWidth(context);
    
    // Calculate appropriate padding
    final defaultPadding = AdaptiveLayoutHelper.getScreenPadding(context);
    final actualPadding = padding ?? defaultPadding;

    Widget content = Container(
      width: double.infinity,
      constraints: BoxConstraints(
        maxWidth: deviceType.isDesktop ? maxWidth : double.infinity,
      ),
      padding: actualPadding,
      child: child,
    );

    // Center the form on desktop if requested
    if (deviceType.isDesktop && centerOnDesktop) {
      content = Center(child: content);
    }

    return content;
  }
}

/// Responsive container that adjusts its layout based on screen size
class ResponsiveContainer extends StatelessWidget {
  final Widget child;
  final double mobileMaxWidth;
  final double tabletMaxWidth;
  final double desktopMaxWidth;
  final EdgeInsetsGeometry? padding;
  final bool center;

  const ResponsiveContainer({
    super.key,
    required this.child,
    this.mobileMaxWidth = double.infinity,
    this.tabletMaxWidth = 768,
    this.desktopMaxWidth = 1200,
    this.padding,
    this.center = true,
  });

  @override
  Widget build(BuildContext context) {
    final deviceType = AdaptiveLayoutHelper.getDeviceType(context);
    
    double maxWidth;
    switch (deviceType) {
      case DeviceType.mobile:
        maxWidth = mobileMaxWidth;
        break;
      case DeviceType.tablet:
        maxWidth = tabletMaxWidth;
        break;
      case DeviceType.desktop:
        maxWidth = desktopMaxWidth;
        break;
    }

    Widget content = Container(
      width: double.infinity,
      constraints: BoxConstraints(maxWidth: maxWidth),
      padding: padding ?? AdaptiveLayoutHelper.getScreenPadding(context),
      child: child,
    );

    if (center && deviceType.isTabletOrDesktop) {
      content = Center(child: content);
    }

    return content;
  }
}

/// Responsive grid that adjusts column count based on screen size
class ResponsiveGrid extends StatelessWidget {
  final List<Widget> children;
  final double spacing;
  final double runSpacing;
  final int mobileColumns;
  final int tabletColumns;
  final int desktopColumns;
  final double? childAspectRatio;
  final EdgeInsetsGeometry? padding;

  const ResponsiveGrid({
    super.key,
    required this.children,
    this.spacing = 16,
    this.runSpacing = 16,
    this.mobileColumns = 1,
    this.tabletColumns = 2,
    this.desktopColumns = 3,
    this.childAspectRatio,
    this.padding,
  });

  @override
  Widget build(BuildContext context) {
    final columnCount = AdaptiveLayoutHelper.getGridColumnCount(
      context,
      mobileColumns: mobileColumns,
      tabletColumns: tabletColumns,
      desktopColumns: desktopColumns,
    );

    return Padding(
      padding: padding ?? EdgeInsets.zero,
      child: GridView.count(
        crossAxisCount: columnCount,
        crossAxisSpacing: spacing,
        mainAxisSpacing: runSpacing,
        childAspectRatio: childAspectRatio ?? 1.0,
        children: children,
      ),
    );
  }
}

/// Responsive row that stacks on mobile and flows horizontally on larger screens
class ResponsiveRow extends StatelessWidget {
  final List<Widget> children;
  final MainAxisAlignment mainAxisAlignment;
  final CrossAxisAlignment crossAxisAlignment;
  final MainAxisSize mainAxisSize;
  final double spacing;
  final bool stackOnMobile;

  const ResponsiveRow({
    super.key,
    required this.children,
    this.mainAxisAlignment = MainAxisAlignment.start,
    this.crossAxisAlignment = CrossAxisAlignment.center,
    this.mainAxisSize = MainAxisSize.max,
    this.spacing = 16,
    this.stackOnMobile = true,
  });

  @override
  Widget build(BuildContext context) {
    final isMobile = AdaptiveLayoutHelper.isMobile(context);

    if (isMobile && stackOnMobile) {
      return Column(
        mainAxisAlignment: mainAxisAlignment,
        crossAxisAlignment: crossAxisAlignment,
        mainAxisSize: mainAxisSize,
        children: _addSpacing(children, spacing, isVertical: true),
      );
    }

    return Row(
      mainAxisAlignment: mainAxisAlignment,
      crossAxisAlignment: crossAxisAlignment,
      mainAxisSize: mainAxisSize,
      children: _addSpacing(children, spacing, isVertical: false),
    );
  }

  List<Widget> _addSpacing(List<Widget> widgets, double spacing, {required bool isVertical}) {
    if (widgets.isEmpty) return widgets;

    final List<Widget> spacedWidgets = [];
    for (int i = 0; i < widgets.length; i++) {
      spacedWidgets.add(widgets[i]);
      if (i < widgets.length - 1) {
        spacedWidgets.add(
          isVertical 
              ? SizedBox(height: spacing)
              : SizedBox(width: spacing),
        );
      }
    }
    return spacedWidgets;
  }
}

/// Responsive card that adjusts its layout based on screen size
class ResponsiveCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double? elevation;
  final Color? color;
  final ShapeBorder? shape;

  const ResponsiveCard({
    super.key,
    required this.child,
    this.padding,
    this.margin,
    this.elevation,
    this.color,
    this.shape,
  });

  @override
  Widget build(BuildContext context) {
    final deviceType = AdaptiveLayoutHelper.getDeviceType(context);
    
    // Adjust padding and margin based on device type
    final cardPadding = padding ?? EdgeInsets.all(
      deviceType.isMobile ? 16 : 24,
    );
    
    final cardMargin = margin ?? EdgeInsets.all(
      deviceType.isMobile ? 8 : 16,
    );

    return Card(
      elevation: elevation,
      color: color,
      shape: shape,
      margin: cardMargin,
      child: Padding(
        padding: cardPadding,
        child: child,
      ),
    );
  }
}

/// Responsive dialog that adjusts its size based on screen
class ResponsiveDialog extends StatelessWidget {
  final Widget child;
  final String? title;
  final List<Widget>? actions;
  final EdgeInsetsGeometry? contentPadding;
  final bool fullScreenOnMobile;

  const ResponsiveDialog({
    super.key,
    required this.child,
    this.title,
    this.actions,
    this.contentPadding,
    this.fullScreenOnMobile = false,
  });

  @override
  Widget build(BuildContext context) {
    final isMobile = AdaptiveLayoutHelper.isMobile(context);

    if (isMobile && fullScreenOnMobile) {
      return Scaffold(
        appBar: AppBar(
          title: title != null ? Text(title!) : null,
          actions: actions?.map((action) => 
            action is TextButton || action is ElevatedButton 
                ? action 
                : IconButton(
                    onPressed: () => Navigator.of(context).pop(),
                    icon: const Icon(Icons.close),
                  ),
          ).toList(),
        ),
        body: Padding(
          padding: contentPadding ?? const EdgeInsets.all(16),
          child: child,
        ),
      );
    }

    return AlertDialog(
      title: title != null ? Text(title!) : null,
      content: ConstrainedBox(
        constraints: BoxConstraints(
          maxWidth: isMobile ? double.infinity : 400,
          maxHeight: MediaQuery.of(context).size.height * 0.8,
        ),
        child: child,
      ),
      contentPadding: contentPadding ?? const EdgeInsets.fromLTRB(24, 20, 24, 24),
      actions: actions,
    );
  }
}