import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/product.dart';
import '../../../core/services/api_service.dart';

/// Products state provider
final productsProvider = StateNotifierProvider<ProductsNotifier, AsyncValue<List<Product>>>((ref) {
  return ProductsNotifier();
});

/// Product details provider for specific product
final productDetailsProvider = FutureProvider.family<Product, String>((ref, productId) async {
  final response = await ApiService.get('/products/$productId');
  return Product.fromJson(response.data);
});

/// Active products provider (not discontinued or inactive)
final activeProductsProvider = Provider<List<Product>>((ref) {
  final productsAsync = ref.watch(productsProvider);
  return productsAsync.maybeWhen(
    data: (products) => products.where((product) => 
      product.status == ProductStatus.active
    ).toList(),
    orElse: () => [],
  );
});

/// Low stock products provider
final lowStockProductsProvider = Provider<List<Product>>((ref) {
  final productsAsync = ref.watch(productsProvider);
  return productsAsync.maybeWhen(
    data: (products) => products.where((product) {
      if (!product.inventory.trackStock || 
          product.inventory.currentStock == null ||
          product.inventory.lowStockThreshold == null) {
        return false;
      }
      return product.inventory.currentStock! <= product.inventory.lowStockThreshold!;
    }).toList(),
    orElse: () => [],
  );
});

/// Out of stock products provider
final outOfStockProductsProvider = Provider<List<Product>>((ref) {
  final productsAsync = ref.watch(productsProvider);
  return productsAsync.maybeWhen(
    data: (products) => products.where((product) => 
      product.status == ProductStatus.outOfStock ||
      (product.inventory.trackStock && 
       product.inventory.currentStock != null &&
       product.inventory.currentStock! <= 0)
    ).toList(),
    orElse: () => [],
  );
});

/// Products by category provider
final productsByCategoryProvider = Provider<Map<ProductCategory, List<Product>>>((ref) {
  final productsAsync = ref.watch(productsProvider);
  return productsAsync.maybeWhen(
    data: (products) {
      final Map<ProductCategory, List<Product>> productsByCategory = {};
      for (final category in ProductCategory.values) {
        productsByCategory[category] = products
            .where((product) => product.category == category)
            .toList();
      }
      return productsByCategory;
    },
    orElse: () => {},
  );
});

/// Total inventory value provider
final inventoryValueProvider = Provider<double>((ref) {
  final productsAsync = ref.watch(productsProvider);
  return productsAsync.maybeWhen(
    data: (products) => products.fold(0.0, (sum, product) {
      if (product.inventory.trackStock && 
          product.inventory.currentStock != null &&
          product.pricing.cost != null) {
        return sum + (product.inventory.currentStock! * product.pricing.cost!);
      }
      return sum;
    }),
    orElse: () => 0.0,
  );
});

/// Products notifier
class ProductsNotifier extends StateNotifier<AsyncValue<List<Product>>> {
  ProductsNotifier() : super(const AsyncValue.loading()) {
    loadProducts();
  }

  /// Load all products
  Future<void> loadProducts() async {
    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/products');
      final products = (response.data['products'] as List)
          .map((productJson) => Product.fromJson(productJson))
          .toList();
      
      state = AsyncValue.data(products);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Create new product
  Future<Product?> createProduct(CreateProductRequest request) async {
    try {
      final response = await ApiService.post('/products', data: request.toJson());
      final newProduct = Product.fromJson(response.data);
      
      // Add to current state
      state.whenData((products) {
        state = AsyncValue.data([newProduct, ...products]);
      });
      
      return newProduct;
    } catch (error) {
      rethrow;
    }
  }

  /// Update existing product
  Future<void> updateProduct(String productId, UpdateProductRequest request) async {
    try {
      final response = await ApiService.put('/products/$productId', data: request.toJson());
      final updatedProduct = Product.fromJson(response.data);
      
      // Update in current state
      state.whenData((products) {
        final updatedProducts = products.map((product) => 
          product.id == productId ? updatedProduct : product
        ).toList();
        state = AsyncValue.data(updatedProducts);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Update product status
  Future<void> updateProductStatus(String productId, ProductStatus newStatus) async {
    await updateProduct(productId, UpdateProductRequest(status: newStatus));
  }

  /// Update product stock
  Future<void> updateStock(String productId, int newStock, {String? reason}) async {
    try {
      await ApiService.post('/products/$productId/stock', data: {
        'quantity': newStock,
        'reason': reason ?? 'Manual adjustment',
      });
      
      // Update local state
      state.whenData((products) {
        final updatedProducts = products.map((product) {
          if (product.id == productId) {
            return product.copyWith(
              inventory: product.inventory.copyWith(currentStock: newStock),
            );
          }
          return product;
        }).toList();
        state = AsyncValue.data(updatedProducts);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Adjust stock (add or subtract)
  Future<void> adjustStock(String productId, int adjustment, {String? reason}) async {
    try {
      await ApiService.post('/products/$productId/stock/adjust', data: {
        'adjustment': adjustment,
        'reason': reason ?? 'Stock adjustment',
      });
      
      // Update local state
      state.whenData((products) {
        final updatedProducts = products.map((product) {
          if (product.id == productId && product.inventory.currentStock != null) {
            final newStock = product.inventory.currentStock! + adjustment;
            return product.copyWith(
              inventory: product.inventory.copyWith(currentStock: newStock),
            );
          }
          return product;
        }).toList();
        state = AsyncValue.data(updatedProducts);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Delete product
  Future<void> deleteProduct(String productId) async {
    try {
      await ApiService.delete('/products/$productId');
      
      // Remove from current state
      state.whenData((products) {
        final updatedProducts = products.where((product) => product.id != productId).toList();
        state = AsyncValue.data(updatedProducts);
      });
    } catch (error) {
      rethrow;
    }
  }

  /// Search products
  Future<void> searchProducts(String query) async {
    if (query.trim().isEmpty) {
      await loadProducts();
      return;
    }

    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/products/search', queryParameters: {
        'q': query,
      });
      
      final products = (response.data['products'] as List)
          .map((productJson) => Product.fromJson(productJson))
          .toList();
      
      state = AsyncValue.data(products);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Filter products by category
  Future<void> filterProductsByCategory(ProductCategory category) async {
    state = const AsyncValue.loading();
    
    try {
      final response = await ApiService.get('/products', queryParameters: {
        'category': category.name,
      });
      
      final products = (response.data['products'] as List)
          .map((productJson) => Product.fromJson(productJson))
          .toList();
      
      state = AsyncValue.data(products);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Filter products by status
  Future<void> filterProductsByStatus(List<ProductStatus> statuses) async {
    state = const AsyncValue.loading();
    
    try {
      final statusStrings = statuses.map((s) => s.name).toList();
      final response = await ApiService.get('/products', queryParameters: {
        'status': statusStrings.join(','),
      });
      
      final products = (response.data['products'] as List)
          .map((productJson) => Product.fromJson(productJson))
          .toList();
      
      state = AsyncValue.data(products);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  /// Refresh products
  Future<void> refresh() async {
    await loadProducts();
  }

  /// Bulk update products
  Future<void> bulkUpdateProducts(List<String> productIds, UpdateProductRequest request) async {
    try {
      await ApiService.post('/products/bulk-update', data: {
        'product_ids': productIds,
        'updates': request.toJson(),
      });
      
      // Refresh to get updated data
      await loadProducts();
    } catch (error) {
      rethrow;
    }
  }

  /// Import products from CSV
  Future<void> importProductsFromCsv(String csvData) async {
    try {
      await ApiService.post('/products/import', data: {
        'csv_data': csvData,
      });
      
      // Refresh to get new products
      await loadProducts();
    } catch (error) {
      rethrow;
    }
  }
}