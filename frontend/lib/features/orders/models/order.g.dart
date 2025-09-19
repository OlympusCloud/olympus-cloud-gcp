// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'order.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$OrderItemImpl _$$OrderItemImplFromJson(Map<String, dynamic> json) =>
    _$OrderItemImpl(
      id: json['id'] as String,
      productId: json['productId'] as String,
      name: json['name'] as String,
      quantity: (json['quantity'] as num).toInt(),
      unitPrice: (json['unitPrice'] as num).toDouble(),
      totalPrice: (json['totalPrice'] as num).toDouble(),
      notes: json['notes'] as String?,
      customizations: json['customizations'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$OrderItemImplToJson(_$OrderItemImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'productId': instance.productId,
      'name': instance.name,
      'quantity': instance.quantity,
      'unitPrice': instance.unitPrice,
      'totalPrice': instance.totalPrice,
      'notes': instance.notes,
      'customizations': instance.customizations,
    };

_$OrderCustomerImpl _$$OrderCustomerImplFromJson(Map<String, dynamic> json) =>
    _$OrderCustomerImpl(
      id: json['id'] as String?,
      name: json['name'] as String,
      email: json['email'] as String?,
      phone: json['phone'] as String?,
      address: json['address'] as String?,
      metadata: json['metadata'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$OrderCustomerImplToJson(_$OrderCustomerImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'email': instance.email,
      'phone': instance.phone,
      'address': instance.address,
      'metadata': instance.metadata,
    };

_$OrderPaymentImpl _$$OrderPaymentImplFromJson(Map<String, dynamic> json) =>
    _$OrderPaymentImpl(
      id: json['id'] as String,
      status: $enumDecode(_$PaymentStatusEnumMap, json['status']),
      method: json['method'] as String,
      amount: (json['amount'] as num).toDouble(),
      paidAmount: (json['paidAmount'] as num?)?.toDouble(),
      transactionId: json['transactionId'] as String?,
      gatewayResponse: json['gatewayResponse'] as String?,
      paidAt: json['paidAt'] == null
          ? null
          : DateTime.parse(json['paidAt'] as String),
    );

Map<String, dynamic> _$$OrderPaymentImplToJson(_$OrderPaymentImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'status': _$PaymentStatusEnumMap[instance.status]!,
      'method': instance.method,
      'amount': instance.amount,
      'paidAmount': instance.paidAmount,
      'transactionId': instance.transactionId,
      'gatewayResponse': instance.gatewayResponse,
      'paidAt': instance.paidAt?.toIso8601String(),
    };

const _$PaymentStatusEnumMap = {
  PaymentStatus.pending: 'pending',
  PaymentStatus.paid: 'paid',
  PaymentStatus.partiallyPaid: 'partially_paid',
  PaymentStatus.refunded: 'refunded',
  PaymentStatus.failed: 'failed',
};

_$OrderImpl _$$OrderImplFromJson(Map<String, dynamic> json) => _$OrderImpl(
      id: json['id'] as String,
      orderNumber: json['orderNumber'] as String,
      status: $enumDecode(_$OrderStatusEnumMap, json['status']),
      priority: $enumDecode(_$OrderPriorityEnumMap, json['priority']),
      items: (json['items'] as List<dynamic>)
          .map((e) => OrderItem.fromJson(e as Map<String, dynamic>))
          .toList(),
      subtotal: (json['subtotal'] as num).toDouble(),
      tax: (json['tax'] as num).toDouble(),
      total: (json['total'] as num).toDouble(),
      customer: json['customer'] == null
          ? null
          : OrderCustomer.fromJson(json['customer'] as Map<String, dynamic>),
      payment: json['payment'] == null
          ? null
          : OrderPayment.fromJson(json['payment'] as Map<String, dynamic>),
      notes: json['notes'] as String?,
      tableNumber: json['tableNumber'] as String?,
      locationId: json['locationId'] as String?,
      staffId: json['staffId'] as String?,
      estimatedCompletionTime: json['estimatedCompletionTime'] == null
          ? null
          : DateTime.parse(json['estimatedCompletionTime'] as String),
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: json['updatedAt'] == null
          ? null
          : DateTime.parse(json['updatedAt'] as String),
      metadata: json['metadata'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$OrderImplToJson(_$OrderImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'orderNumber': instance.orderNumber,
      'status': _$OrderStatusEnumMap[instance.status]!,
      'priority': _$OrderPriorityEnumMap[instance.priority]!,
      'items': instance.items,
      'subtotal': instance.subtotal,
      'tax': instance.tax,
      'total': instance.total,
      'customer': instance.customer,
      'payment': instance.payment,
      'notes': instance.notes,
      'tableNumber': instance.tableNumber,
      'locationId': instance.locationId,
      'staffId': instance.staffId,
      'estimatedCompletionTime':
          instance.estimatedCompletionTime?.toIso8601String(),
      'createdAt': instance.createdAt.toIso8601String(),
      'updatedAt': instance.updatedAt?.toIso8601String(),
      'metadata': instance.metadata,
    };

const _$OrderStatusEnumMap = {
  OrderStatus.pending: 'pending',
  OrderStatus.confirmed: 'confirmed',
  OrderStatus.preparing: 'preparing',
  OrderStatus.ready: 'ready',
  OrderStatus.completed: 'completed',
  OrderStatus.cancelled: 'cancelled',
};

const _$OrderPriorityEnumMap = {
  OrderPriority.low: 'low',
  OrderPriority.normal: 'normal',
  OrderPriority.high: 'high',
  OrderPriority.urgent: 'urgent',
};

_$CreateOrderRequestImpl _$$CreateOrderRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$CreateOrderRequestImpl(
      items: (json['items'] as List<dynamic>)
          .map((e) => OrderItem.fromJson(e as Map<String, dynamic>))
          .toList(),
      customer: json['customer'] == null
          ? null
          : OrderCustomer.fromJson(json['customer'] as Map<String, dynamic>),
      notes: json['notes'] as String?,
      tableNumber: json['tableNumber'] as String?,
      priority: $enumDecodeNullable(_$OrderPriorityEnumMap, json['priority']),
      metadata: json['metadata'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$CreateOrderRequestImplToJson(
        _$CreateOrderRequestImpl instance) =>
    <String, dynamic>{
      'items': instance.items,
      'customer': instance.customer,
      'notes': instance.notes,
      'tableNumber': instance.tableNumber,
      'priority': _$OrderPriorityEnumMap[instance.priority],
      'metadata': instance.metadata,
    };

_$UpdateOrderRequestImpl _$$UpdateOrderRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$UpdateOrderRequestImpl(
      status: $enumDecodeNullable(_$OrderStatusEnumMap, json['status']),
      priority: $enumDecodeNullable(_$OrderPriorityEnumMap, json['priority']),
      items: (json['items'] as List<dynamic>?)
          ?.map((e) => OrderItem.fromJson(e as Map<String, dynamic>))
          .toList(),
      customer: json['customer'] == null
          ? null
          : OrderCustomer.fromJson(json['customer'] as Map<String, dynamic>),
      notes: json['notes'] as String?,
      tableNumber: json['tableNumber'] as String?,
      estimatedCompletionTime: json['estimatedCompletionTime'] == null
          ? null
          : DateTime.parse(json['estimatedCompletionTime'] as String),
      metadata: json['metadata'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$UpdateOrderRequestImplToJson(
        _$UpdateOrderRequestImpl instance) =>
    <String, dynamic>{
      'status': _$OrderStatusEnumMap[instance.status],
      'priority': _$OrderPriorityEnumMap[instance.priority],
      'items': instance.items,
      'customer': instance.customer,
      'notes': instance.notes,
      'tableNumber': instance.tableNumber,
      'estimatedCompletionTime':
          instance.estimatedCompletionTime?.toIso8601String(),
      'metadata': instance.metadata,
    };
