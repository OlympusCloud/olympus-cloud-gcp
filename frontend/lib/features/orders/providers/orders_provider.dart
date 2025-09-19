import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/order.dart';
import '../../../core/services/api_service.dart';

/// Orders state provider
final ordersProvider = StateNotifierProvider<OrdersNotifier, AsyncValue<List<Order>>>((ref) {
  return OrdersNotifier();
});

/// Order details provider for specific order
final orderDetailsProvider = FutureProvider.family<Order, String>((ref, orderId) async {
  final response = await ApiService.get('/orders/$orderId');
  return Order.fromJson(response.data);
});

/// Active orders provider (pending, confirmed, preparing)
final activeOrdersProvider = Provider<List<Order>>((ref) {
  final ordersAsync = ref.watch(ordersProvider);
  return ordersAsync.maybeWhen(
    data: (orders) => orders.where((order) => 
      order.status == OrderStatus.pending ||
      order.status == OrderStatus.confirmed ||
      order.status == OrderStatus.preparing ||
      order.status == OrderStatus.ready
    ).toList(),
    orElse: () => [],
  );
});

/// Completed orders provider
final completedOrdersProvider = Provider<List<Order>>((ref) {
  final ordersAsync = ref.watch(ordersProvider);
  return ordersAsync.maybeWhen(
    data: (orders) => orders.where((order) => 
      order.status == OrderStatus.completed
    ).toList(),
    orElse: () => [],
  );
});

/// Orders by priority provider
final ordersByPriorityProvider = Provider<Map<OrderPriority, List<Order>>>((ref) {
  final ordersAsync = ref.watch(ordersProvider);
  return ordersAsync.maybeWhen(
    data: (orders) {
      final Map<OrderPriority, List<Order>> ordersByPriority = {};
      for (final priority in OrderPriority.values) {
        ordersByPriority[priority] = orders
            .where((order) => order.priority == priority)
            .toList();
      }
      return ordersByPriority;
    },
    orElse: () => {},
  );
});

/// Today's orders provider
final todaysOrdersProvider = Provider<List<Order>>((ref) {
  final ordersAsync = ref.watch(ordersProvider);
  final now = DateTime.now();
  final today = DateTime(now.year, now.month, now.day);
  final tomorrow = today.add(const Duration(days: 1));
  
  return ordersAsync.maybeWhen(
    data: (orders) => orders.where((order) => 
      order.createdAt.isAfter(today) && 
      order.createdAt.isBefore(tomorrow)
    ).toList(),
    orElse: () => [],
  );
});

/// Orders revenue provider
final ordersRevenueProvider = Provider<double>((ref) {
  final ordersAsync = ref.watch(ordersProvider);
  return ordersAsync.maybeWhen(
    data: (orders) => orders
        .where((order) => order.status == OrderStatus.completed)
        .fold(0.0, (sum, order) => sum + order.total),
    orElse: () => 0.0,
  );
});

/// Orders notifier
class OrdersNotifier extends StateNotifier<AsyncValue<List<Order>>> {
  OrdersNotifier() : super(const AsyncValue.loading()) {
    loadOrders();
  }

  /// Load all orders
  Future<void> loadOrders() async {
    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/orders');
      final orders = (response.data['orders'] as List)
          .map((orderJson) => Order.fromJson(orderJson))
          .toList();
      
      state = AsyncValue.data(orders);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Create new order
  Future<Order?> createOrder(CreateOrderRequest request) async {
    try {
      final response = await ApiService.post('/orders', data: request.toJson());
      final newOrder = Order.fromJson(response.data);
      
      // Add to current state
      state.whenData((orders) {
        state = AsyncValue.data([newOrder, ...orders]);
      });
      
      return newOrder;
    } catch (error) {
      // Handle error - could show snackbar or notification
      rethrow;
    }
  }

  /// Update existing order
  Future<void> updateOrder(String orderId, UpdateOrderRequest request) async {
    try {
      final response = await ApiService.put('/orders/$orderId', data: request.toJson());
      final updatedOrder = Order.fromJson(response.data);
      
      // Update in current state
      state.whenData((orders) {
        final updatedOrders = orders.map((order) => 
          order.id == orderId ? updatedOrder : order
        ).toList();
        state = AsyncValue.data(updatedOrders);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Update order status
  Future<void> updateOrderStatus(String orderId, OrderStatus newStatus) async {
    await updateOrder(orderId, UpdateOrderRequest(status: newStatus));
  }

  /// Cancel order
  Future<void> cancelOrder(String orderId, String reason) async {
    try {
      await ApiService.post('/orders/$orderId/cancel', data: {'reason': reason});
      await updateOrderStatus(orderId, OrderStatus.cancelled);
    } catch (error) {
      rethrow;
    }
  }

  /// Delete order
  Future<void> deleteOrder(String orderId) async {
    try {
      await ApiService.delete('/orders/$orderId');
      
      // Remove from current state
      state.whenData((orders) {
        final updatedOrders = orders.where((order) => order.id != orderId).toList();
        state = AsyncValue.data(updatedOrders);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Search orders
  Future<void> searchOrders(String query) async {
    if (query.trim().isEmpty) {
      await loadOrders();
      return;
    }

    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/orders/search', queryParameters: {
        'q': query,
      });
      
      final orders = (response.data['orders'] as List)
          .map((orderJson) => Order.fromJson(orderJson))
          .toList();
      
      state = AsyncValue.data(orders);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Filter orders by status
  Future<void> filterOrdersByStatus(List<OrderStatus> statuses) async {
    state = const AsyncValue.loading();
    
    try {
      final statusStrings = statuses.map((s) => s.name).toList();
      final response = await ApiService.get('/orders', queryParameters: {
        'status': statusStrings.join(','),
      });
      
      final orders = (response.data['orders'] as List)
          .map((orderJson) => Order.fromJson(orderJson))
          .toList();
      
      state = AsyncValue.data(orders);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Filter orders by date range
  Future<void> filterOrdersByDateRange(DateTime startDate, DateTime endDate) async {
    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/orders', queryParameters: {
        'start_date': startDate.toIso8601String(),
        'end_date': endDate.toIso8601String(),
      });
      
      final orders = (response.data['orders'] as List)
          .map((orderJson) => Order.fromJson(orderJson))
          .toList();
      
      state = AsyncValue.data(orders);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Refresh orders
  Future<void> refresh() async {
    await loadOrders();
  }
}