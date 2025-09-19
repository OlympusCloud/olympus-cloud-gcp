// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'product.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

ProductVariant _$ProductVariantFromJson(Map<String, dynamic> json) {
  return _ProductVariant.fromJson(json);
}

/// @nodoc
mixin _$ProductVariant {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  double get price => throw _privateConstructorUsedError;
  String? get sku => throw _privateConstructorUsedError;
  int? get stockQuantity => throw _privateConstructorUsedError;
  Map<String, dynamic>? get attributes => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ProductVariantCopyWith<ProductVariant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ProductVariantCopyWith<$Res> {
  factory $ProductVariantCopyWith(
          ProductVariant value, $Res Function(ProductVariant) then) =
      _$ProductVariantCopyWithImpl<$Res, ProductVariant>;
  @useResult
  $Res call(
      {String id,
      String name,
      double price,
      String? sku,
      int? stockQuantity,
      Map<String, dynamic>? attributes});
}

/// @nodoc
class _$ProductVariantCopyWithImpl<$Res, $Val extends ProductVariant>
    implements $ProductVariantCopyWith<$Res> {
  _$ProductVariantCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? price = null,
    Object? sku = freezed,
    Object? stockQuantity = freezed,
    Object? attributes = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      price: null == price
          ? _value.price
          : price // ignore: cast_nullable_to_non_nullable
              as double,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      stockQuantity: freezed == stockQuantity
          ? _value.stockQuantity
          : stockQuantity // ignore: cast_nullable_to_non_nullable
              as int?,
      attributes: freezed == attributes
          ? _value.attributes
          : attributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ProductVariantImplCopyWith<$Res>
    implements $ProductVariantCopyWith<$Res> {
  factory _$$ProductVariantImplCopyWith(_$ProductVariantImpl value,
          $Res Function(_$ProductVariantImpl) then) =
      __$$ProductVariantImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String name,
      double price,
      String? sku,
      int? stockQuantity,
      Map<String, dynamic>? attributes});
}

/// @nodoc
class __$$ProductVariantImplCopyWithImpl<$Res>
    extends _$ProductVariantCopyWithImpl<$Res, _$ProductVariantImpl>
    implements _$$ProductVariantImplCopyWith<$Res> {
  __$$ProductVariantImplCopyWithImpl(
      _$ProductVariantImpl _value, $Res Function(_$ProductVariantImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? price = null,
    Object? sku = freezed,
    Object? stockQuantity = freezed,
    Object? attributes = freezed,
  }) {
    return _then(_$ProductVariantImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      price: null == price
          ? _value.price
          : price // ignore: cast_nullable_to_non_nullable
              as double,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      stockQuantity: freezed == stockQuantity
          ? _value.stockQuantity
          : stockQuantity // ignore: cast_nullable_to_non_nullable
              as int?,
      attributes: freezed == attributes
          ? _value._attributes
          : attributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ProductVariantImpl implements _ProductVariant {
  const _$ProductVariantImpl(
      {required this.id,
      required this.name,
      required this.price,
      this.sku,
      this.stockQuantity,
      final Map<String, dynamic>? attributes})
      : _attributes = attributes;

  factory _$ProductVariantImpl.fromJson(Map<String, dynamic> json) =>
      _$$ProductVariantImplFromJson(json);

  @override
  final String id;
  @override
  final String name;
  @override
  final double price;
  @override
  final String? sku;
  @override
  final int? stockQuantity;
  final Map<String, dynamic>? _attributes;
  @override
  Map<String, dynamic>? get attributes {
    final value = _attributes;
    if (value == null) return null;
    if (_attributes is EqualUnmodifiableMapView) return _attributes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'ProductVariant(id: $id, name: $name, price: $price, sku: $sku, stockQuantity: $stockQuantity, attributes: $attributes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ProductVariantImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.price, price) || other.price == price) &&
            (identical(other.sku, sku) || other.sku == sku) &&
            (identical(other.stockQuantity, stockQuantity) ||
                other.stockQuantity == stockQuantity) &&
            const DeepCollectionEquality()
                .equals(other._attributes, _attributes));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, id, name, price, sku,
      stockQuantity, const DeepCollectionEquality().hash(_attributes));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ProductVariantImplCopyWith<_$ProductVariantImpl> get copyWith =>
      __$$ProductVariantImplCopyWithImpl<_$ProductVariantImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ProductVariantImplToJson(
      this,
    );
  }
}

abstract class _ProductVariant implements ProductVariant {
  const factory _ProductVariant(
      {required final String id,
      required final String name,
      required final double price,
      final String? sku,
      final int? stockQuantity,
      final Map<String, dynamic>? attributes}) = _$ProductVariantImpl;

  factory _ProductVariant.fromJson(Map<String, dynamic> json) =
      _$ProductVariantImpl.fromJson;

  @override
  String get id;
  @override
  String get name;
  @override
  double get price;
  @override
  String? get sku;
  @override
  int? get stockQuantity;
  @override
  Map<String, dynamic>? get attributes;
  @override
  @JsonKey(ignore: true)
  _$$ProductVariantImplCopyWith<_$ProductVariantImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

ProductPricing _$ProductPricingFromJson(Map<String, dynamic> json) {
  return _ProductPricing.fromJson(json);
}

/// @nodoc
mixin _$ProductPricing {
  double get basePrice => throw _privateConstructorUsedError;
  double? get salePrice => throw _privateConstructorUsedError;
  double? get cost => throw _privateConstructorUsedError;
  double? get margin => throw _privateConstructorUsedError;
  DateTime? get salePriceStart => throw _privateConstructorUsedError;
  DateTime? get salePriceEnd => throw _privateConstructorUsedError;
  Map<String, double>? get tierPricing => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ProductPricingCopyWith<ProductPricing> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ProductPricingCopyWith<$Res> {
  factory $ProductPricingCopyWith(
          ProductPricing value, $Res Function(ProductPricing) then) =
      _$ProductPricingCopyWithImpl<$Res, ProductPricing>;
  @useResult
  $Res call(
      {double basePrice,
      double? salePrice,
      double? cost,
      double? margin,
      DateTime? salePriceStart,
      DateTime? salePriceEnd,
      Map<String, double>? tierPricing});
}

/// @nodoc
class _$ProductPricingCopyWithImpl<$Res, $Val extends ProductPricing>
    implements $ProductPricingCopyWith<$Res> {
  _$ProductPricingCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? basePrice = null,
    Object? salePrice = freezed,
    Object? cost = freezed,
    Object? margin = freezed,
    Object? salePriceStart = freezed,
    Object? salePriceEnd = freezed,
    Object? tierPricing = freezed,
  }) {
    return _then(_value.copyWith(
      basePrice: null == basePrice
          ? _value.basePrice
          : basePrice // ignore: cast_nullable_to_non_nullable
              as double,
      salePrice: freezed == salePrice
          ? _value.salePrice
          : salePrice // ignore: cast_nullable_to_non_nullable
              as double?,
      cost: freezed == cost
          ? _value.cost
          : cost // ignore: cast_nullable_to_non_nullable
              as double?,
      margin: freezed == margin
          ? _value.margin
          : margin // ignore: cast_nullable_to_non_nullable
              as double?,
      salePriceStart: freezed == salePriceStart
          ? _value.salePriceStart
          : salePriceStart // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      salePriceEnd: freezed == salePriceEnd
          ? _value.salePriceEnd
          : salePriceEnd // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      tierPricing: freezed == tierPricing
          ? _value.tierPricing
          : tierPricing // ignore: cast_nullable_to_non_nullable
              as Map<String, double>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ProductPricingImplCopyWith<$Res>
    implements $ProductPricingCopyWith<$Res> {
  factory _$$ProductPricingImplCopyWith(_$ProductPricingImpl value,
          $Res Function(_$ProductPricingImpl) then) =
      __$$ProductPricingImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {double basePrice,
      double? salePrice,
      double? cost,
      double? margin,
      DateTime? salePriceStart,
      DateTime? salePriceEnd,
      Map<String, double>? tierPricing});
}

/// @nodoc
class __$$ProductPricingImplCopyWithImpl<$Res>
    extends _$ProductPricingCopyWithImpl<$Res, _$ProductPricingImpl>
    implements _$$ProductPricingImplCopyWith<$Res> {
  __$$ProductPricingImplCopyWithImpl(
      _$ProductPricingImpl _value, $Res Function(_$ProductPricingImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? basePrice = null,
    Object? salePrice = freezed,
    Object? cost = freezed,
    Object? margin = freezed,
    Object? salePriceStart = freezed,
    Object? salePriceEnd = freezed,
    Object? tierPricing = freezed,
  }) {
    return _then(_$ProductPricingImpl(
      basePrice: null == basePrice
          ? _value.basePrice
          : basePrice // ignore: cast_nullable_to_non_nullable
              as double,
      salePrice: freezed == salePrice
          ? _value.salePrice
          : salePrice // ignore: cast_nullable_to_non_nullable
              as double?,
      cost: freezed == cost
          ? _value.cost
          : cost // ignore: cast_nullable_to_non_nullable
              as double?,
      margin: freezed == margin
          ? _value.margin
          : margin // ignore: cast_nullable_to_non_nullable
              as double?,
      salePriceStart: freezed == salePriceStart
          ? _value.salePriceStart
          : salePriceStart // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      salePriceEnd: freezed == salePriceEnd
          ? _value.salePriceEnd
          : salePriceEnd // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      tierPricing: freezed == tierPricing
          ? _value._tierPricing
          : tierPricing // ignore: cast_nullable_to_non_nullable
              as Map<String, double>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ProductPricingImpl implements _ProductPricing {
  const _$ProductPricingImpl(
      {required this.basePrice,
      this.salePrice,
      this.cost,
      this.margin,
      this.salePriceStart,
      this.salePriceEnd,
      final Map<String, double>? tierPricing})
      : _tierPricing = tierPricing;

  factory _$ProductPricingImpl.fromJson(Map<String, dynamic> json) =>
      _$$ProductPricingImplFromJson(json);

  @override
  final double basePrice;
  @override
  final double? salePrice;
  @override
  final double? cost;
  @override
  final double? margin;
  @override
  final DateTime? salePriceStart;
  @override
  final DateTime? salePriceEnd;
  final Map<String, double>? _tierPricing;
  @override
  Map<String, double>? get tierPricing {
    final value = _tierPricing;
    if (value == null) return null;
    if (_tierPricing is EqualUnmodifiableMapView) return _tierPricing;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'ProductPricing(basePrice: $basePrice, salePrice: $salePrice, cost: $cost, margin: $margin, salePriceStart: $salePriceStart, salePriceEnd: $salePriceEnd, tierPricing: $tierPricing)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ProductPricingImpl &&
            (identical(other.basePrice, basePrice) ||
                other.basePrice == basePrice) &&
            (identical(other.salePrice, salePrice) ||
                other.salePrice == salePrice) &&
            (identical(other.cost, cost) || other.cost == cost) &&
            (identical(other.margin, margin) || other.margin == margin) &&
            (identical(other.salePriceStart, salePriceStart) ||
                other.salePriceStart == salePriceStart) &&
            (identical(other.salePriceEnd, salePriceEnd) ||
                other.salePriceEnd == salePriceEnd) &&
            const DeepCollectionEquality()
                .equals(other._tierPricing, _tierPricing));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      basePrice,
      salePrice,
      cost,
      margin,
      salePriceStart,
      salePriceEnd,
      const DeepCollectionEquality().hash(_tierPricing));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ProductPricingImplCopyWith<_$ProductPricingImpl> get copyWith =>
      __$$ProductPricingImplCopyWithImpl<_$ProductPricingImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ProductPricingImplToJson(
      this,
    );
  }
}

abstract class _ProductPricing implements ProductPricing {
  const factory _ProductPricing(
      {required final double basePrice,
      final double? salePrice,
      final double? cost,
      final double? margin,
      final DateTime? salePriceStart,
      final DateTime? salePriceEnd,
      final Map<String, double>? tierPricing}) = _$ProductPricingImpl;

  factory _ProductPricing.fromJson(Map<String, dynamic> json) =
      _$ProductPricingImpl.fromJson;

  @override
  double get basePrice;
  @override
  double? get salePrice;
  @override
  double? get cost;
  @override
  double? get margin;
  @override
  DateTime? get salePriceStart;
  @override
  DateTime? get salePriceEnd;
  @override
  Map<String, double>? get tierPricing;
  @override
  @JsonKey(ignore: true)
  _$$ProductPricingImplCopyWith<_$ProductPricingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

ProductInventory _$ProductInventoryFromJson(Map<String, dynamic> json) {
  return _ProductInventory.fromJson(json);
}

/// @nodoc
mixin _$ProductInventory {
  bool get trackStock => throw _privateConstructorUsedError;
  int? get currentStock => throw _privateConstructorUsedError;
  int? get lowStockThreshold => throw _privateConstructorUsedError;
  int? get reorderPoint => throw _privateConstructorUsedError;
  int? get reorderQuantity => throw _privateConstructorUsedError;
  String? get stockUnit => throw _privateConstructorUsedError;
  String? get supplier => throw _privateConstructorUsedError;
  Map<String, dynamic>? get locations => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ProductInventoryCopyWith<ProductInventory> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ProductInventoryCopyWith<$Res> {
  factory $ProductInventoryCopyWith(
          ProductInventory value, $Res Function(ProductInventory) then) =
      _$ProductInventoryCopyWithImpl<$Res, ProductInventory>;
  @useResult
  $Res call(
      {bool trackStock,
      int? currentStock,
      int? lowStockThreshold,
      int? reorderPoint,
      int? reorderQuantity,
      String? stockUnit,
      String? supplier,
      Map<String, dynamic>? locations});
}

/// @nodoc
class _$ProductInventoryCopyWithImpl<$Res, $Val extends ProductInventory>
    implements $ProductInventoryCopyWith<$Res> {
  _$ProductInventoryCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? trackStock = null,
    Object? currentStock = freezed,
    Object? lowStockThreshold = freezed,
    Object? reorderPoint = freezed,
    Object? reorderQuantity = freezed,
    Object? stockUnit = freezed,
    Object? supplier = freezed,
    Object? locations = freezed,
  }) {
    return _then(_value.copyWith(
      trackStock: null == trackStock
          ? _value.trackStock
          : trackStock // ignore: cast_nullable_to_non_nullable
              as bool,
      currentStock: freezed == currentStock
          ? _value.currentStock
          : currentStock // ignore: cast_nullable_to_non_nullable
              as int?,
      lowStockThreshold: freezed == lowStockThreshold
          ? _value.lowStockThreshold
          : lowStockThreshold // ignore: cast_nullable_to_non_nullable
              as int?,
      reorderPoint: freezed == reorderPoint
          ? _value.reorderPoint
          : reorderPoint // ignore: cast_nullable_to_non_nullable
              as int?,
      reorderQuantity: freezed == reorderQuantity
          ? _value.reorderQuantity
          : reorderQuantity // ignore: cast_nullable_to_non_nullable
              as int?,
      stockUnit: freezed == stockUnit
          ? _value.stockUnit
          : stockUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      supplier: freezed == supplier
          ? _value.supplier
          : supplier // ignore: cast_nullable_to_non_nullable
              as String?,
      locations: freezed == locations
          ? _value.locations
          : locations // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ProductInventoryImplCopyWith<$Res>
    implements $ProductInventoryCopyWith<$Res> {
  factory _$$ProductInventoryImplCopyWith(_$ProductInventoryImpl value,
          $Res Function(_$ProductInventoryImpl) then) =
      __$$ProductInventoryImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool trackStock,
      int? currentStock,
      int? lowStockThreshold,
      int? reorderPoint,
      int? reorderQuantity,
      String? stockUnit,
      String? supplier,
      Map<String, dynamic>? locations});
}

/// @nodoc
class __$$ProductInventoryImplCopyWithImpl<$Res>
    extends _$ProductInventoryCopyWithImpl<$Res, _$ProductInventoryImpl>
    implements _$$ProductInventoryImplCopyWith<$Res> {
  __$$ProductInventoryImplCopyWithImpl(_$ProductInventoryImpl _value,
      $Res Function(_$ProductInventoryImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? trackStock = null,
    Object? currentStock = freezed,
    Object? lowStockThreshold = freezed,
    Object? reorderPoint = freezed,
    Object? reorderQuantity = freezed,
    Object? stockUnit = freezed,
    Object? supplier = freezed,
    Object? locations = freezed,
  }) {
    return _then(_$ProductInventoryImpl(
      trackStock: null == trackStock
          ? _value.trackStock
          : trackStock // ignore: cast_nullable_to_non_nullable
              as bool,
      currentStock: freezed == currentStock
          ? _value.currentStock
          : currentStock // ignore: cast_nullable_to_non_nullable
              as int?,
      lowStockThreshold: freezed == lowStockThreshold
          ? _value.lowStockThreshold
          : lowStockThreshold // ignore: cast_nullable_to_non_nullable
              as int?,
      reorderPoint: freezed == reorderPoint
          ? _value.reorderPoint
          : reorderPoint // ignore: cast_nullable_to_non_nullable
              as int?,
      reorderQuantity: freezed == reorderQuantity
          ? _value.reorderQuantity
          : reorderQuantity // ignore: cast_nullable_to_non_nullable
              as int?,
      stockUnit: freezed == stockUnit
          ? _value.stockUnit
          : stockUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      supplier: freezed == supplier
          ? _value.supplier
          : supplier // ignore: cast_nullable_to_non_nullable
              as String?,
      locations: freezed == locations
          ? _value._locations
          : locations // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ProductInventoryImpl implements _ProductInventory {
  const _$ProductInventoryImpl(
      {required this.trackStock,
      this.currentStock,
      this.lowStockThreshold,
      this.reorderPoint,
      this.reorderQuantity,
      this.stockUnit,
      this.supplier,
      final Map<String, dynamic>? locations})
      : _locations = locations;

  factory _$ProductInventoryImpl.fromJson(Map<String, dynamic> json) =>
      _$$ProductInventoryImplFromJson(json);

  @override
  final bool trackStock;
  @override
  final int? currentStock;
  @override
  final int? lowStockThreshold;
  @override
  final int? reorderPoint;
  @override
  final int? reorderQuantity;
  @override
  final String? stockUnit;
  @override
  final String? supplier;
  final Map<String, dynamic>? _locations;
  @override
  Map<String, dynamic>? get locations {
    final value = _locations;
    if (value == null) return null;
    if (_locations is EqualUnmodifiableMapView) return _locations;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'ProductInventory(trackStock: $trackStock, currentStock: $currentStock, lowStockThreshold: $lowStockThreshold, reorderPoint: $reorderPoint, reorderQuantity: $reorderQuantity, stockUnit: $stockUnit, supplier: $supplier, locations: $locations)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ProductInventoryImpl &&
            (identical(other.trackStock, trackStock) ||
                other.trackStock == trackStock) &&
            (identical(other.currentStock, currentStock) ||
                other.currentStock == currentStock) &&
            (identical(other.lowStockThreshold, lowStockThreshold) ||
                other.lowStockThreshold == lowStockThreshold) &&
            (identical(other.reorderPoint, reorderPoint) ||
                other.reorderPoint == reorderPoint) &&
            (identical(other.reorderQuantity, reorderQuantity) ||
                other.reorderQuantity == reorderQuantity) &&
            (identical(other.stockUnit, stockUnit) ||
                other.stockUnit == stockUnit) &&
            (identical(other.supplier, supplier) ||
                other.supplier == supplier) &&
            const DeepCollectionEquality()
                .equals(other._locations, _locations));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      trackStock,
      currentStock,
      lowStockThreshold,
      reorderPoint,
      reorderQuantity,
      stockUnit,
      supplier,
      const DeepCollectionEquality().hash(_locations));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ProductInventoryImplCopyWith<_$ProductInventoryImpl> get copyWith =>
      __$$ProductInventoryImplCopyWithImpl<_$ProductInventoryImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ProductInventoryImplToJson(
      this,
    );
  }
}

abstract class _ProductInventory implements ProductInventory {
  const factory _ProductInventory(
      {required final bool trackStock,
      final int? currentStock,
      final int? lowStockThreshold,
      final int? reorderPoint,
      final int? reorderQuantity,
      final String? stockUnit,
      final String? supplier,
      final Map<String, dynamic>? locations}) = _$ProductInventoryImpl;

  factory _ProductInventory.fromJson(Map<String, dynamic> json) =
      _$ProductInventoryImpl.fromJson;

  @override
  bool get trackStock;
  @override
  int? get currentStock;
  @override
  int? get lowStockThreshold;
  @override
  int? get reorderPoint;
  @override
  int? get reorderQuantity;
  @override
  String? get stockUnit;
  @override
  String? get supplier;
  @override
  Map<String, dynamic>? get locations;
  @override
  @JsonKey(ignore: true)
  _$$ProductInventoryImplCopyWith<_$ProductInventoryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

Product _$ProductFromJson(Map<String, dynamic> json) {
  return _Product.fromJson(json);
}

/// @nodoc
mixin _$Product {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get description => throw _privateConstructorUsedError;
  ProductCategory get category => throw _privateConstructorUsedError;
  ProductStatus get status => throw _privateConstructorUsedError;
  ProductPricing get pricing => throw _privateConstructorUsedError;
  ProductInventory get inventory => throw _privateConstructorUsedError;
  String? get sku => throw _privateConstructorUsedError;
  String? get barcode => throw _privateConstructorUsedError;
  List<String>? get images => throw _privateConstructorUsedError;
  List<String>? get tags => throw _privateConstructorUsedError;
  List<ProductVariant>? get variants => throw _privateConstructorUsedError;
  double? get weight => throw _privateConstructorUsedError;
  String? get weightUnit => throw _privateConstructorUsedError;
  Map<String, dynamic>? get nutritionalInfo =>
      throw _privateConstructorUsedError;
  Map<String, dynamic>? get customAttributes =>
      throw _privateConstructorUsedError;
  DateTime get createdAt => throw _privateConstructorUsedError;
  DateTime? get updatedAt => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $ProductCopyWith<Product> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ProductCopyWith<$Res> {
  factory $ProductCopyWith(Product value, $Res Function(Product) then) =
      _$ProductCopyWithImpl<$Res, Product>;
  @useResult
  $Res call(
      {String id,
      String name,
      String description,
      ProductCategory category,
      ProductStatus status,
      ProductPricing pricing,
      ProductInventory inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes,
      DateTime createdAt,
      DateTime? updatedAt});

  $ProductPricingCopyWith<$Res> get pricing;
  $ProductInventoryCopyWith<$Res> get inventory;
}

/// @nodoc
class _$ProductCopyWithImpl<$Res, $Val extends Product>
    implements $ProductCopyWith<$Res> {
  _$ProductCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = null,
    Object? category = null,
    Object? status = null,
    Object? pricing = null,
    Object? inventory = null,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
    Object? createdAt = null,
    Object? updatedAt = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      category: null == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ProductStatus,
      pricing: null == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing,
      inventory: null == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value.images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value.tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value.variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value.nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value.customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as DateTime,
      updatedAt: freezed == updatedAt
          ? _value.updatedAt
          : updatedAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductPricingCopyWith<$Res> get pricing {
    return $ProductPricingCopyWith<$Res>(_value.pricing, (value) {
      return _then(_value.copyWith(pricing: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductInventoryCopyWith<$Res> get inventory {
    return $ProductInventoryCopyWith<$Res>(_value.inventory, (value) {
      return _then(_value.copyWith(inventory: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ProductImplCopyWith<$Res> implements $ProductCopyWith<$Res> {
  factory _$$ProductImplCopyWith(
          _$ProductImpl value, $Res Function(_$ProductImpl) then) =
      __$$ProductImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String name,
      String description,
      ProductCategory category,
      ProductStatus status,
      ProductPricing pricing,
      ProductInventory inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes,
      DateTime createdAt,
      DateTime? updatedAt});

  @override
  $ProductPricingCopyWith<$Res> get pricing;
  @override
  $ProductInventoryCopyWith<$Res> get inventory;
}

/// @nodoc
class __$$ProductImplCopyWithImpl<$Res>
    extends _$ProductCopyWithImpl<$Res, _$ProductImpl>
    implements _$$ProductImplCopyWith<$Res> {
  __$$ProductImplCopyWithImpl(
      _$ProductImpl _value, $Res Function(_$ProductImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = null,
    Object? category = null,
    Object? status = null,
    Object? pricing = null,
    Object? inventory = null,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
    Object? createdAt = null,
    Object? updatedAt = freezed,
  }) {
    return _then(_$ProductImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      category: null == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ProductStatus,
      pricing: null == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing,
      inventory: null == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value._images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value._tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value._variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value._nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value._customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as DateTime,
      updatedAt: freezed == updatedAt
          ? _value.updatedAt
          : updatedAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ProductImpl implements _Product {
  const _$ProductImpl(
      {required this.id,
      required this.name,
      required this.description,
      required this.category,
      required this.status,
      required this.pricing,
      required this.inventory,
      this.sku,
      this.barcode,
      final List<String>? images,
      final List<String>? tags,
      final List<ProductVariant>? variants,
      this.weight,
      this.weightUnit,
      final Map<String, dynamic>? nutritionalInfo,
      final Map<String, dynamic>? customAttributes,
      required this.createdAt,
      this.updatedAt})
      : _images = images,
        _tags = tags,
        _variants = variants,
        _nutritionalInfo = nutritionalInfo,
        _customAttributes = customAttributes;

  factory _$ProductImpl.fromJson(Map<String, dynamic> json) =>
      _$$ProductImplFromJson(json);

  @override
  final String id;
  @override
  final String name;
  @override
  final String description;
  @override
  final ProductCategory category;
  @override
  final ProductStatus status;
  @override
  final ProductPricing pricing;
  @override
  final ProductInventory inventory;
  @override
  final String? sku;
  @override
  final String? barcode;
  final List<String>? _images;
  @override
  List<String>? get images {
    final value = _images;
    if (value == null) return null;
    if (_images is EqualUnmodifiableListView) return _images;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<String>? _tags;
  @override
  List<String>? get tags {
    final value = _tags;
    if (value == null) return null;
    if (_tags is EqualUnmodifiableListView) return _tags;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<ProductVariant>? _variants;
  @override
  List<ProductVariant>? get variants {
    final value = _variants;
    if (value == null) return null;
    if (_variants is EqualUnmodifiableListView) return _variants;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final double? weight;
  @override
  final String? weightUnit;
  final Map<String, dynamic>? _nutritionalInfo;
  @override
  Map<String, dynamic>? get nutritionalInfo {
    final value = _nutritionalInfo;
    if (value == null) return null;
    if (_nutritionalInfo is EqualUnmodifiableMapView) return _nutritionalInfo;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  final Map<String, dynamic>? _customAttributes;
  @override
  Map<String, dynamic>? get customAttributes {
    final value = _customAttributes;
    if (value == null) return null;
    if (_customAttributes is EqualUnmodifiableMapView) return _customAttributes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  final DateTime createdAt;
  @override
  final DateTime? updatedAt;

  @override
  String toString() {
    return 'Product(id: $id, name: $name, description: $description, category: $category, status: $status, pricing: $pricing, inventory: $inventory, sku: $sku, barcode: $barcode, images: $images, tags: $tags, variants: $variants, weight: $weight, weightUnit: $weightUnit, nutritionalInfo: $nutritionalInfo, customAttributes: $customAttributes, createdAt: $createdAt, updatedAt: $updatedAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ProductImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.category, category) ||
                other.category == category) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.pricing, pricing) || other.pricing == pricing) &&
            (identical(other.inventory, inventory) ||
                other.inventory == inventory) &&
            (identical(other.sku, sku) || other.sku == sku) &&
            (identical(other.barcode, barcode) || other.barcode == barcode) &&
            const DeepCollectionEquality().equals(other._images, _images) &&
            const DeepCollectionEquality().equals(other._tags, _tags) &&
            const DeepCollectionEquality().equals(other._variants, _variants) &&
            (identical(other.weight, weight) || other.weight == weight) &&
            (identical(other.weightUnit, weightUnit) ||
                other.weightUnit == weightUnit) &&
            const DeepCollectionEquality()
                .equals(other._nutritionalInfo, _nutritionalInfo) &&
            const DeepCollectionEquality()
                .equals(other._customAttributes, _customAttributes) &&
            (identical(other.createdAt, createdAt) ||
                other.createdAt == createdAt) &&
            (identical(other.updatedAt, updatedAt) ||
                other.updatedAt == updatedAt));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      id,
      name,
      description,
      category,
      status,
      pricing,
      inventory,
      sku,
      barcode,
      const DeepCollectionEquality().hash(_images),
      const DeepCollectionEquality().hash(_tags),
      const DeepCollectionEquality().hash(_variants),
      weight,
      weightUnit,
      const DeepCollectionEquality().hash(_nutritionalInfo),
      const DeepCollectionEquality().hash(_customAttributes),
      createdAt,
      updatedAt);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ProductImplCopyWith<_$ProductImpl> get copyWith =>
      __$$ProductImplCopyWithImpl<_$ProductImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ProductImplToJson(
      this,
    );
  }
}

abstract class _Product implements Product {
  const factory _Product(
      {required final String id,
      required final String name,
      required final String description,
      required final ProductCategory category,
      required final ProductStatus status,
      required final ProductPricing pricing,
      required final ProductInventory inventory,
      final String? sku,
      final String? barcode,
      final List<String>? images,
      final List<String>? tags,
      final List<ProductVariant>? variants,
      final double? weight,
      final String? weightUnit,
      final Map<String, dynamic>? nutritionalInfo,
      final Map<String, dynamic>? customAttributes,
      required final DateTime createdAt,
      final DateTime? updatedAt}) = _$ProductImpl;

  factory _Product.fromJson(Map<String, dynamic> json) = _$ProductImpl.fromJson;

  @override
  String get id;
  @override
  String get name;
  @override
  String get description;
  @override
  ProductCategory get category;
  @override
  ProductStatus get status;
  @override
  ProductPricing get pricing;
  @override
  ProductInventory get inventory;
  @override
  String? get sku;
  @override
  String? get barcode;
  @override
  List<String>? get images;
  @override
  List<String>? get tags;
  @override
  List<ProductVariant>? get variants;
  @override
  double? get weight;
  @override
  String? get weightUnit;
  @override
  Map<String, dynamic>? get nutritionalInfo;
  @override
  Map<String, dynamic>? get customAttributes;
  @override
  DateTime get createdAt;
  @override
  DateTime? get updatedAt;
  @override
  @JsonKey(ignore: true)
  _$$ProductImplCopyWith<_$ProductImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CreateProductRequest _$CreateProductRequestFromJson(Map<String, dynamic> json) {
  return _CreateProductRequest.fromJson(json);
}

/// @nodoc
mixin _$CreateProductRequest {
  String get name => throw _privateConstructorUsedError;
  String get description => throw _privateConstructorUsedError;
  ProductCategory get category => throw _privateConstructorUsedError;
  ProductPricing get pricing => throw _privateConstructorUsedError;
  ProductInventory get inventory => throw _privateConstructorUsedError;
  String? get sku => throw _privateConstructorUsedError;
  String? get barcode => throw _privateConstructorUsedError;
  List<String>? get images => throw _privateConstructorUsedError;
  List<String>? get tags => throw _privateConstructorUsedError;
  List<ProductVariant>? get variants => throw _privateConstructorUsedError;
  double? get weight => throw _privateConstructorUsedError;
  String? get weightUnit => throw _privateConstructorUsedError;
  Map<String, dynamic>? get nutritionalInfo =>
      throw _privateConstructorUsedError;
  Map<String, dynamic>? get customAttributes =>
      throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CreateProductRequestCopyWith<CreateProductRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateProductRequestCopyWith<$Res> {
  factory $CreateProductRequestCopyWith(CreateProductRequest value,
          $Res Function(CreateProductRequest) then) =
      _$CreateProductRequestCopyWithImpl<$Res, CreateProductRequest>;
  @useResult
  $Res call(
      {String name,
      String description,
      ProductCategory category,
      ProductPricing pricing,
      ProductInventory inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes});

  $ProductPricingCopyWith<$Res> get pricing;
  $ProductInventoryCopyWith<$Res> get inventory;
}

/// @nodoc
class _$CreateProductRequestCopyWithImpl<$Res,
        $Val extends CreateProductRequest>
    implements $CreateProductRequestCopyWith<$Res> {
  _$CreateProductRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? description = null,
    Object? category = null,
    Object? pricing = null,
    Object? inventory = null,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
  }) {
    return _then(_value.copyWith(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      category: null == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory,
      pricing: null == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing,
      inventory: null == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value.images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value.tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value.variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value.nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value.customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductPricingCopyWith<$Res> get pricing {
    return $ProductPricingCopyWith<$Res>(_value.pricing, (value) {
      return _then(_value.copyWith(pricing: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductInventoryCopyWith<$Res> get inventory {
    return $ProductInventoryCopyWith<$Res>(_value.inventory, (value) {
      return _then(_value.copyWith(inventory: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$CreateProductRequestImplCopyWith<$Res>
    implements $CreateProductRequestCopyWith<$Res> {
  factory _$$CreateProductRequestImplCopyWith(_$CreateProductRequestImpl value,
          $Res Function(_$CreateProductRequestImpl) then) =
      __$$CreateProductRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String name,
      String description,
      ProductCategory category,
      ProductPricing pricing,
      ProductInventory inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes});

  @override
  $ProductPricingCopyWith<$Res> get pricing;
  @override
  $ProductInventoryCopyWith<$Res> get inventory;
}

/// @nodoc
class __$$CreateProductRequestImplCopyWithImpl<$Res>
    extends _$CreateProductRequestCopyWithImpl<$Res, _$CreateProductRequestImpl>
    implements _$$CreateProductRequestImplCopyWith<$Res> {
  __$$CreateProductRequestImplCopyWithImpl(_$CreateProductRequestImpl _value,
      $Res Function(_$CreateProductRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? description = null,
    Object? category = null,
    Object? pricing = null,
    Object? inventory = null,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
  }) {
    return _then(_$CreateProductRequestImpl(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      category: null == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory,
      pricing: null == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing,
      inventory: null == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value._images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value._tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value._variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value._nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value._customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateProductRequestImpl implements _CreateProductRequest {
  const _$CreateProductRequestImpl(
      {required this.name,
      required this.description,
      required this.category,
      required this.pricing,
      required this.inventory,
      this.sku,
      this.barcode,
      final List<String>? images,
      final List<String>? tags,
      final List<ProductVariant>? variants,
      this.weight,
      this.weightUnit,
      final Map<String, dynamic>? nutritionalInfo,
      final Map<String, dynamic>? customAttributes})
      : _images = images,
        _tags = tags,
        _variants = variants,
        _nutritionalInfo = nutritionalInfo,
        _customAttributes = customAttributes;

  factory _$CreateProductRequestImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateProductRequestImplFromJson(json);

  @override
  final String name;
  @override
  final String description;
  @override
  final ProductCategory category;
  @override
  final ProductPricing pricing;
  @override
  final ProductInventory inventory;
  @override
  final String? sku;
  @override
  final String? barcode;
  final List<String>? _images;
  @override
  List<String>? get images {
    final value = _images;
    if (value == null) return null;
    if (_images is EqualUnmodifiableListView) return _images;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<String>? _tags;
  @override
  List<String>? get tags {
    final value = _tags;
    if (value == null) return null;
    if (_tags is EqualUnmodifiableListView) return _tags;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<ProductVariant>? _variants;
  @override
  List<ProductVariant>? get variants {
    final value = _variants;
    if (value == null) return null;
    if (_variants is EqualUnmodifiableListView) return _variants;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final double? weight;
  @override
  final String? weightUnit;
  final Map<String, dynamic>? _nutritionalInfo;
  @override
  Map<String, dynamic>? get nutritionalInfo {
    final value = _nutritionalInfo;
    if (value == null) return null;
    if (_nutritionalInfo is EqualUnmodifiableMapView) return _nutritionalInfo;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  final Map<String, dynamic>? _customAttributes;
  @override
  Map<String, dynamic>? get customAttributes {
    final value = _customAttributes;
    if (value == null) return null;
    if (_customAttributes is EqualUnmodifiableMapView) return _customAttributes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'CreateProductRequest(name: $name, description: $description, category: $category, pricing: $pricing, inventory: $inventory, sku: $sku, barcode: $barcode, images: $images, tags: $tags, variants: $variants, weight: $weight, weightUnit: $weightUnit, nutritionalInfo: $nutritionalInfo, customAttributes: $customAttributes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateProductRequestImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.category, category) ||
                other.category == category) &&
            (identical(other.pricing, pricing) || other.pricing == pricing) &&
            (identical(other.inventory, inventory) ||
                other.inventory == inventory) &&
            (identical(other.sku, sku) || other.sku == sku) &&
            (identical(other.barcode, barcode) || other.barcode == barcode) &&
            const DeepCollectionEquality().equals(other._images, _images) &&
            const DeepCollectionEquality().equals(other._tags, _tags) &&
            const DeepCollectionEquality().equals(other._variants, _variants) &&
            (identical(other.weight, weight) || other.weight == weight) &&
            (identical(other.weightUnit, weightUnit) ||
                other.weightUnit == weightUnit) &&
            const DeepCollectionEquality()
                .equals(other._nutritionalInfo, _nutritionalInfo) &&
            const DeepCollectionEquality()
                .equals(other._customAttributes, _customAttributes));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      name,
      description,
      category,
      pricing,
      inventory,
      sku,
      barcode,
      const DeepCollectionEquality().hash(_images),
      const DeepCollectionEquality().hash(_tags),
      const DeepCollectionEquality().hash(_variants),
      weight,
      weightUnit,
      const DeepCollectionEquality().hash(_nutritionalInfo),
      const DeepCollectionEquality().hash(_customAttributes));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateProductRequestImplCopyWith<_$CreateProductRequestImpl>
      get copyWith =>
          __$$CreateProductRequestImplCopyWithImpl<_$CreateProductRequestImpl>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateProductRequestImplToJson(
      this,
    );
  }
}

abstract class _CreateProductRequest implements CreateProductRequest {
  const factory _CreateProductRequest(
          {required final String name,
          required final String description,
          required final ProductCategory category,
          required final ProductPricing pricing,
          required final ProductInventory inventory,
          final String? sku,
          final String? barcode,
          final List<String>? images,
          final List<String>? tags,
          final List<ProductVariant>? variants,
          final double? weight,
          final String? weightUnit,
          final Map<String, dynamic>? nutritionalInfo,
          final Map<String, dynamic>? customAttributes}) =
      _$CreateProductRequestImpl;

  factory _CreateProductRequest.fromJson(Map<String, dynamic> json) =
      _$CreateProductRequestImpl.fromJson;

  @override
  String get name;
  @override
  String get description;
  @override
  ProductCategory get category;
  @override
  ProductPricing get pricing;
  @override
  ProductInventory get inventory;
  @override
  String? get sku;
  @override
  String? get barcode;
  @override
  List<String>? get images;
  @override
  List<String>? get tags;
  @override
  List<ProductVariant>? get variants;
  @override
  double? get weight;
  @override
  String? get weightUnit;
  @override
  Map<String, dynamic>? get nutritionalInfo;
  @override
  Map<String, dynamic>? get customAttributes;
  @override
  @JsonKey(ignore: true)
  _$$CreateProductRequestImplCopyWith<_$CreateProductRequestImpl>
      get copyWith => throw _privateConstructorUsedError;
}

UpdateProductRequest _$UpdateProductRequestFromJson(Map<String, dynamic> json) {
  return _UpdateProductRequest.fromJson(json);
}

/// @nodoc
mixin _$UpdateProductRequest {
  String? get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  ProductCategory? get category => throw _privateConstructorUsedError;
  ProductStatus? get status => throw _privateConstructorUsedError;
  ProductPricing? get pricing => throw _privateConstructorUsedError;
  ProductInventory? get inventory => throw _privateConstructorUsedError;
  String? get sku => throw _privateConstructorUsedError;
  String? get barcode => throw _privateConstructorUsedError;
  List<String>? get images => throw _privateConstructorUsedError;
  List<String>? get tags => throw _privateConstructorUsedError;
  List<ProductVariant>? get variants => throw _privateConstructorUsedError;
  double? get weight => throw _privateConstructorUsedError;
  String? get weightUnit => throw _privateConstructorUsedError;
  Map<String, dynamic>? get nutritionalInfo =>
      throw _privateConstructorUsedError;
  Map<String, dynamic>? get customAttributes =>
      throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $UpdateProductRequestCopyWith<UpdateProductRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateProductRequestCopyWith<$Res> {
  factory $UpdateProductRequestCopyWith(UpdateProductRequest value,
          $Res Function(UpdateProductRequest) then) =
      _$UpdateProductRequestCopyWithImpl<$Res, UpdateProductRequest>;
  @useResult
  $Res call(
      {String? name,
      String? description,
      ProductCategory? category,
      ProductStatus? status,
      ProductPricing? pricing,
      ProductInventory? inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes});

  $ProductPricingCopyWith<$Res>? get pricing;
  $ProductInventoryCopyWith<$Res>? get inventory;
}

/// @nodoc
class _$UpdateProductRequestCopyWithImpl<$Res,
        $Val extends UpdateProductRequest>
    implements $UpdateProductRequestCopyWith<$Res> {
  _$UpdateProductRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? description = freezed,
    Object? category = freezed,
    Object? status = freezed,
    Object? pricing = freezed,
    Object? inventory = freezed,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
  }) {
    return _then(_value.copyWith(
      name: freezed == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String?,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      category: freezed == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory?,
      status: freezed == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ProductStatus?,
      pricing: freezed == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing?,
      inventory: freezed == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory?,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value.images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value.tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value.variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value.nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value.customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductPricingCopyWith<$Res>? get pricing {
    if (_value.pricing == null) {
      return null;
    }

    return $ProductPricingCopyWith<$Res>(_value.pricing!, (value) {
      return _then(_value.copyWith(pricing: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ProductInventoryCopyWith<$Res>? get inventory {
    if (_value.inventory == null) {
      return null;
    }

    return $ProductInventoryCopyWith<$Res>(_value.inventory!, (value) {
      return _then(_value.copyWith(inventory: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$UpdateProductRequestImplCopyWith<$Res>
    implements $UpdateProductRequestCopyWith<$Res> {
  factory _$$UpdateProductRequestImplCopyWith(_$UpdateProductRequestImpl value,
          $Res Function(_$UpdateProductRequestImpl) then) =
      __$$UpdateProductRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String? name,
      String? description,
      ProductCategory? category,
      ProductStatus? status,
      ProductPricing? pricing,
      ProductInventory? inventory,
      String? sku,
      String? barcode,
      List<String>? images,
      List<String>? tags,
      List<ProductVariant>? variants,
      double? weight,
      String? weightUnit,
      Map<String, dynamic>? nutritionalInfo,
      Map<String, dynamic>? customAttributes});

  @override
  $ProductPricingCopyWith<$Res>? get pricing;
  @override
  $ProductInventoryCopyWith<$Res>? get inventory;
}

/// @nodoc
class __$$UpdateProductRequestImplCopyWithImpl<$Res>
    extends _$UpdateProductRequestCopyWithImpl<$Res, _$UpdateProductRequestImpl>
    implements _$$UpdateProductRequestImplCopyWith<$Res> {
  __$$UpdateProductRequestImplCopyWithImpl(_$UpdateProductRequestImpl _value,
      $Res Function(_$UpdateProductRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? description = freezed,
    Object? category = freezed,
    Object? status = freezed,
    Object? pricing = freezed,
    Object? inventory = freezed,
    Object? sku = freezed,
    Object? barcode = freezed,
    Object? images = freezed,
    Object? tags = freezed,
    Object? variants = freezed,
    Object? weight = freezed,
    Object? weightUnit = freezed,
    Object? nutritionalInfo = freezed,
    Object? customAttributes = freezed,
  }) {
    return _then(_$UpdateProductRequestImpl(
      name: freezed == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String?,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      category: freezed == category
          ? _value.category
          : category // ignore: cast_nullable_to_non_nullable
              as ProductCategory?,
      status: freezed == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ProductStatus?,
      pricing: freezed == pricing
          ? _value.pricing
          : pricing // ignore: cast_nullable_to_non_nullable
              as ProductPricing?,
      inventory: freezed == inventory
          ? _value.inventory
          : inventory // ignore: cast_nullable_to_non_nullable
              as ProductInventory?,
      sku: freezed == sku
          ? _value.sku
          : sku // ignore: cast_nullable_to_non_nullable
              as String?,
      barcode: freezed == barcode
          ? _value.barcode
          : barcode // ignore: cast_nullable_to_non_nullable
              as String?,
      images: freezed == images
          ? _value._images
          : images // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      tags: freezed == tags
          ? _value._tags
          : tags // ignore: cast_nullable_to_non_nullable
              as List<String>?,
      variants: freezed == variants
          ? _value._variants
          : variants // ignore: cast_nullable_to_non_nullable
              as List<ProductVariant>?,
      weight: freezed == weight
          ? _value.weight
          : weight // ignore: cast_nullable_to_non_nullable
              as double?,
      weightUnit: freezed == weightUnit
          ? _value.weightUnit
          : weightUnit // ignore: cast_nullable_to_non_nullable
              as String?,
      nutritionalInfo: freezed == nutritionalInfo
          ? _value._nutritionalInfo
          : nutritionalInfo // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
      customAttributes: freezed == customAttributes
          ? _value._customAttributes
          : customAttributes // ignore: cast_nullable_to_non_nullable
              as Map<String, dynamic>?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateProductRequestImpl implements _UpdateProductRequest {
  const _$UpdateProductRequestImpl(
      {this.name,
      this.description,
      this.category,
      this.status,
      this.pricing,
      this.inventory,
      this.sku,
      this.barcode,
      final List<String>? images,
      final List<String>? tags,
      final List<ProductVariant>? variants,
      this.weight,
      this.weightUnit,
      final Map<String, dynamic>? nutritionalInfo,
      final Map<String, dynamic>? customAttributes})
      : _images = images,
        _tags = tags,
        _variants = variants,
        _nutritionalInfo = nutritionalInfo,
        _customAttributes = customAttributes;

  factory _$UpdateProductRequestImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateProductRequestImplFromJson(json);

  @override
  final String? name;
  @override
  final String? description;
  @override
  final ProductCategory? category;
  @override
  final ProductStatus? status;
  @override
  final ProductPricing? pricing;
  @override
  final ProductInventory? inventory;
  @override
  final String? sku;
  @override
  final String? barcode;
  final List<String>? _images;
  @override
  List<String>? get images {
    final value = _images;
    if (value == null) return null;
    if (_images is EqualUnmodifiableListView) return _images;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<String>? _tags;
  @override
  List<String>? get tags {
    final value = _tags;
    if (value == null) return null;
    if (_tags is EqualUnmodifiableListView) return _tags;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<ProductVariant>? _variants;
  @override
  List<ProductVariant>? get variants {
    final value = _variants;
    if (value == null) return null;
    if (_variants is EqualUnmodifiableListView) return _variants;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final double? weight;
  @override
  final String? weightUnit;
  final Map<String, dynamic>? _nutritionalInfo;
  @override
  Map<String, dynamic>? get nutritionalInfo {
    final value = _nutritionalInfo;
    if (value == null) return null;
    if (_nutritionalInfo is EqualUnmodifiableMapView) return _nutritionalInfo;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  final Map<String, dynamic>? _customAttributes;
  @override
  Map<String, dynamic>? get customAttributes {
    final value = _customAttributes;
    if (value == null) return null;
    if (_customAttributes is EqualUnmodifiableMapView) return _customAttributes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(value);
  }

  @override
  String toString() {
    return 'UpdateProductRequest(name: $name, description: $description, category: $category, status: $status, pricing: $pricing, inventory: $inventory, sku: $sku, barcode: $barcode, images: $images, tags: $tags, variants: $variants, weight: $weight, weightUnit: $weightUnit, nutritionalInfo: $nutritionalInfo, customAttributes: $customAttributes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateProductRequestImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.category, category) ||
                other.category == category) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.pricing, pricing) || other.pricing == pricing) &&
            (identical(other.inventory, inventory) ||
                other.inventory == inventory) &&
            (identical(other.sku, sku) || other.sku == sku) &&
            (identical(other.barcode, barcode) || other.barcode == barcode) &&
            const DeepCollectionEquality().equals(other._images, _images) &&
            const DeepCollectionEquality().equals(other._tags, _tags) &&
            const DeepCollectionEquality().equals(other._variants, _variants) &&
            (identical(other.weight, weight) || other.weight == weight) &&
            (identical(other.weightUnit, weightUnit) ||
                other.weightUnit == weightUnit) &&
            const DeepCollectionEquality()
                .equals(other._nutritionalInfo, _nutritionalInfo) &&
            const DeepCollectionEquality()
                .equals(other._customAttributes, _customAttributes));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      name,
      description,
      category,
      status,
      pricing,
      inventory,
      sku,
      barcode,
      const DeepCollectionEquality().hash(_images),
      const DeepCollectionEquality().hash(_tags),
      const DeepCollectionEquality().hash(_variants),
      weight,
      weightUnit,
      const DeepCollectionEquality().hash(_nutritionalInfo),
      const DeepCollectionEquality().hash(_customAttributes));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateProductRequestImplCopyWith<_$UpdateProductRequestImpl>
      get copyWith =>
          __$$UpdateProductRequestImplCopyWithImpl<_$UpdateProductRequestImpl>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateProductRequestImplToJson(
      this,
    );
  }
}

abstract class _UpdateProductRequest implements UpdateProductRequest {
  const factory _UpdateProductRequest(
          {final String? name,
          final String? description,
          final ProductCategory? category,
          final ProductStatus? status,
          final ProductPricing? pricing,
          final ProductInventory? inventory,
          final String? sku,
          final String? barcode,
          final List<String>? images,
          final List<String>? tags,
          final List<ProductVariant>? variants,
          final double? weight,
          final String? weightUnit,
          final Map<String, dynamic>? nutritionalInfo,
          final Map<String, dynamic>? customAttributes}) =
      _$UpdateProductRequestImpl;

  factory _UpdateProductRequest.fromJson(Map<String, dynamic> json) =
      _$UpdateProductRequestImpl.fromJson;

  @override
  String? get name;
  @override
  String? get description;
  @override
  ProductCategory? get category;
  @override
  ProductStatus? get status;
  @override
  ProductPricing? get pricing;
  @override
  ProductInventory? get inventory;
  @override
  String? get sku;
  @override
  String? get barcode;
  @override
  List<String>? get images;
  @override
  List<String>? get tags;
  @override
  List<ProductVariant>? get variants;
  @override
  double? get weight;
  @override
  String? get weightUnit;
  @override
  Map<String, dynamic>? get nutritionalInfo;
  @override
  Map<String, dynamic>? get customAttributes;
  @override
  @JsonKey(ignore: true)
  _$$UpdateProductRequestImplCopyWith<_$UpdateProductRequestImpl>
      get copyWith => throw _privateConstructorUsedError;
}
