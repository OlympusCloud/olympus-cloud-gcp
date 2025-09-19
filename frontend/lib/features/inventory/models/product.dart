import 'package:freezed_annotation/freezed_annotation.dart';

part 'product.freezed.dart';
part 'product.g.dart';

/// Product category enumeration
enum ProductCategory {
  @JsonValue('food')
  food,
  @JsonValue('beverage')
  beverage,
  @JsonValue('retail')
  retail,
  @JsonValue('service')
  service,
  @JsonValue('digital')
  digital,
}

/// Product status enumeration
enum ProductStatus {
  @JsonValue('active')
  active,
  @JsonValue('inactive')
  inactive,
  @JsonValue('out_of_stock')
  outOfStock,
  @JsonValue('discontinued')
  discontinued,
}

/// Product variant model
@freezed
class ProductVariant with _$ProductVariant {
  const factory ProductVariant({
    required String id,
    required String name,
    required double price,
    String? sku,
    int? stockQuantity,
    Map<String, dynamic>? attributes,
  }) = _ProductVariant;

  factory ProductVariant.fromJson(Map<String, dynamic> json) => _$ProductVariantFromJson(json);
}

/// Product pricing information
@freezed
class ProductPricing with _$ProductPricing {
  const factory ProductPricing({
    required double basePrice,
    double? salePrice,
    double? cost,
    double? margin,
    DateTime? salePriceStart,
    DateTime? salePriceEnd,
    Map<String, double>? tierPricing,
  }) = _ProductPricing;

  factory ProductPricing.fromJson(Map<String, dynamic> json) => _$ProductPricingFromJson(json);
}

/// Product inventory information
@freezed
class ProductInventory with _$ProductInventory {
  const factory ProductInventory({
    required bool trackStock,
    int? currentStock,
    int? lowStockThreshold,
    int? reorderPoint,
    int? reorderQuantity,
    String? stockUnit,
    String? supplier,
    Map<String, dynamic>? locations,
  }) = _ProductInventory;

  factory ProductInventory.fromJson(Map<String, dynamic> json) => _$ProductInventoryFromJson(json);
}

/// Main Product model
@freezed
class Product with _$Product {
  const factory Product({
    required String id,
    required String name,
    required String description,
    required ProductCategory category,
    required ProductStatus status,
    required ProductPricing pricing,
    required ProductInventory inventory,
    String? sku,
    String? barcode,
    List<String>? images,
    List<String>? tags,
    List<ProductVariant>? variants,
    double? weight,
    String? weightUnit,
    Map<String, dynamic>? nutritionalInfo,
    Map<String, dynamic>? customAttributes,
    required DateTime createdAt,
    DateTime? updatedAt,
  }) = _Product;

  factory Product.fromJson(Map<String, dynamic> json) => _$ProductFromJson(json);
}

/// Product creation request
@freezed
class CreateProductRequest with _$CreateProductRequest {
  const factory CreateProductRequest({
    required String name,
    required String description,
    required ProductCategory category,
    required ProductPricing pricing,
    required ProductInventory inventory,
    String? sku,
    String? barcode,
    List<String>? images,
    List<String>? tags,
    List<ProductVariant>? variants,
    double? weight,
    String? weightUnit,
    Map<String, dynamic>? nutritionalInfo,
    Map<String, dynamic>? customAttributes,
  }) = _CreateProductRequest;

  factory CreateProductRequest.fromJson(Map<String, dynamic> json) => _$CreateProductRequestFromJson(json);
}

/// Product update request
@freezed
class UpdateProductRequest with _$UpdateProductRequest {
  const factory UpdateProductRequest({
    String? name,
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
    Map<String, dynamic>? customAttributes,
  }) = _UpdateProductRequest;

  factory UpdateProductRequest.fromJson(Map<String, dynamic> json) => _$UpdateProductRequestFromJson(json);
}