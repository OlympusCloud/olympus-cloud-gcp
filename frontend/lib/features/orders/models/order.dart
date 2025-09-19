import 'package:freezed_annotation/freezed_annotation.dart';

part 'order.freezed.dart';
part 'order.g.dart';

/// Order status enumeration
enum OrderStatus {
  @JsonValue('pending')
  pending,
  @JsonValue('confirmed')
  confirmed,
  @JsonValue('preparing')
  preparing,
  @JsonValue('ready')
  ready,
  @JsonValue('completed')
  completed,
  @JsonValue('cancelled')
  cancelled,
}

/// Order priority enumeration
enum OrderPriority {
  @JsonValue('low')
  low,
  @JsonValue('normal')
  normal,
  @JsonValue('high')
  high,
  @JsonValue('urgent')
  urgent,
}

/// Order payment status enumeration
enum PaymentStatus {
  @JsonValue('pending')
  pending,
  @JsonValue('paid')
  paid,
  @JsonValue('partially_paid')
  partiallyPaid,
  @JsonValue('refunded')
  refunded,
  @JsonValue('failed')
  failed,
}

/// Order item model
@freezed
class OrderItem with _$OrderItem {
  const factory OrderItem({
    required String id,
    required String productId,
    required String name,
    required int quantity,
    required double unitPrice,
    required double totalPrice,
    String? notes,
    Map<String, dynamic>? customizations,
  }) = _OrderItem;

  factory OrderItem.fromJson(Map<String, dynamic> json) => _$OrderItemFromJson(json);
}

/// Customer information for order
@freezed
class OrderCustomer with _$OrderCustomer {
  const factory OrderCustomer({
    String? id,
    required String name,
    String? email,
    String? phone,
    String? address,
    Map<String, dynamic>? metadata,
  }) = _OrderCustomer;

  factory OrderCustomer.fromJson(Map<String, dynamic> json) => _$OrderCustomerFromJson(json);
}

/// Order payment information
@freezed
class OrderPayment with _$OrderPayment {
  const factory OrderPayment({
    required String id,
    required PaymentStatus status,
    required String method,
    required double amount,
    double? paidAmount,
    String? transactionId,
    String? gatewayResponse,
    DateTime? paidAt,
  }) = _OrderPayment;

  factory OrderPayment.fromJson(Map<String, dynamic> json) => _$OrderPaymentFromJson(json);
}

/// Main Order model
@freezed
class Order with _$Order {
  const factory Order({
    required String id,
    required String orderNumber,
    required OrderStatus status,
    required OrderPriority priority,
    required List<OrderItem> items,
    required double subtotal,
    required double tax,
    required double total,
    OrderCustomer? customer,
    OrderPayment? payment,
    String? notes,
    String? tableNumber,
    String? locationId,
    String? staffId,
    DateTime? estimatedCompletionTime,
    required DateTime createdAt,
    DateTime? updatedAt,
    Map<String, dynamic>? metadata,
  }) = _Order;

  factory Order.fromJson(Map<String, dynamic> json) => _$OrderFromJson(json);
}

/// Order creation request
@freezed
class CreateOrderRequest with _$CreateOrderRequest {
  const factory CreateOrderRequest({
    required List<OrderItem> items,
    OrderCustomer? customer,
    String? notes,
    String? tableNumber,
    OrderPriority? priority,
    Map<String, dynamic>? metadata,
  }) = _CreateOrderRequest;

  factory CreateOrderRequest.fromJson(Map<String, dynamic> json) => _$CreateOrderRequestFromJson(json);
}

/// Order update request
@freezed
class UpdateOrderRequest with _$UpdateOrderRequest {
  const factory UpdateOrderRequest({
    OrderStatus? status,
    OrderPriority? priority,
    List<OrderItem>? items,
    OrderCustomer? customer,
    String? notes,
    String? tableNumber,
    DateTime? estimatedCompletionTime,
    Map<String, dynamic>? metadata,
  }) = _UpdateOrderRequest;

  factory UpdateOrderRequest.fromJson(Map<String, dynamic> json) => _$UpdateOrderRequestFromJson(json);
}