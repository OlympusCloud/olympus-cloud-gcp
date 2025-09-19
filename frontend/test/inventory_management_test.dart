import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/features/inventory/presentation/screens/inventory_management_screen.dart';
import 'package:frontend/features/inventory/models/product.dart'; 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/features/inventory/presentation/screens/inventory_management_screen.dart';
import 'package:frontend/features/inventory/models/product.dart';
import 'package:frontend/features/inventory/providers/products_provider.dart';

void main() {
  group('InventoryManagementScreen Tests', () {
    late ProviderContainer container;

    setUp(() {
      container = ProviderContainer();
    });

    tearDown(() {
      container.dispose();
    });

    testWidgets('should display inventory management screen with tabs', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      // Wait for the widget to settle
      await tester.pumpAndSettle();

      // Verify the AppBar title
      expect(find.text('Inventory Management'), findsOneWidget);

      // Verify tabs are present
      expect(find.text('All (0)'), findsOneWidget);
      expect(find.text('Low Stock (0)'), findsOneWidget);
      expect(find.text('Out of Stock (0)'), findsOneWidget);
      expect(find.text('Categories'), findsOneWidget);

      // Verify search bar
      expect(find.text('Search products by name, SKU, or category...'), findsOneWidget);

      // Verify metrics cards
      expect(find.text('Total Value'), findsOneWidget);
      expect(find.text('Products'), findsOneWidget);
      expect(find.text('Low Stock'), findsOneWidget);
      expect(find.text('Out of Stock'), findsOneWidget);

      // Verify floating action button
      expect(find.text('Add Product'), findsOneWidget);
    });

    testWidgets('should display empty state when no products', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show empty state message
      expect(find.text('No active products'), findsOneWidget);
    });

    testWidgets('should open create product dialog when FAB is tapped', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap the floating action button
      await tester.tap(find.text('Add Product'));
      await tester.pumpAndSettle();

      // Should open create product dialog
      expect(find.text('Create New Product'), findsOneWidget);
      expect(find.text('Basic Information'), findsOneWidget);
      expect(find.text('Product Name *'), findsOneWidget);
      expect(find.text('Description *'), findsOneWidget);
    });

    testWidgets('should show filter dialog when filter button is tapped', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap the filter button
      await tester.tap(find.byIcon(Icons.filter_list));
      await tester.pumpAndSettle();

      // Should open filter dialog
      expect(find.text('Filter Products'), findsOneWidget);
      expect(find.text('Category'), findsOneWidget);
      expect(find.text('Status'), findsOneWidget);
      expect(find.text('All Categories'), findsOneWidget);
      expect(find.text('All Statuses'), findsOneWidget);
    });

    testWidgets('should navigate between tabs', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap on Low Stock tab
      await tester.tap(find.text('Low Stock (0)'));
      await tester.pumpAndSettle();

      // Should show low stock empty state
      expect(find.text('No low stock products'), findsOneWidget);

      // Tap on Out of Stock tab
      await tester.tap(find.text('Out of Stock (0)'));
      await tester.pumpAndSettle();

      // Should show out of stock empty state
      expect(find.text('No out of stock products'), findsOneWidget);

      // Tap on Categories tab
      await tester.tap(find.text('Categories'));
      await tester.pumpAndSettle();

      // Should show categories view (empty for now)
      expect(find.text('No products found'), findsOneWidget);
    });

    testWidgets('should update search query', (WidgetTester tester) async {
      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: const InventoryManagementScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Find the search field
      final searchField = find.byType(TextField);
      expect(searchField, findsOneWidget);

      // Enter search text
      await tester.enterText(searchField, 'test product');
      await tester.pumpAndSettle();

      // Should show clear button
      expect(find.byIcon(Icons.clear), findsOneWidget);
    });

    group('CreateProductDialog Tests', () {
      testWidgets('should validate required fields', (WidgetTester tester) async {
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: container,
            child: MaterialApp(
              home: Scaffold(
                body: Builder(
                  builder: (context) {
                    return ElevatedButton(
                      onPressed: () {
                        showDialog(
                          context: context,
                          builder: (context) => const CreateProductDialog(),
                        );
                      },
                      child: const Text('Open Dialog'),
                    );
                  },
                ),
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Try to create product without required fields
        await tester.tap(find.text('Create Product'));
        await tester.pumpAndSettle();

        // Should show validation errors
        expect(find.text('Please enter product name'), findsOneWidget);
        expect(find.text('Please enter description'), findsOneWidget);
        expect(find.text('Please enter price'), findsOneWidget);
      });

      testWidgets('should create product with valid data', (WidgetTester tester) async {
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: container,
            child: MaterialApp(
              home: Scaffold(
                body: Builder(
                  builder: (context) {
                    return ElevatedButton(
                      onPressed: () {
                        showDialog(
                          context: context,
                          builder: (context) => const CreateProductDialog(),
                        );
                      },
                      child: const Text('Open Dialog'),
                    );
                  },
                ),
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Fill in required fields
        await tester.enterText(find.widgetWithText(TextFormField, 'Product Name *'), 'Test Product');
        await tester.enterText(find.widgetWithText(TextFormField, 'Description *'), 'Test Description');
        await tester.enterText(find.widgetWithText(TextFormField, 'Base Price *'), '10.99');

        // The form should be valid now
        expect(find.text('Test Product'), findsOneWidget);
        expect(find.text('Test Description'), findsOneWidget);
        expect(find.text('10.99'), findsOneWidget);
      });

      testWidgets('should toggle stock tracking', (WidgetTester tester) async {
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: container,
            child: MaterialApp(
              home: Scaffold(
                body: Builder(
                  builder: (context) {
                    return ElevatedButton(
                      onPressed: () {
                        showDialog(
                          context: context,
                          builder: (context) => const CreateProductDialog(),
                        );
                      },
                      child: const Text('Open Dialog'),
                    );
                  },
                ),
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Track Stock should be enabled by default
        expect(find.text('Track Stock'), findsOneWidget);
        expect(find.text('Current Stock'), findsOneWidget);
        expect(find.text('Low Stock Threshold'), findsOneWidget);

        // Toggle off Track Stock
        await tester.tap(find.byType(Checkbox), warnIfMissed: false);
        await tester.pumpAndSettle();

        // Stock fields should be hidden
        expect(find.text('Current Stock'), findsNothing);
        expect(find.text('Low Stock Threshold'), findsNothing);
      });
    });

    group('ProductDetailsDialog Tests', () {
      final testProduct = Product(
        id: 'test-1',
        name: 'Test Product',
        description: 'Test Description',
        category: ProductCategory.food,
        status: ProductStatus.active,
        pricing: ProductPricing(
          basePrice: 10.99,
          cost: 5.00,
        ),
        inventory: ProductInventory(
          trackStock: true,
          currentStock: 100,
          lowStockThreshold: 10,
        ),
        sku: 'TEST-001',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      testWidgets('should display product details', (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showDialog(
                        context: context,
                        builder: (context) => ProductDetailsDialog(product: testProduct),
                      );
                    },
                    child: const Text('Open Dialog'),
                  );
                },
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Should display product information
        expect(find.text('Test Product'), findsOneWidget);
        expect(find.text('Test Description'), findsOneWidget);
        expect(find.text('SKU: TEST-001'), findsOneWidget);
        expect(find.text('Base Price: \$10.99'), findsOneWidget);
        expect(find.text('Cost: \$5.00'), findsOneWidget);
        expect(find.text('Current Stock: 100 units'), findsOneWidget);
        expect(find.text('Low Stock Threshold: 10'), findsOneWidget);
      });
    });

    group('StockAdjustmentDialog Tests', () {
      final testProduct = Product(
        id: 'test-1',
        name: 'Test Product',
        description: 'Test Description',
        category: ProductCategory.food,
        status: ProductStatus.active,
        pricing: ProductPricing(basePrice: 10.99),
        inventory: ProductInventory(
          trackStock: true,
          currentStock: 50,
        ),
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      testWidgets('should display stock adjustment form', (WidgetTester tester) async {
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: container,
            child: MaterialApp(
              home: Scaffold(
                body: Builder(
                  builder: (context) {
                    return ElevatedButton(
                      onPressed: () {
                        showDialog(
                          context: context,
                          builder: (context) => StockAdjustmentDialog(product: testProduct),
                        );
                      },
                      child: const Text('Open Dialog'),
                    );
                  },
                ),
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Should display stock adjustment form
        expect(find.text('Adjust Stock').first, findsOneWidget);
        expect(find.text('Test Product'), findsOneWidget);
        expect(find.text('Current Stock: 50 units'), findsOneWidget);
        expect(find.text('Add'), findsOneWidget);
        expect(find.text('Remove'), findsOneWidget);
        expect(find.text('Quantity to Add'), findsOneWidget);
      });

      testWidgets('should switch between add and remove', (WidgetTester tester) async {
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: container,
            child: MaterialApp(
              home: Scaffold(
                body: Builder(
                  builder: (context) {
                    return ElevatedButton(
                      onPressed: () {
                        showDialog(
                          context: context,
                          builder: (context) => StockAdjustmentDialog(product: testProduct),
                        );
                      },
                      child: const Text('Open Dialog'),
                    );
                  },
                ),
              ),
            ),
          ),
        );

        // Open the dialog
        await tester.tap(find.text('Open Dialog'));
        await tester.pumpAndSettle();

        // Should default to "Add"
        expect(find.text('Quantity to Add'), findsOneWidget);

        // Switch to "Remove"
        await tester.tap(find.text('Remove'));
        await tester.pumpAndSettle();

        // Should change to "Remove"
        expect(find.text('Quantity to Remove'), findsOneWidget);
      });
    });
  });
}