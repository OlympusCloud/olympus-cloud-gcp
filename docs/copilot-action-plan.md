# Copilot Frontend Implementation Action Plan
## Immediate Tasks for Multi-Industry Frontend Development

---

## 🎯 Current Sprint Focus
**Goal**: Transform the existing Flutter frontend into a multi-industry platform with Restaurant Revolution as the primary implementation.

---

## ✅ Task List for Copilot

### Priority 1: Core Theme Infrastructure (Do First)

#### Task 1.1: Create Industry Theme System
**File**: `/frontend/lib/core/theme/industry_theme_manager.dart`
```dart
// TODO: Implement the following:
1. Create abstract IndustryTheme class
2. Implement RestaurantRevolutionTheme with:
   - Primary: #D32F2F (Rich Red)
   - Secondary: #FF6E40 (Orange)
   - Custom font: Playfair Display for headers
3. Add theme caching mechanism
4. Support hot-reload theme switching
```

#### Task 1.2: Update App Theme Configuration
**File**: `/frontend/lib/core/theme/app_theme.dart`
```dart
// TODO: Refactor to use IndustryThemeManager
// Remove hardcoded colors
// Add industry-specific color palettes
// Implement dynamic theme switching
```

#### Task 1.3: Create Industry Configuration Provider
**File**: `/frontend/lib/core/providers/industry_provider.dart`
```dart
// TODO: Create Riverpod providers for:
- industryConfigProvider (main configuration)
- industryTypeProvider (current industry type)
- enabledModulesProvider (feature flags)
- brandingProvider (colors, logos, text)
```

---

### Priority 2: Restaurant Dashboard Implementation

#### Task 2.1: Restaurant-Specific Dashboard
**File**: `/frontend/lib/features/restaurant/dashboard/restaurant_dashboard.dart`
```dart
// TODO: Create Restaurant Revolution dashboard with:
- Active tables widget
- Today's revenue card
- Recent orders list
- Kitchen status indicator
- Quick action buttons (New Order, Table Management, Kitchen View)
```

#### Task 2.2: Restaurant Navigation
**File**: `/frontend/lib/features/restaurant/navigation/restaurant_nav.dart`
```dart
// TODO: Implement restaurant-specific navigation:
NavigationDestinations:
- Overview (dashboard icon)
- Orders (receipt icon)
- Tables (table_restaurant icon)
- Menu (restaurant_menu icon)
- Kitchen (kitchen icon)
- Delivery (delivery_dining icon)
```

#### Task 2.3: Restaurant Widgets Library
**Directory**: `/frontend/lib/features/restaurant/widgets/`
```dart
// TODO: Create reusable restaurant widgets:
- MenuItemCard
- TableStatusCard
- OrderCard
- KitchenOrderDisplay
- DeliveryTracker
- ReservationCard
```

---

### Priority 3: Feature Modules

#### Task 3.1: Menu Management Module
**Directory**: `/frontend/lib/features/restaurant/menu/`
```dart
// TODO: Implement:
- models/menu_item.dart (with modifiers, pricing, images)
- screens/menu_list_screen.dart
- screens/menu_item_detail_screen.dart
- screens/menu_editor_screen.dart
- widgets/menu_category_list.dart
- providers/menu_provider.dart
```

#### Task 3.2: Table Management Module
**Directory**: `/frontend/lib/features/restaurant/tables/`
```dart
// TODO: Implement:
- models/table.dart (status, capacity, location)
- screens/table_layout_screen.dart (visual floor plan)
- screens/table_detail_screen.dart
- widgets/table_grid.dart
- widgets/table_status_indicator.dart
- providers/table_provider.dart
```

#### Task 3.3: Order Management Module
**Directory**: `/frontend/lib/features/restaurant/orders/`
```dart
// TODO: Implement:
- models/restaurant_order.dart
- screens/order_list_screen.dart
- screens/order_detail_screen.dart
- screens/new_order_screen.dart
- widgets/order_status_timeline.dart
- providers/restaurant_orders_provider.dart
```

---

### Priority 4: Dynamic Module Loading

#### Task 4.1: Module Loader System
**File**: `/frontend/lib/core/modules/module_loader.dart`
```dart
// TODO: Create dynamic module loading:
class ModuleLoader {
  static List<AppModule> loadModules(String industryType) {
    // Base modules (always loaded)
    // Industry-specific modules (conditional)
    // Feature flag checking
    // Lazy loading support
  }
}
```

#### Task 4.2: Feature Flags Integration
**File**: `/frontend/lib/core/features/feature_flags.dart`
```dart
// TODO: Implement feature flag system:
- Check Neptune hub for enabled features
- Cache feature flags locally
- Support A/B testing flags
- Runtime feature toggling
```

---

### Priority 5: API Integration

#### Task 5.1: Hub Service Clients
**Directory**: `/frontend/lib/core/services/hubs/`
```dart
// TODO: Create service clients for:
- mercury_service.dart (POS operations)
- ceres_service.dart (Inventory)
- saturn_service.dart (CRM)
- venus_service.dart (Loyalty)
- minerva_service.dart (Analytics)
```

#### Task 5.2: Restaurant API Integration
**File**: `/frontend/lib/features/restaurant/services/restaurant_api.dart`
```dart
// TODO: Implement Restaurant Revolution specific APIs:
- Menu CRUD operations
- Table management
- Order processing
- Kitchen display updates
- Reservation handling
```

---

## 📝 Current Working Files

### Files to Modify NOW:
1. `/frontend/lib/app.dart` - Add industry configuration
2. `/frontend/lib/core/theme/app_theme.dart` - Make industry-aware
3. `/frontend/lib/features/dashboard/presentation/screens/dashboard_screen.dart` - Add industry switching

### Files to Create NOW:
1. `/frontend/lib/core/providers/industry_provider.dart`
2. `/frontend/lib/core/theme/restaurant_revolution_theme.dart`
3. `/frontend/lib/features/restaurant/dashboard/restaurant_dashboard.dart`

---

## 🔄 While Copilot is Working

### Testing Checklist:
```bash
# Run these commands regularly:
flutter analyze
flutter test
flutter run -d chrome  # Test web
flutter run -d ios     # Test iOS simulator
flutter run -d android # Test Android emulator
```

### Hot Reload Points:
- After theme changes: Check color consistency
- After widget creation: Verify responsive behavior
- After provider updates: Test state management
- After API integration: Check error handling

---

## 🎨 Design Resources

### Restaurant Revolution Brand Colors:
```dart
// Primary Palette
const primary = Color(0xFFD32F2F);      // Rich Red
const secondary = Color(0xFFFF6E40);    // Orange
const tertiary = Color(0xFF795548);     // Brown

// Semantic Colors
const success = Color(0xFF4CAF50);      // Green
const warning = Color(0xFFFFA726);      // Amber
const error = Color(0xFFF44336);        // Red
const info = Color(0xFF2196F3);         // Blue

// Neutral Colors
const background = Color(0xFFF5F5F5);   // Light Gray
const surface = Color(0xFFFFFFFF);      // White
const textPrimary = Color(0xFF212121);  // Dark Gray
const textSecondary = Color(0xFF757575); // Medium Gray
```

### Icon Usage:
```dart
// Restaurant-specific icons
Icons.restaurant_menu    // Menu
Icons.table_restaurant   // Tables
Icons.kitchen           // Kitchen
Icons.delivery_dining   // Delivery
Icons.book_online      // Reservations
Icons.receipt_long     // Orders
Icons.people_alt       // Staff
Icons.loyalty          // Loyalty program
```

---

## 🚀 Performance Targets

While implementing, ensure:
- Initial load < 2 seconds
- Theme switch < 100ms
- Screen transitions < 300ms
- API calls show loading state within 50ms
- Smooth scrolling at 60fps

---

## 📊 Progress Tracking

### Completed: ✅
- [x] Basic Flutter project setup
- [x] Core navigation structure
- [x] Basic theme implementation

### In Progress: 🔄
- [ ] Industry theme system
- [ ] Restaurant dashboard
- [ ] Dynamic module loading

### Upcoming: 📋
- [ ] Menu management
- [ ] Table management
- [ ] Order processing
- [ ] Kitchen display
- [ ] API integration

---

## 🆘 If Stuck

1. **Theme not applying?**
   - Check `MaterialApp` theme property
   - Verify provider is being watched
   - Hot restart instead of hot reload

2. **Navigation not working?**
   - Check GoRouter configuration
   - Verify route names match
   - Check navigation guards

3. **State not updating?**
   - Ensure using `ref.watch` not `ref.read`
   - Check provider scope
   - Verify notifier is calling `state = `

4. **API calls failing?**
   - Check CORS configuration
   - Verify JWT token is attached
   - Check network connectivity

---

## 💡 Quick Commands

```bash
# Generate freezed models
flutter pub run build_runner build --delete-conflicting-outputs

# Run on all platforms
flutter run -d all

# Clear cache and rebuild
flutter clean && flutter pub get && flutter run

# Generate app icons
flutter pub run flutter_launcher_icons

# Build for production
flutter build web --release --web-renderer html
flutter build apk --release
flutter build ios --release
```

---

*Keep this document open and check off tasks as you complete them. Focus on Priority 1 first, then move down the list. The frontend should be functional with Restaurant Revolution branding by the end of today's work session.*