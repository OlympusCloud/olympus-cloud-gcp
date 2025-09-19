import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../auth/auth_models.dart';

/// Storage service for app data persistence
class StorageService {
  static const String _authBoxName = 'auth';
  static const String _appBoxName = 'app';
  
  static const String _accessTokenKey = 'access_token';
  static const String _refreshTokenKey = 'refresh_token';
  static const String _currentUserKey = 'current_user';
  static const String _industryKey = 'selected_industry';
  static const String _brandingKey = 'selected_branding';
  static const String _onboardingCompleteKey = 'onboarding_complete';
  static const String _themeKey = 'theme_mode';
  static const String _localeKey = 'locale';
  
  late final Box _authBox;
  late final Box _appBox;
  late final SharedPreferences _prefs;
  
  /// Initialize storage service
  Future<void> initialize() async {
    await Hive.initFlutter();
    _authBox = await Hive.openBox(_authBoxName);
    _appBox = await Hive.openBox(_appBoxName);
    _prefs = await SharedPreferences.getInstance();
  }
  
  /// Authentication token methods
  
  Future<void> saveTokens({
    required String accessToken,
    required String refreshToken,
  }) async {
    await _authBox.put(_accessTokenKey, accessToken);
    await _authBox.put(_refreshTokenKey, refreshToken);
  }
  
  Future<String?> getAccessToken() async {
    return _authBox.get(_accessTokenKey);
  }
  
  Future<String?> getRefreshToken() async {
    return _authBox.get(_refreshTokenKey);
  }
  
  /// User data methods
  
  Future<void> saveCurrentUser(User user) async {
    await _authBox.put(_currentUserKey, jsonEncode(user.toJson()));
  }
  
  Future<User?> getCurrentUser() async {
    final userJson = _authBox.get(_currentUserKey);
    if (userJson == null) return null;
    
    try {
      final userData = jsonDecode(userJson);
      return User.fromJson(userData);
    } catch (e) {
      return null;
    }
  }
  
  /// Clear all authentication data
  Future<void> clearAuth() async {
    await _authBox.delete(_accessTokenKey);
    await _authBox.delete(_refreshTokenKey);
    await _authBox.delete(_currentUserKey);
  }
  
  /// Industry and branding methods
  
  Future<void> saveSelectedIndustry(String industry) async {
    await _appBox.put(_industryKey, industry);
  }
  
  Future<String?> getSelectedIndustry() async {
    return _appBox.get(_industryKey);
  }
  
  Future<void> saveSelectedBranding(String branding) async {
    await _appBox.put(_brandingKey, branding);
  }
  
  Future<String?> getSelectedBranding() async {
    return _appBox.get(_brandingKey);
  }
  
  /// Onboarding methods
  
  Future<void> setOnboardingComplete(bool complete) async {
    await _appBox.put(_onboardingCompleteKey, complete);
  }
  
  Future<bool> isOnboardingComplete() async {
    return _appBox.get(_onboardingCompleteKey, defaultValue: false);
  }
  
  /// Theme methods
  
  Future<void> saveThemeMode(String themeMode) async {
    await _appBox.put(_themeKey, themeMode);
  }
  
  Future<String?> getThemeMode() async {
    return _appBox.get(_themeKey);
  }
  
  /// Locale methods
  
  Future<void> saveLocale(String locale) async {
    await _appBox.put(_localeKey, locale);
  }
  
  Future<String?> getLocale() async {
    return _appBox.get(_localeKey);
  }
  
  /// Generic methods for app-specific data
  
  Future<void> setValue(String key, dynamic value) async {
    await _appBox.put(key, value);
  }
  
  T? getValue<T>(String key, {T? defaultValue}) {
    return _appBox.get(key, defaultValue: defaultValue);
  }
  
  Future<void> removeValue(String key) async {
    await _appBox.delete(key);
  }
  
  /// Clear all app data
  Future<void> clearAll() async {
    await _authBox.clear();
    await _appBox.clear();
    await _prefs.clear();
  }
  
  /// Secure preferences for sensitive data
  
  Future<void> setSecureString(String key, String value) async {
    await _prefs.setString(key, value);
  }
  
  Future<String?> getSecureString(String key) async {
    return _prefs.getString(key);
  }
  
  Future<void> setSecureBool(String key, bool value) async {
    await _prefs.setBool(key, value);
  }
  
  Future<bool?> getSecureBool(String key) async {
    return _prefs.getBool(key);
  }
  
  Future<void> setSecureInt(String key, int value) async {
    await _prefs.setInt(key, value);
  }
  
  Future<int?> getSecureInt(String key) async {
    return _prefs.getInt(key);
  }
  
  Future<void> removeSecure(String key) async {
    await _prefs.remove(key);
  }
}

/// Provider for StorageService
final storageServiceProvider = Provider<StorageService>((ref) {
  throw UnimplementedError('StorageService must be overridden in main()');
});