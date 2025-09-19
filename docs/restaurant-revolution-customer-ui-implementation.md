# Restaurant Revolution App - Customer UI/UX Implementation
## Complete Customer Dining Experience Design

---

## 🎯 Application Overview

**Restaurant Revolution App** is the customer-facing application providing a complete digital dining experience from browsing menus to placing orders, earning rewards, and making payments.

**Target Platforms:** iOS, Android, Web (PWA), Kiosks
**Primary Users:** Restaurant customers, diners, takeout/delivery customers

---

## 🎨 Visual Design System

### Color Palette - Appetizing & Inviting

```dart
// Customer App Theme - Warm & Welcoming
class CustomerAppTheme {
  // Primary Brand Colors
  static const primary = Color(0xFFD32F2F);       // Restaurant Red
  static const secondary = Color(0xFFFF6E40);     // Warm Orange
  static const tertiary = Color(0xFF795548);      // Rich Brown
  
  // Accent Colors
  static const freshGreen = Color(0xFF66BB6A);    // Fresh ingredients
  static const golden = Color(0xFFFFC107);        // Premium/rewards
  static const deepPurple = Color(0xFF7E57C2);    // Special offers
  
  // UI Colors
  static const background = Color(0xFFFAF7F5);    // Warm white
  static const surface = Color(0xFFFFFFFF);       // Pure white
  static const surfaceAlt = Color(0xFFFFF8E1);    // Cream
  
  // Semantic Colors
  static const success = Color(0xFF4CAF50);       // Order confirmed
  static const warning = Color(0xFFFFA726);       // Limited items
  static const error = Color(0xFFE53935);         // Out of stock
  static const info = Color(0xFF29B6F6);          // Information
}
```

### Typography - Elegant & Readable

```dart
// Appetizing and elegant typography
static TextTheme customerTextTheme = TextTheme(
  // Restaurant Name/Headers
  displayLarge: GoogleFonts.playfairDisplay(
    fontSize: 36,
    fontWeight: FontWeight.w700,
    height: 1.2,
    letterSpacing: -0.5,
  ),
  // Menu Categories
  headlineLarge: GoogleFonts.merriweather(
    fontSize: 28,
    fontWeight: FontWeight.w600,
    height: 1.3,
  ),
  // Menu Items
  titleLarge: GoogleFonts.lato(
    fontSize: 20,
    fontWeight: FontWeight.w600,
    height: 1.4,
  ),
  // Descriptions
  bodyLarge: GoogleFonts.openSans(
    fontSize: 16,
    fontWeight: FontWeight.w400,
    height: 1.6,
    letterSpacing: 0.15,
  ),
);
```

---

## 📱 Core Screen Layouts

### 1. Home Screen - Restaurant Landing

```dart
class RestaurantHomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CustomScrollView(
        slivers: [
          // Hero Image with Restaurant Info
          SliverAppBar(
            expandedHeight: 300,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              background: Stack(
                fit: StackFit.expand,
                children: [
                  // Hero Image
                  CachedNetworkImage(
                    imageUrl: restaurant.heroImage,
                    fit: BoxFit.cover,
                  ),
                  // Gradient Overlay
                  Container(
                    decoration: BoxDecoration(
                      gradient: LinearGradient(
                        begin: Alignment.topCenter,
                        end: Alignment.bottomCenter,
                        colors: [
                          Colors.transparent,
                          Colors.black.withOpacity(0.7),
                        ],
                      ),
                    ),
                  ),
                  // Restaurant Info
                  Positioned(
                    bottom: 20,
                    left: 20,
                    right: 20,
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          restaurant.name,
                          style: TextStyle(
                            fontSize: 32,
                            fontWeight: FontWeight.bold,
                            color: Colors.white,
                          ),
                        ),
                        Row(
                          children: [
                            RatingStars(rating: 4.5),
                            Text(' 4.5 ', style: TextStyle(color: Colors.white)),
                            Text('(1,234 reviews)', style: TextStyle(color: Colors.white70)),
                          ],
                        ),
                        Text(
                          restaurant.cuisine,
                          style: TextStyle(color: Colors.white70),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
            actions: [
              IconButton(
                icon: Icon(Icons.favorite_border),
                onPressed: () => toggleFavorite(),
              ),
              IconButton(
                icon: Icon(Icons.share),
                onPressed: () => share(),
              ),
            ],
          ),
          
          // Quick Actions
          SliverToBoxAdapter(
            child: Container(
              padding: EdgeInsets.all(16),
              child: Row(
                children: [
                  Expanded(
                    child: QuickActionCard(
                      icon: Icons.restaurant_menu,
                      title: 'Order Now',
                      subtitle: 'Dine-in or Takeout',
                      color: primary,
                      onTap: () => navigateToMenu(),
                    ),
                  ),
                  SizedBox(width: 12),
                  Expanded(
                    child: QuickActionCard(
                      icon: Icons.book_online,
                      title: 'Reserve',
                      subtitle: 'Book a table',
                      color: secondary,
                      onTap: () => makeReservation(),
                    ),
                  ),
                ],
              ),
            ),
          ),
          
          // Special Offers Banner
          SliverToBoxAdapter(
            child: Container(
              height: 120,
              margin: EdgeInsets.symmetric(horizontal: 16),
              child: PageView(
                children: [
                  OfferBanner(
                    title: 'Happy Hour',
                    subtitle: '50% off drinks 4-6pm',
                    gradient: [Colors.purple, Colors.pink],
                  ),
                  OfferBanner(
                    title: 'Lunch Special',
                    subtitle: '\$12.99 lunch combos',
                    gradient: [Colors.orange, Colors.red],
                  ),
                ],
              ),
            ),
          ),
          
          // Featured Items
          SliverToBoxAdapter(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Padding(
                  padding: EdgeInsets.all(16),
                  child: Text(
                    'Chef\'s Recommendations',
                    style: Theme.of(context).textTheme.headlineMedium,
                  ),
                ),
                Container(
                  height: 280,
                  child: ListView.builder(
                    scrollDirection: Axis.horizontal,
                    padding: EdgeInsets.symmetric(horizontal: 16),
                    itemBuilder: (context, index) {
                      return FeaturedItemCard(
                        item: featuredItems[index],
                        onTap: () => addToCart(featuredItems[index]),
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
          
          // Menu Categories
          SliverToBoxAdapter(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Padding(
                  padding: EdgeInsets.all(16),
                  child: Text(
                    'Menu Categories',
                    style: Theme.of(context).textTheme.headlineMedium,
                  ),
                ),
                GridView.count(
                  shrinkWrap: true,
                  physics: NeverScrollableScrollPhysics(),
                  crossAxisCount: 2,
                  padding: EdgeInsets.all(16),
                  children: categories.map((category) => 
                    CategoryCard(
                      name: category.name,
                      image: category.image,
                      itemCount: category.itemCount,
                      onTap: () => navigateToCategory(category),
                    ),
                  ).toList(),
                ),
              ],
            ),
          ),
        ],
      ),
      
      // Bottom Navigation
      bottomNavigationBar: CustomerBottomNav(
        currentIndex: 0,
        cartItemCount: 3,
      ),
    );
  }
}
```

### 2. Menu Browsing Screen

```dart
class MenuBrowsingScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          // Left: Category Sidebar (tablets/desktop)
          if (MediaQuery.of(context).size.width > 600)
            Container(
              width: 200,
              child: CategorySidebar(
                categories: menuCategories,
                selected: selectedCategory,
                onSelect: (category) => scrollToCategory(category),
              ),
            ),
          
          // Right: Menu Items
          Expanded(
            child: CustomScrollView(
              slivers: [
                // Search Bar
                SliverAppBar(
                  floating: true,
                  title: SearchBar(
                    hintText: 'Search menu...',
                    onSearch: (query) => searchMenu(query),
                    filters: [
                      'Vegetarian',
                      'Gluten-Free',
                      'Spicy',
                      'Popular',
                    ],
                  ),
                ),
                
                // Dietary Preference Pills
                SliverToBoxAdapter(
                  child: Container(
                    height: 40,
                    child: ListView(
                      scrollDirection: Axis.horizontal,
                      padding: EdgeInsets.symmetric(horizontal: 16),
                      children: [
                        FilterChip(
                          label: Text('🌱 Vegetarian'),
                          selected: filters.contains('vegetarian'),
                          onSelected: (selected) => toggleFilter('vegetarian'),
                        ),
                        SizedBox(width: 8),
                        FilterChip(
                          label: Text('🌾 Gluten-Free'),
                          selected: filters.contains('gluten-free'),
                          onSelected: (selected) => toggleFilter('gluten-free'),
                        ),
                        SizedBox(width: 8),
                        FilterChip(
                          label: Text('🥜 Nut-Free'),
                          selected: filters.contains('nut-free'),
                          onSelected: (selected) => toggleFilter('nut-free'),
                        ),
                      ],
                    ),
                  ),
                ),
                
                // Menu Items by Category
                ...menuCategories.map((category) => [
                  SliverToBoxAdapter(
                    child: CategoryHeader(
                      name: category.name,
                      description: category.description,
                    ),
                  ),
                  SliverList(
                    delegate: SliverChildBuilderDelegate(
                      (context, index) {
                        final item = category.items[index];
                        return MenuItemTile(
                          item: item,
                          onTap: () => showItemDetails(item),
                        );
                      },
                      childCount: category.items.length,
                    ),
                  ),
                ]).expand((x) => x),
              ],
            ),
          ),
        ],
      ),
      
      // Floating Cart Button
      floatingActionButton: CartFloatingButton(
        itemCount: cartItemCount,
        total: cartTotal,
        onPressed: () => navigateToCart(),
      ),
    );
  }
}

// Menu Item Tile Component
class MenuItemTile extends StatelessWidget {
  final MenuItem item;
  
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: EdgeInsets.all(12),
          child: Row(
            children: [
              // Item Image
              ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: CachedNetworkImage(
                  imageUrl: item.imageUrl,
                  width: 100,
                  height: 100,
                  fit: BoxFit.cover,
                  placeholder: (context, url) => Shimmer.fromColors(
                    baseColor: Colors.grey[300]!,
                    highlightColor: Colors.grey[100]!,
                    child: Container(color: Colors.white),
                  ),
                ),
              ),
              SizedBox(width: 16),
              
              // Item Details
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Expanded(
                          child: Text(
                            item.name,
                            style: TextStyle(
                              fontSize: 18,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ),
                        if (item.isPopular)
                          Container(
                            padding: EdgeInsets.symmetric(
                              horizontal: 8,
                              vertical: 4,
                            ),
                            decoration: BoxDecoration(
                              color: Colors.orange[100],
                              borderRadius: BorderRadius.circular(4),
                            ),
                            child: Text(
                              '🔥 Popular',
                              style: TextStyle(
                                fontSize: 12,
                                color: Colors.orange[800],
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                          ),
                      ],
                    ),
                    SizedBox(height: 4),
                    Text(
                      item.description,
                      style: TextStyle(
                        fontSize: 14,
                        color: Colors.grey[600],
                        height: 1.4,
                      ),
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                    ),
                    SizedBox(height: 8),
                    Row(
                      children: [
                        Text(
                          '\$${item.price.toStringAsFixed(2)}',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            color: Theme.of(context).primaryColor,
                          ),
                        ),
                        if (item.originalPrice != null)
                          Padding(
                            padding: EdgeInsets.only(left: 8),
                            child: Text(
                              '\$${item.originalPrice!.toStringAsFixed(2)}',
                              style: TextStyle(
                                fontSize: 14,
                                color: Colors.grey,
                                decoration: TextDecoration.lineThrough,
                              ),
                            ),
                          ),
                        Spacer(),
                        // Dietary Icons
                        if (item.isVegetarian)
                          Icon(Icons.eco, size: 20, color: Colors.green),
                        if (item.isGlutenFree)
                          Padding(
                            padding: EdgeInsets.only(left: 4),
                            child: Icon(Icons.grain, size: 20, color: Colors.brown),
                          ),
                        if (item.isSpicy)
                          Padding(
                            padding: EdgeInsets.only(left: 4),
                            child: Text('🌶️', style: TextStyle(fontSize: 16)),
                          ),
                      ],
                    ),
                  ],
                ),
              ),
              
              // Add Button
              Container(
                margin: EdgeInsets.only(left: 12),
                child: item.inCart 
                  ? QuantitySelector(
                      quantity: item.cartQuantity,
                      onIncrease: () => increaseQuantity(item),
                      onDecrease: () => decreaseQuantity(item),
                    )
                  : IconButton(
                      style: IconButton.styleFrom(
                        backgroundColor: Theme.of(context).primaryColor,
                        foregroundColor: Colors.white,
                      ),
                      icon: Icon(Icons.add),
                      onPressed: () => addToCart(item),
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

### 3. Item Details & Customization

```dart
class ItemDetailsScreen extends StatefulWidget {
  final MenuItem item;
  
  @override
  _ItemDetailsScreenState createState() => _ItemDetailsScreenState();
}

class _ItemDetailsScreenState extends State<ItemDetailsScreen> {
  int quantity = 1;
  Set<String> selectedModifiers = {};
  String? specialInstructions;
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          // Item Image
          Container(
            height: 300,
            child: Stack(
              fit: StackFit.expand,
              children: [
                CachedNetworkImage(
                  imageUrl: widget.item.imageUrl,
                  fit: BoxFit.cover,
                ),
                Positioned(
                  top: 40,
                  left: 16,
                  child: CircleAvatar(
                    backgroundColor: Colors.white,
                    child: IconButton(
                      icon: Icon(Icons.arrow_back),
                      onPressed: () => Navigator.pop(context),
                    ),
                  ),
                ),
              ],
            ),
          ),
          
          // Item Details
          Expanded(
            child: SingleChildScrollView(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Name and Price
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              widget.item.name,
                              style: TextStyle(
                                fontSize: 24,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            SizedBox(height: 8),
                            Text(
                              widget.item.description,
                              style: TextStyle(
                                fontSize: 16,
                                color: Colors.grey[600],
                                height: 1.4,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Column(
                        children: [
                          Text(
                            '\$${widget.item.price.toStringAsFixed(2)}',
                            style: TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                              color: Theme.of(context).primaryColor,
                            ),
                          ),
                          Text(
                            '${widget.item.calories} cal',
                            style: TextStyle(
                              fontSize: 14,
                              color: Colors.grey,
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                  
                  SizedBox(height: 24),
                  
                  // Nutritional Info
                  Card(
                    child: Padding(
                      padding: EdgeInsets.all(12),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'Nutritional Information',
                            style: TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          SizedBox(height: 8),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceAround,
                            children: [
                              NutritionItem(
                                label: 'Protein',
                                value: '${widget.item.protein}g',
                              ),
                              NutritionItem(
                                label: 'Carbs',
                                value: '${widget.item.carbs}g',
                              ),
                              NutritionItem(
                                label: 'Fat',
                                value: '${widget.item.fat}g',
                              ),
                              NutritionItem(
                                label: 'Fiber',
                                value: '${widget.item.fiber}g',
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                  
                  SizedBox(height: 24),
                  
                  // Allergens
                  if (widget.item.allergens.isNotEmpty) ...[
                    Text(
                      'Allergen Information',
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    SizedBox(height: 8),
                    Wrap(
                      spacing: 8,
                      children: widget.item.allergens.map((allergen) =>
                        Chip(
                          label: Text(allergen),
                          backgroundColor: Colors.red[50],
                          labelStyle: TextStyle(color: Colors.red[700]),
                        ),
                      ).toList(),
                    ),
                    SizedBox(height: 24),
                  ],
                  
                  // Customization Options
                  if (widget.item.modifierGroups.isNotEmpty) ...[
                    Text(
                      'Customize Your Order',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    SizedBox(height: 16),
                    ...widget.item.modifierGroups.map((group) =>
                      ModifierGroup(
                        title: group.name,
                        required: group.required,
                        maxSelections: group.maxSelections,
                        modifiers: group.modifiers,
                        selected: selectedModifiers,
                        onChanged: (modifier) {
                          setState(() {
                            if (selectedModifiers.contains(modifier.id)) {
                              selectedModifiers.remove(modifier.id);
                            } else {
                              selectedModifiers.add(modifier.id);
                            }
                          });
                        },
                      ),
                    ),
                  ],
                  
                  SizedBox(height: 24),
                  
                  // Special Instructions
                  TextField(
                    decoration: InputDecoration(
                      labelText: 'Special Instructions',
                      hintText: 'Any allergies or preferences?',
                      border: OutlineInputBorder(),
                    ),
                    maxLines: 3,
                    onChanged: (value) => specialInstructions = value,
                  ),
                  
                  SizedBox(height: 24),
                  
                  // Quantity Selector
                  Row(
                    children: [
                      Text(
                        'Quantity',
                        style: TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      Spacer(),
                      Container(
                        decoration: BoxDecoration(
                          border: Border.all(color: Colors.grey[300]!),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: Row(
                          children: [
                            IconButton(
                              icon: Icon(Icons.remove),
                              onPressed: quantity > 1 
                                ? () => setState(() => quantity--)
                                : null,
                            ),
                            Container(
                              padding: EdgeInsets.symmetric(horizontal: 16),
                              child: Text(
                                quantity.toString(),
                                style: TextStyle(
                                  fontSize: 18,
                                  fontWeight: FontWeight.w600,
                                ),
                              ),
                            ),
                            IconButton(
                              icon: Icon(Icons.add),
                              onPressed: () => setState(() => quantity++),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
          
          // Add to Cart Button
          Container(
            padding: EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.1),
                  offset: Offset(0, -2),
                  blurRadius: 8,
                ),
              ],
            ),
            child: SafeArea(
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Theme.of(context).primaryColor,
                  padding: EdgeInsets.symmetric(vertical: 16),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
                onPressed: () => addToCart(),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.shopping_cart),
                    SizedBox(width: 8),
                    Text(
                      'Add to Cart - \$${(widget.item.price * quantity).toStringAsFixed(2)}',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
```

### 4. Checkout & Payment

```dart
class CheckoutScreen extends StatefulWidget {
  @override
  _CheckoutScreenState createState() => _CheckoutScreenState();
}

class _CheckoutScreenState extends State<CheckoutScreen> {
  OrderType orderType = OrderType.dineIn;
  int? tableNumber;
  DateTime? pickupTime;
  String? deliveryAddress;
  PaymentMethod? selectedPayment;
  double tipPercentage = 18.0;
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Checkout'),
      ),
      body: SingleChildScrollView(
        child: Column(
          children: [
            // Order Type Selection
            Container(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'How would you like your order?',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  SizedBox(height: 12),
                  Row(
                    children: [
                      Expanded(
                        child: OrderTypeCard(
                          icon: Icons.restaurant,
                          title: 'Dine In',
                          selected: orderType == OrderType.dineIn,
                          onTap: () => setState(() => orderType = OrderType.dineIn),
                        ),
                      ),
                      SizedBox(width: 8),
                      Expanded(
                        child: OrderTypeCard(
                          icon: Icons.takeout_dining,
                          title: 'Takeout',
                          selected: orderType == OrderType.takeout,
                          onTap: () => setState(() => orderType = OrderType.takeout),
                        ),
                      ),
                      SizedBox(width: 8),
                      Expanded(
                        child: OrderTypeCard(
                          icon: Icons.delivery_dining,
                          title: 'Delivery',
                          selected: orderType == OrderType.delivery,
                          onTap: () => setState(() => orderType = OrderType.delivery),
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            
            // Order-type specific options
            if (orderType == OrderType.dineIn)
              TableNumberSelector(
                selectedTable: tableNumber,
                onSelect: (table) => setState(() => tableNumber = table),
              ),
            if (orderType == OrderType.takeout)
              PickupTimeSelector(
                selectedTime: pickupTime,
                onSelect: (time) => setState(() => pickupTime = time),
              ),
            if (orderType == OrderType.delivery)
              DeliveryAddressInput(
                address: deliveryAddress,
                onChanged: (address) => setState(() => deliveryAddress = address),
              ),
            
            Divider(),
            
            // Order Summary
            OrderSummaryCard(
              items: cartItems,
              subtotal: subtotal,
              tax: tax,
              deliveryFee: orderType == OrderType.delivery ? 4.99 : 0,
            ),
            
            Divider(),
            
            // Loyalty Points
            if (hasLoyaltyAccount)
              LoyaltyPointsCard(
                availablePoints: 1250,
                pointsValue: 12.50,
                onApply: (points) => applyLoyaltyPoints(points),
              ),
            
            // Promo Code
            PromoCodeInput(
              onApply: (code) => applyPromoCode(code),
            ),
            
            Divider(),
            
            // Tip Selection
            TipSelector(
              subtotal: subtotal,
              selectedPercentage: tipPercentage,
              onSelect: (percentage) => setState(() => tipPercentage = percentage),
            ),
            
            Divider(),
            
            // Payment Method
            PaymentMethodSelector(
              methods: [
                PaymentMethod(
                  id: 'card',
                  type: 'Credit Card',
                  last4: '4242',
                  brand: 'Visa',
                ),
                PaymentMethod(
                  id: 'apple',
                  type: 'Apple Pay',
                ),
                PaymentMethod(
                  id: 'cash',
                  type: 'Cash',
                ),
              ],
              selected: selectedPayment,
              onSelect: (method) => setState(() => selectedPayment = method),
              onAddNew: () => addPaymentMethod(),
            ),
            
            // Order Notes
            Container(
              padding: EdgeInsets.all(16),
              child: TextField(
                decoration: InputDecoration(
                  labelText: 'Add a note for the restaurant',
                  border: OutlineInputBorder(),
                ),
                maxLines: 2,
              ),
            ),
            
            // Total and Place Order
            Container(
              padding: EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.1),
                    offset: Offset(0, -2),
                    blurRadius: 8,
                  ),
                ],
              ),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        'Total',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      Text(
                        '\$${total.toStringAsFixed(2)}',
                        style: TextStyle(
                          fontSize: 24,
                          fontWeight: FontWeight.bold,
                          color: Theme.of(context).primaryColor,
                        ),
                      ),
                    ],
                  ),
                  SizedBox(height: 16),
                  ElevatedButton(
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Theme.of(context).primaryColor,
                      padding: EdgeInsets.symmetric(vertical: 16),
                      minimumSize: Size(double.infinity, 50),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                    onPressed: selectedPayment != null ? () => placeOrder() : null,
                    child: Text(
                      'Place Order',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
```

---

## 🎁 Loyalty & Rewards UI

```dart
class LoyaltyDashboard extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CustomScrollView(
        slivers: [
          // Loyalty Card Header
          SliverAppBar(
            expandedHeight: 200,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              background: Container(
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [Color(0xFFFFD700), Color(0xFFFFA500)],
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                  ),
                ),
                child: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        '1,250',
                        style: TextStyle(
                          fontSize: 48,
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                        ),
                      ),
                      Text(
                        'POINTS',
                        style: TextStyle(
                          fontSize: 16,
                          letterSpacing: 2,
                          color: Colors.white,
                        ),
                      ),
                      SizedBox(height: 8),
                      Container(
                        padding: EdgeInsets.symmetric(
                          horizontal: 12,
                          vertical: 4,
                        ),
                        decoration: BoxDecoration(
                          color: Colors.white.withOpacity(0.2),
                          borderRadius: BorderRadius.circular(20),
                        ),
                        child: Text(
                          'Gold Member',
                          style: TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
          
          // Points Progress
          SliverToBoxAdapter(
            child: Container(
              padding: EdgeInsets.all(16),
              child: Card(
                child: Padding(
                  padding: EdgeInsets.all(16),
                  child: Column(
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('Current Tier: Gold'),
                          Text('Next: Platinum'),
                        ],
                      ),
                      SizedBox(height: 8),
                      LinearProgressIndicator(
                        value: 0.75,
                        backgroundColor: Colors.grey[200],
                        valueColor: AlwaysStoppedAnimation(Colors.gold),
                      ),
                      SizedBox(height: 8),
                      Text(
                        '250 points to Platinum',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.grey,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
          
          // Available Rewards
          SliverToBoxAdapter(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Padding(
                  padding: EdgeInsets.all(16),
                  child: Text(
                    'Available Rewards',
                    style: TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                Container(
                  height: 200,
                  child: ListView.builder(
                    scrollDirection: Axis.horizontal,
                    padding: EdgeInsets.symmetric(horizontal: 16),
                    itemBuilder: (context, index) {
                      return RewardCard(
                        title: 'Free Appetizer',
                        points: 500,
                        imageUrl: 'appetizer.jpg',
                        onRedeem: () => redeemReward(),
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
```

---

## 📱 Mobile-Specific Features

### 1. Quick Reorder
- One-tap reorder from order history
- Smart suggestions based on time/day
- Favorite meals shortcuts

### 2. Location-Based Features
- Auto-detect nearest location
- Store hours and directions
- Location-specific menus

### 3. Push Notifications
- Order ready alerts
- Special offers
- Loyalty point reminders
- Birthday rewards

### 4. Social Features
- Share dishes on social media
- Refer friends for rewards
- Review and rate items

---

## 🎯 Navigation Flow

```yaml
Bottom Navigation:
  - Home (Restaurant info, specials)
  - Menu (Browse, search, filter)
  - Cart (Review, modify order)
  - Orders (History, track current)
  - Account (Profile, loyalty, settings)

Deep Links:
  - /menu/{category}
  - /item/{itemId}
  - /cart
  - /checkout
  - /order/{orderId}
  - /loyalty
```

---

## ⚡ Performance Optimizations

1. **Image Loading**
   - Progressive image loading
   - Lazy loading for menu items
   - Cached network images
   - Optimized thumbnails

2. **Smooth Animations**
   - 60fps scrolling
   - Hero animations for items
   - Smooth cart updates
   - Skeleton loading screens

3. **Offline Support**
   - Cache menu data
   - Save cart locally
   - Queue orders when offline
   - Sync when connected

---

*This implementation provides a delightful, appetizing customer experience with intuitive navigation and seamless ordering flow.*