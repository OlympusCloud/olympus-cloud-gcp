import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../api_client.dart';

/// Restaurant API service for Python analytics service endpoints
class RestaurantApiService {
  final ApiClient _client;
  
  RestaurantApiService(this._client);
  
  /// Get restaurant analytics
  Future<RestaurantAnalytics> getAnalytics({
    required String tenantId,
    String? locationId,
  }) async {
    final response = await _client.get(
      '${ApiClient.restaurantService}/analytics',
      queryParameters: {
        'tenant_id': tenantId,
        if (locationId != null) 'location_id': locationId,
      },
    );
    
    return RestaurantAnalytics.fromJson(response.data);
  }
  
  /// Get AI-powered restaurant recommendations
  Future<List<RestaurantRecommendation>> getRecommendations({
    required String tenantId,
    String? locationId,
  }) async {
    final response = await _client.get(
      '${ApiClient.restaurantService}/recommendations',
      queryParameters: {
        'tenant_id': tenantId,
        if (locationId != null) 'location_id': locationId,
      },
    );
    
    return (response.data as List)
        .map((item) => RestaurantRecommendation.fromJson(item))
        .toList();
  }
  
  /// Get kitchen display orders
  Future<List<KitchenOrder>> getKitchenOrders({
    required String tenantId,
    required String locationId,
  }) async {
    final response = await _client.get(
      '${ApiClient.restaurantService}/kitchen/orders',
      queryParameters: {
        'tenant_id': tenantId,
        'location_id': locationId,
      },
    );
    
    return (response.data as List)
        .map((item) => KitchenOrder.fromJson(item))
        .toList();
  }
  
  /// Get table status distribution
  Future<TableStatusSummary> getTableStatus({
    required String tenantId,
    required String locationId,
  }) async {
    final response = await _client.get(
      '${ApiClient.restaurantService}/tables/status',
      queryParameters: {
        'tenant_id': tenantId,
        'location_id': locationId,
      },
    );
    
    return TableStatusSummary.fromJson(response.data);
  }
  
  /// Update order status (kitchen operations)
  Future<void> updateOrderStatus({
    required String orderId,
    required String status,
    String? notes,
  }) async {
    await _client.put(
      '${ApiClient.restaurantService}/kitchen/orders/$orderId',
      data: {
        'status': status,
        if (notes != null) 'notes': notes,
      },
    );
  }
  
  /// Update table status
  Future<void> updateTableStatus({
    required String tableId,
    required String status,
    String? notes,
  }) async {
    await _client.put(
      '${ApiClient.restaurantService}/tables/$tableId',
      data: {
        'status': status,
        if (notes != null) 'notes': notes,
      },
    );
  }
  
  /// Get reservation metrics
  Future<ReservationMetrics> getReservationMetrics({
    required String tenantId,
    String? locationId,
    DateTime? date,
  }) async {
    final response = await _client.get(
      '${ApiClient.restaurantService}/reservations/metrics',
      queryParameters: {
        'tenant_id': tenantId,
        if (locationId != null) 'location_id': locationId,
        if (date != null) 'date': date.toIso8601String(),
      },
    );
    
    return ReservationMetrics.fromJson(response.data);
  }
}

/// Restaurant analytics data model
class RestaurantAnalytics {
  final String tenantId;
  final String? locationId;
  final DateTime date;
  
  // Table metrics
  final double tableTransferRate;
  final double averageDiningDuration;
  final double tableUtilizationRate;
  
  // Service metrics
  final double averageWaitTime;
  final double averagePrepTime;
  final double orderAccuracyRate;
  
  // Revenue metrics
  final double revenuePerTable;
  final double revenuePerSeat;
  final double averageCheckSize;
  
  // Popular items and peak hours
  final List<Map<String, dynamic>> topMenuItems;
  final List<Map<String, dynamic>> peakHours;
  
  // Efficiency metrics
  final double kitchenEfficiency;
  final double serverEfficiency;
  
  RestaurantAnalytics({
    required this.tenantId,
    this.locationId,
    required this.date,
    required this.tableTransferRate,
    required this.averageDiningDuration,
    required this.tableUtilizationRate,
    required this.averageWaitTime,
    required this.averagePrepTime,
    required this.orderAccuracyRate,
    required this.revenuePerTable,
    required this.revenuePerSeat,
    required this.averageCheckSize,
    required this.topMenuItems,
    required this.peakHours,
    required this.kitchenEfficiency,
    required this.serverEfficiency,
  });
  
  factory RestaurantAnalytics.fromJson(Map<String, dynamic> json) {
    return RestaurantAnalytics(
      tenantId: json['tenant_id'],
      locationId: json['location_id'],
      date: DateTime.parse(json['date']),
      tableTransferRate: (json['table_turnover_rate'] as num).toDouble(),
      averageDiningDuration: (json['average_dining_duration'] as num).toDouble(),
      tableUtilizationRate: (json['table_utilization_rate'] as num).toDouble(),
      averageWaitTime: (json['average_wait_time'] as num).toDouble(),
      averagePrepTime: (json['average_prep_time'] as num).toDouble(),
      orderAccuracyRate: (json['order_accuracy_rate'] as num).toDouble(),
      revenuePerTable: (json['revenue_per_table'] as num).toDouble(),
      revenuePerSeat: (json['revenue_per_seat'] as num).toDouble(),
      averageCheckSize: (json['average_check_size'] as num).toDouble(),
      topMenuItems: List<Map<String, dynamic>>.from(json['top_menu_items'] ?? []),
      peakHours: List<Map<String, dynamic>>.from(json['peak_hours'] ?? []),
      kitchenEfficiency: (json['kitchen_efficiency'] as num).toDouble(),
      serverEfficiency: (json['server_efficiency'] as num).toDouble(),
    );
  }
}

/// Restaurant recommendation data model
class RestaurantRecommendation {
  final String type;
  final String title;
  final String description;
  final String impact;
  final int priority;
  final Map<String, dynamic> data;
  
  RestaurantRecommendation({
    required this.type,
    required this.title,
    required this.description,
    required this.impact,
    required this.priority,
    required this.data,
  });
  
  factory RestaurantRecommendation.fromJson(Map<String, dynamic> json) {
    return RestaurantRecommendation(
      type: json['type'],
      title: json['title'],
      description: json['description'],
      impact: json['impact'],
      priority: json['priority'],
      data: Map<String, dynamic>.from(json['data'] ?? {}),
    );
  }
}

/// Kitchen order data model
class KitchenOrder {
  final String id;
  final DateTime createdAt;
  final String status;
  final double totalAmount;
  final List<Map<String, dynamic>> items;
  final int waitTimeMinutes;
  
  KitchenOrder({
    required this.id,
    required this.createdAt,
    required this.status,
    required this.totalAmount,
    required this.items,
    required this.waitTimeMinutes,
  });
  
  factory KitchenOrder.fromJson(Map<String, dynamic> json) {
    return KitchenOrder(
      id: json['id'],
      createdAt: DateTime.parse(json['created_at']),
      status: json['status'],
      totalAmount: (json['total_amount'] as num).toDouble(),
      items: List<Map<String, dynamic>>.from(json['items'] ?? []),
      waitTimeMinutes: json['wait_time_minutes'],
    );
  }
}

/// Table status summary data model
class TableStatusSummary {
  final int available;
  final int occupied;
  final int reserved;
  final int needsCleaning;
  final List<TableInfo> tables;
  
  TableStatusSummary({
    required this.available,
    required this.occupied,
    required this.reserved,
    required this.needsCleaning,
    required this.tables,
  });
  
  factory TableStatusSummary.fromJson(Map<String, dynamic> json) {
    return TableStatusSummary(
      available: json['available'] ?? 0,
      occupied: json['occupied'] ?? 0,
      reserved: json['reserved'] ?? 0,
      needsCleaning: json['needs_cleaning'] ?? 0,
      tables: (json['tables'] as List?)
          ?.map((item) => TableInfo.fromJson(item))
          .toList() ?? [],
    );
  }
}

/// Individual table information
class TableInfo {
  final String id;
  final String number;
  final String status;
  final int seats;
  final String? customerName;
  final DateTime? reservedAt;
  final DateTime? seatedAt;
  
  TableInfo({
    required this.id,
    required this.number,
    required this.status,
    required this.seats,
    this.customerName,
    this.reservedAt,
    this.seatedAt,
  });
  
  factory TableInfo.fromJson(Map<String, dynamic> json) {
    return TableInfo(
      id: json['id'],
      number: json['number'],
      status: json['status'],
      seats: json['seats'],
      customerName: json['customer_name'],
      reservedAt: json['reserved_at'] != null ? DateTime.parse(json['reserved_at']) : null,
      seatedAt: json['seated_at'] != null ? DateTime.parse(json['seated_at']) : null,
    );
  }
}

/// Reservation metrics data model
class ReservationMetrics {
  final int totalReservations;
  final int confirmedReservations;
  final int cancelledReservations;
  final int noShows;
  final double averagePartySize;
  final List<Map<String, dynamic>> hourlyDistribution;
  
  ReservationMetrics({
    required this.totalReservations,
    required this.confirmedReservations,
    required this.cancelledReservations,
    required this.noShows,
    required this.averagePartySize,
    required this.hourlyDistribution,
  });
  
  factory ReservationMetrics.fromJson(Map<String, dynamic> json) {
    return ReservationMetrics(
      totalReservations: json['total_reservations'] ?? 0,
      confirmedReservations: json['confirmed_reservations'] ?? 0,
      cancelledReservations: json['cancelled_reservations'] ?? 0,
      noShows: json['no_shows'] ?? 0,
      averagePartySize: (json['average_party_size'] as num?)?.toDouble() ?? 0.0,
      hourlyDistribution: List<Map<String, dynamic>>.from(json['hourly_distribution'] ?? []),
    );
  }
}

/// Provider for RestaurantApiService
final restaurantApiServiceProvider = Provider<RestaurantApiService>((ref) {
  final client = ref.watch(apiClientProvider);
  return RestaurantApiService(client);
});