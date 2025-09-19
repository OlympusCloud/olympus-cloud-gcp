import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../models/product.dart';
import '../../providers/products_provider.dart';
import '../../../../shared/widgets/loading_widgets.dart';

/// Inventory management screen with products list, stock management, and CRUD operations
class InventoryManagementScreen extends ConsumerStatefulWidget {
  const InventoryManagementScreen({super.key});

  @override
  ConsumerState<InventoryManagementScreen> createState() => _InventoryManagementScreenState();
}

class _InventoryManagementScreenState extends ConsumerState<InventoryManagementScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  String _searchQuery = '';
  ProductCategory? _selectedCategory;
  ProductStatus? _selectedStatus;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final productsAsync = ref.watch(productsProvider);
    final activeProducts = ref.watch(activeProductsProvider);
    final lowStockProducts = ref.watch(lowStockProductsProvider);
    final outOfStockProducts = ref.watch(outOfStockProductsProvider);
    final inventoryValue = ref.watch(inventoryValueProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Inventory Management'),
        elevation: 0,
        backgroundColor: theme.colorScheme.surface,
        foregroundColor: theme.colorScheme.onSurface,
        bottom: TabBar(
          controller: _tabController,
          indicatorColor: theme.colorScheme.primary,
          labelColor: theme.colorScheme.primary,
          unselectedLabelColor: theme.colorScheme.onSurface.withValues(alpha: 0.6),
          tabs: [
            Tab(
              text: 'All (${activeProducts.length})',
              icon: const Icon(Icons.inventory_2),
            ),
            Tab(
              text: 'Low Stock (${lowStockProducts.length})',
              icon: const Icon(Icons.warning),
            ),
            Tab(
              text: 'Out of Stock (${outOfStockProducts.length})',
              icon: const Icon(Icons.error),
            ),
            Tab(
              text: 'Categories',
              icon: const Icon(Icons.category),
            ),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: _showFilterDialog,
          ),
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () => ref.read(productsProvider.notifier).refresh(),
          ),
          PopupMenuButton<String>(
            onSelected: _handleMenuAction,
            itemBuilder: (context) => [
              const PopupMenuItem(
                value: 'import',
                child: Row(
                  children: [
                    Icon(Icons.upload),
                    SizedBox(width: 8),
                    Text('Import Products'),
                  ],
                ),
              ),
              const PopupMenuItem(
                value: 'export',
                child: Row(
                  children: [
                    Icon(Icons.download),
                    SizedBox(width: 8),
                    Text('Export Data'),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
      body: Column(
        children: [
          // Search and Metrics Bar
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: theme.colorScheme.surface,
              boxShadow: [
                BoxShadow(
                  color: theme.shadowColor.withValues(alpha: 0.1),
                  blurRadius: 4,
                  offset: const Offset(0, 2),
                ),
              ],
            ),
            child: Column(
              children: [
                // Search Bar
                TextField(
                  decoration: InputDecoration(
                    hintText: 'Search products by name, SKU, or category...',
                    prefixIcon: const Icon(Icons.search),
                    suffixIcon: _searchQuery.isNotEmpty
                        ? IconButton(
                            icon: const Icon(Icons.clear),
                            onPressed: () {
                              setState(() => _searchQuery = '');
                              ref.read(productsProvider.notifier).loadProducts();
                            },
                          )
                        : null,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                      borderSide: BorderSide.none,
                    ),
                    filled: true,
                    fillColor: theme.colorScheme.surfaceContainerHighest,
                  ),
                  onChanged: (value) {
                    setState(() => _searchQuery = value);
                    if (value.isNotEmpty) {
                      ref.read(productsProvider.notifier).searchProducts(value);
                    } else {
                      ref.read(productsProvider.notifier).loadProducts();
                    }
                  },
                ),
                const SizedBox(height: 12),
                // Quick Metrics
                Row(
                  children: [
                    Expanded(
                      child: _buildMetricCard(
                        'Total Value',
                        '\$${inventoryValue.toStringAsFixed(2)}',
                        Icons.attach_money,
                        theme.colorScheme.primary,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: _buildMetricCard(
                        'Products',
                        '${activeProducts.length}',
                        Icons.inventory,
                        Colors.blue,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: _buildMetricCard(
                        'Low Stock',
                        '${lowStockProducts.length}',
                        Icons.warning,
                        Colors.orange,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: _buildMetricCard(
                        'Out of Stock',
                        '${outOfStockProducts.length}',
                        Icons.error,
                        Colors.red,
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
          // Products List
          Expanded(
            child: TabBarView(
              controller: _tabController,
              children: [
                _buildProductsList(activeProducts, 'No active products'),
                _buildProductsList(lowStockProducts, 'No low stock products'),
                _buildProductsList(outOfStockProducts, 'No out of stock products'),
                _buildCategoriesView(),
              ],
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _createNewProduct,
        icon: const Icon(Icons.add),
        label: const Text('Add Product'),
      ),
    );
  }

  Widget _buildMetricCard(String title, String value, IconData icon, Color color) {
    final theme = Theme.of(context);
    
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: color.withValues(alpha: 0.3)),
      ),
      child: Column(
        children: [
          Icon(icon, color: color, size: 20),
          const SizedBox(height: 4),
          Text(
            value,
            style: theme.textTheme.titleMedium?.copyWith(
              fontWeight: FontWeight.bold,
              color: color,
            ),
          ),
          Text(
            title,
            style: theme.textTheme.bodySmall?.copyWith(
              color: theme.colorScheme.onSurface.withValues(alpha: 0.7),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildProductsList(List<Product> products, String emptyMessage) {
    if (products.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.inventory_2_outlined,
              size: 64,
              color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5),
            ),
            const SizedBox(height: 16),
            Text(
              emptyMessage,
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
              ),
            ),
          ],
        ),
      );
    }

    return RefreshIndicator(
      onRefresh: () => ref.read(productsProvider.notifier).refresh(),
      child: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: products.length,
        itemBuilder: (context, index) {
          final product = products[index];
          return _buildProductCard(product);
        },
      ),
    );
  }

  Widget _buildProductCard(Product product) {
    final theme = Theme.of(context);
    final statusColor = _getStatusColor(product.status);
    final categoryColor = _getCategoryColor(product.category);
    
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 2,
      child: InkWell(
        onTap: () => _showProductDetails(product),
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header with name and status
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Expanded(
                    child: Text(
                      product.name,
                      style: theme.textTheme.titleMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  Row(
                    children: [
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: categoryColor.withValues(alpha: 0.1),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(color: categoryColor),
                        ),
                        child: Text(
                          product.category.name.toUpperCase(),
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: categoryColor,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: statusColor.withValues(alpha: 0.1),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(color: statusColor),
                        ),
                        child: Text(
                          product.status.name.toUpperCase(),
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: statusColor,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 8),
              // SKU and description
              if (product.sku != null) ...[
                Row(
                  children: [
                    Icon(
                      Icons.qr_code,
                      size: 16,
                      color: theme.colorScheme.onSurface.withValues(alpha: 0.6),
                    ),
                    const SizedBox(width: 4),
                    Text(
                      'SKU: ${product.sku}',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.onSurface.withValues(alpha: 0.8),
                        fontFamily: 'monospace',
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
              ],
              Text(
                product.description,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: theme.colorScheme.onSurface.withValues(alpha: 0.7),
                ),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
              const SizedBox(height: 8),
              // Price and stock info
              Row(
                children: [
                  Text(
                    '\$${product.pricing.basePrice.toStringAsFixed(2)}',
                    style: theme.textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.bold,
                      color: theme.colorScheme.primary,
                    ),
                  ),
                  if (product.pricing.salePrice != null) ...[
                    const SizedBox(width: 8),
                    Text(
                      '\$${product.pricing.salePrice!.toStringAsFixed(2)}',
                      style: theme.textTheme.bodyMedium?.copyWith(
                        decoration: TextDecoration.lineThrough,
                        color: theme.colorScheme.onSurface.withValues(alpha: 0.5),
                      ),
                    ),
                  ],
                  const Spacer(),
                  if (product.inventory.trackStock && product.inventory.currentStock != null) ...[
                    Icon(
                      Icons.inventory,
                      size: 16,
                      color: _getStockColor(product),
                    ),
                    const SizedBox(width: 4),
                    Text(
                      '${product.inventory.currentStock} ${product.inventory.stockUnit ?? 'units'}',
                      style: theme.textTheme.bodyMedium?.copyWith(
                        color: _getStockColor(product),
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ],
              ),
              // Quick actions
              const SizedBox(height: 12),
              Row(
                children: [
                  Expanded(
                    child: _buildQuickActionButton(
                      'View',
                      Icons.visibility,
                      () => _showProductDetails(product),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: _buildQuickActionButton(
                      'Edit',
                      Icons.edit,
                      () => _editProduct(product),
                    ),
                  ),
                  const SizedBox(width: 8),
                  if (product.inventory.trackStock)
                    Expanded(
                      child: _buildQuickActionButton(
                        'Stock',
                        Icons.add_circle,
                        () => _adjustStock(product),
                      ),
                    ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildQuickActionButton(String label, IconData icon, VoidCallback onPressed) {
    final theme = Theme.of(context);
    return OutlinedButton.icon(
      onPressed: onPressed,
      icon: Icon(icon, size: 16),
      label: Text(label),
      style: OutlinedButton.styleFrom(
        padding: const EdgeInsets.symmetric(vertical: 8),
        side: BorderSide(color: theme.colorScheme.outline),
      ),
    );
  }

  Widget _buildCategoriesView() {
    final productsByCategory = ref.watch(productsByCategoryProvider);
    
    if (productsByCategory.isEmpty) {
      return const Center(
        child: Text('No products found'),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: ProductCategory.values.length,
      itemBuilder: (context, index) {
        final category = ProductCategory.values[index];
        final products = productsByCategory[category] ?? [];
        
        if (products.isEmpty) return const SizedBox.shrink();
        
        return _buildCategorySection(category, products);
      },
    );
  }

  Widget _buildCategorySection(ProductCategory category, List<Product> products) {
    final theme = Theme.of(context);
    final categoryColor = _getCategoryColor(category);
    
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      child: ExpansionTile(
        title: Row(
          children: [
            Icon(
              _getCategoryIcon(category),
              color: categoryColor,
              size: 20,
            ),
            const SizedBox(width: 8),
            Text(
              category.name.toUpperCase(),
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
                color: categoryColor,
              ),
            ),
            const Spacer(),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: categoryColor.withValues(alpha: 0.1),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                '${products.length}',
                style: theme.textTheme.labelMedium?.copyWith(
                  color: categoryColor,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
        ),
        children: products.map((product) => ListTile(
          title: Text(product.name),
          subtitle: Text('\$${product.pricing.basePrice.toStringAsFixed(2)}'),
          trailing: product.inventory.trackStock && product.inventory.currentStock != null
              ? Text(
                  '${product.inventory.currentStock}',
                  style: TextStyle(
                    color: _getStockColor(product),
                    fontWeight: FontWeight.bold,
                  ),
                )
              : null,
          onTap: () => _showProductDetails(product),
        )).toList(),
      ),
    );
  }

  Color _getStatusColor(ProductStatus status) {
    switch (status) {
      case ProductStatus.active:
        return Colors.green;
      case ProductStatus.inactive:
        return Colors.grey;
      case ProductStatus.outOfStock:
        return Colors.red;
      case ProductStatus.discontinued:
        return Colors.brown;
    }
  }

  Color _getCategoryColor(ProductCategory category) {
    switch (category) {
      case ProductCategory.food:
        return Colors.orange;
      case ProductCategory.beverage:
        return Colors.blue;
      case ProductCategory.retail:
        return Colors.purple;
      case ProductCategory.service:
        return Colors.teal;
      case ProductCategory.digital:
        return Colors.indigo;
    }
  }

  IconData _getCategoryIcon(ProductCategory category) {
    switch (category) {
      case ProductCategory.food:
        return Icons.restaurant;
      case ProductCategory.beverage:
        return Icons.local_drink;
      case ProductCategory.retail:
        return Icons.shopping_bag;
      case ProductCategory.service:
        return Icons.build;
      case ProductCategory.digital:
        return Icons.computer;
    }
  }

  Color _getStockColor(Product product) {
    if (!product.inventory.trackStock || product.inventory.currentStock == null) {
      return Colors.grey;
    }
    
    final stock = product.inventory.currentStock!;
    final lowThreshold = product.inventory.lowStockThreshold ?? 10;
    
    if (stock <= 0) return Colors.red;
    if (stock <= lowThreshold) return Colors.orange;
    return Colors.green;
  }

  void _showProductDetails(Product product) {
    showDialog(
      context: context,
      builder: (context) => ProductDetailsDialog(product: product),
    );
  }

  void _editProduct(Product product) {
    showDialog(
      context: context,
      builder: (context) => EditProductDialog(product: product),
    );
  }

  void _adjustStock(Product product) {
    showDialog(
      context: context,
      builder: (context) => StockAdjustmentDialog(product: product),
    );
  }

  void _createNewProduct() {
    showDialog(
      context: context,
      builder: (context) => const CreateProductDialog(),
    );
  }

  void _showFilterDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Filter Products'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Category'),
            const SizedBox(height: 8),
            DropdownButtonFormField<ProductCategory?>(
              value: _selectedCategory,
              decoration: const InputDecoration(
                border: OutlineInputBorder(),
                contentPadding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
              ),
              items: [
                const DropdownMenuItem<ProductCategory?>(
                  value: null,
                  child: Text('All Categories'),
                ),
                ...ProductCategory.values.map((category) => DropdownMenuItem(
                  value: category,
                  child: Text(category.name.toUpperCase()),
                )),
              ],
              onChanged: (value) => setState(() => _selectedCategory = value),
            ),
            const SizedBox(height: 16),
            const Text('Status'),
            const SizedBox(height: 8),
            DropdownButtonFormField<ProductStatus?>(
              value: _selectedStatus,
              decoration: const InputDecoration(
                border: OutlineInputBorder(),
                contentPadding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
              ),
              items: [
                const DropdownMenuItem<ProductStatus?>(
                  value: null,
                  child: Text('All Statuses'),
                ),
                ...ProductStatus.values.map((status) => DropdownMenuItem(
                  value: status,
                  child: Text(status.name.toUpperCase()),
                )),
              ],
              onChanged: (value) => setState(() => _selectedStatus = value),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.of(context).pop();
              _applyFilters();
            },
            child: const Text('Apply'),
          ),
        ],
      ),
    );
  }

  void _applyFilters() {
    // Apply category filter
    if (_selectedCategory != null) {
      ref.read(productsProvider.notifier).filterProductsByCategory(_selectedCategory!);
    }
    
    // Apply status filter
    if (_selectedStatus != null) {
      ref.read(productsProvider.notifier).filterProductsByStatus([_selectedStatus!]);
    }
    
    // If no filters, load all products
    if (_selectedCategory == null && _selectedStatus == null) {
      ref.read(productsProvider.notifier).loadProducts();
    }
  }

  void _handleMenuAction(String action) {
    switch (action) {
      case 'import':
        _showImportDialog();
        break;
      case 'export':
        _exportData();
        break;
    }
  }

  void _showImportDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Import Products'),
        content: const Text('Import functionality will be implemented here'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Import'),
          ),
        ],
      ),
    );
  }

  void _exportData() {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Export functionality will be implemented'),
      ),
    );
  }
}

/// Product details dialog showing complete product information
class ProductDetailsDialog extends StatelessWidget {
  final Product product;

  const ProductDetailsDialog({super.key, required this.product});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Dialog(
      child: Container(
        width: 600,
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  product.name,
                  style: theme.textTheme.headlineSmall?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                IconButton(
                  onPressed: () => Navigator.of(context).pop(),
                  icon: const Icon(Icons.close),
                ),
              ],
            ),
            const SizedBox(height: 16),
            // Status and category
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                  decoration: BoxDecoration(
                    color: _getStatusColor(product.status).withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: _getStatusColor(product.status)),
                  ),
                  child: Text(
                    product.status.name.toUpperCase(),
                    style: theme.textTheme.labelMedium?.copyWith(
                      color: _getStatusColor(product.status),
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 16),
                Text(
                  'Category: ${product.category.name.toUpperCase()}',
                  style: theme.textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            // Product info
            Text(
              'Description',
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(product.description),
            if (product.sku != null) ...[
              const SizedBox(height: 16),
              Text('SKU: ${product.sku}'),
            ],
            const SizedBox(height: 16),
            // Pricing
            Text(
              'Pricing',
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text('Base Price: \$${product.pricing.basePrice.toStringAsFixed(2)}'),
            if (product.pricing.salePrice != null)
              Text('Sale Price: \$${product.pricing.salePrice!.toStringAsFixed(2)}'),
            if (product.pricing.cost != null)
              Text('Cost: \$${product.pricing.cost!.toStringAsFixed(2)}'),
            const SizedBox(height: 16),
            // Inventory
            if (product.inventory.trackStock) ...[
              Text(
                'Inventory',
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 8),
              if (product.inventory.currentStock != null)
                Text('Current Stock: ${product.inventory.currentStock} ${product.inventory.stockUnit ?? 'units'}'),
              if (product.inventory.lowStockThreshold != null)
                Text('Low Stock Threshold: ${product.inventory.lowStockThreshold}'),
              if (product.inventory.reorderPoint != null)
                Text('Reorder Point: ${product.inventory.reorderPoint}'),
              const SizedBox(height: 16),
            ],
            // Actions
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton(
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Text('Close'),
                ),
                const SizedBox(width: 8),
                ElevatedButton.icon(
                  onPressed: () {
                    Navigator.of(context).pop();
                    // TODO: Navigate to edit product
                  },
                  icon: const Icon(Icons.edit),
                  label: const Text('Edit Product'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Color _getStatusColor(ProductStatus status) {
    switch (status) {
      case ProductStatus.active:
        return Colors.green;
      case ProductStatus.inactive:
        return Colors.grey;
      case ProductStatus.outOfStock:
        return Colors.red;
      case ProductStatus.discontinued:
        return Colors.brown;
    }
  }
}

/// Create new product dialog
class CreateProductDialog extends ConsumerStatefulWidget {
  const CreateProductDialog({super.key});

  @override
  ConsumerState<CreateProductDialog> createState() => _CreateProductDialogState();
}

class _CreateProductDialogState extends ConsumerState<CreateProductDialog> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _skuController = TextEditingController();
  final _priceController = TextEditingController();
  final _costController = TextEditingController();
  final _stockController = TextEditingController();
  final _lowStockController = TextEditingController();
  
  ProductCategory _category = ProductCategory.food;
  bool _trackStock = true;

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    _skuController.dispose();
    _priceController.dispose();
    _costController.dispose();
    _stockController.dispose();
    _lowStockController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Dialog(
      child: Container(
        width: 600,
        height: 700,
        padding: const EdgeInsets.all(24),
        child: Form(
          key: _formKey,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'Create New Product',
                    style: theme.textTheme.headlineSmall?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  IconButton(
                    onPressed: () => Navigator.of(context).pop(),
                    icon: const Icon(Icons.close),
                  ),
                ],
              ),
              const SizedBox(height: 20),
              Expanded(
                child: SingleChildScrollView(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Basic Information
                      Text(
                        'Basic Information',
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextFormField(
                        controller: _nameController,
                        decoration: const InputDecoration(
                          labelText: 'Product Name *',
                          border: OutlineInputBorder(),
                        ),
                        validator: (value) {
                          if (value == null || value.isEmpty) {
                            return 'Please enter product name';
                          }
                          return null;
                        },
                      ),
                      const SizedBox(height: 12),
                      TextFormField(
                        controller: _descriptionController,
                        decoration: const InputDecoration(
                          labelText: 'Description *',
                          border: OutlineInputBorder(),
                        ),
                        maxLines: 3,
                        validator: (value) {
                          if (value == null || value.isEmpty) {
                            return 'Please enter description';
                          }
                          return null;
                        },
                      ),
                      const SizedBox(height: 12),
                      Row(
                        children: [
                          Expanded(
                            child: TextFormField(
                              controller: _skuController,
                              decoration: const InputDecoration(
                                labelText: 'SKU (optional)',
                                border: OutlineInputBorder(),
                              ),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: DropdownButtonFormField<ProductCategory>(
                              value: _category,
                              decoration: const InputDecoration(
                                labelText: 'Category',
                                border: OutlineInputBorder(),
                              ),
                              items: ProductCategory.values.map((category) => 
                                DropdownMenuItem(
                                  value: category,
                                  child: Text(category.name.toUpperCase()),
                                ),
                              ).toList(),
                              onChanged: (value) {
                                if (value != null) {
                                  setState(() => _category = value);
                                }
                              },
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 20),
                      // Pricing
                      Text(
                        'Pricing',
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 12),
                      Row(
                        children: [
                          Expanded(
                            child: TextFormField(
                              controller: _priceController,
                              decoration: const InputDecoration(
                                labelText: 'Base Price *',
                                prefixText: '\$',
                                border: OutlineInputBorder(),
                              ),
                              keyboardType: TextInputType.number,
                              validator: (value) {
                                if (value == null || value.isEmpty) {
                                  return 'Please enter price';
                                }
                                if (double.tryParse(value) == null) {
                                  return 'Please enter valid price';
                                }
                                return null;
                              },
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: TextFormField(
                              controller: _costController,
                              decoration: const InputDecoration(
                                labelText: 'Cost (optional)',
                                prefixText: '\$',
                                border: OutlineInputBorder(),
                              ),
                              keyboardType: TextInputType.number,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 20),
                      // Inventory
                      Text(
                        'Inventory',
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 12),
                      CheckboxListTile(
                        title: const Text('Track Stock'),
                        value: _trackStock,
                        onChanged: (value) => setState(() => _trackStock = value ?? false),
                      ),
                      if (_trackStock) ...[
                        const SizedBox(height: 12),
                        Row(
                          children: [
                            Expanded(
                              child: TextFormField(
                                controller: _stockController,
                                decoration: const InputDecoration(
                                  labelText: 'Current Stock',
                                  border: OutlineInputBorder(),
                                ),
                                keyboardType: TextInputType.number,
                              ),
                            ),
                            const SizedBox(width: 12),
                            Expanded(
                              child: TextFormField(
                                controller: _lowStockController,
                                decoration: const InputDecoration(
                                  labelText: 'Low Stock Threshold',
                                  border: OutlineInputBorder(),
                                ),
                                keyboardType: TextInputType.number,
                              ),
                            ),
                          ],
                        ),
                      ],
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 20),
              // Actions
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  TextButton(
                    onPressed: () => Navigator.of(context).pop(),
                    child: const Text('Cancel'),
                  ),
                  const SizedBox(width: 12),
                  LoadingButton.elevated(
                    isLoading: false, // TODO: Implement loading state
                    onPressed: _createProduct,
                    child: const Text('Create Product'),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _createProduct() async {
    if (_formKey.currentState!.validate()) {
      try {
        final pricing = ProductPricing(
          basePrice: double.parse(_priceController.text),
          cost: _costController.text.isNotEmpty ? double.tryParse(_costController.text) : null,
        );

        final inventory = ProductInventory(
          trackStock: _trackStock,
          currentStock: _trackStock && _stockController.text.isNotEmpty 
              ? int.tryParse(_stockController.text) 
              : null,
          lowStockThreshold: _trackStock && _lowStockController.text.isNotEmpty 
              ? int.tryParse(_lowStockController.text) 
              : null,
        );

        final request = CreateProductRequest(
          name: _nameController.text,
          description: _descriptionController.text,
          category: _category,
          pricing: pricing,
          inventory: inventory,
          sku: _skuController.text.isNotEmpty ? _skuController.text : null,
        );

        await ref.read(productsProvider.notifier).createProduct(request);

        if (mounted) {
          Navigator.of(context).pop();
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
              content: Text('Product created successfully!'),
              backgroundColor: Colors.green,
            ),
          );
        }
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Failed to create product: ${e.toString()}'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    }
  }
}

/// Edit product dialog (placeholder)
class EditProductDialog extends ConsumerWidget {
  final Product product;

  const EditProductDialog({super.key, required this.product});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Dialog(
      child: Container(
        width: 400,
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              'Edit ${product.name}',
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 20),
            const Text('Edit product functionality will be implemented here'),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton(
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Text('Cancel'),
                ),
                const SizedBox(width: 12),
                ElevatedButton(
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Text('Save Changes'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

/// Stock adjustment dialog
class StockAdjustmentDialog extends ConsumerStatefulWidget {
  final Product product;

  const StockAdjustmentDialog({super.key, required this.product});

  @override
  ConsumerState<StockAdjustmentDialog> createState() => _StockAdjustmentDialogState();
}

class _StockAdjustmentDialogState extends ConsumerState<StockAdjustmentDialog> {
  final _adjustmentController = TextEditingController();
  final _reasonController = TextEditingController();
  bool _isAddition = true;

  @override
  void dispose() {
    _adjustmentController.dispose();
    _reasonController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final currentStock = widget.product.inventory.currentStock ?? 0;
    
    return Dialog(
      child: Container(
        width: 400,
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Adjust Stock',
              style: theme.textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              widget.product.name,
              style: theme.textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            Text('Current Stock: $currentStock ${widget.product.inventory.stockUnit ?? 'units'}'),
            const SizedBox(height: 16),
            Row(
              children: [
                Expanded(
                  child: RadioListTile<bool>(
                    title: const Text('Add'),
                    value: true,
                    groupValue: _isAddition,
                    onChanged: (value) => setState(() => _isAddition = value ?? true),
                  ),
                ),
                Expanded(
                  child: RadioListTile<bool>(
                    title: const Text('Remove'),
                    value: false,
                    groupValue: _isAddition,
                    onChanged: (value) => setState(() => _isAddition = value ?? true),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _adjustmentController,
              decoration: InputDecoration(
                labelText: _isAddition ? 'Quantity to Add' : 'Quantity to Remove',
                border: const OutlineInputBorder(),
              ),
              keyboardType: TextInputType.number,
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _reasonController,
              decoration: const InputDecoration(
                labelText: 'Reason (optional)',
                border: OutlineInputBorder(),
              ),
              maxLines: 2,
            ),
            const SizedBox(height: 24),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton(
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Text('Cancel'),
                ),
                const SizedBox(width: 12),
                LoadingButton.elevated(
                  isLoading: false, // TODO: Implement loading state
                  onPressed: _adjustStock,
                  child: const Text('Adjust Stock'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  void _adjustStock() async {
    final adjustmentText = _adjustmentController.text;
    if (adjustmentText.isEmpty) return;

    final adjustment = int.tryParse(adjustmentText);
    if (adjustment == null || adjustment <= 0) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please enter a valid quantity'),
          backgroundColor: Colors.red,
        ),
      );
      return;
    }

    try {
      final finalAdjustment = _isAddition ? adjustment : -adjustment;
      await ref.read(productsProvider.notifier).adjustStock(
        widget.product.id,
        finalAdjustment,
        reason: _reasonController.text.isNotEmpty ? _reasonController.text : null,
      );

      if (mounted) {
        Navigator.of(context).pop();
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Stock ${_isAddition ? 'added' : 'removed'} successfully!'),
            backgroundColor: Colors.green,
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to adjust stock: ${e.toString()}'),
            backgroundColor: Colors.red,
          ),
        );
      }
    }
  }
}