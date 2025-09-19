import 'package:hive_flutter/hive_flutter.dart';

/// Service for managing local storage using Hive
class StorageService {
  static late Box _settingsBox;
  static late Box _userBox;
  static late Box _cacheBox;

  // Box names
  static const String _settingsBoxName = 'settings';
  static const String _userBoxName = 'user';
  static const String _cacheBoxName = 'cache';

  /// Initialize storage service
  static Future<void> initialize() async {
    try {
      _settingsBox = await Hive.openBox(_settingsBoxName);
      _userBox = await Hive.openBox(_userBoxName);
      _cacheBox = await Hive.openBox(_cacheBoxName);
    } catch (e) {
      throw Exception('Failed to initialize storage: $e');
    }
  }

  // Settings Storage
  static Future<void> saveSetting(String key, dynamic value) async {
    await _settingsBox.put(key, value);
  }

  static T? getSetting<T>(String key, {T? defaultValue}) {
    return _settingsBox.get(key, defaultValue: defaultValue) as T?;
  }

  static Future<void> removeSetting(String key) async {
    await _settingsBox.delete(key);
  }

  // User Data Storage
  static Future<void> saveUserData(String key, dynamic value) async {
    await _userBox.put(key, value);
  }

  static T? getUserData<T>(String key, {T? defaultValue}) {
    return _userBox.get(key, defaultValue: defaultValue) as T?;
  }

  static Future<void> removeUserData(String key) async {
    await _userBox.delete(key);
  }

  static Future<void> clearUserData() async {
    await _userBox.clear();
  }

  // Cache Storage
  static Future<void> saveCache(String key, dynamic value) async {
    await _cacheBox.put(key, value);
  }

  static T? getCache<T>(String key, {T? defaultValue}) {
    return _cacheBox.get(key, defaultValue: defaultValue) as T?;
  }

  static Future<void> removeCache(String key) async {
    await _cacheBox.delete(key);
  }

  static Future<void> clearCache() async {
    await _cacheBox.clear();
  }

  // Utility methods
  static bool hasSetting(String key) {
    return _settingsBox.containsKey(key);
  }

  static bool hasUserData(String key) {
    return _userBox.containsKey(key);
  }

  static bool hasCache(String key) {
    return _cacheBox.containsKey(key);
  }

  // Get all keys
  static Iterable<dynamic> getSettingsKeys() {
    return _settingsBox.keys;
  }

  static Iterable<dynamic> getUserDataKeys() {
    return _userBox.keys;
  }

  static Iterable<dynamic> getCacheKeys() {
    return _cacheBox.keys;
  }

  /// Clear all storage
  static Future<void> clearAll() async {
    await Future.wait([
      _settingsBox.clear(),
      _userBox.clear(),
      _cacheBox.clear(),
    ]);
  }

  /// Close all boxes
  static Future<void> close() async {
    await Future.wait([
      _settingsBox.close(),
      _userBox.close(),
      _cacheBox.close(),
    ]);
  }
}