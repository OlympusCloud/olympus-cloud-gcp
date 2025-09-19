import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';

import 'app.dart';
import 'core/services/storage_service.dart';
import 'core/services/api_service.dart';
import 'core/constants/app_constants.dart';
import 'core/platform/platform_performance.dart';

Future<void> main() async {
  // Ensure Flutter binding is initialized
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Hive for local storage
  await Hive.initFlutter();
  await StorageService.initialize();

  // Initialize API service
  ApiService.initialize();

  // Initialize platform-specific performance optimizations
  await PlatformPerformance.initialize();

  // Preload critical resources
  await PlatformPerformance.preloadCriticalResources();

  // Set system UI overlay style
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarIconBrightness: Brightness.dark,
      systemNavigationBarColor: Colors.transparent,
      systemNavigationBarIconBrightness: Brightness.dark,
    ),
  );

  // Set preferred orientations for mobile
  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
    DeviceOrientation.landscapeLeft,
    DeviceOrientation.landscapeRight,
  ]);

  // Run the app with Riverpod provider scope
  runApp(
    ProviderScope(
      observers: [
        if (AppConstants.isDebugMode) AppProviderObserver(),
      ],
      child: const OlympusApp(),
    ),
  );
}

/// Provider observer for debugging in development
class AppProviderObserver extends ProviderObserver {
  @override
  void didAddProvider(
    ProviderBase<Object?> provider,
    Object? value,
    ProviderContainer container,
  ) {
    if (AppConstants.isDebugMode) {
      debugPrint('Provider added: ${provider.name ?? provider.runtimeType}');
    }
  }

  @override
  void didDisposeProvider(
    ProviderBase<Object?> provider,
    ProviderContainer container,
  ) {
    if (AppConstants.isDebugMode) {
      debugPrint('Provider disposed: ${provider.name ?? provider.runtimeType}');
    }
  }

  @override
  void didUpdateProvider(
    ProviderBase<Object?> provider,
    Object? previousValue,
    Object? newValue,
    ProviderContainer container,
  ) {
    if (AppConstants.isDebugMode) {
      debugPrint('Provider updated: ${provider.name ?? provider.runtimeType}');
    }
  }
}
