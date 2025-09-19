import 'package:flutter/material.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import 'core/router/app_router.dart';
import 'core/theme/app_theme.dart';
import 'core/branding/branding_provider.dart';
import 'core/constants/app_constants.dart';
import 'core/platform/platform_info.dart';
import 'core/platform/platform_performance.dart';
import 'core/platform/platform_input.dart';
import 'core/platform/responsive_layout.dart';

class OlympusApp extends ConsumerWidget {
  const OlympusApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isDarkMode = ref.watch(isDarkModeProvider);
    final branding = ref.watch(brandingProvider);
    
    // Get industry-specific theme
    final theme = ref.watch(brandingThemeProvider(false));
    final darkTheme = ref.watch(brandingThemeProvider(true));

    return PerformanceMonitor(
      enabled: AppConstants.isDebugMode,
      child: KeyboardShortcutHandler(
        shortcuts: _getKeyboardShortcuts(),
        child: MaterialApp.router(
          title: branding.brandName,
          debugShowCheckedModeBanner: false,
          
          // Dynamic theme based on industry branding
          theme: theme,
          darkTheme: darkTheme,
          themeMode: isDarkMode ? ThemeMode.dark : ThemeMode.light,
          
          // Internationalization
          localizationsDelegates: const [
            GlobalMaterialLocalizations.delegate,
            GlobalWidgetsLocalizations.delegate,
            GlobalCupertinoLocalizations.delegate,
          ],
          supportedLocales: const [
            Locale('en', 'US'),
            Locale('es', 'ES'),
            Locale('fr', 'FR'),
            Locale('de', 'DE'),
            Locale('ja', 'JP'),
          ],
          
          // Navigation
          routerConfig: AppRouter.router,
          
          // Platform-optimized responsive UI
          builder: (context, child) {
            return MediaQuery(
              data: MediaQuery.of(context).copyWith(
                textScaler: TextScaler.linear(
                  MediaQuery.of(context).textScaler.scale(
                    ResponsiveLayout.getFontSizeMultiplier(context)
                  ).clamp(0.8, 1.5),
                ),
              ),
              child: child != null ? PlatformAware(
                mobile: _buildMobileLayout(child),
                desktop: _buildDesktopLayout(child),
                web: _buildWebLayout(child),
                fallback: child,
              ) : const SizedBox.shrink(),
            );
          },
          
          // Platform-specific scroll behavior
          scrollBehavior: _PlatformScrollBehavior(),
        ),
      ),
    );
  }

  /// Get platform-specific keyboard shortcuts
  Map<LogicalKeySet, VoidCallback> _getKeyboardShortcuts() {
    return {
      LogicalKeySet(LogicalKeyboardKey.f5): () {
        // Refresh action
      },
      LogicalKeySet(LogicalKeyboardKey.f1): () {
        // Help action
      },
      LogicalKeySet(LogicalKeyboardKey.escape): () {
        // Dismiss action
      },
    };
  }

  /// Build mobile-optimized layout
  Widget _buildMobileLayout(Widget child) {
    return child;
  }

  /// Build desktop-optimized layout
  Widget _buildDesktopLayout(Widget child) {
    return child;
  }

  /// Build web-optimized layout
  Widget _buildWebLayout(Widget child) {
    return child;
  }
}

// Dark mode state provider
final isDarkModeProvider = StateProvider<bool>((ref) {
  // Default to system theme preference
  final brightness = 
      WidgetsBinding.instance.platformDispatcher.platformBrightness;
  return brightness == Brightness.dark;
});

/// Platform-specific scroll behavior
class _PlatformScrollBehavior extends MaterialScrollBehavior {
  @override
  Set<PointerDeviceKind> get dragDevices => {
    PointerDeviceKind.touch,
    PointerDeviceKind.mouse,
    PointerDeviceKind.stylus,
    PointerDeviceKind.trackpad,
  };

  @override
  ScrollPhysics getScrollPhysics(BuildContext context) {
    return PlatformPerformance.getOptimizedScrollPhysics();
  }
}