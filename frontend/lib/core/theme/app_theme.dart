import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

/// Application theme configuration
class AppTheme {
  static const Color _primaryColor = Color(0xFF1E3A8A); // Deep blue
  static const Color _secondaryColor = Color(0xFF10B981); // Emerald green
  static const Color _errorColor = Color(0xFFDC2626); // Red
  static const Color _warningColor = Color(0xFFF59E0B); // Amber
  static const Color _successColor = Color(0xFF059669); // Green
  static const Color _infoColor = Color(0xFF3B82F6); // Blue

  // Light theme colors
  static const Color _lightBackground = Color(0xFFFAFAFA);
  static const Color _lightSurface = Color(0xFFFFFFFF);
  static const Color _lightOnBackground = Color(0xFF1F2937);
  static const Color _lightOnSurface = Color(0xFF374151);

  // Dark theme colors
  static const Color _darkBackground = Color(0xFF111827);
  static const Color _darkSurface = Color(0xFF1F2937);
  static const Color _darkOnBackground = Color(0xFFF9FAFB);
  static const Color _darkOnSurface = Color(0xFFE5E7EB);

  /// Light theme configuration
  static ThemeData lightTheme = ThemeData(
    useMaterial3: true,
    brightness: Brightness.light,
    colorScheme: const ColorScheme.light(
      primary: _primaryColor,
      secondary: _secondaryColor,
      error: _errorColor,
      surface: _lightBackground,
      onSurface: _lightOnBackground,
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      onError: Colors.white,
    ),
    textTheme: _getTextTheme(Brightness.light),
    appBarTheme: _getAppBarTheme(Brightness.light),
    elevatedButtonTheme: _getElevatedButtonTheme(),
    outlinedButtonTheme: _getOutlinedButtonTheme(),
    textButtonTheme: _getTextButtonTheme(),
    inputDecorationTheme: _getInputDecorationTheme(Brightness.light),
    cardTheme: _getCardTheme(Brightness.light),
    dialogTheme: _getDialogTheme(),
    snackBarTheme: _getSnackBarTheme(),
    bottomNavigationBarTheme: _getBottomNavTheme(Brightness.light),
    navigationRailTheme: _getNavigationRailTheme(Brightness.light),
    drawerTheme: _getDrawerTheme(Brightness.light),
    dividerTheme: const DividerThemeData(
      color: Color(0xFFE5E7EB),
      thickness: 1,
    ),
  );

  /// Dark theme configuration
  static ThemeData darkTheme = ThemeData(
    useMaterial3: true,
    brightness: Brightness.dark,
    colorScheme: const ColorScheme.dark(
      primary: _primaryColor,
      secondary: _secondaryColor,
      error: _errorColor,
      surface: _darkBackground,
      onSurface: _darkOnBackground,
      onPrimary: Colors.white,
      onSecondary: Colors.white,
      onError: Colors.white,
    ),
    textTheme: _getTextTheme(Brightness.dark),
    appBarTheme: _getAppBarTheme(Brightness.dark),
    elevatedButtonTheme: _getElevatedButtonTheme(),
    outlinedButtonTheme: _getOutlinedButtonTheme(),
    textButtonTheme: _getTextButtonTheme(),
    inputDecorationTheme: _getInputDecorationTheme(Brightness.dark),
    cardTheme: _getCardTheme(Brightness.dark),
    dialogTheme: _getDialogTheme(),
    snackBarTheme: _getSnackBarTheme(),
    bottomNavigationBarTheme: _getBottomNavTheme(Brightness.dark),
    navigationRailTheme: _getNavigationRailTheme(Brightness.dark),
    drawerTheme: _getDrawerTheme(Brightness.dark),
    dividerTheme: const DividerThemeData(
      color: Color(0xFF374151),
      thickness: 1,
    ),
  );

  /// Get text theme for the app
  static TextTheme _getTextTheme(Brightness brightness) {
    final textColor = brightness == Brightness.light 
        ? _lightOnBackground 
        : _darkOnBackground;
    
    return GoogleFonts.interTextTheme().copyWith(
      displayLarge: GoogleFonts.inter(
        fontSize: 32,
        fontWeight: FontWeight.w700,
        color: textColor,
        height: 1.2,
      ),
      displayMedium: GoogleFonts.inter(
        fontSize: 28,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.3,
      ),
      displaySmall: GoogleFonts.inter(
        fontSize: 24,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.3,
      ),
      headlineLarge: GoogleFonts.inter(
        fontSize: 22,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.4,
      ),
      headlineMedium: GoogleFonts.inter(
        fontSize: 20,
        fontWeight: FontWeight.w600,
        color: textColor,
        height: 1.4,
      ),
      headlineSmall: GoogleFonts.inter(
        fontSize: 18,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      titleLarge: GoogleFonts.inter(
        fontSize: 16,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      titleMedium: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      titleSmall: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.5,
      ),
      bodyLarge: GoogleFonts.inter(
        fontSize: 16,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      bodyMedium: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      bodySmall: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: textColor,
        height: 1.6,
      ),
      labelLarge: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      labelMedium: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
      labelSmall: GoogleFonts.inter(
        fontSize: 10,
        fontWeight: FontWeight.w500,
        color: textColor,
        height: 1.4,
      ),
    );
  }

  /// App bar theme
  static AppBarTheme _getAppBarTheme(Brightness brightness) {
    return AppBarTheme(
      elevation: 0,
      centerTitle: false,
      backgroundColor: brightness == Brightness.light 
          ? _lightSurface 
          : _darkSurface,
      foregroundColor: brightness == Brightness.light 
          ? _lightOnSurface 
          : _darkOnSurface,
      titleTextStyle: GoogleFonts.inter(
        fontSize: 18,
        fontWeight: FontWeight.w600,
        color: brightness == Brightness.light 
            ? _lightOnSurface 
            : _darkOnSurface,
      ),
    );
  }

  /// Elevated button theme
  static ElevatedButtonThemeData _getElevatedButtonTheme() {
    return ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        minimumSize: const Size(0, 48),
        backgroundColor: _primaryColor,
        foregroundColor: Colors.white,
        elevation: 2,
        shadowColor: _primaryColor.withAlpha(77),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
        textStyle: GoogleFonts.inter(
          fontSize: 14,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  /// Outlined button theme
  static OutlinedButtonThemeData _getOutlinedButtonTheme() {
    return OutlinedButtonThemeData(
      style: OutlinedButton.styleFrom(
        minimumSize: const Size(0, 48),
        foregroundColor: _primaryColor,
        side: const BorderSide(color: _primaryColor, width: 1.5),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
        textStyle: GoogleFonts.inter(
          fontSize: 14,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  /// Text button theme
  static TextButtonThemeData _getTextButtonTheme() {
    return TextButtonThemeData(
      style: TextButton.styleFrom(
        minimumSize: const Size(0, 48),
        foregroundColor: _primaryColor,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
        textStyle: GoogleFonts.inter(
          fontSize: 14,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  /// Input decoration theme
  static InputDecorationTheme _getInputDecorationTheme(Brightness brightness) {
    final borderColor = brightness == Brightness.light 
        ? const Color(0xFFD1D5DB) 
        : const Color(0xFF374151);
    
    final fillColor = brightness == Brightness.light 
        ? _lightSurface 
        : _darkSurface;
    
    return InputDecorationTheme(
      filled: true,
      fillColor: fillColor,
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: borderColor),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: borderColor),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: _primaryColor, width: 2),
      ),
      errorBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: _errorColor, width: 1),
      ),
      focusedErrorBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: _errorColor, width: 2),
      ),
      labelStyle: GoogleFonts.inter(fontSize: 14),
      hintStyle: GoogleFonts.inter(
        fontSize: 14,
        color: brightness == Brightness.light 
            ? const Color(0xFF9CA3AF) 
            : const Color(0xFF6B7280),
      ),
    );
  }

  /// Card theme
  static CardThemeData _getCardTheme(Brightness brightness) {
    return CardThemeData(
      elevation: 2,
      shadowColor: Colors.black.withAlpha(25),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      color: brightness == Brightness.light 
          ? _lightSurface 
          : _darkSurface,
    );
  }

  /// Dialog theme
  static DialogThemeData _getDialogTheme() {
    return DialogThemeData(
      elevation: 8,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
    );
  }

  /// SnackBar theme
  static SnackBarThemeData _getSnackBarTheme() {
    return SnackBarThemeData(
      behavior: SnackBarBehavior.floating,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(8),
      ),
      contentTextStyle: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: Colors.white,
      ),
    );
  }

  /// Bottom navigation bar theme
  static BottomNavigationBarThemeData _getBottomNavTheme(Brightness brightness) {
    return BottomNavigationBarThemeData(
      type: BottomNavigationBarType.fixed,
      backgroundColor: brightness == Brightness.light 
          ? _lightSurface 
          : _darkSurface,
      selectedItemColor: _primaryColor,
      unselectedItemColor: brightness == Brightness.light 
          ? const Color(0xFF6B7280) 
          : const Color(0xFF9CA3AF),
      selectedLabelStyle: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w600,
      ),
      unselectedLabelStyle: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w500,
      ),
      elevation: 8,
    );
  }

  /// Navigation rail theme
  static NavigationRailThemeData _getNavigationRailTheme(Brightness brightness) {
    return NavigationRailThemeData(
      backgroundColor: brightness == Brightness.light 
          ? _lightSurface 
          : _darkSurface,
      selectedIconTheme: const IconThemeData(
        color: _primaryColor,
        size: 24,
      ),
      unselectedIconTheme: IconThemeData(
        color: brightness == Brightness.light 
            ? const Color(0xFF6B7280) 
            : const Color(0xFF9CA3AF),
        size: 24,
      ),
      selectedLabelTextStyle: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w600,
        color: _primaryColor,
      ),
      unselectedLabelTextStyle: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: brightness == Brightness.light 
            ? const Color(0xFF6B7280) 
            : const Color(0xFF9CA3AF),
      ),
    );
  }

  /// Drawer theme
  static DrawerThemeData _getDrawerTheme(Brightness brightness) {
    return DrawerThemeData(
      backgroundColor: brightness == Brightness.light 
          ? _lightSurface 
          : _darkSurface,
      elevation: 16,
    );
  }

  /// Custom color palette for business-specific use cases
  static const Map<String, Color> customColors = {
    'success': _successColor,
    'warning': _warningColor,
    'info': _infoColor,
    'neutral': Color(0xFF6B7280),
    'restaurant': Color(0xFFEF4444), // Red for restaurants
    'retail': Color(0xFF8B5CF6), // Purple for retail
    'salon': Color(0xFFEC4899), // Pink for salons
    'events': Color(0xFF06B6D4), // Cyan for events
  };

  /// Status colors for different states
  static const Map<String, Color> statusColors = {
    'active': _successColor,
    'inactive': Color(0xFF6B7280),
    'pending': _warningColor,
    'cancelled': _errorColor,
    'completed': _successColor,
    'draft': Color(0xFF6B7280),
  };
}