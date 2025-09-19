import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/branding/industry_branding.dart';
import '../../core/branding/branding_provider.dart';
import '../../shared/widgets/adaptive_layout.dart';

/// Industry selection screen for onboarding
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
    _fadeAnimation = FadeTransition(
      opacity: CurvedAnimation(
        parent: _animationController,
        curve: Curves.easeInOut,
      ),
    ).animation;
    _animationController.forward();
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    final isDesktop = size.width > 1200;
    final isTablet = size.width > 600;

    return AdaptiveLayout(
      child: Scaffold(
        body: AnimatedBuilder(
          animation: _fadeAnimation,
          builder: (context, child) {
            return Opacity(
              opacity: _fadeAnimation.value,
              child: Container(
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                    colors: [
                      Theme.of(context).colorScheme.primary.withOpacity(0.1),
                      Theme.of(context).colorScheme.secondary.withOpacity(0.1),
                    ],
                  ),
                ),
                child: SafeArea(
                  child: Center(
                    child: SingleChildScrollView(
                      padding: EdgeInsets.all(isDesktop ? 32 : 16),
                      child: Container(
                        constraints: BoxConstraints(
                          maxWidth: isDesktop ? 1200 : (isTablet ? 800 : double.infinity),
                        ),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            _buildHeader(context),
                            const SizedBox(height: 48),
                            _buildIndustryGrid(context, isDesktop, isTablet),
                            const SizedBox(height: 32),
                            _buildContinueButton(context),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _buildHeader(BuildContext context) {
    final theme = Theme.of(context);
    
    return Column(
      children: [
        // Logo placeholder
        Container(
          width: 80,
          height: 80,
          decoration: BoxDecoration(
            color: theme.colorScheme.primary,
            borderRadius: BorderRadius.circular(20),
          ),
          child: Icon(
            Icons.business,
            size: 40,
            color: theme.colorScheme.onPrimary,
          ),
        ),
        
        const SizedBox(height: 24),
        
        Text(
          'Welcome to Olympus Cloud',
          style: theme.textTheme.displayMedium?.copyWith(
            fontWeight: FontWeight.bold,
            color: theme.colorScheme.onSurface,
          ),
          textAlign: TextAlign.center,
        ),
        
        const SizedBox(height: 12),
        
        Text(
          'Choose your industry to get started with a customized experience',
          style: theme.textTheme.titleLarge?.copyWith(
            color: theme.colorScheme.onSurface.withOpacity(0.7),
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }

  Widget _buildIndustryGrid(BuildContext context, bool isDesktop, bool isTablet) {
    final industries = [
      IndustryBrandings.restaurantRevolution,
      IndustryBrandings.retailEdge,
      IndustryBrandings.salonLuxe,
      IndustryBrandings.eventMaster,
      IndustryBrandings.hotelHaven,
      IndustryBrandings.olympusDefault,
    ];

    final crossAxisCount = isDesktop ? 3 : (isTablet ? 2 : 1);
    
    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: crossAxisCount,
        crossAxisSpacing: 16,
        mainAxisSpacing: 16,
        childAspectRatio: isDesktop ? 1.2 : (isTablet ? 1.1 : 1.5),
      ),
      itemCount: industries.length,
      itemBuilder: (context, index) {
        final industry = industries[index];
        return _buildIndustryCard(context, industry);
      },
    );
  }

  Widget _buildIndustryCard(BuildContext context, IndustryBranding industry) {
    final isSelected = selectedIndustry == industry.industryType;
    final theme = Theme.of(context);
    
    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeInOut,
      transform: Matrix4.identity()..scale(isSelected ? 1.02 : 1.0),
      child: Card(
        elevation: isSelected ? 8 : 2,
        shadowColor: industry.lightColorScheme.primary.withOpacity(0.3),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(20),
          side: BorderSide(
            color: isSelected 
                ? industry.lightColorScheme.primary
                : Colors.transparent,
            width: 2,
          ),
        ),
        child: InkWell(
          onTap: () {
            setState(() {
              selectedIndustry = industry.industryType;
            });
          },
          borderRadius: BorderRadius.circular(20),
          child: Container(
            padding: const EdgeInsets.all(24),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(20),
              gradient: LinearGradient(
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
                colors: [
                  industry.lightColorScheme.primary.withOpacity(0.1),
                  industry.lightColorScheme.secondary.withOpacity(0.1),
                ],
              ),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                // Industry icon
                Container(
                  width: 60,
                  height: 60,
                  decoration: BoxDecoration(
                    color: industry.lightColorScheme.primary,
                    borderRadius: BorderRadius.circular(15),
                  ),
                  child: Icon(
                    _getIndustryIcon(industry.industryType),
                    size: 30,
                    color: industry.lightColorScheme.onPrimary,
                  ),
                ),
                
                const SizedBox(height: 16),
                
                // Brand name
                Text(
                  industry.brandName,
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                    color: industry.lightColorScheme.primary,
                  ),
                  textAlign: TextAlign.center,
                ),
                
                const SizedBox(height: 8),
                
                // Tagline
                Text(
                  industry.tagline,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: theme.colorScheme.onSurface.withOpacity(0.7),
                  ),
                  textAlign: TextAlign.center,
                ),
                
                const SizedBox(height: 12),
                
                // Description
                Text(
                  industry.description,
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: theme.colorScheme.onSurface.withOpacity(0.6),
                  ),
                  textAlign: TextAlign.center,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
                
                if (isSelected) ...[
                  const SizedBox(height: 12),
                  Icon(
                    Icons.check_circle,
                    color: industry.lightColorScheme.primary,
                    size: 24,
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildContinueButton(BuildContext context) {
    final theme = Theme.of(context);
    final isEnabled = selectedIndustry != null;
    
    return AnimatedOpacity(
      opacity: isEnabled ? 1.0 : 0.5,
      duration: const Duration(milliseconds: 200),
      child: SizedBox(
        width: double.infinity,
        height: 56,
        child: ElevatedButton(
          onPressed: isEnabled ? _handleContinue : null,
          style: ElevatedButton.styleFrom(
            backgroundColor: isEnabled 
                ? theme.colorScheme.primary 
                : theme.colorScheme.onSurface.withOpacity(0.3),
            foregroundColor: Colors.white,
            elevation: isEnabled ? 4 : 0,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
            ),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                'Continue with ${selectedIndustry != null ? IndustryBrandings.getBranding(selectedIndustry!).brandName : "Selection"}',
                style: theme.textTheme.titleMedium?.copyWith(
                  color: Colors.white,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(width: 8),
              const Icon(
                Icons.arrow_forward,
                color: Colors.white,
              ),
            ],
          ),
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