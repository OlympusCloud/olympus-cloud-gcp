import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

/// Industry types supported by the platform
enum IndustryType {
  restaurant,
  retail,
  salon,
  hospitality,
  events,
  other;

  /// Display name for the industry
  String get displayName {
    switch (this) {
      case IndustryType.restaurant:
        return 'Restaurant';
      case IndustryType.retail:
        return 'Retail Store';
      case IndustryType.salon:
        return 'Salon/Spa';
      case IndustryType.hospitality:
        return 'Hospitality';
      case IndustryType.events:
        return 'Event Management';
      case IndustryType.other:
        return 'Other';
    }
  }

  /// Get default features for the industry
  List<String> get defaultFeatures {
    switch (this) {
      case IndustryType.restaurant:
        return [
          'table_management',
          'menu_management',
          'kitchen_display',
          'delivery_tracking',
          'reservation_system',
          'pos_system',
        ];
      case IndustryType.retail:
        return [
          'inventory_management',
          'e_commerce',
          'barcode_scanning',
          'customer_loyalty',
          'pos_system',
          'supplier_management',
        ];
      case IndustryType.salon:
        return [
          'appointment_booking',
          'service_management',
          'staff_scheduling',
          'customer_profiles',
          'inventory_management',
          'pos_system',
        ];
      case IndustryType.hospitality:
        return [
          'room_management',
          'reservation_system',
          'guest_services',
          'housekeeping',
          'event_management',
          'concierge_services',
        ];
      case IndustryType.events:
        return [
          'event_planning',
          'venue_management',
          'vendor_coordination',
          'guest_management',
          'ticketing_system',
          'catering_management',
        ];
      case IndustryType.other:
        return [
          'inventory_management',
          'customer_management',
          'appointment_booking',
          'pos_system',
        ];
    }
  }
}

/// Sub-industry types for specialized branding (e.g., Restaurant Revolution)
enum SubIndustryType {
  // Restaurant sub-types
  restaurantFinedining,
  restaurantCasual,
  restaurantFastFood,
  restaurantBar,
  restaurantNightclub,
  restaurantCafe,
  
  // Retail sub-types
  retailFashion,
  retailElectronics,
  retailGrocery,
  retailPharmacy,
  retailBookstore,
  
  // Salon sub-types
  salonHairStyling,
  salonNailSalon,
  salonSpa,
  salonBarberShop,
  salonMedSpa,
  
  // Events sub-types
  eventsWedding,
  eventsCorporate,
  eventsParty,
  eventsConference,
  eventsConcert,
  
  // Generic
  generic;

  /// Display name for the sub-industry
  String get displayName {
    switch (this) {
      case SubIndustryType.restaurantFinedining:
        return 'Fine Dining';
      case SubIndustryType.restaurantCasual:
        return 'Casual Dining';
      case SubIndustryType.restaurantFastFood:
        return 'Fast Food';
      case SubIndustryType.restaurantBar:
        return 'Bar & Grill';
      case SubIndustryType.restaurantNightclub:
        return 'Nightclub';
      case SubIndustryType.restaurantCafe:
        return 'Cafe';
      case SubIndustryType.retailFashion:
        return 'Fashion Retail';
      case SubIndustryType.retailElectronics:
        return 'Electronics Store';
      case SubIndustryType.retailGrocery:
        return 'Grocery Store';
      case SubIndustryType.retailPharmacy:
        return 'Pharmacy';
      case SubIndustryType.retailBookstore:
        return 'Bookstore';
      case SubIndustryType.salonHairStyling:
        return 'Hair Salon';
      case SubIndustryType.salonNailSalon:
        return 'Nail Salon';
      case SubIndustryType.salonSpa:
        return 'Spa';
      case SubIndustryType.salonBarberShop:
        return 'Barber Shop';
      case SubIndustryType.salonMedSpa:
        return 'Medical Spa';
      case SubIndustryType.eventsWedding:
        return 'Wedding Planning';
      case SubIndustryType.eventsCorporate:
        return 'Corporate Events';
      case SubIndustryType.eventsParty:
        return 'Party Planning';
      case SubIndustryType.eventsConference:
        return 'Conference Management';
      case SubIndustryType.eventsConcert:
        return 'Concert/Festival';
      case SubIndustryType.generic:
        return 'General Business';
    }
  }

  /// Get the parent industry type
  IndustryType get parentIndustry {
    switch (this) {
      case SubIndustryType.restaurantFinedining:
      case SubIndustryType.restaurantCasual:
      case SubIndustryType.restaurantFastFood:
      case SubIndustryType.restaurantBar:
      case SubIndustryType.restaurantNightclub:
      case SubIndustryType.restaurantCafe:
        return IndustryType.restaurant;
      case SubIndustryType.retailFashion:
      case SubIndustryType.retailElectronics:
      case SubIndustryType.retailGrocery:
      case SubIndustryType.retailPharmacy:
      case SubIndustryType.retailBookstore:
        return IndustryType.retail;
      case SubIndustryType.salonHairStyling:
      case SubIndustryType.salonNailSalon:
      case SubIndustryType.salonSpa:
      case SubIndustryType.salonBarberShop:
      case SubIndustryType.salonMedSpa:
        return IndustryType.salon;
      case SubIndustryType.eventsWedding:
      case SubIndustryType.eventsCorporate:
      case SubIndustryType.eventsParty:
      case SubIndustryType.eventsConference:
      case SubIndustryType.eventsConcert:
        return IndustryType.events;
      case SubIndustryType.generic:
        return IndustryType.other;
    }
  }
}

/// Brand configuration for different industry verticals
class IndustryBranding {
  final String brandName;
  final String tagline;
  final IndustryType industry;
  final SubIndustryType? subIndustry;
  final Color primaryColor;
  final Color secondaryColor;
  final Color accentColor;
  final String logoAssetPath;
  final String iconAssetPath;
  final TextStyle primaryFont;
  final TextStyle headingFont;
  final Map<String, Color> statusColors;
  final Map<String, IconData> featureIcons;

  IndustryBranding({
    required this.brandName,
    required this.tagline,
    required this.industry,
    this.subIndustry,
    required this.primaryColor,
    required this.secondaryColor,
    required this.accentColor,
    required this.logoAssetPath,
    required this.iconAssetPath,
    required this.primaryFont,
    required this.headingFont,
    required this.statusColors,
    required this.featureIcons,
  });

  /// Generate a complete theme based on this branding
  ThemeData generateTheme({bool isDark = false}) {
    final brightness = isDark ? Brightness.dark : Brightness.light;
    
    // Background colors
    final backgroundColor = isDark ? const Color(0xFF111827) : const Color(0xFFFAFAFA);
    final surfaceColor = isDark ? const Color(0xFF1F2937) : const Color(0xFFFFFFFF);
    final onBackgroundColor = isDark ? const Color(0xFFF9FAFB) : const Color(0xFF1F2937);
    final onSurfaceColor = isDark ? const Color(0xFFE5E7EB) : const Color(0xFF374151);

    return ThemeData(
      useMaterial3: true,
      brightness: brightness,
      colorScheme: ColorScheme.fromSeed(
        seedColor: primaryColor,
        brightness: brightness,
        primary: primaryColor,
        secondary: secondaryColor,
        tertiary: accentColor,
        surface: backgroundColor,
        onSurface: onBackgroundColor,
      ),
      textTheme: _generateTextTheme(brightness, onBackgroundColor),
      appBarTheme: AppBarTheme(
        elevation: 0,
        centerTitle: false,
        backgroundColor: surfaceColor,
        foregroundColor: onSurfaceColor,
        titleTextStyle: headingFont.copyWith(
          fontSize: 18,
          fontWeight: FontWeight.w600,
          color: onSurfaceColor,
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          minimumSize: const Size(0, 48),
          backgroundColor: primaryColor,
          foregroundColor: Colors.white,
          elevation: 2,
          shadowColor: primaryColor.withOpacity(0.3),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
          textStyle: primaryFont.copyWith(
            fontSize: 14,
            fontWeight: FontWeight.w600,
          ),
        ),
      ),
      cardTheme: CardThemeData(
        elevation: 2,
        shadowColor: Colors.black.withOpacity(0.1),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
        color: surfaceColor,
      ),
    );
  }

  /// Generate text theme with brand fonts
  TextTheme _generateTextTheme(Brightness brightness, Color textColor) {
    return TextTheme(
      displayLarge: headingFont.copyWith(
        fontSize: 32,
        fontWeight: FontWeight.w700,
        color: textColor,
        height: 1.2,
      ),
      displayMedium: headingFont.copyWith(
        fontSize: 28,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.3,
      ),
      displaySmall: headingFont.copyWith(
        fontSize: 24,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.3,
      ),
      headlineLarge: headingFont.copyWith(
        fontSize: 22,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.4,
      ),
      headlineMedium: headingFont.copyWith(
        fontSize: 20,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.4,
      ),
      headlineSmall: headingFont.copyWith(
        fontSize: 18,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      titleLarge: primaryFont.copyWith(
        fontSize: 16,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      titleMedium: primaryFont.copyWith(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      titleSmall: primaryFont.copyWith(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      bodyLarge: primaryFont.copyWith(
        fontSize: 16,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      bodyMedium: primaryFont.copyWith(
        fontSize: 14,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      bodySmall: primaryFont.copyWith(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      labelLarge: primaryFont.copyWith(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      labelMedium: primaryFont.copyWith(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      labelSmall: primaryFont.copyWith(
        fontSize: 10,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
    );
  }
}

/// Predefined industry brandings
class IndustryBrandings {
  /// Restaurant Revolution branding for restaurants, bars, and nightclubs
  static final restaurantRevolution = IndustryBranding(
    brandName: 'Restaurant Revolution',
    tagline: 'Transforming dining experiences',
    industry: IndustryType.restaurant,
    primaryColor: Color(0xFFDC2626), // Bold red
    secondaryColor: Color(0xFFF59E0B), // Warm amber
    accentColor: Color(0xFF059669), // Fresh green
    logoAssetPath: 'assets/branding/restaurant_revolution_logo.png',
    iconAssetPath: 'assets/branding/restaurant_revolution_icon.png',
    primaryFont: GoogleFonts.roboto(),
    headingFont: GoogleFonts.playfairDisplay(),
    statusColors: {
      'active': Color(0xFF059669),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFDC2626),
      'completed': Color(0xFF059669),
      'draft': Color(0xFF6B7280),
      'occupied': Color(0xFFDC2626),
      'available': Color(0xFF059669),
      'reserved': Color(0xFFF59E0B),
    },
    featureIcons: {
      'table_management': Icons.table_restaurant,
      'menu_management': Icons.restaurant_menu,
      'kitchen_display': Icons.kitchen,
      'delivery_tracking': Icons.delivery_dining,
      'reservation_system': Icons.event_seat,
      'pos_system': Icons.point_of_sale,
    },
  );

  /// Retail Pro branding for retail stores
  static final retailPro = IndustryBranding(
    brandName: 'Retail Pro',
    tagline: 'Empowering retail excellence',
    industry: IndustryType.retail,
    primaryColor: Color(0xFF8B5CF6), // Royal purple
    secondaryColor: Color(0xFF06B6D4), // Bright cyan
    accentColor: Color(0xFFEF4444), // Vibrant red
    logoAssetPath: 'assets/branding/retail_pro_logo.png',
    iconAssetPath: 'assets/branding/retail_pro_icon.png',
    primaryFont: GoogleFonts.inter(),
    headingFont: GoogleFonts.montserrat(),
    statusColors: {
      'active': Color(0xFF059669),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFEF4444),
      'completed': Color(0xFF059669),
      'draft': Color(0xFF6B7280),
      'in_stock': Color(0xFF059669),
      'low_stock': Color(0xFFF59E0B),
      'out_of_stock': Color(0xFFEF4444),
    },
    featureIcons: {
      'inventory_management': Icons.inventory,
      'e_commerce': Icons.shopping_cart,
      'barcode_scanning': Icons.qr_code_scanner,
      'customer_loyalty': Icons.loyalty,
      'pos_system': Icons.point_of_sale,
      'supplier_management': Icons.local_shipping,
    },
  );

  /// Salon Suite branding for salons and spas
  static final salonSuite = IndustryBranding(
    brandName: 'Salon Suite',
    tagline: 'Beauty and wellness redefined',
    industry: IndustryType.salon,
    primaryColor: Color(0xFFEC4899), // Elegant pink
    secondaryColor: Color(0xFF8B5CF6), // Luxurious purple
    accentColor: Color(0xFF10B981), // Fresh mint
    logoAssetPath: 'assets/branding/salon_suite_logo.png',
    iconAssetPath: 'assets/branding/salon_suite_icon.png',
    primaryFont: GoogleFonts.poppins(),
    headingFont: GoogleFonts.playfairDisplay(),
    statusColors: {
      'active': Color(0xFF10B981),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFEF4444),
      'completed': Color(0xFF10B981),
      'draft': Color(0xFF6B7280),
      'booked': Color(0xFFEC4899),
      'available': Color(0xFF10B981),
      'blocked': Color(0xFF6B7280),
    },
    featureIcons: {
      'appointment_booking': Icons.event,
      'service_management': Icons.spa,
      'staff_scheduling': Icons.schedule,
      'customer_profiles': Icons.person,
      'inventory_management': Icons.inventory,
      'pos_system': Icons.point_of_sale,
    },
  );

  /// Events Master branding for event management
  static final eventsMaster = IndustryBranding(
    brandName: 'Events Master',
    tagline: 'Creating unforgettable moments',
    industry: IndustryType.events,
    primaryColor: Color(0xFF06B6D4), // Dynamic cyan
    secondaryColor: Color(0xFF8B5CF6), // Creative purple
    accentColor: Color(0xFFF59E0B), // Energetic amber
    logoAssetPath: 'assets/branding/events_master_logo.png',
    iconAssetPath: 'assets/branding/events_master_icon.png',
    primaryFont: GoogleFonts.openSans(),
    headingFont: GoogleFonts.oswald(),
    statusColors: {
      'active': Color(0xFF059669),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFEF4444),
      'completed': Color(0xFF059669),
      'draft': Color(0xFF6B7280),
      'confirmed': Color(0xFF06B6D4),
      'tentative': Color(0xFFF59E0B),
      'postponed': Color(0xFF8B5CF6),
    },
    featureIcons: {
      'event_planning': Icons.event,
      'venue_management': Icons.location_on,
      'vendor_coordination': Icons.handshake,
      'guest_management': Icons.groups,
      'ticketing_system': Icons.confirmation_number,
      'catering_management': Icons.restaurant,
    },
  );

  /// Hotel Haven branding for hospitality
  static final hotelHaven = IndustryBranding(
    brandName: 'Hotel Haven',
    tagline: 'Hospitality excellence',
    industry: IndustryType.hospitality,
    primaryColor: Color(0xFF1E3A8A), // Classic navy
    secondaryColor: Color(0xFF10B981), // Welcoming green
    accentColor: Color(0xFFF59E0B), // Warm gold
    logoAssetPath: 'assets/branding/hotel_haven_logo.png',
    iconAssetPath: 'assets/branding/hotel_haven_icon.png',
    primaryFont: GoogleFonts.sourceSerif4(),
    headingFont: GoogleFonts.cormorantGaramond(),
    statusColors: {
      'active': Color(0xFF10B981),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFEF4444),
      'completed': Color(0xFF10B981),
      'draft': Color(0xFF6B7280),
      'occupied': Color(0xFF1E3A8A),
      'available': Color(0xFF10B981),
      'maintenance': Color(0xFFF59E0B),
      'out_of_order': Color(0xFFEF4444),
    },
    featureIcons: {
      'room_management': Icons.hotel,
      'reservation_system': Icons.event_seat,
      'guest_services': Icons.support_agent,
      'housekeeping': Icons.cleaning_services,
      'event_management': Icons.event,
      'concierge_services': Icons.support_agent,
    },
  );

  /// Generic Olympus branding for other industries
  static final olympus = IndustryBranding(
    brandName: 'Olympus',
    tagline: 'Business excellence platform',
    industry: IndustryType.other,
    primaryColor: Color(0xFF1E3A8A), // Deep blue
    secondaryColor: Color(0xFF10B981), // Emerald green
    accentColor: Color(0xFF8B5CF6), // Purple
    logoAssetPath: 'assets/branding/olympus_logo.png',
    iconAssetPath: 'assets/branding/olympus_icon.png',
    primaryFont: GoogleFonts.inter(),
    headingFont: GoogleFonts.inter(),
    statusColors: {
      'active': Color(0xFF10B981),
      'inactive': Color(0xFF6B7280),
      'pending': Color(0xFFF59E0B),
      'cancelled': Color(0xFFEF4444),
      'completed': Color(0xFF10B981),
      'draft': Color(0xFF6B7280),
    },
    featureIcons: {
      'inventory_management': Icons.inventory,
      'customer_management': Icons.people,
      'appointment_booking': Icons.event,
      'pos_system': Icons.point_of_sale,
    },
  );

  /// Get branding based on industry type
  static IndustryBranding getBrandingForIndustry(IndustryType industry, {SubIndustryType? subIndustry}) {
    switch (industry) {
      case IndustryType.restaurant:
        return restaurantRevolution;
      case IndustryType.retail:
        return retailPro;
      case IndustryType.salon:
        return salonSuite;
      case IndustryType.events:
        return eventsMaster;
      case IndustryType.hospitality:
        return hotelHaven;
      case IndustryType.other:
        return olympus;
    }
  }

  /// Get all available brandings
  static List<IndustryBranding> get allBrandings => [
    restaurantRevolution,
    retailPro,
    salonSuite,
    eventsMaster,
    hotelHaven,
    olympus,
  ];
}