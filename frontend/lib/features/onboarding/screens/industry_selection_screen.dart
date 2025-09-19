import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/branding/industry_branding.dart';
import '../../../core/branding/branding_provider.dart';
import '../../../core/platform/responsive_layout.dart';

/// Industry selection screen for onboarding with visual branding previews
class IndustrySelectionScreen extends ConsumerStatefulWidget {
  const IndustrySelectionScreen({super.key});

  @override
  ConsumerState<IndustrySelectionScreen> createState() => _IndustrySelectionScreenState();
}

class _IndustrySelectionScreenState extends ConsumerState<IndustrySelectionScreen> 
    with TickerProviderStateMixin {
  String? selectedIndustry;
  late AnimationController _animationController;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();
    
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 800),
      vsync: this,
    );

    _fadeAnimation = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(CurvedAnimation(
      parent: _animationController,
      curve: Curves.easeInOut,
    ));

    _animationController.forward();
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: FadeTransition(
        opacity: _fadeAnimation,
        child: ResponsiveWidget(
          mobile: _buildMobileLayout(context),
          tablet: _buildTabletLayout(context),
          desktop: _buildDesktopLayout(context),
        ),
      ),
    );
  }

  Widget _buildMobileLayout(BuildContext context) {
    return CustomScrollView(
      slivers: [
        _buildAppBar(context),
        _buildIndustryGrid(context, crossAxisCount: 1),
        _buildContinueButton(context),
      ],
    );
  }

  Widget _buildTabletLayout(BuildContext context) {
    return CustomScrollView(
      slivers: [
        _buildAppBar(context),
        _buildIndustryGrid(context, crossAxisCount: 2),
        _buildContinueButton(context),
      ],
    );
  }

  Widget _buildDesktopLayout(BuildContext context) {
    return CustomScrollView(
      slivers: [
        _buildAppBar(context),
        _buildIndustryGrid(context, crossAxisCount: 3),
        _buildContinueButton(context),
      ],
    );
  }

  Widget _buildAppBar(BuildContext context) {
    return SliverAppBar(
      expandedHeight: 120,
      floating: false,
      pinned: true,
      backgroundColor: Colors.transparent,
      flexibleSpace: FlexibleSpaceBar(
        title: Text(
          'Choose Your Industry',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            fontWeight: FontWeight.bold,
            color: Theme.of(context).colorScheme.onSurface,
          ),
        ),
        centerTitle: true,
        titlePadding: const EdgeInsets.all(16),
      ),
    );
  }

  Widget _buildIndustryGrid(BuildContext context, {required int crossAxisCount}) {
    return SliverPadding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
      sliver: SliverGrid(
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: crossAxisCount,
          mainAxisSpacing: 16,
          crossAxisSpacing: 16,
          childAspectRatio: 1.1,
        ),
        delegate: SliverChildBuilderDelegate(
          (context, index) {
            final industry = IndustryBrandings.all.keys.elementAt(index);
            final branding = IndustryBrandings.all[industry]!;
            
            return _buildIndustryCard(context, industry, branding);
          },
          childCount: IndustryBrandings.all.length,
        ),
      ),
    );
  }

  Widget _buildIndustryCard(BuildContext context, String industry, IndustryBranding branding) {
    final isSelected = selectedIndustry == industry;
    
    return GestureDetector(
      onTap: () {
        setState(() {
          selectedIndustry = industry;
        });
      },
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              branding.primaryColor.withValues(alpha: 0.1),
              branding.secondaryColor.withValues(alpha: 0.1),
            ],
          ),
          border: Border.all(
            color: isSelected 
                ? branding.primaryColor 
                : branding.primaryColor.withValues(alpha: 0.3),
            width: isSelected ? 3 : 1,
          ),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header with icon
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: branding.primaryColor.withValues(alpha: 0.2),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Icon(
                      _getIndustryIcon(industry),
                      color: branding.primaryColor,
                      size: 32,
                    ),
                  ),
                  const Spacer(),
                  if (isSelected)
                    Container(
                      padding: const EdgeInsets.all(4),
                      decoration: BoxDecoration(
                        color: branding.primaryColor,
                        shape: BoxShape.circle,
                      ),
                      child: const Icon(
                        Icons.check,
                        color: Colors.white,
                        size: 16,
                      ),
                    ),
                ],
              ),
              const SizedBox(height: 16),
              // Industry name
              Text(
                branding.name,
                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: branding.primaryColor,
                ),
              ),
              const SizedBox(height: 8),
              // Industry description
              Text(
                branding.tagline,
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
                ),
              ),
              const Spacer(),
              // Feature highlights
              Wrap(
                spacing: 4,
                children: branding.features.take(3).map((feature) {
                  return Chip(
                    label: Text(
                      feature,
                      style: TextStyle(
                        fontSize: 10,
                        color: branding.primaryColor,
                      ),
                    ),
                    backgroundColor: branding.primaryColor.withValues(alpha: 0.1),
                    side: BorderSide.none,
                    padding: const EdgeInsets.symmetric(horizontal: 4),
                  );
                }).toList(),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildContinueButton(BuildContext context) {
    if (selectedIndustry == null) {
      return const SliverToBoxAdapter(child: SizedBox.shrink());
    }

    final branding = IndustryBrandings.all[selectedIndustry]!;
    
    return SliverToBoxAdapter(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
        child: Column(
          children: [
            const SizedBox(height: 32),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _handleContinue,
                style: ElevatedButton.styleFrom(
                  backgroundColor: branding.primaryColor,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
                child: Text(
                  'Continue with ${branding.name}',
                  style: const TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 24),
          ],
        ),
      ),
    );
  }

  IconData _getIndustryIcon(String industryType) {
    switch (industryType) {
      case 'restaurant':
        return Icons.restaurant_menu;
      case 'retail':
        return Icons.shopping_bag;
      case 'salon':
        return Icons.content_cut;
      case 'event':
        return Icons.celebration;
      case 'hospitality':
        return Icons.hotel;
      case 'general':
      default:
        return Icons.business;
    }
  }

  Future<void> _handleContinue() async {
    if (selectedIndustry == null) return;

    try {
      // Set the industry branding
      await ref.read(brandingProvider.notifier).setIndustry(selectedIndustry!);
      
      // Navigate to dashboard
      if (mounted) {
        context.go('/dashboard');
      }
    } catch (e) {
      // Show error message
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to set industry: $e'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    }
  }
}