import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

/// Industry branding configuration and theme data
class IndustryBranding {
  final String industryType;
  final String brandName;
  final String tagline;
  final String description;
  final ColorScheme lightColorScheme;
  final ColorScheme darkColorScheme;
  final TextTheme textTheme;
  final String? logoPath;
  final Map<String, IconData> featureIcons;
  final List<String> enabledModules;
  final Map<String, dynamic> customSettings;

  const IndustryBranding({
    required this.industryType,
    required this.brandName,
    required this.tagline,
    required this.description,
    required this.lightColorScheme,
    required this.darkColorScheme,
    required this.textTheme,
    this.logoPath,
    required this.featureIcons,
    required this.enabledModules,
    required this.customSettings,
  });

  // Convenience getters that default to light theme
  String get name => brandName;
  Color get primaryColor => lightColorScheme.primary;
  Color get secondaryColor => lightColorScheme.secondary;
  List<String> get features => enabledModules;

  /// Get primary color for current brightness
  Color getPrimaryColor(Brightness brightness) {
    return brightness == Brightness.light 
        ? lightColorScheme.primary 
        : darkColorScheme.primary;
  }

  /// Get secondary color for current brightness
  Color getSecondaryColor(Brightness brightness) {
    return brightness == Brightness.light 
        ? lightColorScheme.secondary 
        : darkColorScheme.secondary;
  }

  /// Build complete theme for this industry
  ThemeData buildTheme(Brightness brightness) {
    final colorScheme = brightness == Brightness.light 
        ? lightColorScheme 
        : darkColorScheme;

    return ThemeData(
      useMaterial3: true,
      brightness: brightness,
      colorScheme: colorScheme,
      textTheme: textTheme,
      
      // App bar theme
      appBarTheme: AppBarTheme(
        elevation: 0,
        centerTitle: false,
        backgroundColor: colorScheme.surface,
        foregroundColor: colorScheme.onSurface,
        titleTextStyle: textTheme.titleLarge?.copyWith(
          color: colorScheme.onSurface,
          fontWeight: FontWeight.w600,
        ),
      ),

      // Card theme
      cardTheme: CardThemeData(
        elevation: 2,
        shadowColor: colorScheme.primary.withOpacity(0.1),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
        color: colorScheme.surface,
      ),

      // Elevated button theme
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: colorScheme.primary,
          foregroundColor: colorScheme.onPrimary,
          elevation: 2,
          shadowColor: colorScheme.primary.withOpacity(0.3),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          textStyle: textTheme.labelLarge?.copyWith(
            fontWeight: FontWeight.w600,
          ),
        ),
      ),

      // Outlined button theme
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: colorScheme.primary,
          side: BorderSide(color: colorScheme.primary, width: 1.5),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
        ),
      ),

      // Navigation rail theme
      navigationRailTheme: NavigationRailThemeData(
        backgroundColor: colorScheme.surface,
        selectedIconTheme: IconThemeData(
          color: colorScheme.primary,
          size: 24,
        ),
        unselectedIconTheme: IconThemeData(
          color: colorScheme.onSurface.withOpacity(0.6),
          size: 24,
        ),
        selectedLabelTextStyle: textTheme.labelMedium?.copyWith(
          color: colorScheme.primary,
          fontWeight: FontWeight.w600,
        ),
        unselectedLabelTextStyle: textTheme.labelMedium?.copyWith(
          color: colorScheme.onSurface.withOpacity(0.6),
        ),
      ),

      // Bottom navigation bar theme
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: colorScheme.surface,
        selectedItemColor: colorScheme.primary,
        unselectedItemColor: colorScheme.onSurface.withOpacity(0.6),
        selectedLabelStyle: textTheme.labelSmall?.copyWith(
          fontWeight: FontWeight.w600,
        ),
        unselectedLabelStyle: textTheme.labelSmall,
        type: BottomNavigationBarType.fixed,
        elevation: 8,
      ),

      // Input decoration theme
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: colorScheme.surface,
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.outline),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.outline),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.primary, width: 2),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.error),
        ),
        focusedErrorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.error, width: 2),
        ),
      ),
    );
  }
}

/// Static configuration for all industry brandings
class IndustryBrandings {
  /// Restaurant Revolution - For restaurants, bars, nightclubs, cafes
  static const IndustryBranding restaurantRevolution = IndustryBranding(
    industryType: 'restaurant',
    brandName: 'Restaurant Revolution',
    tagline: 'Revolutionize Your Restaurant Operations',
    description: 'Complete restaurant management platform for dining establishments, bars, and nightclubs',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFFD32F2F),        // Rich Red
      secondary: Color(0xFFFF6E40),      // Orange Accent
      tertiary: Color(0xFF795548),       // Brown
      error: Color(0xFFC62828),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFFFF6B6B),
      secondary: Color(0xFFFF8E53),
      tertiary: Color(0xFF8D6E63),
      error: Color(0xFFEF5350),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Color(0xFF2C0000),
      onSecondary: Color(0xFF3A1600),
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(), // Will be overridden with Google Fonts
    logoPath: 'assets/branding/restaurant/logo.png',
    
    featureIcons: {
      'menu': Icons.restaurant_menu,
      'tables': Icons.table_restaurant,
      'kitchen': Icons.restaurant,
      'orders': Icons.receipt_long,
      'delivery': Icons.delivery_dining,
      'reservations': Icons.book_online,
      'analytics': Icons.analytics,
      'staff': Icons.people,
      'payments': Icons.payment,
    },
    
    enabledModules: [
      'dashboard',
      'menu',
      'tables',
      'kitchen',
      'orders',
      'delivery',
      'reservations',
      'analytics',
      'staff',
      'payments',
      'loyalty',
    ],
    
    customSettings: {
      'currency': 'USD',
      'timeFormat': '12h',
      'defaultTipOptions': [15, 18, 20, 25],
      'tableManagement': true,
      'kitchenDisplay': true,
      'deliveryIntegration': true,
    },
  );

  /// Retail Edge - For retail stores, e-commerce, shops
  static const IndustryBranding retailEdge = IndustryBranding(
    industryType: 'retail',
    brandName: 'Retail Edge',
    tagline: 'Stay Ahead in Retail',
    description: 'Advanced retail management for stores, e-commerce, and multi-channel sales',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFF6200EA),        // Deep Purple
      secondary: Color(0xFF00BFA5),      // Teal
      tertiary: Color(0xFFFF6D00),       // Orange
      error: Color(0xFFD50000),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFF9C27B0),
      secondary: Color(0xFF26A69A),
      tertiary: Color(0xFFFF8A50),
      error: Color(0xFFEF5350),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Color(0xFF2A0845),
      onSecondary: Color(0xFF00251A),
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(),
    logoPath: 'assets/branding/retail/logo.png',
    
    featureIcons: {
      'catalog': Icons.inventory_2,
      'cart': Icons.shopping_cart,
      'checkout': Icons.point_of_sale,
      'inventory': Icons.warehouse,
      'customers': Icons.people,
      'promotions': Icons.local_offer,
      'analytics': Icons.trending_up,
      'reports': Icons.assessment,
      'returns': Icons.assignment_return,
    },
    
    enabledModules: [
      'dashboard',
      'catalog',
      'cart',
      'checkout',
      'inventory',
      'customers',
      'promotions',
      'analytics',
      'reports',
      'returns',
      'loyalty',
    ],
    
    customSettings: {
      'currency': 'USD',
      'taxCalculation': true,
      'barcodeScanning': true,
      'loyaltyProgram': true,
      'multiChannel': true,
    },
  );

  /// Salon Luxe - For salons, spas, beauty services
  static const IndustryBranding salonLuxe = IndustryBranding(
    industryType: 'salon',
    brandName: 'Salon Luxe',
    tagline: 'Elevate Your Beauty Business',
    description: 'Premium salon and spa management for beauty professionals',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFFE91E63),        // Pink
      secondary: Color(0xFF9C27B0),      // Purple
      tertiary: Color(0xFFFF4081),       // Pink Accent
      error: Color(0xFFD81B60),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFFF06292),
      secondary: Color(0xFFBA68C8),
      tertiary: Color(0xFFFF5722),
      error: Color(0xFFEF5350),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Color(0xFF4A0E1B),
      onSecondary: Color(0xFF2A0845),
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(),
    logoPath: 'assets/branding/salon/logo.png',
    
    featureIcons: {
      'appointments': Icons.schedule,
      'services': Icons.content_cut,
      'staff': Icons.people,
      'clients': Icons.person,
      'calendar': Icons.calendar_today,
      'products': Icons.shopping_bag,
      'analytics': Icons.insights,
      'payments': Icons.payment,
      'marketing': Icons.campaign,
    },
    
    enabledModules: [
      'dashboard',
      'appointments',
      'services',
      'staff',
      'clients',
      'calendar',
      'products',
      'analytics',
      'payments',
      'marketing',
      'loyalty',
    ],
    
    customSettings: {
      'currency': 'USD',
      'appointmentDuration': 30,
      'commissionTracking': true,
      'clientHistory': true,
      'servicePackages': true,
    },
  );

  /// Event Master - For event planning and management
  static const IndustryBranding eventMaster = IndustryBranding(
    industryType: 'event',
    brandName: 'Event Master',
    tagline: 'Master Every Event',
    description: 'Professional event planning and management platform',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFF1976D2),        // Blue
      secondary: Color(0xFFF50057),      // Pink
      tertiary: Color(0xFF00ACC1),       // Cyan
      error: Color(0xFFD32F2F),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFF42A5F5),
      secondary: Color(0xFFE91E63),
      tertiary: Color(0xFF26C6DA),
      error: Color(0xFFEF5350),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Color(0xFF0D2A4B),
      onSecondary: Color(0xFF4A0E1B),
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(),
    logoPath: 'assets/branding/event/logo.png',
    
    featureIcons: {
      'events': Icons.event,
      'venues': Icons.location_on,
      'tickets': Icons.confirmation_number,
      'attendees': Icons.group,
      'vendors': Icons.business,
      'timeline': Icons.timeline,
      'budget': Icons.account_balance_wallet,
      'marketing': Icons.campaign,
      'analytics': Icons.bar_chart,
    },
    
    enabledModules: [
      'dashboard',
      'events',
      'venues',
      'tickets',
      'attendees',
      'vendors',
      'timeline',
      'budget',
      'marketing',
      'analytics',
      'reports',
    ],
    
    customSettings: {
      'currency': 'USD',
      'ticketing': true,
      'venueManagement': true,
      'vendorCoordination': true,
      'budgetTracking': true,
    },
  );

  /// Hotel Haven - For hospitality and accommodation
  static const IndustryBranding hotelHaven = IndustryBranding(
    industryType: 'hospitality',
    brandName: 'Hotel Haven',
    tagline: 'Your Hospitality Command Center',
    description: 'Complete hotel and hospitality management platform',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFF1A237E),        // Deep Blue
      secondary: Color(0xFFFFB300),      // Amber
      tertiary: Color(0xFF00695C),       // Teal
      error: Color(0xFFD32F2F),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.black,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFF3F51B5),
      secondary: Color(0xFFFFC107),
      tertiary: Color(0xFF009688),
      error: Color(0xFFEF5350),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Colors.white,
      onSecondary: Colors.black,
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(),
    logoPath: 'assets/branding/hotel/logo.png',
    
    featureIcons: {
      'rooms': Icons.hotel,
      'reservations': Icons.book_online,
      'guests': Icons.people,
      'housekeeping': Icons.cleaning_services,
      'concierge': Icons.person_pin_circle,
      'billing': Icons.receipt,
      'amenities': Icons.pool,
      'maintenance': Icons.build,
      'analytics': Icons.insights,
    },
    
    enabledModules: [
      'dashboard',
      'rooms',
      'reservations',
      'guests',
      'housekeeping',
      'concierge',
      'billing',
      'amenities',
      'maintenance',
      'analytics',
      'reports',
    ],
    
    customSettings: {
      'currency': 'USD',
      'roomTypes': ['Standard', 'Deluxe', 'Suite', 'Presidential'],
      'checkInTime': '15:00',
      'checkOutTime': '11:00',
      'housekeepingSchedule': true,
    },
  );

  /// Olympus Default - Generic business platform
  static const IndustryBranding olympusDefault = IndustryBranding(
    industryType: 'general',
    brandName: 'Olympus Cloud',
    tagline: 'Elevate Your Business',
    description: 'Comprehensive business management platform',
    
    lightColorScheme: ColorScheme.light(
      primary: Color(0xFF1E3A8A),        // Deep Blue
      secondary: Color(0xFF10B981),      // Emerald Green
      tertiary: Color(0xFF7C3AED),       // Violet
      error: Color(0xFFDC2626),
      surface: Color(0xFFFFFBFE),
      onSurface: Color(0xFF1C1B1F),
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      outline: Color(0xFFD1D5DB),
    ),
    
    darkColorScheme: ColorScheme.dark(
      primary: Color(0xFF3B82F6),
      secondary: Color(0xFF34D399),
      tertiary: Color(0xFF8B5CF6),
      error: Color(0xFFEF4444),
      surface: Color(0xFF1E1E1E),
      onSurface: Color(0xFFE6E1E5),
      onPrimary: Colors.white,
      onSecondary: Colors.black,
      outline: Color(0xFF374151),
    ),
    
    textTheme: TextTheme(),
    logoPath: 'assets/branding/olympus/logo.png',
    
    featureIcons: {
      'dashboard': Icons.dashboard,
      'analytics': Icons.analytics,
      'customers': Icons.people,
      'orders': Icons.receipt_long,
      'inventory': Icons.inventory,
      'reports': Icons.assessment,
      'settings': Icons.settings,
      'help': Icons.help,
    },
    
    enabledModules: [
      'dashboard',
      'analytics',
      'customers',
      'orders',
      'inventory',
      'reports',
      'settings',
    ],
    
    customSettings: {
      'currency': 'USD',
      'dateFormat': 'MM/DD/YYYY',
      'timeFormat': '12h',
    },
  );

  /// Get all available industry brandings
  static Map<String, IndustryBranding> get all => {
    'restaurant': restaurantRevolution,
    'retail': retailEdge,
    'salon': salonLuxe,
    'event': eventMaster,
    'hospitality': hotelHaven,
    'general': olympusDefault,
  };

  /// Get branding by industry type
  static IndustryBranding getBranding(String industryType) {
    return all[industryType] ?? olympusDefault;
  }

  /// Get list of all industry types
  static List<String> get industryTypes => all.keys.toList();

  /// Create text theme with Google Fonts for the industry
  static TextTheme createTextTheme(String industryType, Brightness brightness) {
    final textColor = brightness == Brightness.light 
        ? const Color(0xFF1C1B1F) 
        : const Color(0xFFE6E1E5);

    switch (industryType) {
      case 'restaurant':
        return GoogleFonts.interTextTheme().copyWith(
          displayLarge: GoogleFonts.playfairDisplay(
            fontSize: 32, fontWeight: FontWeight.w700, color: textColor, height: 1.2,
          ),
          displayMedium: GoogleFonts.playfairDisplay(
            fontSize: 28, fontWeight: FontWeight.w600, color: textColor, height: 1.3,
          ),
        );
      
      case 'salon':
        return GoogleFonts.ralewayTextTheme().copyWith(
          displayLarge: GoogleFonts.cormorant(
            fontSize: 32, fontWeight: FontWeight.w700, color: textColor, height: 1.2,
          ),
          displayMedium: GoogleFonts.cormorant(
            fontSize: 28, fontWeight: FontWeight.w600, color: textColor, height: 1.3,
          ),
        );
      
      case 'event':
        return GoogleFonts.poppinsTextTheme().copyWith(
          displayLarge: GoogleFonts.bebasNeue(
            fontSize: 32, fontWeight: FontWeight.w700, color: textColor, height: 1.2,
          ),
          displayMedium: GoogleFonts.bebasNeue(
            fontSize: 28, fontWeight: FontWeight.w600, color: textColor, height: 1.3,
          ),
        );
      
      default:
        return GoogleFonts.interTextTheme().apply(
          bodyColor: textColor,
          displayColor: textColor,
        );
    }
  }
}