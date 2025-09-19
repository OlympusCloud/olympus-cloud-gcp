// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'order.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

OrderItem _$OrderItemFromJson(Map<String, dynamic> json) {
  return _OrderItem.fromJson(json);
}

/// @nodoc
mixin _$OrderItem {
  String get id => throw _privateConstructorUsedError;
  String get productId => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  int get quantity => throw _privateConstructorUsedError;
  double get unitPrice => throw _privateConstructorUsedError;
  double get totalPrice => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  Map<String, dynamic>? get customizations =>
      throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $OrderItemCopyWith<OrderItem> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrderItemCopyWith<$Res> {
  factory $OrderItemCopyWith(OrderItem value, $Res Function(OrderItem) then) =
      _$OrderItemCopyWithImpl<$Res, OrderItem>;
  @useResult
  $Res call(
      {String id,
      String productId,
      String name,
      int quantity,
      double unitPrice,
      double totalPrice,
      String? notes,
      Map<String, dynamic>? customizations});
}

/// @nodoc
class _$OrderItemCopyWithImpl<$Res, $Val extends OrderItem>
    implements $OrderItemCopyWith<$Res> {
  _$OrderItemCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? productId = null,
    Object? name = null,
    Object? quantity = null,
    Object? unitPrice = null,
    Object? totalPrice = null,
    Object? notes = freezed,
    Object? customizations = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      productId: null == productId
          ? _value.productId
          : productId // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      quantity: null == quantity
          ? _value.quantity
          : quantity // ignore: cast_nullable_to_non_nullable
              as int,
      unitPrice: null == unitPrice
          ? _value.unitPrice
          : unitPrice // ignore: cast_nullable_to_non_nullable
              as double,
      totalPrice: null == totalPrice
          ? _value.totalPrice
          : totalPrice // ignore: cast_nullable_to_non_nullable
              as double,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      customizations: freezed == customizations
          ? _value.customizations
          : customizations // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$OrderItemImplCopyWith<$Res>
    implements $OrderItemCopyWith<$Res> {
  factory _$$OrderItemImplCopyWith(
          _$OrderItemImpl value, $Res Function(_$OrderItemImpl) then) =
      __$$OrderItemImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String productId,
      String name,
      int quantity,
      double unitPrice,
      double totalPrice,
      String? notes,
      Map<String, dynamic>? customizations});
}

/// @nodoc
class __$$OrderItemImplCopyWithImpl<$Res>
    extends _$OrderItemCopyWithImpl<$Res, _$OrderItemImpl>
    implements _$$OrderItemImplCopyWith<$Res> {
  __$$OrderItemImplCopyWithImpl(
      _$OrderItemImpl _value, $Res Function(_$OrderItemImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? productId = null,
    Object? name = null,
    Object? quantity = null,
    Object? unitPrice = null,
    Object? totalPrice = null,
    Object? notes = freezed,
    Object? customizations = freezed,
  }) {
    return _then(_$OrderItemImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      productId: null == productId
          ? _value.productId
          : productId // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      quantity: null == quantity
          ? _value.quantity
          : quantity // ignore: cast_nullable_to_non_nullable
              as int,
      unitPrice: null == unitPrice
          ? _value.unitPrice
          : unitPrice // ignore: cast_nullable_to_non_nullable
              as double,
      totalPrice: null == totalPrice
          ? _value.totalPrice
          : totalPrice // ignore: cast_nullable_to_non_nullable
              as double,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      customizations: freezed == customizations
          ? _value._customizations
          : customizations // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$OrderItemImpl implements _OrderItem {
  const _$OrderItemImpl(
      {required this.id,
      required this.productId,
      required this.name,
      required this.quantity,
      required this.unitPrice,
      required this.totalPrice,
      this.notes,
      final Map<String, dynamic>? customizations})
      : _customizations = customizations;

  factory _$OrderItemImpl.fromJson(Map<String, dynamic> json) =>
      _$$OrderItemImplFromJson(json);

  @override
  final String id;
  @override
  final String productId;
  @override
  final String name;
  @override
  final int quantity;
  @override
  final double unitPrice;
  @override
  final double totalPrice;
  @override
  final String? notes;
  final Map<String, dynamic>? _customizations;
  @override
  Map<String, dynamic>? get customizations {
    final value = _customizations;
    if (value == null) return null;
    if (_customizations is EqualUnmodifiableMapView) return _customizations;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'OrderItem(id: $id, productId: $productId, name: $name, quantity: $quantity, unitPrice: $unitPrice, totalPrice: $totalPrice, notes: $notes, customizations: $customizations)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderItemImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.productId, productId) ||
                other.productId == productId) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.quantity, quantity) ||
                other.quantity == quantity) &&
            (identical(other.unitPrice, unitPrice) ||
                other.unitPrice == unitPrice) &&
            (identical(other.totalPrice, totalPrice) ||
                other.totalPrice == totalPrice) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            const DeepCollectionEquality()
                .equals(other._customizations, _customizations));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      id,
      productId,
      name,
      quantity,
      unitPrice,
      totalPrice,
      notes,
      const DeepCollectionEquality().hash(_customizations));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderItemImplCopyWith<_$OrderItemImpl> get copyWith =>
      __$$OrderItemImplCopyWithImpl<_$OrderItemImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$OrderItemImplToJson(
      this,
    );
  }
}

abstract class _OrderItem implements OrderItem {
  const factory _OrderItem(
      {required final String id,
      required final String productId,
      required final String name,
      required final int quantity,
      required final double unitPrice,
      required final double totalPrice,
      final String? notes,
      final Map<String, dynamic>? customizations}) = _$OrderItemImpl;

  factory _OrderItem.fromJson(Map<String, dynamic> json) =
      _$OrderItemImpl.fromJson;

  @override
  String get id;
  @override
  String get productId;
  @override
  String get name;
  @override
  int get quantity;
  @override
  double get unitPrice;
  @override
  double get totalPrice;
  @override
  String? get notes;
  @override
  Map<String, dynamic>? get customizations;
  @override
  @JsonKey(ignore: true)
  _$$OrderItemImplCopyWith<_$OrderItemImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

OrderCustomer _$OrderCustomerFromJson(Map<String, dynamic> json) {
  return _OrderCustomer.fromJson(json);
}

/// @nodoc
mixin _$OrderCustomer {
  String? get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String? get email => throw _privateConstructorUsedError;
  String? get phone => throw _privateConstructorUsedError;
  String? get address => throw _privateConstructorUsedError;
  Map<String, dynamic>? get metadata => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $OrderCustomerCopyWith<OrderCustomer> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrderCustomerCopyWith<$Res> {
  factory $OrderCustomerCopyWith(
          OrderCustomer value, $Res Function(OrderCustomer) then) =
      _$OrderCustomerCopyWithImpl<$Res, OrderCustomer>;
  @useResult
  $Res call(
      {String? id,
      String name,
      String? email,
      String? phone,
      String? address,
      Map<String, dynamic>? metadata});
}

/// @nodoc
class _$OrderCustomerCopyWithImpl<$Res, $Val extends OrderCustomer>
    implements $OrderCustomerCopyWith<$Res> {
  _$OrderCustomerCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = freezed,
    Object? name = null,
    Object? email = freezed,
    Object? phone = freezed,
    Object? address = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_value.copyWith(
      id: freezed == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String?,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      email: freezed == email
          ? _value.email
          : email // ignore: cast_nullable_to_non_nullable
              as String?,
      phone: freezed == phone
          ? _value.phone
          : phone // ignore: cast_nullable_to_non_nullable
              as String?,
      address: freezed == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String?,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$OrderCustomerImplCopyWith<$Res>
    implements $OrderCustomerCopyWith<$Res> {
  factory _$$OrderCustomerImplCopyWith(
          _$OrderCustomerImpl value, $Res Function(_$OrderCustomerImpl) then) =
      __$$OrderCustomerImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String? id,
      String name,
      String? email,
      String? phone,
      String? address,
      Map<String, dynamic>? metadata});
}

/// @nodoc
class __$$OrderCustomerImplCopyWithImpl<$Res>
    extends _$OrderCustomerCopyWithImpl<$Res, _$OrderCustomerImpl>
    implements _$$OrderCustomerImplCopyWith<$Res> {
  __$$OrderCustomerImplCopyWithImpl(
      _$OrderCustomerImpl _value, $Res Function(_$OrderCustomerImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = freezed,
    Object? name = null,
    Object? email = freezed,
    Object? phone = freezed,
    Object? address = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_$OrderCustomerImpl(
      id: freezed == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String?,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      email: freezed == email
          ? _value.email
          : email // ignore: cast_nullable_to_non_nullable
              as String?,
      phone: freezed == phone
          ? _value.phone
          : phone // ignore: cast_nullable_to_non_nullable
              as String?,
      address: freezed == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String?,
      metadata: freezed == metadata
          ? _value._metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$OrderCustomerImpl implements _OrderCustomer {
  const _$OrderCustomerImpl(
      {this.id,
      required this.name,
      this.email,
      this.phone,
      this.address,
      final Map<String, dynamic>? metadata})
      : _metadata = metadata;

  factory _$OrderCustomerImpl.fromJson(Map<String, dynamic> json) =>
      _$$OrderCustomerImplFromJson(json);

  @override
  final String? id;
  @override
  final String name;
  @override
  final String? email;
  @override
  final String? phone;
  @override
  final String? address;
  final Map<String, dynamic>? _metadata;
  @override
  Map<String, dynamic>? get metadata {
    final value = _metadata;
    if (value == null) return null;
    if (_metadata is EqualUnmodifiableMapView) return _metadata;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'OrderCustomer(id: $id, name: $name, email: $email, phone: $phone, address: $address, metadata: $metadata)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderCustomerImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.email, email) || other.email == email) &&
            (identical(other.phone, phone) || other.phone == phone) &&
            (identical(other.address, address) || other.address == address) &&
            const DeepCollectionEquality().equals(other._metadata, _metadata));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, id, name, email, phone, address,
      const DeepCollectionEquality().hash(_metadata));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderCustomerImplCopyWith<_$OrderCustomerImpl> get copyWith =>
      __$$OrderCustomerImplCopyWithImpl<_$OrderCustomerImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$OrderCustomerImplToJson(
      this,
    );
  }
}

abstract class _OrderCustomer implements OrderCustomer {
  const factory _OrderCustomer(
      {final String? id,
      required final String name,
      final String? email,
      final String? phone,
      final String? address,
      final Map<String, dynamic>? metadata}) = _$OrderCustomerImpl;

  factory _OrderCustomer.fromJson(Map<String, dynamic> json) =
      _$OrderCustomerImpl.fromJson;

  @override
  String? get id;
  @override
  String get name;
  @override
  String? get email;
  @override
  String? get phone;
  @override
  String? get address;
  @override
  Map<String, dynamic>? get metadata;
  @override
  @JsonKey(ignore: true)
  _$$OrderCustomerImplCopyWith<_$OrderCustomerImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

OrderPayment _$OrderPaymentFromJson(Map<String, dynamic> json) {
  return _OrderPayment.fromJson(json);
}

/// @nodoc
mixin _$OrderPayment {
  String get id => throw _privateConstructorUsedError;
  PaymentStatus get status => throw _privateConstructorUsedError;
  String get method => throw _privateConstructorUsedError;
  double get amount => throw _privateConstructorUsedError;
  double? get paidAmount => throw _privateConstructorUsedError;
  String? get transactionId => throw _privateConstructorUsedError;
  String? get gatewayResponse => throw _privateConstructorUsedError;
  DateTime? get paidAt => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $OrderPaymentCopyWith<OrderPayment> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrderPaymentCopyWith<$Res> {
  factory $OrderPaymentCopyWith(
          OrderPayment value, $Res Function(OrderPayment) then) =
      _$OrderPaymentCopyWithImpl<$Res, OrderPayment>;
  @useResult
  $Res call(
      {String id,
      PaymentStatus status,
      String method,
      double amount,
      double? paidAmount,
      String? transactionId,
      String? gatewayResponse,
      DateTime? paidAt});
}

/// @nodoc
class _$OrderPaymentCopyWithImpl<$Res, $Val extends OrderPayment>
    implements $OrderPaymentCopyWith<$Res> {
  _$OrderPaymentCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? status = null,
    Object? method = null,
    Object? amount = null,
    Object? paidAmount = freezed,
    Object? transactionId = freezed,
    Object? gatewayResponse = freezed,
    Object? paidAt = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as PaymentStatus,
      method: null == method
          ? _value.method
          : method // ignore: cast_nullable_to_non_nullable
              as String,
      amount: null == amount
          ? _value.amount
          : amount // ignore: cast_nullable_to_non_nullable
              as double,
      paidAmount: freezed == paidAmount
          ? _value.paidAmount
          : paidAmount // ignore: cast_nullable_to_non_nullable
              as double?,
      transactionId: freezed == transactionId
          ? _value.transactionId
          : transactionId // ignore: cast_nullable_to_non_nullable
              as String?,
      gatewayResponse: freezed == gatewayResponse
          ? _value.gatewayResponse
          : gatewayResponse // ignore: cast_nullable_to_non_nullable
              as String?,
      paidAt: freezed == paidAt
          ? _value.paidAt
          : paidAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$OrderPaymentImplCopyWith<$Res>
    implements $OrderPaymentCopyWith<$Res> {
  factory _$$OrderPaymentImplCopyWith(
          _$OrderPaymentImpl value, $Res Function(_$OrderPaymentImpl) then) =
      __$$OrderPaymentImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      PaymentStatus status,
      String method,
      double amount,
      double? paidAmount,
      String? transactionId,
      String? gatewayResponse,
      DateTime? paidAt});
}

/// @nodoc
class __$$OrderPaymentImplCopyWithImpl<$Res>
    extends _$OrderPaymentCopyWithImpl<$Res, _$OrderPaymentImpl>
    implements _$$OrderPaymentImplCopyWith<$Res> {
  __$$OrderPaymentImplCopyWithImpl(
      _$OrderPaymentImpl _value, $Res Function(_$OrderPaymentImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? status = null,
    Object? method = null,
    Object? amount = null,
    Object? paidAmount = freezed,
    Object? transactionId = freezed,
    Object? gatewayResponse = freezed,
    Object? paidAt = freezed,
  }) {
    return _then(_$OrderPaymentImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as PaymentStatus,
      method: null == method
          ? _value.method
          : method // ignore: cast_nullable_to_non_nullable
              as String,
      amount: null == amount
          ? _value.amount
          : amount // ignore: cast_nullable_to_non_nullable
              as double,
      paidAmount: freezed == paidAmount
          ? _value.paidAmount
          : paidAmount // ignore: cast_nullable_to_non_nullable
              as double?,
      transactionId: freezed == transactionId
          ? _value.transactionId
          : transactionId // ignore: cast_nullable_to_non_nullable
              as String?,
      gatewayResponse: freezed == gatewayResponse
          ? _value.gatewayResponse
          : gatewayResponse // ignore: cast_nullable_to_non_nullable
              as String?,
      paidAt: freezed == paidAt
          ? _value.paidAt
          : paidAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$OrderPaymentImpl implements _OrderPayment {
  const _$OrderPaymentImpl(
      {required this.id,
      required this.status,
      required this.method,
      required this.amount,
      this.paidAmount,
      this.transactionId,
      this.gatewayResponse,
      this.paidAt});

  factory _$OrderPaymentImpl.fromJson(Map<String, dynamic> json) =>
      _$$OrderPaymentImplFromJson(json);

  @override
  final String id;
  @override
  final PaymentStatus status;
  @override
  final String method;
  @override
  final double amount;
  @override
  final double? paidAmount;
  @override
  final String? transactionId;
  @override
  final String? gatewayResponse;
  @override
  final DateTime? paidAt;

  @override
  String toString() {
    return 'OrderPayment(id: $id, status: $status, method: $method, amount: $amount, paidAmount: $paidAmount, transactionId: $transactionId, gatewayResponse: $gatewayResponse, paidAt: $paidAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderPaymentImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.method, method) || other.method == method) &&
            (identical(other.amount, amount) || other.amount == amount) &&
            (identical(other.paidAmount, paidAmount) ||
                other.paidAmount == paidAmount) &&
            (identical(other.transactionId, transactionId) ||
                other.transactionId == transactionId) &&
            (identical(other.gatewayResponse, gatewayResponse) ||
                other.gatewayResponse == gatewayResponse) &&
            (identical(other.paidAt, paidAt) || other.paidAt == paidAt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, id, status, method, amount,
      paidAmount, transactionId, gatewayResponse, paidAt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderPaymentImplCopyWith<_$OrderPaymentImpl> get copyWith =>
      __$$OrderPaymentImplCopyWithImpl<_$OrderPaymentImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$OrderPaymentImplToJson(
      this,
    );
  }
}

abstract class _OrderPayment implements OrderPayment {
  const factory _OrderPayment(
      {required final String id,
      required final PaymentStatus status,
      required final String method,
      required final double amount,
      final double? paidAmount,
      final String? transactionId,
      final String? gatewayResponse,
      final DateTime? paidAt}) = _$OrderPaymentImpl;

  factory _OrderPayment.fromJson(Map<String, dynamic> json) =
      _$OrderPaymentImpl.fromJson;

  @override
  String get id;
  @override
  PaymentStatus get status;
  @override
  String get method;
  @override
  double get amount;
  @override
  double? get paidAmount;
  @override
  String? get transactionId;
  @override
  String? get gatewayResponse;
  @override
  DateTime? get paidAt;
  @override
  @JsonKey(ignore: true)
  _$$OrderPaymentImplCopyWith<_$OrderPaymentImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

Order _$OrderFromJson(Map<String, dynamic> json) {
  return _Order.fromJson(json);
}

/// @nodoc
mixin _$Order {
  String get id => throw _privateConstructorUsedError;
  String get orderNumber => throw _privateConstructorUsedError;
  OrderStatus get status => throw _privateConstructorUsedError;
  OrderPriority get priority => throw _privateConstructorUsedError;
  List<OrderItem> get items => throw _privateConstructorUsedError;
  double get subtotal => throw _privateConstructorUsedError;
  double get tax => throw _privateConstructorUsedError;
  double get total => throw _privateConstructorUsedError;
  OrderCustomer? get customer => throw _privateConstructorUsedError;
  OrderPayment? get payment => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  String? get tableNumber => throw _privateConstructorUsedError;
  String? get locationId => throw _privateConstructorUsedError;
  String? get staffId => throw _privateConstructorUsedError;
  DateTime? get estimatedCompletionTime => throw _privateConstructorUsedError;
  DateTime get createdAt => throw _privateConstructorUsedError;
  DateTime? get updatedAt => throw _privateConstructorUsedError;
  Map<String, dynamic>? get metadata => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $OrderCopyWith<Order> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrderCopyWith<$Res> {
  factory $OrderCopyWith(Order value, $Res Function(Order) then) =
      _$OrderCopyWithImpl<$Res, Order>;
  @useResult
  $Res call(
      {String id,
      String orderNumber,
      OrderStatus status,
      OrderPriority priority,
      List<OrderItem> items,
      double subtotal,
      double tax,
      double total,
      OrderCustomer? customer,
      OrderPayment? payment,
      String? notes,
      String? tableNumber,
      String? locationId,
      String? staffId,
      DateTime? estimatedCompletionTime,
      DateTime createdAt,
      DateTime? updatedAt,
      Map<String, dynamic>? metadata});

  $OrderCustomerCopyWith<$Res>? get customer;
  $OrderPaymentCopyWith<$Res>? get payment;
}

/// @nodoc
class _$OrderCopyWithImpl<$Res, $Val extends Order>
    implements $OrderCopyWith<$Res> {
  _$OrderCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? orderNumber = null,
    Object? status = null,
    Object? priority = null,
    Object? items = null,
    Object? subtotal = null,
    Object? tax = null,
    Object? total = null,
    Object? customer = freezed,
    Object? payment = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? locationId = freezed,
    Object? staffId = freezed,
    Object? estimatedCompletionTime = freezed,
    Object? createdAt = null,
    Object? updatedAt = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      orderNumber: null == orderNumber
          ? _value.orderNumber
          : orderNumber // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as OrderStatus,
      priority: null == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority,
      items: null == items
          ? _value.items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>,
      subtotal: null == subtotal
          ? _value.subtotal
          : subtotal // ignore: cast_nullable_to_non_nullable
              as double,
      tax: null == tax
          ? _value.tax
          : tax // ignore: cast_nullable_to_non_nullable
              as double,
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as double,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      payment: freezed == payment
          ? _value.payment
          : payment // ignore: cast_nullable_to_non_nullable
              as OrderPayment?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      locationId: freezed == locationId
          ? _value.locationId
          : locationId // ignore: cast_nullable_to_non_nullable
              as String?,
      staffId: freezed == staffId
          ? _value.staffId
          : staffId // ignore: cast_nullable_to_non_nullable
              as String?,
      estimatedCompletionTime: freezed == estimatedCompletionTime
          ? _value.estimatedCompletionTime
          : estimatedCompletionTime // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as DateTime,
      updatedAt: freezed == updatedAt
          ? _value.updatedAt
          : updatedAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OrderCustomerCopyWith<$Res>? get customer {
    if (_value.customer == null) {
      return null;
    }

    return $OrderCustomerCopyWith<$Res>(_value.customer!, (value) {
      return _then(_value.copyWith(customer: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $OrderPaymentCopyWith<$Res>? get payment {
    if (_value.payment == null) {
      return null;
    }

    return $OrderPaymentCopyWith<$Res>(_value.payment!, (value) {
      return _then(_value.copyWith(payment: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$OrderImplCopyWith<$Res> implements $OrderCopyWith<$Res> {
  factory _$$OrderImplCopyWith(
          _$OrderImpl value, $Res Function(_$OrderImpl) then) =
      __$$OrderImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String orderNumber,
      OrderStatus status,
      OrderPriority priority,
      List<OrderItem> items,
      double subtotal,
      double tax,
      double total,
      OrderCustomer? customer,
      OrderPayment? payment,
      String? notes,
      String? tableNumber,
      String? locationId,
      String? staffId,
      DateTime? estimatedCompletionTime,
      DateTime createdAt,
      DateTime? updatedAt,
      Map<String, dynamic>? metadata});

  @override
  $OrderCustomerCopyWith<$Res>? get customer;
  @override
  $OrderPaymentCopyWith<$Res>? get payment;
}

/// @nodoc
class __$$OrderImplCopyWithImpl<$Res>
    extends _$OrderCopyWithImpl<$Res, _$OrderImpl>
    implements _$$OrderImplCopyWith<$Res> {
  __$$OrderImplCopyWithImpl(
      _$OrderImpl _value, $Res Function(_$OrderImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? orderNumber = null,
    Object? status = null,
    Object? priority = null,
    Object? items = null,
    Object? subtotal = null,
    Object? tax = null,
    Object? total = null,
    Object? customer = freezed,
    Object? payment = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? locationId = freezed,
    Object? staffId = freezed,
    Object? estimatedCompletionTime = freezed,
    Object? createdAt = null,
    Object? updatedAt = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_$OrderImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      orderNumber: null == orderNumber
          ? _value.orderNumber
          : orderNumber // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as OrderStatus,
      priority: null == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority,
      items: null == items
          ? _value._items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>,
      subtotal: null == subtotal
          ? _value.subtotal
          : subtotal // ignore: cast_nullable_to_non_nullable
              as double,
      tax: null == tax
          ? _value.tax
          : tax // ignore: cast_nullable_to_non_nullable
              as double,
      total: null == total
          ? _value.total
          : total // ignore: cast_nullable_to_non_nullable
              as double,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      payment: freezed == payment
          ? _value.payment
          : payment // ignore: cast_nullable_to_non_nullable
              as OrderPayment?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      locationId: freezed == locationId
          ? _value.locationId
          : locationId // ignore: cast_nullable_to_non_nullable
              as String?,
      staffId: freezed == staffId
          ? _value.staffId
          : staffId // ignore: cast_nullable_to_non_nullable
              as String?,
      estimatedCompletionTime: freezed == estimatedCompletionTime
          ? _value.estimatedCompletionTime
          : estimatedCompletionTime // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as DateTime,
      updatedAt: freezed == updatedAt
          ? _value.updatedAt
          : updatedAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      metadata: freezed == metadata
          ? _value._metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$OrderImpl implements _Order {
  const _$OrderImpl(
      {required this.id,
      required this.orderNumber,
      required this.status,
      required this.priority,
      required final List<OrderItem> items,
      required this.subtotal,
      required this.tax,
      required this.total,
      this.customer,
      this.payment,
      this.notes,
      this.tableNumber,
      this.locationId,
      this.staffId,
      this.estimatedCompletionTime,
      required this.createdAt,
      this.updatedAt,
      final Map<String, dynamic>? metadata})
      : _items = items,
        _metadata = metadata;

  factory _$OrderImpl.fromJson(Map<String, dynamic> json) =>
      _$$OrderImplFromJson(json);

  @override
  final String id;
  @override
  final String orderNumber;
  @override
  final OrderStatus status;
  @override
  final OrderPriority priority;
  final List<OrderItem> _items;
  @override
  List<OrderItem> get items {
    if (_items is EqualUnmodifiableListView) return _items;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_items);
  }

  @override
  final double subtotal;
  @override
  final double tax;
  @override
  final double total;
  @override
  final OrderCustomer? customer;
  @override
  final OrderPayment? payment;
  @override
  final String? notes;
  @override
  final String? tableNumber;
  @override
  final String? locationId;
  @override
  final String? staffId;
  @override
  final DateTime? estimatedCompletionTime;
  @override
  final DateTime createdAt;
  @override
  final DateTime? updatedAt;
  final Map<String, dynamic>? _metadata;
  @override
  Map<String, dynamic>? get metadata {
    final value = _metadata;
    if (value == null) return null;
    if (_metadata is EqualUnmodifiableMapView) return _metadata;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'Order(id: $id, orderNumber: $orderNumber, status: $status, priority: $priority, items: $items, subtotal: $subtotal, tax: $tax, total: $total, customer: $customer, payment: $payment, notes: $notes, tableNumber: $tableNumber, locationId: $locationId, staffId: $staffId, estimatedCompletionTime: $estimatedCompletionTime, createdAt: $createdAt, updatedAt: $updatedAt, metadata: $metadata)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.orderNumber, orderNumber) ||
                other.orderNumber == orderNumber) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.priority, priority) ||
                other.priority == priority) &&
            const DeepCollectionEquality().equals(other._items, _items) &&
            (identical(other.subtotal, subtotal) ||
                other.subtotal == subtotal) &&
            (identical(other.tax, tax) || other.tax == tax) &&
            (identical(other.total, total) || other.total == total) &&
            (identical(other.customer, customer) ||
                other.customer == customer) &&
            (identical(other.payment, payment) || other.payment == payment) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            (identical(other.tableNumber, tableNumber) ||
                other.tableNumber == tableNumber) &&
            (identical(other.locationId, locationId) ||
                other.locationId == locationId) &&
            (identical(other.staffId, staffId) || other.staffId == staffId) &&
            (identical(
                    other.estimatedCompletionTime, estimatedCompletionTime) ||
                other.estimatedCompletionTime == estimatedCompletionTime) &&
            (identical(other.createdAt, createdAt) ||
                other.createdAt == createdAt) &&
            (identical(other.updatedAt, updatedAt) ||
                other.updatedAt == updatedAt) &&
            const DeepCollectionEquality().equals(other._metadata, _metadata));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      id,
      orderNumber,
      status,
      priority,
      const DeepCollectionEquality().hash(_items),
      subtotal,
      tax,
      total,
      customer,
      payment,
      notes,
      tableNumber,
      locationId,
      staffId,
      estimatedCompletionTime,
      createdAt,
      updatedAt,
      const DeepCollectionEquality().hash(_metadata));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderImplCopyWith<_$OrderImpl> get copyWith =>
      __$$OrderImplCopyWithImpl<_$OrderImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$OrderImplToJson(
      this,
    );
  }
}

abstract class _Order implements Order {
  const factory _Order(
      {required final String id,
      required final String orderNumber,
      required final OrderStatus status,
      required final OrderPriority priority,
      required final List<OrderItem> items,
      required final double subtotal,
      required final double tax,
      required final double total,
      final OrderCustomer? customer,
      final OrderPayment? payment,
      final String? notes,
      final String? tableNumber,
      final String? locationId,
      final String? staffId,
      final DateTime? estimatedCompletionTime,
      required final DateTime createdAt,
      final DateTime? updatedAt,
      final Map<String, dynamic>? metadata}) = _$OrderImpl;

  factory _Order.fromJson(Map<String, dynamic> json) = _$OrderImpl.fromJson;

  @override
  String get id;
  @override
  String get orderNumber;
  @override
  OrderStatus get status;
  @override
  OrderPriority get priority;
  @override
  List<OrderItem> get items;
  @override
  double get subtotal;
  @override
  double get tax;
  @override
  double get total;
  @override
  OrderCustomer? get customer;
  @override
  OrderPayment? get payment;
  @override
  String? get notes;
  @override
  String? get tableNumber;
  @override
  String? get locationId;
  @override
  String? get staffId;
  @override
  DateTime? get estimatedCompletionTime;
  @override
  DateTime get createdAt;
  @override
  DateTime? get updatedAt;
  @override
  Map<String, dynamic>? get metadata;
  @override
  @JsonKey(ignore: true)
  _$$OrderImplCopyWith<_$OrderImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CreateOrderRequest _$CreateOrderRequestFromJson(Map<String, dynamic> json) {
  return _CreateOrderRequest.fromJson(json);
}

/// @nodoc
mixin _$CreateOrderRequest {
  List<OrderItem> get items => throw _privateConstructorUsedError;
  OrderCustomer? get customer => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  String? get tableNumber => throw _privateConstructorUsedError;
  OrderPriority? get priority => throw _privateConstructorUsedError;
  Map<String, dynamic>? get metadata => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CreateOrderRequestCopyWith<CreateOrderRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateOrderRequestCopyWith<$Res> {
  factory $CreateOrderRequestCopyWith(
          CreateOrderRequest value, $Res Function(CreateOrderRequest) then) =
      _$CreateOrderRequestCopyWithImpl<$Res, CreateOrderRequest>;
  @useResult
  $Res call(
      {List<OrderItem> items,
      OrderCustomer? customer,
      String? notes,
      String? tableNumber,
      OrderPriority? priority,
      Map<String, dynamic>? metadata});

  $OrderCustomerCopyWith<$Res>? get customer;
}

/// @nodoc
class _$CreateOrderRequestCopyWithImpl<$Res, $Val extends CreateOrderRequest>
    implements $CreateOrderRequestCopyWith<$Res> {
  _$CreateOrderRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? items = null,
    Object? customer = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? priority = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_value.copyWith(
      items: null == items
          ? _value.items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority?,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OrderCustomerCopyWith<$Res>? get customer {
    if (_value.customer == null) {
      return null;
    }

    return $OrderCustomerCopyWith<$Res>(_value.customer!, (value) {
      return _then(_value.copyWith(customer: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$CreateOrderRequestImplCopyWith<$Res>
    implements $CreateOrderRequestCopyWith<$Res> {
  factory _$$CreateOrderRequestImplCopyWith(_$CreateOrderRequestImpl value,
          $Res Function(_$CreateOrderRequestImpl) then) =
      __$$CreateOrderRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {List<OrderItem> items,
      OrderCustomer? customer,
      String? notes,
      String? tableNumber,
      OrderPriority? priority,
      Map<String, dynamic>? metadata});

  @override
  $OrderCustomerCopyWith<$Res>? get customer;
}

/// @nodoc
class __$$CreateOrderRequestImplCopyWithImpl<$Res>
    extends _$CreateOrderRequestCopyWithImpl<$Res, _$CreateOrderRequestImpl>
    implements _$$CreateOrderRequestImplCopyWith<$Res> {
  __$$CreateOrderRequestImplCopyWithImpl(_$CreateOrderRequestImpl _value,
      $Res Function(_$CreateOrderRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? items = null,
    Object? customer = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? priority = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_$CreateOrderRequestImpl(
      items: null == items
          ? _value._items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority?,
      metadata: freezed == metadata
          ? _value._metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateOrderRequestImpl implements _CreateOrderRequest {
  const _$CreateOrderRequestImpl(
      {required final List<OrderItem> items,
      this.customer,
      this.notes,
      this.tableNumber,
      this.priority,
      final Map<String, dynamic>? metadata})
      : _items = items,
        _metadata = metadata;

  factory _$CreateOrderRequestImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateOrderRequestImplFromJson(json);

  final List<OrderItem> _items;
  @override
  List<OrderItem> get items {
    if (_items is EqualUnmodifiableListView) return _items;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_items);
  }

  @override
  final OrderCustomer? customer;
  @override
  final String? notes;
  @override
  final String? tableNumber;
  @override
  final OrderPriority? priority;
  final Map<String, dynamic>? _metadata;
  @override
  Map<String, dynamic>? get metadata {
    final value = _metadata;
    if (value == null) return null;
    if (_metadata is EqualUnmodifiableMapView) return _metadata;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'CreateOrderRequest(items: $items, customer: $customer, notes: $notes, tableNumber: $tableNumber, priority: $priority, metadata: $metadata)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateOrderRequestImpl &&
            const DeepCollectionEquality().equals(other._items, _items) &&
            (identical(other.customer, customer) ||
                other.customer == customer) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            (identical(other.tableNumber, tableNumber) ||
                other.tableNumber == tableNumber) &&
            (identical(other.priority, priority) ||
                other.priority == priority) &&
            const DeepCollectionEquality().equals(other._metadata, _metadata));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_items),
      customer,
      notes,
      tableNumber,
      priority,
      const DeepCollectionEquality().hash(_metadata));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateOrderRequestImplCopyWith<_$CreateOrderRequestImpl> get copyWith =>
      __$$CreateOrderRequestImplCopyWithImpl<_$CreateOrderRequestImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateOrderRequestImplToJson(
      this,
    );
  }
}

abstract class _CreateOrderRequest implements CreateOrderRequest {
  const factory _CreateOrderRequest(
      {required final List<OrderItem> items,
      final OrderCustomer? customer,
      final String? notes,
      final String? tableNumber,
      final OrderPriority? priority,
      final Map<String, dynamic>? metadata}) = _$CreateOrderRequestImpl;

  factory _CreateOrderRequest.fromJson(Map<String, dynamic> json) =
      _$CreateOrderRequestImpl.fromJson;

  @override
  List<OrderItem> get items;
  @override
  OrderCustomer? get customer;
  @override
  String? get notes;
  @override
  String? get tableNumber;
  @override
  OrderPriority? get priority;
  @override
  Map<String, dynamic>? get metadata;
  @override
  @JsonKey(ignore: true)
  _$$CreateOrderRequestImplCopyWith<_$CreateOrderRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

UpdateOrderRequest _$UpdateOrderRequestFromJson(Map<String, dynamic> json) {
  return _UpdateOrderRequest.fromJson(json);
}

/// @nodoc
mixin _$UpdateOrderRequest {
  OrderStatus? get status => throw _privateConstructorUsedError;
  OrderPriority? get priority => throw _privateConstructorUsedError;
  List<OrderItem>? get items => throw _privateConstructorUsedError;
  OrderCustomer? get customer => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  String? get tableNumber => throw _privateConstructorUsedError;
  DateTime? get estimatedCompletionTime => throw _privateConstructorUsedError;
  Map<String, dynamic>? get metadata => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $UpdateOrderRequestCopyWith<UpdateOrderRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateOrderRequestCopyWith<$Res> {
  factory $UpdateOrderRequestCopyWith(
          UpdateOrderRequest value, $Res Function(UpdateOrderRequest) then) =
      _$UpdateOrderRequestCopyWithImpl<$Res, UpdateOrderRequest>;
  @useResult
  $Res call(
      {OrderStatus? status,
      OrderPriority? priority,
      List<OrderItem>? items,
      OrderCustomer? customer,
      String? notes,
      String? tableNumber,
      DateTime? estimatedCompletionTime,
      Map<String, dynamic>? metadata});

  $OrderCustomerCopyWith<$Res>? get customer;
}

/// @nodoc
class _$UpdateOrderRequestCopyWithImpl<$Res, $Val extends UpdateOrderRequest>
    implements $UpdateOrderRequestCopyWith<$Res> {
  _$UpdateOrderRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? status = freezed,
    Object? priority = freezed,
    Object? items = freezed,
    Object? customer = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? estimatedCompletionTime = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_value.copyWith(
      status: freezed == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as OrderStatus?,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority?,
      items: freezed == items
          ? _value.items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>?,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      estimatedCompletionTime: freezed == estimatedCompletionTime
          ? _value.estimatedCompletionTime
          : estimatedCompletionTime // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OrderCustomerCopyWith<$Res>? get customer {
    if (_value.customer == null) {
      return null;
    }

    return $OrderCustomerCopyWith<$Res>(_value.customer!, (value) {
      return _then(_value.copyWith(customer: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$UpdateOrderRequestImplCopyWith<$Res>
    implements $UpdateOrderRequestCopyWith<$Res> {
  factory _$$UpdateOrderRequestImplCopyWith(_$UpdateOrderRequestImpl value,
          $Res Function(_$UpdateOrderRequestImpl) then) =
      __$$UpdateOrderRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {OrderStatus? status,
      OrderPriority? priority,
      List<OrderItem>? items,
      OrderCustomer? customer,
      String? notes,
      String? tableNumber,
      DateTime? estimatedCompletionTime,
      Map<String, dynamic>? metadata});

  @override
  $OrderCustomerCopyWith<$Res>? get customer;
}

/// @nodoc
class __$$UpdateOrderRequestImplCopyWithImpl<$Res>
    extends _$UpdateOrderRequestCopyWithImpl<$Res, _$UpdateOrderRequestImpl>
    implements _$$UpdateOrderRequestImplCopyWith<$Res> {
  __$$UpdateOrderRequestImplCopyWithImpl(_$UpdateOrderRequestImpl _value,
      $Res Function(_$UpdateOrderRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? status = freezed,
    Object? priority = freezed,
    Object? items = freezed,
    Object? customer = freezed,
    Object? notes = freezed,
    Object? tableNumber = freezed,
    Object? estimatedCompletionTime = freezed,
    Object? metadata = freezed,
  }) {
    return _then(_$UpdateOrderRequestImpl(
      status: freezed == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as OrderStatus?,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as OrderPriority?,
      items: freezed == items
          ? _value._items
          : items // ignore: cast_nullable_to_non_nullable
              as List<OrderItem>?,
      customer: freezed == customer
          ? _value.customer
          : customer // ignore: cast_nullable_to_non_nullable
              as OrderCustomer?,
      notes: freezed == notes
          ? _value.notes
          : notes // ignore: cast_nullable_to_non_nullable
              as String?,
      tableNumber: freezed == tableNumber
          ? _value.tableNumber
          : tableNumber // ignore: cast_nullable_to_non_nullable
              as String?,
      estimatedCompletionTime: freezed == estimatedCompletionTime
          ? _value.estimatedCompletionTime
          : estimatedCompletionTime // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      metadata: freezed == metadata
          ? _value._metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateOrderRequestImpl implements _UpdateOrderRequest {
  const _$UpdateOrderRequestImpl(
      {this.status,
      this.priority,
      final List<OrderItem>? items,
      this.customer,
      this.notes,
      this.tableNumber,
      this.estimatedCompletionTime,
      final Map<String, dynamic>? metadata})
      : _items = items,
        _metadata = metadata;

  factory _$UpdateOrderRequestImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateOrderRequestImplFromJson(json);

  @override
  final OrderStatus? status;
  @override
  final OrderPriority? priority;
  final List<OrderItem>? _items;
  @override
  List<OrderItem>? get items {
    final value = _items;
    if (value == null) return null;
    if (_items is EqualUnmodifiableListView) return _items;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final OrderCustomer? customer;
  @override
  final String? notes;
  @override
  final String? tableNumber;
  @override
  final DateTime? estimatedCompletionTime;
  final Map<String, dynamic>? _metadata;
  @override
  Map<String, dynamic>? get metadata {
    final value = _metadata;
    if (value == null) return null;
    if (_metadata is EqualUnmodifiableMapView) return _metadata;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'UpdateOrderRequest(status: $status, priority: $priority, items: $items, customer: $customer, notes: $notes, tableNumber: $tableNumber, estimatedCompletionTime: $estimatedCompletionTime, metadata: $metadata)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateOrderRequestImpl &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.priority, priority) ||
                other.priority == priority) &&
            const DeepCollectionEquality().equals(other._items, _items) &&
            (identical(other.customer, customer) ||
                other.customer == customer) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            (identical(other.tableNumber, tableNumber) ||
                other.tableNumber == tableNumber) &&
            (identical(
                    other.estimatedCompletionTime, estimatedCompletionTime) ||
                other.estimatedCompletionTime == estimatedCompletionTime) &&
            const DeepCollectionEquality().equals(other._metadata, _metadata));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      status,
      priority,
      const DeepCollectionEquality().hash(_items),
      customer,
      notes,
      tableNumber,
      estimatedCompletionTime,
      const DeepCollectionEquality().hash(_metadata));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateOrderRequestImplCopyWith<_$UpdateOrderRequestImpl> get copyWith =>
      __$$UpdateOrderRequestImplCopyWithImpl<_$UpdateOrderRequestImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateOrderRequestImplToJson(
      this,
    );
  }
}

abstract class _UpdateOrderRequest implements UpdateOrderRequest {
  const factory _UpdateOrderRequest(
      {final OrderStatus? status,
      final OrderPriority? priority,
      final List<OrderItem>? items,
      final OrderCustomer? customer,
      final String? notes,
      final String? tableNumber,
      final DateTime? estimatedCompletionTime,
      final Map<String, dynamic>? metadata}) = _$UpdateOrderRequestImpl;

  factory _UpdateOrderRequest.fromJson(Map<String, dynamic> json) =
      _$UpdateOrderRequestImpl.fromJson;

  @override
  OrderStatus? get status;
  @override
  OrderPriority? get priority;
  @override
  List<OrderItem>? get items;
  @override
  OrderCustomer? get customer;
  @override
  String? get notes;
  @override
  String? get tableNumber;
  @override
  DateTime? get estimatedCompletionTime;
  @override
  Map<String, dynamic>? get metadata;
  @override
  @JsonKey(ignore: true)
  _$$UpdateOrderRequestImplCopyWith<_$UpdateOrderRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
