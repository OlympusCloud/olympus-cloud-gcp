import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../constants/app_constants.dart';
import '../services/storage_service.dart';

/// Theme mode provider
final themeModeProvider = StateNotifierProvider<ThemeModeNotifier, bool>((ref) {
  return ThemeModeNotifier();
});

/// Theme mode notifier
class ThemeModeNotifier extends StateNotifier<bool> {
  ThemeModeNotifier() : super(false) {
    _loadThemeMode();
  }

  /// Load theme mode from storage
  Future<void> _loadThemeMode() async {
    final isDarkMode = StorageService.getUserData<bool>(AppConstants.themeKey) ?? false;
    state = isDarkMode;
  }

  /// Toggle theme mode
  Future<void> toggleTheme() async {
    state = !state;
    await StorageService.saveUserData(AppConstants.themeKey, state);
  }

  /// Set specific theme mode
  Future<void> setDarkMode(bool isDark) async {
    state = isDark;
    await StorageService.saveUserData(AppConstants.themeKey, state);
  }
}

/// Language provider
final languageProvider = StateNotifierProvider<LanguageNotifier, String>((ref) {
  return LanguageNotifier();
});

/// Language notifier
class LanguageNotifier extends StateNotifier<String> {
  LanguageNotifier() : super('en_US') {
    _loadLanguage();
  }

  /// Load language from storage
  Future<void> _loadLanguage() async {
    final language = StorageService.getUserData<String>(AppConstants.languageKey) ?? 'en_US';
    state = language;
  }

  /// Set language
  Future<void> setLanguage(String languageCode) async {
    if (AppConstants.supportedLocales.contains(languageCode)) {
      state = languageCode;
      await StorageService.saveUserData(AppConstants.languageKey, languageCode);
    }
  }
}

/// App settings provider
final appSettingsProvider = StateNotifierProvider<AppSettingsNotifier, AppSettings>((ref) {
  return AppSettingsNotifier();
});

/// App settings state
class AppSettings {
  final bool enableNotifications;
  final bool enableAnalytics;
  final bool enableNaturalLanguage;
  final bool enableWebSocket;
  final String defaultBusinessView;
  final int autoLogoutMinutes;

  const AppSettings({
    this.enableNotifications = true,
    this.enableAnalytics = true,
    this.enableNaturalLanguage = true,
    this.enableWebSocket = true,
    this.defaultBusinessView = 'dashboard',
    this.autoLogoutMinutes = 30,
  });

  AppSettings copyWith({
    bool? enableNotifications,
    bool? enableAnalytics,
    bool? enableNaturalLanguage,
    bool? enableWebSocket,
    String? defaultBusinessView,
    int? autoLogoutMinutes,
  }) {
    return AppSettings(
      enableNotifications: enableNotifications ?? this.enableNotifications,
      enableAnalytics: enableAnalytics ?? this.enableAnalytics,
      enableNaturalLanguage: enableNaturalLanguage ?? this.enableNaturalLanguage,
      enableWebSocket: enableWebSocket ?? this.enableWebSocket,
      defaultBusinessView: defaultBusinessView ?? this.defaultBusinessView,
      autoLogoutMinutes: autoLogoutMinutes ?? this.autoLogoutMinutes,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'enableNotifications': enableNotifications,
      'enableAnalytics': enableAnalytics,
      'enableNaturalLanguage': enableNaturalLanguage,
      'enableWebSocket': enableWebSocket,
      'defaultBusinessView': defaultBusinessView,
      'autoLogoutMinutes': autoLogoutMinutes,
    };
  }

  factory AppSettings.fromJson(Map<String, dynamic> json) {
    return AppSettings(
      enableNotifications: json['enableNotifications'] ?? true,
      enableAnalytics: json['enableAnalytics'] ?? true,
      enableNaturalLanguage: json['enableNaturalLanguage'] ?? true,
      enableWebSocket: json['enableWebSocket'] ?? true,
      defaultBusinessView: json['defaultBusinessView'] ?? 'dashboard',
      autoLogoutMinutes: json['autoLogoutMinutes'] ?? 30,
    );
  }
}

/// App settings notifier
class AppSettingsNotifier extends StateNotifier<AppSettings> {
  AppSettingsNotifier() : super(const AppSettings()) {
    _loadSettings();
  }

  /// Load settings from storage
  Future<void> _loadSettings() async {
    final settingsData = StorageService.getUserData<Map<String, dynamic>>(AppConstants.settingsKey);
    if (settingsData != null) {
      state = AppSettings.fromJson(settingsData);
    }
  }

  /// Update settings
  Future<void> updateSettings(AppSettings newSettings) async {
    state = newSettings;
    await StorageService.saveUserData(AppConstants.settingsKey, state.toJson());
  }

  /// Toggle specific setting
  Future<void> toggleNotifications() async {
    final newSettings = state.copyWith(enableNotifications: !state.enableNotifications);
    await updateSettings(newSettings);
  }

  Future<void> toggleAnalytics() async {
    final newSettings = state.copyWith(enableAnalytics: !state.enableAnalytics);
    await updateSettings(newSettings);
  }

  Future<void> toggleNaturalLanguage() async {
    final newSettings = state.copyWith(enableNaturalLanguage: !state.enableNaturalLanguage);
    await updateSettings(newSettings);
  }

  Future<void> toggleWebSocket() async {
    final newSettings = state.copyWith(enableWebSocket: !state.enableWebSocket);
    await updateSettings(newSettings);
  }

  /// Set default business view
  Future<void> setDefaultView(String view) async {
    final newSettings = state.copyWith(defaultBusinessView: view);
    await updateSettings(newSettings);
  }

  /// Set auto logout time
  Future<void> setAutoLogoutMinutes(int minutes) async {
    final newSettings = state.copyWith(autoLogoutMinutes: minutes);
    await updateSettings(newSettings);
  }
}