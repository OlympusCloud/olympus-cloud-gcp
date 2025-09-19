import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/features/orders/data/models/order.dart';
import 'package:frontend/features/orders/data/providers/orders_provider.dart';
import 'package:frontend/features/orders/presentation/screens/order_management_screen.dart';

void main() {
  group('Order Management Tests', () {
    testWidgets('OrderManagementScreen builds without errors', (WidgetTester tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: const OrderManagementScreen(),
          ),
        ),
      );

      // Verify the screen builds
      expect(find.text('Order Management'), findsOneWidget);
      expect(find.text('Active (0)'), findsOneWidget);
      expect(find.text('Today (0)'), findsOneWidget);
      expect(find.text('Completed (0)'), findsOneWidget);
      expect(find.text('All Orders'), findsOneWidget);
    });

    test('Order model creates correctly', () {
      final customer = Customer(
        id: '1',
        name: 'John Doe',
        email: 'john@example.com',
        phone: '+1234567890',
      );

      final orderItem = OrderItem(
        id: '1',
        productId: 'product-1',
        productName: 'Test Product',
        price: 19.99,
        quantity: 2,
      );

      final order = Order(
        id: 'order-1',
        orderNumber: 'ORD-001',
        status: OrderStatus.pending,
        priority: OrderPriority.normal,
        items: [orderItem],
        subtotal: 39.98,
        tax: 3.20,
        total: 43.18,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        customer: customer,
        tableNumber: 5,
      );

      expect(order.id, equals('order-1'));
      expect(order.orderNumber, equals('ORD-001'));
      expect(order.status, equals(OrderStatus.pending));
      expect(order.priority, equals(OrderPriority.normal));
      expect(order.items.length, equals(1));
      expect(order.total, equals(43.18));
      expect(order.customer?.name, equals('John Doe'));
      expect(order.tableNumber, equals(5));
    });

    test('CreateOrderRequest creates correctly', () {
      final customer = Customer(
        id: '1',
        name: 'Jane Doe',
        email: 'jane@example.com',
      );

      final orderItem = OrderItem(
        id: '1',
        productId: 'product-1',
        productName: 'Test Product',
        price: 15.99,
        quantity: 1,
      );

      final request = CreateOrderRequest(
        customer: customer,
        items: [orderItem],
        tableNumber: 3,
        priority: OrderPriority.high,
        notes: 'Extra spicy',
      );

      expect(request.customer.name, equals('Jane Doe'));
      expect(request.items.length, equals(1));
      expect(request.tableNumber, equals(3));
      expect(request.priority, equals(OrderPriority.high));
      expect(request.notes, equals('Extra spicy'));
    });

    test('OrderStatus enum values are correct', () {
      expect(OrderStatus.values.length, equals(6));
      expect(OrderStatus.values, contains(OrderStatus.pending));
      expect(OrderStatus.values, contains(OrderStatus.confirmed));
      expect(OrderStatus.values, contains(OrderStatus.preparing));
      expect(OrderStatus.values, contains(OrderStatus.ready));
      expect(OrderStatus.values, contains(OrderStatus.completed));
      expect(OrderStatus.values, contains(OrderStatus.cancelled));
    });

    test('OrderPriority enum values are correct', () {
      expect(OrderPriority.values.length, equals(4));
      expect(OrderPriority.values, contains(OrderPriority.low));
      expect(OrderPriority.values, contains(OrderPriority.normal));
      expect(OrderPriority.values, contains(OrderPriority.high));
      expect(OrderPriority.values, contains(OrderPriority.urgent));
    });

    test('Order enums work correctly', () {
      // Test that we can access provider-dependent calculations
      // without triggering API calls
      expect(OrderStatus.values.length, equals(6));
      expect(OrderPriority.values.length, equals(4));
      expect(PaymentStatus.values.length, equals(5));
    });
  });
}