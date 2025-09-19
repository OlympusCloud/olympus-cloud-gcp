import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'industry_branding.dart';

/// Provider for managing the current industry branding
class BrandingNotifier extends StateNotifier<IndustryBranding> {
  BrandingNotifier() : super(IndustryBrandings.olympus) {
    _loadBranding();
  }

  static const String _brandingBoxName = 'branding';
  static const String _industryKey = 'industry_type';
  static const String _subIndustryKey = 'sub_industry_type';

  /// Load branding from local storage
  Future<void> _loadBranding() async {
    try {
      final box = await Hive.openBox(_brandingBoxName);
      final industryString = box.get(_industryKey) as String?;
      final subIndustryString = box.get(_subIndustryKey) as String?;

      if (industryString != null) {
        final industry = IndustryType.values.firstWhere(
          (e) => e.name == industryString,
          orElse: () => IndustryType.other,
        );

        SubIndustryType? subIndustry;
        if (subIndustryString != null) {
          subIndustry = SubIndustryType.values.firstWhere(
            (e) => e.name == subIndustryString,
            orElse: () => SubIndustryType.generic,
          );
        }

        final branding = IndustryBrandings.getBrandingForIndustry(
          industry,
          subIndustry: subIndustry,
        );
        state = branding;
      }
    } catch (e) {
      // If loading fails, keep the default branding
      print('Failed to load branding: $e');
    }
  }

  /// Update the current branding
  Future<void> updateBranding(IndustryType industry, {SubIndustryType? subIndustry}) async {
    final branding = IndustryBrandings.getBrandingForIndustry(
      industry,
      subIndustry: subIndustry,
    );
    
    state = branding;
    
    // Save to local storage
    try {
      final box = await Hive.openBox(_brandingBoxName);
      await box.put(_industryKey, industry.name);
      if (subIndustry != null) {
        await box.put(_subIndustryKey, subIndustry.name);
      } else {
        await box.delete(_subIndustryKey);
      }
    } catch (e) {
      print('Failed to save branding: $e');
    }
  }

  /// Reset to default branding
  Future<void> resetBranding() async {
    state = IndustryBrandings.olympus;
    
    try {
      final box = await Hive.openBox(_brandingBoxName);
      await box.clear();
    } catch (e) {
      print('Failed to reset branding: $e');
    }
  }
}

/// Provider for the current branding
final brandingProvider = StateNotifierProvider<BrandingNotifier, IndustryBranding>(
  (ref) => BrandingNotifier(),
);

/// Provider for the current industry type
final currentIndustryProvider = Provider<IndustryType>((ref) {
  final branding = ref.watch(brandingProvider);
  return branding.industry;
});

/// Provider for the current sub-industry type
final currentSubIndustryProvider = Provider<SubIndustryType?>((ref) {
  final branding = ref.watch(brandingProvider);
  return branding.subIndustry;
});

/// Provider for industry-specific status colors
final industryStatusColorsProvider = Provider<Map<String, Color>>((ref) {
  final branding = ref.watch(brandingProvider);
  return branding.statusColors;
});

/// Provider for industry-specific feature icons
final industryFeatureIconsProvider = Provider<Map<String, IconData>>((ref) {
  final branding = ref.watch(brandingProvider);
  return branding.featureIcons;
});

/// Provider for checking if a specific feature is available for current industry
final featureAvailableProvider = Provider.family<bool, String>((ref, feature) {
  final branding = ref.watch(brandingProvider);
  return branding.industry.defaultFeatures.contains(feature);
});

/// Provider for the current theme based on branding
final brandingThemeProvider = Provider.family<ThemeData, bool>((ref, isDark) {
  final branding = ref.watch(brandingProvider);
  return branding.generateTheme(isDark: isDark);
});