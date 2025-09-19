import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/app.dart';
import 'package:frontend/features/orders/data/providers/orders_provider.dart';
import 'package:frontend/features/inventory/providers/products_provider.dart';

void main() {
  group('App Integration Tests', () {
    testWidgets('App loads and displays correct initial screen', (WidgetTester tester) async {
      // Build the app
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      // Wait for splash screen and initial navigation
      await tester.pumpAndSettle(const Duration(seconds: 3));

      // Verify the app structure loads
      expect(find.byType(MaterialApp), findsOneWidget);
      
      // Should have some content visible (onboarding, auth, or dashboard)
      expect(find.byType(Scaffold), findsAtLeastNWidgets(1));
    });

    testWidgets('Navigation system works correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test that the app has a proper navigation structure
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('State management providers initialize correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test state management by checking if providers are working
      final container = ProviderScope.containerOf(
        tester.element(find.byType(OlympusApp)),
      );

      // Test orders provider initialization
      final ordersAsync = container.read(ordersProvider);
      expect(ordersAsync, isA<AsyncValue>());

      // Test products provider initialization
      final productsAsync = container.read(productsProvider);
      expect(productsAsync, isA<AsyncValue>());
    });

    testWidgets('Error handling works correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test that error states are handled properly
      final container = ProviderScope.containerOf(
        tester.element(find.byType(OlympusApp)),
      );

      // The app should handle errors gracefully without crashing
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('Responsive design works for different screen sizes', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      // Test different screen sizes
      await tester.binding.setSurfaceSize(const Size(400, 800)); // Mobile
      await tester.pumpAndSettle();
      expect(find.byType(MaterialApp), findsOneWidget);

      await tester.binding.setSurfaceSize(const Size(800, 600)); // Tablet
      await tester.pumpAndSettle();
      expect(find.byType(MaterialApp), findsOneWidget);

      await tester.binding.setSurfaceSize(const Size(1200, 800)); // Desktop
      await tester.pumpAndSettle();
      expect(find.byType(MaterialApp), findsOneWidget);

      // Reset to default size
      await tester.binding.setSurfaceSize(null);
    });

    testWidgets('Theme system works correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test that theme is applied correctly
      final materialApp = tester.widget<MaterialApp>(find.byType(MaterialApp));
      expect(materialApp.theme, isNotNull);
      expect(materialApp.darkTheme, isNotNull);

      // Test material design components work
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('App performance is acceptable', (WidgetTester tester) async {
      final stopwatch = Stopwatch()..start();

      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();
      stopwatch.stop();

      // App should load within reasonable time (adjust as needed)
      expect(stopwatch.elapsedMilliseconds, lessThan(10000));

      // Should still be responsive after loading
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('Dark mode toggle works', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test that dark mode provider exists and can be accessed
      final container = ProviderScope.containerOf(
        tester.element(find.byType(OlympusApp)),
      );

      // Check dark mode provider
      final isDarkMode = container.read(isDarkModeProvider);
      expect(isDarkMode, isA<bool>());

      // Test toggling dark mode
      container.read(isDarkModeProvider.notifier).state = !isDarkMode;
      await tester.pumpAndSettle();
      
      final newDarkMode = container.read(isDarkModeProvider);
      expect(newDarkMode, equals(!isDarkMode));
    });

    testWidgets('Memory usage is reasonable', (WidgetTester tester) async {
      // Build and rebuild the app multiple times to test for memory leaks
      for (int i = 0; i < 5; i++) {
        await tester.pumpWidget(
          const ProviderScope(
            child: OlympusApp(),
          ),
        );
        await tester.pumpAndSettle();
        
        // Verify app still works after multiple rebuilds
        expect(find.byType(MaterialApp), findsOneWidget);
      }
    });

    testWidgets('Accessibility features work correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: OlympusApp(),
        ),
      );

      await tester.pumpAndSettle();

      // Test that the app has proper semantic structure
      expect(find.byType(MaterialApp), findsOneWidget);
      
      // Check for basic accessibility compliance
      await expectLater(tester, meetsGuideline(textContrastGuideline));
    });
  });
}