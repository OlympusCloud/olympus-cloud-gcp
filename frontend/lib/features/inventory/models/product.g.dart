// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$ProductVariantImpl _$$ProductVariantImplFromJson(Map<String, dynamic> json) =>
    _$ProductVariantImpl(
      id: json['id'] as String,
      name: json['name'] as String,
      price: (json['price'] as num).toDouble(),
      sku: json['sku'] as String?,
      stockQuantity: (json['stockQuantity'] as num?)?.toInt(),
      attributes: json['attributes'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$ProductVariantImplToJson(
        _$ProductVariantImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'price': instance.price,
      'sku': instance.sku,
      'stockQuantity': instance.stockQuantity,
      'attributes': instance.attributes,
    };

_$ProductPricingImpl _$$ProductPricingImplFromJson(Map<String, dynamic> json) =>
    _$ProductPricingImpl(
      basePrice: (json['basePrice'] as num).toDouble(),
      salePrice: (json['salePrice'] as num?)?.toDouble(),
      cost: (json['cost'] as num?)?.toDouble(),
      margin: (json['margin'] as num?)?.toDouble(),
      salePriceStart: json['salePriceStart'] == null
          ? null
          : DateTime.parse(json['salePriceStart'] as String),
      salePriceEnd: json['salePriceEnd'] == null
          ? null
          : DateTime.parse(json['salePriceEnd'] as String),
      tierPricing: (json['tierPricing'] as Map<String, dynamic>?)?.map(
        (k, e) => MapEntry(k, (e as num).toDouble()),
      ),
    );

Map<String, dynamic> _$$ProductPricingImplToJson(
        _$ProductPricingImpl instance) =>
    <String, dynamic>{
      'basePrice': instance.basePrice,
      'salePrice': instance.salePrice,
      'cost': instance.cost,
      'margin': instance.margin,
      'salePriceStart': instance.salePriceStart?.toIso8601String(),
      'salePriceEnd': instance.salePriceEnd?.toIso8601String(),
      'tierPricing': instance.tierPricing,
    };

_$ProductInventoryImpl _$$ProductInventoryImplFromJson(
        Map<String, dynamic> json) =>
    _$ProductInventoryImpl(
      trackStock: json['trackStock'] as bool,
      currentStock: (json['currentStock'] as num?)?.toInt(),
      lowStockThreshold: (json['lowStockThreshold'] as num?)?.toInt(),
      reorderPoint: (json['reorderPoint'] as num?)?.toInt(),
      reorderQuantity: (json['reorderQuantity'] as num?)?.toInt(),
      stockUnit: json['stockUnit'] as String?,
      supplier: json['supplier'] as String?,
      locations: json['locations'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$ProductInventoryImplToJson(
        _$ProductInventoryImpl instance) =>
    <String, dynamic>{
      'trackStock': instance.trackStock,
      'currentStock': instance.currentStock,
      'lowStockThreshold': instance.lowStockThreshold,
      'reorderPoint': instance.reorderPoint,
      'reorderQuantity': instance.reorderQuantity,
      'stockUnit': instance.stockUnit,
      'supplier': instance.supplier,
      'locations': instance.locations,
    };

_$ProductImpl _$$ProductImplFromJson(Map<String, dynamic> json) =>
    _$ProductImpl(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String,
      category: $enumDecode(_$ProductCategoryEnumMap, json['category']),
      status: $enumDecode(_$ProductStatusEnumMap, json['status']),
      pricing: ProductPricing.fromJson(json['pricing'] as Map<String, dynamic>),
      inventory:
          ProductInventory.fromJson(json['inventory'] as Map<String, dynamic>),
      sku: json['sku'] as String?,
      barcode: json['barcode'] as String?,
      images:
          (json['images'] as List<dynamic>?)?.map((e) => e as String).toList(),
      tags: (json['tags'] as List<dynamic>?)?.map((e) => e as String).toList(),
      variants: (json['variants'] as List<dynamic>?)
          ?.map((e) => ProductVariant.fromJson(e as Map<String, dynamic>))
          .toList(),
      weight: (json['weight'] as num?)?.toDouble(),
      weightUnit: json['weightUnit'] as String?,
      nutritionalInfo: json['nutritionalInfo'] as Map<String, dynamic>?,
      customAttributes: json['customAttributes'] as Map<String, dynamic>?,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: json['updatedAt'] == null
          ? null
          : DateTime.parse(json['updatedAt'] as String),
    );

Map<String, dynamic> _$$ProductImplToJson(_$ProductImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'description': instance.description,
      'category': _$ProductCategoryEnumMap[instance.category]!,
      'status': _$ProductStatusEnumMap[instance.status]!,
      'pricing': instance.pricing,
      'inventory': instance.inventory,
      'sku': instance.sku,
      'barcode': instance.barcode,
      'images': instance.images,
      'tags': instance.tags,
      'variants': instance.variants,
      'weight': instance.weight,
      'weightUnit': instance.weightUnit,
      'nutritionalInfo': instance.nutritionalInfo,
      'customAttributes': instance.customAttributes,
      'createdAt': instance.createdAt.toIso8601String(),
      'updatedAt': instance.updatedAt?.toIso8601String(),
    };

const _$ProductCategoryEnumMap = {
  ProductCategory.food: 'food',
  ProductCategory.beverage: 'beverage',
  ProductCategory.retail: 'retail',
  ProductCategory.service: 'service',
  ProductCategory.digital: 'digital',
};

const _$ProductStatusEnumMap = {
  ProductStatus.active: 'active',
  ProductStatus.inactive: 'inactive',
  ProductStatus.outOfStock: 'out_of_stock',
  ProductStatus.discontinued: 'discontinued',
};

_$CreateProductRequestImpl _$$CreateProductRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$CreateProductRequestImpl(
      name: json['name'] as String,
      description: json['description'] as String,
      category: $enumDecode(_$ProductCategoryEnumMap, json['category']),
      pricing: ProductPricing.fromJson(json['pricing'] as Map<String, dynamic>),
      inventory:
          ProductInventory.fromJson(json['inventory'] as Map<String, dynamic>),
      sku: json['sku'] as String?,
      barcode: json['barcode'] as String?,
      images:
          (json['images'] as List<dynamic>?)?.map((e) => e as String).toList(),
      tags: (json['tags'] as List<dynamic>?)?.map((e) => e as String).toList(),
      variants: (json['variants'] as List<dynamic>?)
          ?.map((e) => ProductVariant.fromJson(e as Map<String, dynamic>))
          .toList(),
      weight: (json['weight'] as num?)?.toDouble(),
      weightUnit: json['weightUnit'] as String?,
      nutritionalInfo: json['nutritionalInfo'] as Map<String, dynamic>?,
      customAttributes: json['customAttributes'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$CreateProductRequestImplToJson(
        _$CreateProductRequestImpl instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'category': _$ProductCategoryEnumMap[instance.category]!,
      'pricing': instance.pricing,
      'inventory': instance.inventory,
      'sku': instance.sku,
      'barcode': instance.barcode,
      'images': instance.images,
      'tags': instance.tags,
      'variants': instance.variants,
      'weight': instance.weight,
      'weightUnit': instance.weightUnit,
      'nutritionalInfo': instance.nutritionalInfo,
      'customAttributes': instance.customAttributes,
    };

_$UpdateProductRequestImpl _$$UpdateProductRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$UpdateProductRequestImpl(
      name: json['name'] as String?,
      description: json['description'] as String?,
      category: $enumDecodeNullable(_$ProductCategoryEnumMap, json['category']),
      status: $enumDecodeNullable(_$ProductStatusEnumMap, json['status']),
      pricing: json['pricing'] == null
          ? null
          : ProductPricing.fromJson(json['pricing'] as Map<String, dynamic>),
      inventory: json['inventory'] == null
          ? null
          : ProductInventory.fromJson(
              json['inventory'] as Map<String, dynamic>),
      sku: json['sku'] as String?,
      barcode: json['barcode'] as String?,
      images:
          (json['images'] as List<dynamic>?)?.map((e) => e as String).toList(),
      tags: (json['tags'] as List<dynamic>?)?.map((e) => e as String).toList(),
      variants: (json['variants'] as List<dynamic>?)
          ?.map((e) => ProductVariant.fromJson(e as Map<String, dynamic>))
          .toList(),
      weight: (json['weight'] as num?)?.toDouble(),
      weightUnit: json['weightUnit'] as String?,
      nutritionalInfo: json['nutritionalInfo'] as Map<String, dynamic>?,
      customAttributes: json['customAttributes'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$UpdateProductRequestImplToJson(
        _$UpdateProductRequestImpl instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'category': _$ProductCategoryEnumMap[instance.category],
      'status': _$ProductStatusEnumMap[instance.status],
      'pricing': instance.pricing,
      'inventory': instance.inventory,
      'sku': instance.sku,
      'barcode': instance.barcode,
      'images': instance.images,
      'tags': instance.tags,
      'variants': instance.variants,
      'weight': instance.weight,
      'weightUnit': instance.weightUnit,
      'nutritionalInfo': instance.nutritionalInfo,
      'customAttributes': instance.customAttributes,
    };
