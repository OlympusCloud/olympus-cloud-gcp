import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/branding/industry_branding.dart';
import '../../dashboard/widgets/industry_dashboard.dart';

/// Retail Edge dashboard with retail-specific features
class RetailDashboard extends BaseDashboard {
  const RetailDashboard({super.key});

  @override
  Widget buildHeader(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    final theme = Theme.of(context);
    
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            branding.lightColorScheme.primary,
            branding.lightColorScheme.secondary,
          ],
        ),
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: Colors.white.withAlpha(51),
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: Icon(
                      Icons.storefront,
                      color: branding.lightColorScheme.onPrimary,
                      size: 32,
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          branding.brandName,
                          style: theme.textTheme.headlineSmall?.copyWith(
                            color: branding.lightColorScheme.onPrimary,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        Text(
                          'Your Retail Command Center',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: branding.lightColorScheme.onPrimary.withAlpha(230),
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 24),
              Row(
                children: [
                  Expanded(
                    child: _buildQuickMetric(
                      context,
                      'Today\'s Sales',
                      '\$12,847',
                      Icons.trending_up,
                      Colors.white,
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: _buildQuickMetric(
                      context,
                      'Transactions',
                      '247',
                      Icons.receipt,
                      Colors.white,
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

  @override
  Widget buildQuickStats(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        buildSectionHeader(
          context,
          title: 'Store Performance',
          subtitle: 'Key retail metrics',
        ),
        GridView.count(
          crossAxisCount: 2,
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          crossAxisSpacing: 12,
          mainAxisSpacing: 12,
          childAspectRatio: 1.2,
          children: [
            buildStatCard(
              context,
              icon: Icons.inventory_2,
              title: 'Products',
              value: '1,247',
              subtitle: '23 low stock',
              color: branding.lightColorScheme.primary,
              onTap: () => context.push('/inventory'),
            ),
            buildStatCard(
              context,
              icon: Icons.shopping_cart,
              title: 'Cart Value',
              value: '\$67.43',
              subtitle: 'Average order',
              color: branding.lightColorScheme.secondary,
              onTap: () => context.push('/analytics'),
            ),
            buildStatCard(
              context,
              icon: Icons.people,
              title: 'Customers',
              value: '342',
              subtitle: 'Unique today',
              color: branding.lightColorScheme.tertiary,
              onTap: () => context.push('/customers'),
            ),
            buildStatCard(
              context,
              icon: Icons.local_offer,
              title: 'Promotions',
              value: '5 Active',
              subtitle: '12% conversion',
              color: const Color(0xFFEF4444),
              onTap: () => context.push('/promotions'),
            ),
          ],
        ),
      ],
    );
  }

  @override
  Widget buildMainContent(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        // Top Products Section
        buildSectionHeader(
          context,
          title: 'Top Selling Products',
          subtitle: 'Best performers today',
          action: TextButton(
            onPressed: () => context.push('/analytics'),
            child: const Text('View Report'),
          ),
        ),
        _buildTopProductsList(context, branding),
        
        const SizedBox(height: 24),
        
        // Recent Transactions
        buildSectionHeader(
          context,
          title: 'Recent Transactions',
          subtitle: 'Latest customer purchases',
          action: TextButton(
            onPressed: () => context.push('/transactions'),
            child: const Text('View All'),
          ),
        ),
        _buildRecentTransactions(context, branding),
        
        const SizedBox(height: 24),
        
        // Inventory Alerts
        buildSectionHeader(
          context,
          title: 'Inventory Alerts',
          subtitle: 'Items requiring attention',
        ),
        _buildInventoryAlerts(context, branding),
      ],
    );
  }

  @override
  Widget buildQuickActions(BuildContext context, WidgetRef ref, IndustryBranding branding) {
    return Column(
      children: [
        buildSectionHeader(
          context,
          title: 'Quick Actions',
          subtitle: 'Common retail tasks',
        ),
        GridView.count(
          crossAxisCount: 4,
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          crossAxisSpacing: 12,
          mainAxisSpacing: 12,
          childAspectRatio: 0.8,
          children: [
            buildActionButton(
              context,
              icon: Icons.add_shopping_cart,
              label: 'New Sale',
              color: branding.lightColorScheme.primary,
              onTap: () => _startNewSale(context),
            ),
            buildActionButton(
              context,
              icon: Icons.inventory,
              label: 'Add Product',
              color: branding.lightColorScheme.secondary,
              onTap: () => context.push('/products/add'),
            ),
            buildActionButton(
              context,
              icon: Icons.assignment_return,
              label: 'Returns',
              color: branding.lightColorScheme.tertiary,
              onTap: () => context.push('/returns'),
            ),
            buildActionButton(
              context,
              icon: Icons.discount,
              label: 'Promotions',
              color: const Color(0xFFEF4444),
              onTap: () => context.push('/promotions'),
            ),
            buildActionButton(
              context,
              icon: Icons.bar_chart,
              label: 'Reports',
              color: const Color(0xFF10B981),
              onTap: () => context.push('/reports'),
            ),
            buildActionButton(
              context,
              icon: Icons.qr_code_scanner,
              label: 'Scan Item',
              color: const Color(0xFF8B5CF6),
              onTap: () => _scanBarcode(context),
            ),
            buildActionButton(
              context,
              icon: Icons.person_add,
              label: 'New Customer',
              color: const Color(0xFFEAB308),
              onTap: () => context.push('/customers/add'),
            ),
            buildActionButton(
              context,
              icon: Icons.settings,
              label: 'Settings',
              color: const Color(0xFF6B7280),
              onTap: () => context.push('/settings'),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildQuickMetric(
    BuildContext context,
    String label,
    String value,
    IconData icon,
    Color color,
  ) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white.withAlpha(25),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: Colors.white.withAlpha(51),
        ),
      ),
      child: Row(
        children: [
          Icon(icon, color: color, size: 24),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  value,
                  style: TextStyle(
                    color: color,
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Text(
                  label,
                  style: TextStyle(
                    color: color.withAlpha(204),
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTopProductsList(BuildContext context, IndustryBranding branding) {
    final products = [
      {'name': 'Premium Coffee Blend', 'sales': '47 sold', 'revenue': '\$234.50', 'trend': '+12%'},
      {'name': 'Artisan Chocolate Bar', 'sales': '32 sold', 'revenue': '\$192.00', 'trend': '+8%'},
      {'name': 'Organic Tea Collection', 'sales': '28 sold', 'revenue': '\$168.00', 'trend': '+15%'},
    ];

    return Card(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: products.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final product = products[index];
          
          return ListTile(
            leading: CircleAvatar(
              backgroundColor: branding.lightColorScheme.primary.withAlpha(25),
              child: Text(
                '${index + 1}',
                style: TextStyle(
                  color: branding.lightColorScheme.primary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            title: Text(product['name']!),
            subtitle: Text(product['sales']!),
            trailing: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  product['revenue']!,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: const Color(0xFF10B981).withAlpha(25),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    product['trend']!,
                    style: const TextStyle(
                      color: Color(0xFF10B981),
                      fontSize: 10,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildRecentTransactions(BuildContext context, IndustryBranding branding) {
    final transactions = [
      {'id': '#T-1247', 'customer': 'John Smith', 'amount': '\$67.50', 'payment': 'Card', 'time': '2m ago'},
      {'id': '#T-1246', 'customer': 'Sarah Jones', 'amount': '\$124.99', 'payment': 'Cash', 'time': '8m ago'},
      {'id': '#T-1245', 'customer': 'Mike Wilson', 'amount': '\$43.25', 'payment': 'Card', 'time': '12m ago'},
    ];

    return Card(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: transactions.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final transaction = transactions[index];
          
          return ListTile(
            leading: CircleAvatar(
              backgroundColor: branding.lightColorScheme.secondary.withAlpha(25),
              child: Icon(
                Icons.receipt,
                color: branding.lightColorScheme.secondary,
                size: 20,
              ),
            ),
            title: Text(transaction['customer']!),
            subtitle: Text('${transaction['id']} • ${transaction['time']}'),
            trailing: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  transaction['amount']!,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                Text(
                  transaction['payment']!,
                  style: TextStyle(
                    fontSize: 12,
                    color: Theme.of(context).colorScheme.onSurface.withAlpha(153),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildInventoryAlerts(BuildContext context, IndustryBranding branding) {
    final alerts = [
      {'product': 'Premium Coffee Blend', 'stock': '3 left', 'level': 'critical'},
      {'product': 'Organic Tea Collection', 'stock': '8 left', 'level': 'low'},
      {'product': 'Artisan Chocolate Bar', 'stock': '12 left', 'level': 'warning'},
    ];

    return Card(
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: alerts.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final alert = alerts[index];
          final level = alert['level']!;
          final alertColor = _getAlertColor(level);
          
          return ListTile(
            leading: Icon(
              Icons.warning,
              color: alertColor,
            ),
            title: Text(alert['product']!),
            subtitle: Text(alert['stock']!),
            trailing: Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: alertColor.withAlpha(25),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                level.toUpperCase(),
                style: TextStyle(
                  color: alertColor,
                  fontSize: 10,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
            onTap: () => _reorderProduct(context, alert['product']!),
          );
        },
      ),
    );
  }

  Color _getAlertColor(String level) {
    switch (level) {
      case 'critical':
        return const Color(0xFFEF4444);
      case 'low':
        return const Color(0xFFEAB308);
      case 'warning':
        return const Color(0xFFEAB308);
      default:
        return const Color(0xFF6B7280);
    }
  }

  void _startNewSale(BuildContext context) {
    // TODO: Implement new sale
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Starting new sale...')),
    );
  }

  void _scanBarcode(BuildContext context) {
    // TODO: Implement barcode scanning
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Barcode scanner opening...')),
    );
  }

  void _reorderProduct(BuildContext context, String productName) {
    // TODO: Implement product reordering
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Reordering $productName')),
    );
  }

  Widget buildActionButton(
    BuildContext context, {
    required IconData icon,
    required String label,
    required Color color,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: color.withAlpha(25),
              borderRadius: BorderRadius.circular(12),
            ),
            child: Icon(icon, color: color, size: 24),
          ),
          const SizedBox(height: 8),
          Text(
            label,
            style: Theme.of(context).textTheme.bodySmall,
            textAlign: TextAlign.center,
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }

  Widget buildSectionHeader(
    BuildContext context, {
    required String title,
    String? subtitle,
    Widget? action,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12, top: 24),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
              ),
              if (subtitle != null)
                Text(
                  subtitle,
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
            ],
          ),
          if (action != null) action,
        ],
      ),
    );
  }

  Widget buildStatCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String value,
    String? subtitle,
    required Color color,
    VoidCallback? onTap,
  }) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    title,
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                  Icon(icon, color: color, size: 24),
                ],
              ),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    value,
                    style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                          fontWeight: FontWeight.bold,
                          color: color,
                        ),
                  ),
                  if (subtitle != null)
                    Text(
                      subtitle,
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}