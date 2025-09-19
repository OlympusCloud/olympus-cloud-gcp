import 'package:flutter/foundation.dart';

/// Application-wide constants
class AppConstants {
  // App Information
  static const String appName = 'Olympus Cloud Business OS';
  static const String appVersion = '1.0.0';
  static const String appBuildNumber = '1';

  // Environment Configuration
  static const bool isDebugMode = kDebugMode;
  static const bool isProfileMode = kProfileMode;
  static const bool isReleaseMode = kReleaseMode;

  // API Configuration
  static const String apiUrl = String.fromEnvironment(
    'API_URL',
    defaultValue: 'http://localhost:8081/api/v1',
  );

  static const String wsUrl = String.fromEnvironment(
    'WS_URL', 
    defaultValue: 'ws://localhost:8081/ws',
  );
  
  // WebSocket URL for service
  static const String websocketUrl = String.fromEnvironment(
    'WS_URL', 
    defaultValue: 'ws://localhost:8081/ws',
  );

  // API Endpoints
  static const String authEndpoint = '/api/v1/auth';
  static const String platformEndpoint = '/api/v1/platform';
  static const String commerceEndpoint = '/api/v1/commerce';
  static const String analyticsEndpoint = '/api/v1/analytics';

  // Storage Keys
  static const String accessTokenKey = 'access_token';
  static const String refreshTokenKey = 'refresh_token';
  static const String userDataKey = 'user_data';
  static const String tenantDataKey = 'tenant_data';
  static const String settingsKey = 'app_settings';
  static const String firstTimeKey = 'first_time_user';
  static const String themeKey = 'theme_mode';
  static const String languageKey = 'language_code';

  // UI Constants
  static const double mobileBreakpoint = 600;
  static const double tabletBreakpoint = 900;
  static const double desktopBreakpoint = 1200;

  // Animation Durations
  static const Duration fastAnimation = Duration(milliseconds: 200);
  static const Duration normalAnimation = Duration(milliseconds: 300);
  static const Duration slowAnimation = Duration(milliseconds: 500);

  // Network
  static const Duration networkTimeout = Duration(seconds: 30);
  static const int maxRetries = 3;

  // Pagination
  static const int defaultPageSize = 20;
  static const int maxPageSize = 100;

  // File Upload
  static const int maxFileSize = 10 * 1024 * 1024; // 10MB
  static const List<String> allowedImageTypes = [
    'image/jpeg',
    'image/png',
    'image/gif',
    'image/webp',
  ];

  // Business Types
  static const List<String> supportedBusinessTypes = [
    'restaurant',
    'retail',
    'salon',
    'events',
    'services',
  ];

  // Supported Locales
  static const List<String> supportedLocales = [
    'en_US',
    'es_ES', 
    'fr_FR',
    'de_DE',
    'ja_JP',
  ];
}