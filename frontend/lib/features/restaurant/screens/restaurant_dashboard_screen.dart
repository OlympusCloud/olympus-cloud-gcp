import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/branding/branding_provider.dart';
import '../../../shared/presentation/widgets/adaptive_layout.dart';
import '../widgets/dashboard_widgets.dart';
import '../widgets/analytics_widgets.dart';
import '../../../core/branding/industry_branding.dart';

/// Restaurant Revolution dashboard with comprehensive restaurant management features
class RestaurantDashboardScreen extends ConsumerStatefulWidget {
  const RestaurantDashboardScreen({super.key});

  @override
  ConsumerState<RestaurantDashboardScreen> createState() => _RestaurantDashboardScreenState();
}

class _RestaurantDashboardScreenState extends ConsumerState<RestaurantDashboardScreen> {
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final branding = ref.watch(brandingProvider);
    final size = MediaQuery.of(context).size;
    final isDesktop = size.width > 1200;
    final isTablet = size.width > 800 && size.width <= 1200;

    return AdaptiveLayout(
      child: Scaffold(
        appBar: AppBar(
          title: Row(
            children: [
              if (branding.logoPath != null) ...[
                Image.asset(
                  branding.logoPath!,
                  height: 32,
                  width: 32,
                ),
                const SizedBox(width: 12),
              ],
              Text(
                branding.name,
                style: TextStyle(
                  color: branding.primaryColor,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          backgroundColor: Colors.transparent,
          elevation: 0,
          actions: [
            // Quick action buttons
            IconButton(
              onPressed: () {
                // TODO: Navigate to orders
              },
              icon: const Icon(Icons.receipt_long),
              tooltip: 'Orders',
            ),
            IconButton(
              onPressed: () {
                // TODO: Navigate to kitchen
              },
              icon: const Icon(Icons.restaurant),
              tooltip: 'Kitchen',
            ),
            IconButton(
              onPressed: () {
                // TODO: Navigate to tables
              },
              icon: const Icon(Icons.table_restaurant),
              tooltip: 'Tables',
            ),
            IconButton(
              onPressed: () {
                // TODO: Show notifications
              },
              icon: Badge(
                label: const Text('3'),
                child: const Icon(Icons.notifications),
              ),
              tooltip: 'Notifications',
            ),
            const SizedBox(width: 8),
          ],
        ),
        body: SingleChildScrollView(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Welcome header
              _buildWelcomeHeader(theme, branding),
              
              const SizedBox(height: 24),
              
              // Quick stats
              const QuickStatsWidget(height: 120),
              
              const SizedBox(height: 16),
              
              // Main dashboard content
              if (isDesktop) 
                _buildDesktopLayout()
              else if (isTablet)
                _buildTabletLayout()
              else
                _buildMobileLayout(),
            ],
          ),
        ),
        floatingActionButton: FloatingActionButton.extended(
          onPressed: () {
            // TODO: Quick order entry
          },
          backgroundColor: branding.primaryColor,
          icon: const Icon(Icons.add, color: Colors.white),
          label: const Text(
            'New Order',
            style: TextStyle(color: Colors.white),
          ),
        ),
      ),
    );
  }

  Widget _buildWelcomeHeader(ThemeData theme, IndustryBranding branding) {
    final now = DateTime.now();
    final hour = now.hour;
    String greeting;
    
    if (hour < 12) {
      greeting = 'Good morning';
    } else if (hour < 17) {
      greeting = 'Good afternoon';
    } else {
      greeting = 'Good evening';
    }

    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            branding.primaryColor.withOpacity(0.1),
            branding.secondaryColor.withOpacity(0.1),
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: branding.primaryColor.withOpacity(0.2),
        ),
      ),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  greeting,
                  style: theme.textTheme.headlineSmall?.copyWith(
                    color: branding.primaryColor,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Welcome to ${branding.name}',
                  style: theme.textTheme.bodyLarge?.copyWith(
                    color: theme.colorScheme.onSurface.withOpacity(0.7),
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  'Today • ${_formatDate(now)}',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: theme.colorScheme.onSurface.withOpacity(0.5),
                  ),
                ),
              ],
            ),
          ),
          Icon(
            Icons.restaurant_menu,
            size: 48,
            color: branding.primaryColor.withOpacity(0.7),
          ),
        ],
      ),
    );
  }

  Widget _buildDesktopLayout() {
    return Column(
      children: [
        // Top row: Kitchen Display + Table Status
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              flex: 2,
              child: Container(
                height: 400,
                child: const KitchenDisplayWidget(),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              flex: 1,
              child: Container(
                height: 400,
                child: const TableStatusWidget(),
              ),
            ),
          ],
        ),
        
        const SizedBox(height: 16),
        
        // Middle row: Service Metrics + Revenue Analytics
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: Container(
                height: 250,
                child: const ServiceMetricsWidget(),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Container(
                height: 250,
                child: const RevenueAnalyticsWidget(),
              ),
            ),
          ],
        ),
        
        const SizedBox(height: 16),
        
        // Bottom row: Additional widgets or recommendations
        _buildRecommendationsSection(),
      ],
    );
  }

  Widget _buildTabletLayout() {
    return Column(
      children: [
        // Top row: Kitchen Display
        Container(
          height: 350,
          child: const KitchenDisplayWidget(),
        ),
        
        const SizedBox(height: 16),
        
        // Second row: Table Status + Service Metrics
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: Container(
                height: 300,
                child: const TableStatusWidget(),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Container(
                height: 300,
                child: const ServiceMetricsWidget(),
              ),
            ),
          ],
        ),
        
        const SizedBox(height: 16),
        
        // Third row: Revenue Analytics
        Container(
          height: 300,
          child: const RevenueAnalyticsWidget(),
        ),
        
        const SizedBox(height: 16),
        
        _buildRecommendationsSection(),
      ],
    );
  }

  Widget _buildMobileLayout() {
    return Column(
      children: [
        // Kitchen Display
        Container(
          height: 300,
          child: const KitchenDisplayWidget(),
        ),
        
        const SizedBox(height: 16),
        
        // Table Status
        Container(
          height: 250,
          child: const TableStatusWidget(),
        ),
        
        const SizedBox(height: 16),
        
        // Service Metrics
        Container(
          height: 200,
          child: const ServiceMetricsWidget(),
        ),
        
        const SizedBox(height: 16),
        
        // Revenue Analytics
        Container(
          height: 350,
          child: const RevenueAnalyticsWidget(),
        ),
        
        const SizedBox(height: 16),
        
        _buildRecommendationsSection(),
      ],
    );
  }

  Widget _buildRecommendationsSection() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  Icons.lightbulb_outline,
                  color: Theme.of(context).colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'AI Recommendations',
                  style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 12),
            
            // Placeholder for recommendations
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surfaceVariant.withOpacity(0.3),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                children: [
                  Icon(
                    Icons.auto_awesome,
                    color: Theme.of(context).colorScheme.primary,
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Optimize table turnover',
                          style: Theme.of(context).textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          'Consider reducing table 5\'s setup time to improve turnover rate by 15%',
                          style: Theme.of(context).textTheme.bodyMedium,
                        ),
                      ],
                    ),
                  ),
                  TextButton(
                    onPressed: () {
                      // TODO: Handle recommendation action
                    },
                    child: const Text('View Details'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _formatDate(DateTime date) {
    final months = [
      'January', 'February', 'March', 'April', 'May', 'June',
      'July', 'August', 'September', 'October', 'November', 'December'
    ];
    
    return '${months[date.month - 1]} ${date.day}, ${date.year}';
  }
}