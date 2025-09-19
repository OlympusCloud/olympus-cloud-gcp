import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'platform_info.dart';

/// Performance optimization utilities for different platforms
class PlatformPerformance {
  static bool _isInitialized = false;
  
  /// Initialize platform-specific performance optimizations
  static Future<void> initialize() async {
    if (_isInitialized) return;
    
    // Web-specific optimizations
    if (PlatformInfo.isWeb) {
      await _initializeWebOptimizations();
    }
    
    // Mobile-specific optimizations
    if (PlatformInfo.isMobile) {
      await _initializeMobileOptimizations();
    }
    
    // Desktop-specific optimizations
    if (PlatformInfo.isDesktop) {
      await _initializeDesktopOptimizations();
    }
    
    _isInitialized = true;
  }
  
  /// Web-specific performance optimizations
  static Future<void> _initializeWebOptimizations() async {
    // Enable web-specific features
    if (kDebugMode) {
      debugPrint('Initializing web performance optimizations');
    }
    
    // Configure caching strategies for web
    await _configureWebCaching();
  }
  
  /// Mobile-specific performance optimizations
  static Future<void> _initializeMobileOptimizations() async {
    // Configure battery and memory optimizations
    if (kDebugMode) {
      debugPrint('Initializing mobile performance optimizations');
    }
    
    // Set system UI overlay style for better performance
    SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      systemNavigationBarColor: Colors.transparent,
    ));
    
    // Configure orientation based on platform
    if (PlatformInfo.isAndroid || PlatformInfo.isIOS) {
      await SystemChrome.setPreferredOrientations([
        DeviceOrientation.portraitUp,
        DeviceOrientation.portraitDown,
        DeviceOrientation.landscapeLeft,
        DeviceOrientation.landscapeRight,
      ]);
    }
  }
  
  /// Desktop-specific performance optimizations
  static Future<void> _initializeDesktopOptimizations() async {
    if (kDebugMode) {
      debugPrint('Initializing desktop performance optimizations');
    }
    
    // Configure window settings for desktop
    await _configureDesktopWindow();
  }
  
  /// Configure web caching strategies
  static Future<void> _configureWebCaching() async {
    // Web-specific caching logic would go here
    // This is a placeholder for actual implementation
  }
  
  /// Configure desktop window settings
  static Future<void> _configureDesktopWindow() async {
    // Desktop window configuration would go here
    // This is a placeholder for actual implementation
  }
  
  /// Get optimized cache size based on platform
  static int getOptimizedCacheSize() {
    if (PlatformInfo.isMobile) {
      return 50; // Smaller cache for mobile devices
    } else if (PlatformInfo.isDesktop) {
      return 200; // Larger cache for desktop
    } else if (PlatformInfo.isWeb) {
      return 100; // Medium cache for web
    }
    return 100; // Default
  }
  
  /// Get optimized concurrent request limit
  static int getOptimizedRequestLimit() {
    if (PlatformInfo.isMobile) {
      return 3; // Conservative for mobile
    } else if (PlatformInfo.isDesktop) {
      return 8; // Higher for desktop
    } else if (PlatformInfo.isWeb) {
      return 6; // Medium for web
    }
    return 4; // Default
  }
  
  /// Check if hardware acceleration should be enabled
  static bool shouldEnableHardwareAcceleration() {
    if (PlatformInfo.isWeb) {
      return true; // Usually beneficial for web
    } else if (PlatformInfo.isMobile) {
      return true; // Generally good for mobile
    } else if (PlatformInfo.isDesktop) {
      return true; // Desktop can handle it
    }
    return false;
  }
  
  /// Get optimized animation settings
  static AnimationSettings getOptimizedAnimationSettings() {
    if (PlatformInfo.isMobile) {
      return const AnimationSettings(
        enableComplexAnimations: false,
        reducedMotion: false,
        defaultDuration: Duration(milliseconds: 200),
        enableHeroAnimations: true,
        enablePageTransitions: true,
      );
    } else if (PlatformInfo.isDesktop) {
      return const AnimationSettings(
        enableComplexAnimations: true,
        reducedMotion: false,
        defaultDuration: Duration(milliseconds: 300),
        enableHeroAnimations: true,
        enablePageTransitions: true,
      );
    } else if (PlatformInfo.isWeb) {
      return const AnimationSettings(
        enableComplexAnimations: false,
        reducedMotion: true,
        defaultDuration: Duration(milliseconds: 250),
        enableHeroAnimations: false,
        enablePageTransitions: true,
      );
    }
    
    return const AnimationSettings(
      enableComplexAnimations: false,
      reducedMotion: true,
      defaultDuration: Duration(milliseconds: 300),
      enableHeroAnimations: true,
      enablePageTransitions: true,
    );
  }
  
  /// Get optimized scroll physics
  static ScrollPhysics getOptimizedScrollPhysics() {
    if (PlatformInfo.isIOS) {
      return const BouncingScrollPhysics();
    } else if (PlatformInfo.isAndroid) {
      return const ClampingScrollPhysics();
    } else if (PlatformInfo.isDesktop || PlatformInfo.isWeb) {
      return const ClampingScrollPhysics();
    }
    return const ScrollPhysics();
  }
  
  /// Enable performance monitoring
  static void enablePerformanceMonitoring() {
    if (kDebugMode) {
      // Enable timeline in debug mode for performance analysis
      debugProfileBuildsEnabled = true;
    }
  }
  
  /// Disable performance monitoring
  static void disablePerformanceMonitoring() {
    if (kDebugMode) {
      debugProfileBuildsEnabled = false;
    }
  }
  
  /// Preload critical resources
  static Future<void> preloadCriticalResources() async {
    // Preload essential images, fonts, etc.
    if (kDebugMode) {
      debugPrint('Preloading critical resources for ${PlatformInfo.currentPlatform}');
    }
    
    // Platform-specific preloading logic would go here
  }
  
  /// Optimize memory usage
  static void optimizeMemoryUsage() {
    if (PlatformInfo.isMobile) {
      // Clear image cache periodically on mobile
      PaintingBinding.instance.imageCache.clear();
      PaintingBinding.instance.imageCache.clearLiveImages();
    }
  }
  
  /// Get memory pressure level
  static MemoryPressureLevel getMemoryPressureLevel() {
    // This would integrate with platform-specific memory monitoring
    // For now, return a default value
    return MemoryPressureLevel.normal;
  }
}

/// Animation settings configuration
class AnimationSettings {
  final bool enableComplexAnimations;
  final bool reducedMotion;
  final Duration defaultDuration;
  final bool enableHeroAnimations;
  final bool enablePageTransitions;
  
  const AnimationSettings({
    required this.enableComplexAnimations,
    required this.reducedMotion,
    required this.defaultDuration,
    required this.enableHeroAnimations,
    required this.enablePageTransitions,
  });
}

/// Memory pressure levels
enum MemoryPressureLevel {
  low,
  normal,
  high,
  critical,
}

/// Performance monitoring widget
class PerformanceMonitor extends StatefulWidget {
  final Widget child;
  final bool enabled;
  
  const PerformanceMonitor({
    super.key,
    required this.child,
    this.enabled = false,
  });
  
  @override
  State<PerformanceMonitor> createState() => _PerformanceMonitorState();
}

class _PerformanceMonitorState extends State<PerformanceMonitor> {
  @override
  void initState() {
    super.initState();
    if (widget.enabled && kDebugMode) {
      PlatformPerformance.enablePerformanceMonitoring();
    }
  }
  
  @override
  void dispose() {
    if (widget.enabled && kDebugMode) {
      PlatformPerformance.disablePerformanceMonitoring();
    }
    super.dispose();
  }
  
  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}

/// Optimized list view based on platform
class OptimizedListView extends StatelessWidget {
  final int itemCount;
  final Widget Function(BuildContext, int) itemBuilder;
  final ScrollController? controller;
  final bool shrinkWrap;
  final EdgeInsetsGeometry? padding;
  
  const OptimizedListView({
    super.key,
    required this.itemCount,
    required this.itemBuilder,
    this.controller,
    this.shrinkWrap = false,
    this.padding,
  });
  
  @override
  Widget build(BuildContext context) {
    final scrollPhysics = PlatformPerformance.getOptimizedScrollPhysics();
    
    if (PlatformInfo.isMobile || itemCount > 100) {
      // Use ListView.builder for better performance on mobile or large lists
      return ListView.builder(
        controller: controller,
        physics: scrollPhysics,
        shrinkWrap: shrinkWrap,
        padding: padding,
        itemCount: itemCount,
        itemBuilder: itemBuilder,
        cacheExtent: PlatformInfo.isMobile ? 200 : 500,
      );
    } else {
      // Use regular ListView for smaller lists on desktop/web
      return ListView.builder(
        controller: controller,
        physics: scrollPhysics,
        shrinkWrap: shrinkWrap,
        padding: padding,
        itemCount: itemCount,
        itemBuilder: itemBuilder,
      );
    }
  }
}

/// Platform-optimized image widget
class OptimizedImage extends StatelessWidget {
  final String? url;
  final String? assetPath;
  final double? width;
  final double? height;
  final BoxFit? fit;
  final Widget? placeholder;
  final Widget? errorWidget;
  
  const OptimizedImage({
    super.key,
    this.url,
    this.assetPath,
    this.width,
    this.height,
    this.fit,
    this.placeholder,
    this.errorWidget,
  });
  
  @override
  Widget build(BuildContext context) {
    // Platform-specific image optimization
    if (url != null) {
      // Network image with platform-specific caching
      return Image.network(
        url!,
        width: width,
        height: height,
        fit: fit,
        loadingBuilder: (context, child, loadingProgress) {
          if (loadingProgress == null) return child;
          return placeholder ?? const CircularProgressIndicator();
        },
        errorBuilder: (context, error, stackTrace) {
          return errorWidget ?? const Icon(Icons.error);
        },
        // Enable caching based on platform capabilities
        cacheWidth: PlatformInfo.isMobile ? 500 : null,
        cacheHeight: PlatformInfo.isMobile ? 500 : null,
      );
    } else if (assetPath != null) {
      // Asset image
      return Image.asset(
        assetPath!,
        width: width,
        height: height,
        fit: fit,
        // Optimize for platform
        cacheWidth: PlatformInfo.isMobile ? 500 : null,
        cacheHeight: PlatformInfo.isMobile ? 500 : null,
      );
    }
    
    return errorWidget ?? const SizedBox.shrink();
  }
}