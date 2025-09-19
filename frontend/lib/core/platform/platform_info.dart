import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'dart:io' show Platform;

/// Platform detection and configuration utilities
class PlatformInfo {
  static bool get isWeb => kIsWeb;
  static bool get isAndroid => !kIsWeb && Platform.isAndroid;
  static bool get isIOS => !kIsWeb && Platform.isIOS;
  static bool get isMacOS => !kIsWeb && Platform.isMacOS;
  static bool get isWindows => !kIsWeb && Platform.isWindows;
  static bool get isLinux => !kIsWeb && Platform.isLinux;
  static bool get isFuchsia => !kIsWeb && Platform.isFuchsia;
  
  static bool get isMobile => isAndroid || isIOS;
  static bool get isDesktop => isMacOS || isWindows || isLinux;
  
  /// Get the current platform type
  static PlatformType get currentPlatform {
    if (isWeb) return PlatformType.web;
    if (isAndroid) return PlatformType.android;
    if (isIOS) return PlatformType.ios;
    if (isMacOS) return PlatformType.macos;
    if (isWindows) return PlatformType.windows;
    if (isLinux) return PlatformType.linux;
    if (isFuchsia) return PlatformType.fuchsia;
    return PlatformType.unknown;
  }
  
  /// Check if the current platform supports a specific feature
  static bool supportsFeature(PlatformFeature feature) {
    switch (feature) {
      case PlatformFeature.camera:
        return isMobile;
      case PlatformFeature.biometrics:
        return isMobile;
      case PlatformFeature.pushNotifications:
        return isMobile || isWeb;
      case PlatformFeature.fileSystem:
        return !isWeb;
      case PlatformFeature.clipboard:
        return true;
      case PlatformFeature.sharing:
        return isMobile || isWeb;
      case PlatformFeature.systemTheme:
        return true;
      case PlatformFeature.windowControls:
        return isDesktop;
      case PlatformFeature.multiWindow:
        return isDesktop || isWeb;
      case PlatformFeature.dragAndDrop:
        return isDesktop || isWeb;
      case PlatformFeature.contextMenu:
        return isDesktop || isWeb;
      case PlatformFeature.keyboard:
        return isDesktop || isWeb;
      case PlatformFeature.mouse:
        return isDesktop || isWeb;
      case PlatformFeature.touch:
        return isMobile || isWeb;
    }
  }
  
  /// Get platform-specific configuration
  static PlatformConfig getConfig() {
    return PlatformConfig(
      platform: currentPlatform,
      defaultPageSize: isMobile ? 10 : 20,
      maxConcurrentRequests: isMobile ? 3 : 6,
      cacheSize: isMobile ? 50 : 100,
      enableAnimations: !isWeb || isDesktop,
      enableHapticFeedback: isMobile,
      enableKeyboardShortcuts: isDesktop || isWeb,
      enableContextMenu: isDesktop || isWeb,
      enableDragAndDrop: isDesktop || isWeb,
      preferNativeControls: isMobile,
      useSystemNavigationBar: isMobile,
      enableFullscreenMode: isDesktop,
      defaultWindowSize: isDesktop ? const Size(1200, 800) : null,
      minWindowSize: isDesktop ? const Size(800, 600) : null,
    );
  }
  
  /// Get platform-specific UI constants
  static PlatformUIConstants getUIConstants() {
    if (isMobile) {
      return const PlatformUIConstants(
        defaultPadding: 16.0,
        compactPadding: 8.0,
        largePadding: 24.0,
        borderRadius: 12.0,
        iconSize: 24.0,
        appBarHeight: 56.0,
        navigationBarHeight: 56.0,
        fabSize: 56.0,
        listItemHeight: 56.0,
        buttonHeight: 48.0,
        inputHeight: 48.0,
        cardElevation: 2.0,
        maxContentWidth: double.infinity,
        breakpointMobile: 600.0,
        breakpointTablet: 900.0,
        breakpointDesktop: 1200.0,
      );
    } else if (isDesktop) {
      return const PlatformUIConstants(
        defaultPadding: 24.0,
        compactPadding: 12.0,
        largePadding: 32.0,
        borderRadius: 8.0,
        iconSize: 20.0,
        appBarHeight: 64.0,
        navigationBarHeight: 64.0,
        fabSize: 56.0,
        listItemHeight: 48.0,
        buttonHeight: 40.0,
        inputHeight: 40.0,
        cardElevation: 1.0,
        maxContentWidth: 1200.0,
        breakpointMobile: 600.0,
        breakpointTablet: 900.0,
        breakpointDesktop: 1200.0,
      );
    } else {
      // Web defaults
      return const PlatformUIConstants(
        defaultPadding: 20.0,
        compactPadding: 10.0,
        largePadding: 28.0,
        borderRadius: 6.0,
        iconSize: 22.0,
        appBarHeight: 60.0,
        navigationBarHeight: 60.0,
        fabSize: 56.0,
        listItemHeight: 52.0,
        buttonHeight: 44.0,
        inputHeight: 44.0,
        cardElevation: 1.5,
        maxContentWidth: 1400.0,
        breakpointMobile: 600.0,
        breakpointTablet: 900.0,
        breakpointDesktop: 1200.0,
      );
    }
  }
}

/// Platform types
enum PlatformType {
  web,
  android,
  ios,
  macos,
  windows,
  linux,
  fuchsia,
  unknown,
}

/// Platform features
enum PlatformFeature {
  camera,
  biometrics,
  pushNotifications,
  fileSystem,
  clipboard,
  sharing,
  systemTheme,
  windowControls,
  multiWindow,
  dragAndDrop,
  contextMenu,
  keyboard,
  mouse,
  touch,
}

/// Platform-specific configuration
class PlatformConfig {
  final PlatformType platform;
  final int defaultPageSize;
  final int maxConcurrentRequests;
  final int cacheSize;
  final bool enableAnimations;
  final bool enableHapticFeedback;
  final bool enableKeyboardShortcuts;
  final bool enableContextMenu;
  final bool enableDragAndDrop;
  final bool preferNativeControls;
  final bool useSystemNavigationBar;
  final bool enableFullscreenMode;
  final Size? defaultWindowSize;
  final Size? minWindowSize;
  
  const PlatformConfig({
    required this.platform,
    required this.defaultPageSize,
    required this.maxConcurrentRequests,
    required this.cacheSize,
    required this.enableAnimations,
    required this.enableHapticFeedback,
    required this.enableKeyboardShortcuts,
    required this.enableContextMenu,
    required this.enableDragAndDrop,
    required this.preferNativeControls,
    required this.useSystemNavigationBar,
    required this.enableFullscreenMode,
    this.defaultWindowSize,
    this.minWindowSize,
  });
}

/// Platform-specific UI constants
class PlatformUIConstants {
  final double defaultPadding;
  final double compactPadding;
  final double largePadding;
  final double borderRadius;
  final double iconSize;
  final double appBarHeight;
  final double navigationBarHeight;
  final double fabSize;
  final double listItemHeight;
  final double buttonHeight;
  final double inputHeight;
  final double cardElevation;
  final double maxContentWidth;
  final double breakpointMobile;
  final double breakpointTablet;
  final double breakpointDesktop;
  
  const PlatformUIConstants({
    required this.defaultPadding,
    required this.compactPadding,
    required this.largePadding,
    required this.borderRadius,
    required this.iconSize,
    required this.appBarHeight,
    required this.navigationBarHeight,
    required this.fabSize,
    required this.listItemHeight,
    required this.buttonHeight,
    required this.inputHeight,
    required this.cardElevation,
    required this.maxContentWidth,
    required this.breakpointMobile,
    required this.breakpointTablet,
    required this.breakpointDesktop,
  });
}

/// Platform-aware widget that adapts its content based on the current platform
class PlatformAware extends StatelessWidget {
  final Widget? mobile;
  final Widget? tablet;
  final Widget? desktop;
  final Widget? web;
  final Widget? android;
  final Widget? ios;
  final Widget? macos;
  final Widget? windows;
  final Widget? linux;
  final Widget fallback;
  
  const PlatformAware({
    super.key,
    this.mobile,
    this.tablet,
    this.desktop,
    this.web,
    this.android,
    this.ios,
    this.macos,
    this.windows,
    this.linux,
    required this.fallback,
  });
  
  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final uiConstants = PlatformInfo.getUIConstants();
    
    // Platform-specific widgets
    if (PlatformInfo.isAndroid && android != null) return android!;
    if (PlatformInfo.isIOS && ios != null) return ios!;
    if (PlatformInfo.isMacOS && macos != null) return macos!;
    if (PlatformInfo.isWindows && windows != null) return windows!;
    if (PlatformInfo.isLinux && linux != null) return linux!;
    if (PlatformInfo.isWeb && web != null) return web!;
    
    // Screen size-based widgets
    if (screenWidth < uiConstants.breakpointMobile && mobile != null) {
      return mobile!;
    } else if (screenWidth < uiConstants.breakpointTablet && tablet != null) {
      return tablet!;
    } else if (screenWidth >= uiConstants.breakpointDesktop && desktop != null) {
      return desktop!;
    }
    
    // Category-based fallbacks
    if (PlatformInfo.isMobile && mobile != null) return mobile!;
    if (PlatformInfo.isDesktop && desktop != null) return desktop!;
    if (PlatformInfo.isWeb && web != null) return web!;
    
    return fallback;
  }
}