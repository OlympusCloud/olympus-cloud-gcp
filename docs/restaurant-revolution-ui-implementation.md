# 🎨 Restaurant Revolution UI/UX Implementation Guide

> **For GitHub Copilot: Step-by-step UI implementation for Restaurant Revolution For Restaurants**

## 📱 Screen Implementation Order

### Phase 1: Core Screens (Week 1)

#### 1. Dashboard Screen (`/lib/features/restaurant/screens/dashboard_screen.dart`)

```dart
// Visual Design Requirements
Container(
  // Top Section: Welcome + Quick Stats
  Column(
    children: [
      // Welcome Header
      Container(
        height: 120,
        decoration: BoxDecoration(
          gradient: LinearGradient(
            colors: [Color(0xFFD32F2F), Color(0xFFFF6E40)],
          ),
          borderRadius: BorderRadius.circular(16),
        ),
        child: // Restaurant name, date, greeting
      ),
      
      // Quick Stats Row (Animated Cards)
      Row(
        children: [
          MetricCard(
            icon: Icons.attach_money,
            label: "Today's Revenue",
            value: "\$3,458",
            trend: "+12%",
            color: Colors.green,
          ),
          MetricCard(
            icon: Icons.receipt_long,
            label: "Active Orders",
            value: "14",
            trend: "3 pending",
            color: Colors.orange,
          ),
          MetricCard(
            icon: Icons.table_restaurant,
            label: "Tables",
            value: "18/25",
            trend: "72% occupied",
            color: Colors.blue,
          ),
          MetricCard(
            icon: Icons.timer,
            label: "Avg Wait",
            value: "24m",
            trend: "-3m",
            color: Colors.purple,
          ),
        ],
      ),
      
      // Main Content Area
      Row(
        children: [
          // Left: Active Orders List
          Expanded(
            flex: 3,
            child: ActiveOrdersList(),
          ),
          // Right: Table Grid
          Expanded(
            flex: 2,
            child: TableQuickView(),
          ),
        ],
      ),
      
      // Bottom: AI Recommendations
      AIRecommendationsCard(),
    ],
  ),
)
```

#### 2. Order Entry Screen (`/lib/features/restaurant/screens/order_entry_screen.dart`)

```dart
// Layout Structure
Row(
  children: [
    // Left: Menu Categories & Items
    Expanded(
      flex: 3,
      child: Column(
        children: [
          // Category Tabs
          TabBar(
            tabs: [
              Tab(text: "Appetizers"),
              Tab(text: "Mains"),
              Tab(text: "Drinks"),
              Tab(text: "Desserts"),
            ],
          ),
          // Menu Grid
          GridView.builder(
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 3, // Responsive based on screen
              childAspectRatio: 1.2,
            ),
            itemBuilder: (context, index) => MenuItemCard(
              image: "item_image.jpg",
              name: "Grilled Salmon",
              price: 24.99,
              onTap: () => addToOrder(item),
            ),
          ),
        ],
      ),
    ),
    
    // Right: Current Order
    Expanded(
      flex: 2,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.white,
          border: Border(left: BorderSide(color: Colors.grey[300])),
        ),
        child: Column(
          children: [
            // Order Header
            OrderHeader(
              table: "Table 5",
              server: "John D.",
              guests: 4,
            ),
            // Order Items
            Expanded(
              child: OrderItemsList(),
            ),
            // Order Summary
            OrderSummary(
              subtotal: 89.50,
              tax: 7.16,
              total: 96.66,
            ),
            // Action Buttons
            Row(
              children: [
                ElevatedButton(
                  onPressed: sendToKitchen,
                  child: Text("Send to Kitchen"),
                ),
                ElevatedButton(
                  onPressed: processPayment,
                  child: Text("Payment"),
                ),
              ],
            ),
          ],
        ),
      ),
    ),
  ],
)
```

#### 3. Table Management Screen (`/lib/features/restaurant/screens/table_management_screen.dart`)

```dart
// Interactive Floor Plan
class TableManagementScreen extends StatefulWidget {
  @override
  Widget build(BuildContext context) {
    return InteractiveViewer(
      boundaryMargin: EdgeInsets.all(20),
      minScale: 0.5,
      maxScale: 2.0,
      child: CustomPaint(
        painter: FloorPlanPainter(),
        child: Stack(
          children: [
            // Draggable Tables
            for (var table in tables)
              Positioned(
                left: table.x,
                top: table.y,
                child: Draggable<Table>(
                  data: table,
                  feedback: TableWidget(table, isDragging: true),
                  child: GestureDetector(
                    onTap: () => showTableDetails(table),
                    child: TableWidget(
                      table,
                      color: getTableColor(table.status),
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

// Table Widget Design
class TableWidget extends StatelessWidget {
  final Table table;
  final Color color;
  
  @override
  Widget build(BuildContext context) {
    return Container(
      width: 80,
      height: 80,
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(
          table.shape == 'round' ? 40 : 8,
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black26,
            blurRadius: 4,
            offset: Offset(2, 2),
          ),
        ],
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(
            table.number.toString(),
            style: TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          Text(
            "${table.seats} seats",
            style: TextStyle(
              fontSize: 12,
              color: Colors.white70,
            ),
          ),
          if (table.status == 'occupied')
            Text(
              table.timeSeated,
              style: TextStyle(
                fontSize: 10,
                color: Colors.white60,
              ),
            ),
        ],
      ),
    );
  }
}
```

#### 4. Kitchen Display Screen (`/lib/features/restaurant/screens/kitchen_display_screen.dart`)

```dart
// Kitchen Display Layout
class KitchenDisplayScreen extends StatefulWidget {
  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 4, // Stations
      child: Column(
        children: [
          // Station Tabs
          TabBar(
            tabs: [
              Tab(text: "All Orders"),
              Tab(text: "Grill"),
              Tab(text: "Sauté"),
              Tab(text: "Salad/Cold"),
            ],
            indicatorColor: Color(0xFFD32F2F),
          ),
          
          // Orders Grid
          Expanded(
            child: TabBarView(
              children: [
                // All Orders View
                GridView.builder(
                  gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 4,
                    childAspectRatio: 0.75,
                  ),
                  itemBuilder: (context, index) => KitchenOrderCard(),
                ),
                // Station-specific views...
              ],
            ),
          ),
        ],
      ),
    );
  }
}

// Kitchen Order Card Design
class KitchenOrderCard extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Card(
      color: getUrgencyColor(order.waitTime),
      child: Column(
        children: [
          // Header with Timer
          Container(
            color: Colors.black87,
            padding: EdgeInsets.all(8),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  "Table ${order.table}",
                  style: TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                TimerWidget(
                  startTime: order.createdAt,
                  warningAt: Duration(minutes: 10),
                  criticalAt: Duration(minutes: 15),
                ),
              ],
            ),
          ),
          
          // Order Items
          Expanded(
            child: ListView(
              padding: EdgeInsets.all(8),
              children: order.items.map((item) => 
                OrderItemRow(
                  quantity: item.quantity,
                  name: item.name,
                  modifiers: item.modifiers,
                  isReady: item.isReady,
                ),
              ).toList(),
            ),
          ),
          
          // Bump Button
          ElevatedButton(
            onPressed: () => bumpOrder(order.id),
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.green,
              minimumSize: Size(double.infinity, 48),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.zero,
              ),
            ),
            child: Text(
              "BUMP",
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
```

### Phase 2: Advanced Screens (Week 2)

#### 5. Payment Processing (`/lib/features/restaurant/screens/payment_screen.dart`)

```dart
// Payment Screen Layout
class PaymentScreen extends StatefulWidget {
  final Order order;
  
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        // Left: Order Summary
        Expanded(
          child: OrderSummaryPanel(order: order),
        ),
        
        // Right: Payment Methods
        Expanded(
          child: Column(
            children: [
              // Payment Method Selection
              PaymentMethodSelector(
                methods: [
                  PaymentMethod.cash,
                  PaymentMethod.card,
                  PaymentMethod.applePay,
                  PaymentMethod.googlePay,
                ],
              ),
              
              // Split Bill Options
              SplitBillWidget(
                total: order.total,
                guestCount: order.guests,
              ),
              
              // Tip Calculator
              TipCalculator(
                subtotal: order.subtotal,
                presets: [15, 18, 20, 25],
              ),
              
              // Process Payment Button
              ProcessPaymentButton(
                onPressed: processPayment,
              ),
            ],
          ),
        ),
      ],
    );
  }
}
```

#### 6. Staff Management (`/lib/features/restaurant/screens/staff_screen.dart`)

```dart
// Staff Dashboard
class StaffScreen extends StatefulWidget {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Clock In/Out Section
        ClockInOutWidget(),
        
        // Today's Schedule
        ScheduleWidget(
          shifts: todayShifts,
        ),
        
        // Performance Metrics
        StaffPerformanceGrid(
          metrics: [
            "Sales", "Tables Served", 
            "Avg Turn Time", "Tips"
          ],
        ),
        
        // Break Management
        BreakManagementPanel(),
      ],
    );
  }
}
```

## 🎨 Component Library

### Core Components to Build

```dart
// 1. Metric Card (Animated)
class MetricCard extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;
  final String trend;
  final Color color;
  
  // Implement with AnimatedContainer
  // Add shimmer effect while loading
  // Include trend arrow animation
}

// 2. Order Item Row
class OrderItemRow extends StatelessWidget {
  final int quantity;
  final String name;
  final List<String> modifiers;
  final double price;
  
  // Show modifiers in smaller text
  // Strike through for removed items
  // Highlight for new items
}

// 3. Timer Widget
class TimerWidget extends StatefulWidget {
  final DateTime startTime;
  final Duration warningAt;
  final Duration criticalAt;
  
  // Change color based on time
  // Pulse animation when critical
  // Format as MM:SS
}

// 4. Table Status Indicator
class TableStatusIndicator extends StatelessWidget {
  final TableStatus status;
  
  // Colors:
  // Available: Green
  // Occupied: Blue
  // Reserved: Yellow
  // Cleaning: Orange
  // Add pulse animation for new orders
}

// 5. Floating Action Menu
class QuickActionMenu extends StatelessWidget {
  final List<QuickAction> actions;
  
  // Expandable FAB
  // Icon + Label for each action
  // Smooth expand/collapse animation
}
```

## 🎨 Visual Style Guide

### Color Usage

```dart
class RestaurantColors {
  // Status Colors
  static const available = Color(0xFF4CAF50);
  static const occupied = Color(0xFF2196F3);
  static const reserved = Color(0xFFFFC107);
  static const alert = Color(0xFFFF5722);
  
  // Urgency Colors (for Kitchen)
  static const normal = Colors.white;
  static const warning = Color(0xFFFFF3E0);  // Light orange
  static const urgent = Color(0xFFFFEBEE);   // Light red
  static const critical = Color(0xFFFFCDD2); // Red
  
  // Background Colors
  static const dashboardBg = Color(0xFFF5F5F5);
  static const cardBg = Colors.white;
  static const headerBg = Color(0xFFD32F2F);
}
```

### Typography

```dart
class RestaurantTextStyles {
  // Headers
  static final screenTitle = GoogleFonts.playfairDisplay(
    fontSize: 28,
    fontWeight: FontWeight.bold,
    color: RestaurantColors.primaryRed,
  );
  
  // Cards
  static final cardTitle = GoogleFonts.inter(
    fontSize: 16,
    fontWeight: FontWeight.w600,
  );
  
  // Metrics
  static final metricValue = GoogleFonts.robotoMono(
    fontSize: 32,
    fontWeight: FontWeight.bold,
  );
  
  // Table Numbers
  static final tableNumber = GoogleFonts.inter(
    fontSize: 24,
    fontWeight: FontWeight.bold,
    color: Colors.white,
  );
}
```

### Animations

```dart
class RestaurantAnimations {
  // Page Transitions
  static const pageSlide = Duration(milliseconds: 300);
  
  // Card Animations
  static const cardHover = Duration(milliseconds: 200);
  static const cardTap = Duration(milliseconds: 100);
  
  // Data Updates
  static const dataRefresh = Duration(milliseconds: 500);
  
  // Loading States
  static Widget shimmerLoading() {
    return Shimmer.fromColors(
      baseColor: Colors.grey[300],
      highlightColor: Colors.grey[100],
      child: Container(),
    );
  }
  
  // Success/Error Feedback
  static void showSuccess(BuildContext context, String message) {
    // Green snackbar with check icon
  }
  
  static void showError(BuildContext context, String message) {
    // Red snackbar with error icon
  }
}
```

## 📱 Responsive Breakpoints

```dart
class ResponsiveBreakpoints {
  static bool isMobile(BuildContext context) => 
    MediaQuery.of(context).size.width < 600;
    
  static bool isTablet(BuildContext context) => 
    MediaQuery.of(context).size.width >= 600 && 
    MediaQuery.of(context).size.width < 1200;
    
  static bool isDesktop(BuildContext context) => 
    MediaQuery.of(context).size.width >= 1200;
    
  static int getGridColumns(BuildContext context) {
    if (isMobile(context)) return 2;
    if (isTablet(context)) return 3;
    return 4;
  }
}
```

## 🔄 State Management Patterns

```dart
// Use Riverpod for all state management

// Example: Orders State
final ordersProvider = StateNotifierProvider<OrdersNotifier, OrdersState>((ref) {
  return OrdersNotifier(ref);
});

class OrdersNotifier extends StateNotifier<OrdersState> {
  OrdersNotifier(this.ref) : super(OrdersState.initial());
  
  final Ref ref;
  
  Future<void> loadOrders() async {
    state = state.copyWith(isLoading: true);
    try {
      final orders = await ref.read(apiProvider).getOrders();
      state = state.copyWith(
        orders: orders,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        error: e.toString(),
        isLoading: false,
      );
    }
  }
  
  void addOrder(Order order) {
    state = state.copyWith(
      orders: [...state.orders, order],
    );
  }
}
```

## 🧪 Testing Requirements

### Widget Tests

```dart
// Test each screen renders correctly
testWidgets('Dashboard displays metrics', (tester) async {
  await tester.pumpWidget(
    ProviderScope(
      child: MaterialApp(home: DashboardScreen()),
    ),
  );
  
  expect(find.text("Today's Revenue"), findsOneWidget);
  expect(find.byType(MetricCard), findsNWidgets(4));
});

// Test interactions
testWidgets('Order can be added to cart', (tester) async {
  // Test tap on menu item adds to order
  // Test modifiers can be selected
  // Test quantity can be changed
});
```

### Integration Tests

```dart
// Test complete flows
test('Complete order flow', () async {
  // 1. Login
  // 2. Navigate to orders
  // 3. Add items
  // 4. Send to kitchen
  // 5. Process payment
  // 6. Verify order complete
});
```

## 📝 Implementation Checklist

### Week 1 Deliverables
- [ ] Dashboard with animated metrics
- [ ] Order entry with menu grid
- [ ] Table management with drag-drop
- [ ] Kitchen display with timers
- [ ] Basic navigation working

### Week 2 Deliverables
- [ ] Payment processing flow
- [ ] Staff management interface
- [ ] Menu editor
- [ ] Analytics dashboard
- [ ] Settings and configuration

---

**Remember:** Focus on smooth interactions, clear visual hierarchy, and efficient workflows. Every tap should feel responsive and purposeful.

*This guide is specifically for Restaurant Revolution For Restaurants implementation*
