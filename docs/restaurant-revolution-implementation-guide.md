# Restaurant Revolution Frontend Implementation Guide
## Immediate Actions for Multi-Industry Branding

---

## 🚀 Quick Start Implementation

### Step 1: Update Theme Configuration

Replace `/frontend/lib/core/theme/app_theme.dart` with industry-aware theming:

```dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class IndustryThemeManager {
  static const Map<String, IndustryTheme> themes = {
    'restaurant': RestaurantRevolutionTheme(),
    'retail': RetailEdgeTheme(),
    'salon': SalonLuxeTheme(),
    'event': EventMasterTheme(),
  };
  
  static ThemeData getTheme(String industryType, Brightness brightness) {
    final industryTheme = themes[industryType] ?? themes['restaurant']!;
    return industryTheme.buildTheme(brightness);
  }
}

abstract class IndustryTheme {
  ThemeData buildTheme(Brightness brightness);
  ColorScheme get lightColorScheme;
  ColorScheme get darkColorScheme;
  TextTheme get textTheme;
  String get fontFamily;
}

class RestaurantRevolutionTheme implements IndustryTheme {
  @override
  ColorScheme get lightColorScheme => const ColorScheme.light(
    primary: Color(0xFFD32F2F),        // Rich Red
    secondary: Color(0xFFFF6E40),      // Orange
    tertiary: Color(0xFF795548),       // Brown
    error: Color(0xFFC62828),
    background: Color(0xFFFFFBFE),
    surface: Color(0xFFF5F5F5),
    onPrimary: Colors.white,
    onSecondary: Colors.white,
    onBackground: Color(0xFF1C1B1F),
    onSurface: Color(0xFF1C1B1F),
  );
  
  @override
  ColorScheme get darkColorScheme => const ColorScheme.dark(
    primary: Color(0xFFFF6B6B),
    secondary: Color(0xFFFF8E53),
    tertiary: Color(0xFF8D6E63),
    error: Color(0xFFEF5350),
    background: Color(0xFF121212),
    surface: Color(0xFF1E1E1E),
    onPrimary: Color(0xFF2C0000),
    onSecondary: Color(0xFF3A1600),
    onBackground: Color(0xFFE6E1E5),
    onSurface: Color(0xFFE6E1E5),
  );
  
  @override
  String get fontFamily => 'Inter';
  
  @override
  TextTheme get textTheme => GoogleFonts.interTextTheme().copyWith(
    displayLarge: GoogleFonts.playfairDisplay(
      fontSize: 32,
      fontWeight: FontWeight.w700,
      height: 1.2,
    ),
    displayMedium: GoogleFonts.playfairDisplay(
      fontSize: 28,
      fontWeight: FontWeight.w600,
      height: 1.3,
    ),
    headlineLarge: GoogleFonts.inter(
      fontSize: 24,
      fontWeight: FontWeight.w600,
      height: 1.4,
    ),
  );
  
  @override
  ThemeData buildTheme(Brightness brightness) {
    final colorScheme = brightness == Brightness.light 
        ? lightColorScheme 
        : darkColorScheme;
    
    return ThemeData(
      useMaterial3: true,
      colorScheme: colorScheme,
      textTheme: textTheme,
      // Restaurant-specific components
      cardTheme: CardTheme(
        elevation: 2,
        shadowColor: colorScheme.primary.withOpacity(0.1),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: colorScheme.primary,
          foregroundColor: colorScheme.onPrimary,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
        ),
      ),
    );
  }
}
```

---

### Step 2: Create Industry Configuration Provider

Create `/frontend/lib/core/providers/industry_provider.dart`:

```dart
import 'package:flutter_riverpod/flutter_riverpod.dart';

class IndustryConfiguration {
  final String industryType;
  final String brandName;
  final String businessName;
  final Map<String, bool> enabledModules;
  final Map<String, dynamic> customSettings;
  
  const IndustryConfiguration({
    required this.industryType,
    required this.brandName,
    required this.businessName,
    required this.enabledModules,
    required this.customSettings,
  });
  
  factory IndustryConfiguration.restaurant() {
    return const IndustryConfiguration(
      industryType: 'restaurant',
      brandName: 'Restaurant Revolution',
      businessName: 'My Restaurant',
      enabledModules: {
        'menu': true,
        'tables': true,
        'kitchen': true,
        'delivery': true,
        'loyalty': true,
        'reservations': true,
      },
      customSettings: {
        'currency': 'USD',
        'timeFormat': '12h',
        'defaultTipOptions': [15, 18, 20, 25],
      },
    );
  }
}

class IndustryConfigNotifier extends StateNotifier<IndustryConfiguration> {
  IndustryConfigNotifier() : super(IndustryConfiguration.restaurant());
  
  void loadConfiguration(String tenantId) async {
    // Load from Neptune Hub API
    // For now, use default restaurant configuration
    state = IndustryConfiguration.restaurant();
  }
  
  void updateIndustry(String industryType) {
    state = switch (industryType) {
      'restaurant' => IndustryConfiguration.restaurant(),
      'retail' => IndustryConfiguration.retail(),
      'salon' => IndustryConfiguration.salon(),
      'event' => IndustryConfiguration.event(),
      _ => IndustryConfiguration.restaurant(),
    };
  }
}

final industryConfigProvider = 
    StateNotifierProvider<IndustryConfigNotifier, IndustryConfiguration>(
  (ref) => IndustryConfigNotifier(),
);

// Helper providers
final industryTypeProvider = Provider<String>((ref) {
  return ref.watch(industryConfigProvider).industryType;
});

final enabledModulesProvider = Provider<Map<String, bool>>((ref) {
  return ref.watch(industryConfigProvider).enabledModules;
});
```

---

### Step 3: Update Main App Widget

Update `/frontend/lib/app.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'core/providers/industry_provider.dart';
import 'core/theme/industry_theme_manager.dart';

class OlympusApp extends ConsumerWidget {
  const OlympusApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final industryConfig = ref.watch(industryConfigProvider);
    final isDarkMode = ref.watch(isDarkModeProvider);
    
    return MaterialApp.router(
      title: industryConfig.brandName,
      debugShowCheckedModeBanner: false,
      
      // Dynamic theme based on industry
      theme: IndustryThemeManager.getTheme(
        industryConfig.industryType,
        Brightness.light,
      ),
      darkTheme: IndustryThemeManager.getTheme(
        industryConfig.industryType,
        Brightness.dark,
      ),
      themeMode: isDarkMode ? ThemeMode.dark : ThemeMode.light,
      
      // ... rest of configuration
    );
  }
}
```

---

### Step 4: Create Industry-Specific Dashboard

Update `/frontend/lib/features/dashboard/presentation/screens/dashboard_screen.dart`:

```dart
class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({super.key});

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen> {
  @override
  Widget build(BuildContext context) {
    final industryConfig = ref.watch(industryConfigProvider);
    
    return switch (industryConfig.industryType) {
      'restaurant' => RestaurantDashboard(config: industryConfig),
      'retail' => RetailDashboard(config: industryConfig),
      'salon' => SalonDashboard(config: industryConfig),
      'event' => EventDashboard(config: industryConfig),
      _ => DefaultDashboard(config: industryConfig),
    };
  }
}

class RestaurantDashboard extends StatelessWidget {
  final IndustryConfiguration config;
  
  const RestaurantDashboard({required this.config, super.key});
  
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Scaffold(
      appBar: AppBar(
        title: Row(
          children: [
            // Restaurant-specific branding
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: theme.colorScheme.primary.withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                Icons.restaurant_menu,
                color: theme.colorScheme.primary,
                size: 24,
              ),
            ),
            const SizedBox(width: 12),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  config.businessName,
                  style: theme.textTheme.titleMedium,
                ),
                Text(
                  'Restaurant Revolution',
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: theme.colorScheme.primary,
                  ),
                ),
              ],
            ),
          ],
        ),
        actions: [
          // Restaurant-specific quick actions
          IconButton(
            icon: const Icon(Icons.table_restaurant),
            onPressed: () => _navigateToTables(context),
            tooltip: 'Table Management',
          ),
          IconButton(
            icon: const Icon(Icons.delivery_dining),
            onPressed: () => _navigateToDelivery(context),
            tooltip: 'Delivery Orders',
          ),
          IconButton(
            icon: const Icon(Icons.kitchen),
            onPressed: () => _navigateToKitchen(context),
            tooltip: 'Kitchen Display',
          ),
        ],
      ),
      body: _buildRestaurantDashboard(context),
    );
  }
  
  Widget _buildRestaurantDashboard(BuildContext context) {
    return CustomScrollView(
      slivers: [
        // Today's Overview
        SliverToBoxAdapter(
          child: _buildTodayOverview(context),
        ),
        
        // Quick Stats Grid
        SliverPadding(
          padding: const EdgeInsets.all(16),
          sliver: SliverGrid.count(
            crossAxisCount: 2,
            childAspectRatio: 1.5,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            children: [
              _buildStatCard(
                context,
                icon: Icons.restaurant,
                title: 'Active Tables',
                value: '12/25',
                color: Colors.orange,
              ),
              _buildStatCard(
                context,
                icon: Icons.receipt_long,
                title: 'Orders Today',
                value: '47',
                color: Colors.blue,
              ),
              _buildStatCard(
                context,
                icon: Icons.attach_money,
                title: 'Revenue',
                value: '\$3,847',
                color: Colors.green,
              ),
              _buildStatCard(
                context,
                icon: Icons.people,
                title: 'Guests',
                value: '156',
                color: Colors.purple,
              ),
            ],
          ),
        ),
        
        // Recent Orders
        SliverToBoxAdapter(
          child: _buildRecentOrders(context),
        ),
        
        // Kitchen Status
        SliverToBoxAdapter(
          child: _buildKitchenStatus(context),
        ),
      ],
    );
  }
  
  Widget _buildStatCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String value,
    required Color color,
  }) {
    final theme = Theme.of(context);
    
    return Card(
      child: InkWell(
        onTap: () {},
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(icon, color: color, size: 28),
              const Spacer(),
              Text(
                value,
                style: theme.textTheme.headlineSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: color,
                ),
              ),
              Text(
                title,
                style: theme.textTheme.bodySmall?.copyWith(
                  color: theme.colorScheme.onSurface.withOpacity(0.6),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
```

---

### Step 5: Add Restaurant-Specific Features

Create `/frontend/lib/features/restaurant/` directory with modules:

```dart
// menu/models/menu_item.dart
class MenuItem {
  final String id;
  final String name;
  final String description;
  final double price;
  final String category;
  final List<String> modifiers;
  final String? imageUrl;
  final bool available;
  final Map<String, dynamic> nutritionInfo;
  final List<String> allergens;
  
  const MenuItem({
    required this.id,
    required this.name,
    required this.description,
    required this.price,
    required this.category,
    this.modifiers = const [],
    this.imageUrl,
    this.available = true,
    this.nutritionInfo = const {},
    this.allergens = const [],
  });
}

// table/models/table.dart
class RestaurantTable {
  final String id;
  final String number;
  final int capacity;
  final TableStatus status;
  final String? serverId;
  final String? currentOrderId;
  final DateTime? seatedAt;
  
  const RestaurantTable({
    required this.id,
    required this.number,
    required this.capacity,
    required this.status,
    this.serverId,
    this.currentOrderId,
    this.seatedAt,
  });
}

enum TableStatus {
  available,
  occupied,
  reserved,
  cleaning,
  blocked,
}
```

---

## 📁 Updated Project Structure

```
frontend/
├── lib/
│   ├── core/
│   │   ├── theme/
│   │   │   ├── industry_theme_manager.dart
│   │   │   ├── restaurant_theme.dart
│   │   │   ├── retail_theme.dart
│   │   │   ├── salon_theme.dart
│   │   │   └── event_theme.dart
│   │   ├── providers/
│   │   │   ├── industry_provider.dart
│   │   │   └── tenant_provider.dart
│   │   └── constants/
│   │       └── industry_constants.dart
│   ├── features/
│   │   ├── restaurant/  # Restaurant-specific features
│   │   │   ├── menu/
│   │   │   ├── tables/
│   │   │   ├── kitchen/
│   │   │   ├── delivery/
│   │   │   └── reservations/
│   │   ├── retail/      # Retail-specific features
│   │   │   ├── catalog/
│   │   │   ├── cart/
│   │   │   └── checkout/
│   │   ├── salon/       # Salon-specific features
│   │   │   ├── appointments/
│   │   │   ├── services/
│   │   │   └── staff/
│   │   └── shared/      # Shared across industries
│   │       ├── dashboard/
│   │       ├── analytics/
│   │       └── customers/
│   └── main.dart
```

---

## 🎨 Restaurant Revolution Visual Assets

### Required Assets
```yaml
assets/
├── branding/
│   ├── restaurant/
│   │   ├── logo.svg
│   │   ├── logo_dark.svg
│   │   ├── splash.png
│   │   └── icon.png
│   ├── retail/
│   ├── salon/
│   └── event/
├── animations/
│   ├── restaurant/
│   │   ├── steam.json        # Lottie animation
│   │   ├── plate_slide.json
│   │   └── order_ready.json
│   └── shared/
│       └── loading.json
└── icons/
    ├── restaurant/
    │   ├── menu.svg
    │   ├── table.svg
    │   ├── kitchen.svg
    │   └── delivery.svg
    └── shared/
```

---

## 🔗 Integration with Olympus Hubs

### Hub Connections

```dart
// Connect to Mercury Hub (POS)
class MercuryService {
  Future<Order> createOrder(OrderRequest request) async {
    final response = await dio.post(
      '/api/v1/mercury/orders',
      data: request.toJson(),
    );
    return Order.fromJson(response.data);
  }
}

// Connect to Ceres Hub (Inventory)
class CeresService {
  Future<InventoryStatus> checkInventory(String itemId) async {
    final response = await dio.get(
      '/api/v1/ceres/inventory/$itemId/status',
    );
    return InventoryStatus.fromJson(response.data);
  }
}

// Connect to Saturn Hub (CRM)
class SaturnService {
  Future<Customer> getCustomer(String customerId) async {
    final response = await dio.get(
      '/api/v1/saturn/customers/$customerId',
    );
    return Customer.fromJson(response.data);
  }
}
```

---

## 🚦 Next Immediate Steps

1. **Today**: 
   - Update theme configuration with Restaurant Revolution colors
   - Create industry provider and configuration
   - Test theme switching

2. **Tomorrow**:
   - Implement restaurant-specific dashboard
   - Add menu management feature module
   - Create table management UI

3. **This Week**:
   - Complete all Restaurant Revolution features
   - Test with mock data
   - Integrate with backend APIs

4. **Next Week**:
   - Add retail industry theme and features
   - Implement dynamic module loading
   - Performance optimization

---

## 📊 Success Criteria

- [ ] Restaurant Revolution theme applies correctly
- [ ] Dashboard shows restaurant-specific widgets
- [ ] Navigation adapts to enabled modules
- [ ] Industry switching works without restart
- [ ] All Restaurant Revolution features accessible
- [ ] Performance targets met (<2s load time)

---

*This implementation guide provides the concrete steps to transform your current Flutter app into a multi-industry platform starting with Restaurant Revolution.*