import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/material.dart';

import 'industry_branding.dart';
import '../services/storage_service.dart';

/// State notifier for managing industry branding configuration
class BrandingNotifier extends StateNotifier<IndustryBranding> {
  BrandingNotifier() : super(IndustryBrandings.olympusDefault) {
    _loadSavedBranding();
  }

  /// Load branding configuration from storage
  Future<void> _loadSavedBranding() async {
    try {
      final savedIndustryType = StorageService.getSetting<String>('industry_type');
      if (savedIndustryType != null) {
        final branding = IndustryBrandings.getBranding(savedIndustryType);
        state = branding.copyWith(
          textTheme: IndustryBrandings.createTextTheme(
            savedIndustryType,
            Brightness.light, // Default to light, will be updated by theme provider
          ),
        );
      }
    } catch (e) {
      debugPrint('Error loading saved branding: $e');
    }
  }

  /// Set industry branding and save to storage
  Future<void> setIndustry(String industryType) async {
    try {
      final branding = IndustryBrandings.getBranding(industryType);
      state = branding.copyWith(
        textTheme: IndustryBrandings.createTextTheme(
          industryType,
          Brightness.light, // Will be updated by theme changes
        ),
      );
      
      // Save to storage
      await StorageService.saveSetting('industry_type', industryType);
      await StorageService.saveSetting('brand_name', branding.brandName);
      
      debugPrint('Industry branding set to: ${branding.brandName}');
    } catch (e) {
      debugPrint('Error setting industry branding: $e');
    }
  }

  /// Update text theme for brightness changes (deprecated - theme handles this)
  @deprecated
  void updateTextTheme(Brightness brightness) {
    // No longer needed - theme provider handles brightness changes
  }

  /// Check if a module is enabled for current industry
  bool isModuleEnabled(String moduleName) {
    return state.enabledModules.contains(moduleName);
  }

  /// Get custom setting value
  T? getCustomSetting<T>(String key) {
    return state.customSettings[key] as T?;
  }
}

/// Extension to add copyWith method to IndustryBranding
extension IndustryBrandingCopyWith on IndustryBranding {
  IndustryBranding copyWith({
    String? industryType,
    String? brandName,
    String? tagline,
    String? description,
    ColorScheme? lightColorScheme,
    ColorScheme? darkColorScheme,
    TextTheme? textTheme,
    String? logoPath,
    Map<String, IconData>? featureIcons,
    List<String>? enabledModules,
    Map<String, dynamic>? customSettings,
  }) {
    return IndustryBranding(
      industryType: industryType ?? this.industryType,
      brandName: brandName ?? this.brandName,
      tagline: tagline ?? this.tagline,
      description: description ?? this.description,
      lightColorScheme: lightColorScheme ?? this.lightColorScheme,
      darkColorScheme: darkColorScheme ?? this.darkColorScheme,
      textTheme: textTheme ?? this.textTheme,
      logoPath: logoPath ?? this.logoPath,
      featureIcons: featureIcons ?? this.featureIcons,
      enabledModules: enabledModules ?? this.enabledModules,
      customSettings: customSettings ?? this.customSettings,
    );
  }
}

/// Provider for industry branding configuration
final brandingProvider = StateNotifierProvider<BrandingNotifier, IndustryBranding>((ref) {
  return BrandingNotifier();
});

/// Provider for current industry type
final industryTypeProvider = Provider<String>((ref) {
  return ref.watch(brandingProvider).industryType;
});

/// Provider for enabled modules
final enabledModulesProvider = Provider<List<String>>((ref) {
  return ref.watch(brandingProvider).enabledModules;
});

/// Provider for feature icons
final featureIconsProvider = Provider<Map<String, IconData>>((ref) {
  return ref.watch(brandingProvider).featureIcons;
});

/// Provider for custom settings
final customSettingsProvider = Provider<Map<String, dynamic>>((ref) {
  return ref.watch(brandingProvider).customSettings;
});

/// Provider to check if a specific module is enabled
final moduleEnabledProvider = Provider.family<bool, String>((ref, moduleName) {
  return ref.watch(brandingProvider.notifier).isModuleEnabled(moduleName);
});

/// Provider to get custom setting value
final customSettingProvider = Provider.family<dynamic, String>((ref, key) {
  return ref.watch(brandingProvider.notifier).getCustomSetting(key);
});

/// Provider for industry-specific theme
final industryThemeProvider = Provider.family<ThemeData, Brightness>((ref, brightness) {
  final branding = ref.watch(brandingProvider);
  
  // Create theme with proper text theme for brightness
  final textTheme = IndustryBrandings.createTextTheme(
    branding.industryType,
    brightness,
  );
  
  return branding.copyWith(textTheme: textTheme).buildTheme(brightness);
});

/// Provider for primary color based on current theme mode
final primaryColorProvider = Provider<Color>((ref) {
  final branding = ref.watch(brandingProvider);
  final isDark = ref.watch(isDarkModeProvider);
  return branding.getPrimaryColor(isDark ? Brightness.dark : Brightness.light);
});

/// Provider for secondary color based on current theme mode
final secondaryColorProvider = Provider<Color>((ref) {
  final branding = ref.watch(brandingProvider);
  final isDark = ref.watch(isDarkModeProvider);
  return branding.getSecondaryColor(isDark ? Brightness.dark : Brightness.light);
});

/// Dark mode provider (moved here for consistency)
final isDarkModeProvider = StateProvider<bool>((ref) {
  final brightness = WidgetsBinding.instance.platformDispatcher.platformBrightness;
  return brightness == Brightness.dark;
});

/// Theme mode provider
final themeModeProvider = Provider<ThemeMode>((ref) {
  final isDark = ref.watch(isDarkModeProvider);
  return isDark ? ThemeMode.dark : ThemeMode.light;
});