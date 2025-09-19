import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../core/branding/industry_branding.dart';
import '../../core/branding/branding_provider.dart';
import '../../shared/widgets/industry_widgets.dart';

class IndustrySelectionScreen extends ConsumerStatefulWidget {
  const IndustrySelectionScreen({super.key});

  @override
  ConsumerState<IndustrySelectionScreen> createState() => _IndustrySelectionScreenState();
}

class _IndustrySelectionScreenState extends ConsumerState<IndustrySelectionScreen>
    with TickerProviderStateMixin {
  IndustryType? selectedIndustry;
  SubIndustryType? selectedSubIndustry;
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
    final theme = Theme.of(context);
    
    return Scaffold(
      body: SafeArea(
        child: FadeTransition(
          opacity: _fadeAnimation,
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const SizedBox(height: 20),
                
                // Header
                Text(
                  'Welcome to Olympus',
                  style: theme.textTheme.displaySmall?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  'Choose your industry to get a customized experience',
                  style: theme.textTheme.bodyLarge?.copyWith(
                    color: theme.textTheme.bodyLarge?.color?.withOpacity(0.7),
                  ),
                ),
                
                const SizedBox(height: 40),
                
                // Industry cards
                Expanded(
                  child: GridView.count(
                    crossAxisCount: 2,
                    mainAxisSpacing: 16,
                    crossAxisSpacing: 16,
                    childAspectRatio: 0.9,
                    children: [
                      _buildIndustryCard(
                        IndustryType.restaurant,
                        IndustryBrandings.restaurantRevolution,
                        Icons.restaurant,
                        'Perfect for restaurants, bars, and nightclubs',
                      ),
                      _buildIndustryCard(
                        IndustryType.retail,
                        IndustryBrandings.retailPro,
                        Icons.storefront,
                        'Ideal for retail stores and e-commerce',
                      ),
                      _buildIndustryCard(
                        IndustryType.salon,
                        IndustryBrandings.salonSuite,
                        Icons.content_cut,
                        'Great for salons, spas, and beauty services',
                      ),
                      _buildIndustryCard(
                        IndustryType.events,
                        IndustryBrandings.eventsMaster,
                        Icons.event,
                        'Excellent for event planning and management',
                      ),
                      _buildIndustryCard(
                        IndustryType.hospitality,
                        IndustryBrandings.hotelHaven,
                        Icons.hotel,
                        'Perfect for hotels and hospitality',
                      ),
                      _buildIndustryCard(
                        IndustryType.other,
                        IndustryBrandings.olympus,
                        Icons.business,
                        'For any other type of business',
                      ),
                    ],
                  ),
                ),
                
                const SizedBox(height: 20),
                
                // Continue button
                SizedBox(
                  width: double.infinity,
                  child: ElevatedButton(
                    onPressed: selectedIndustry != null ? _onContinue : null,
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(vertical: 16),
                      backgroundColor: selectedIndustry != null 
                          ? IndustryBrandings.getBrandingForIndustry(selectedIndustry!).primaryColor
                          : null,
                    ),
                    child: const Text(
                      'Continue',
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                ),
                
                const SizedBox(height: 12),
                
                // Skip option
                Center(
                  child: TextButton(
                    onPressed: () => _onSkip(),
                    child: Text(
                      'Skip for now',
                      style: TextStyle(
                        color: theme.textTheme.bodyMedium?.color?.withOpacity(0.7),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildIndustryCard(
    IndustryType industry,
    IndustryBranding branding,
    IconData icon,
    String description,
  ) {
    final isSelected = selectedIndustry == industry;
    
    return GestureDetector(
      onTap: () => _selectIndustry(industry),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        decoration: BoxDecoration(
          color: isSelected ? branding.primaryColor.withOpacity(0.1) : null,
          border: Border.all(
            color: isSelected ? branding.primaryColor : Colors.grey.shade300,
            width: isSelected ? 2 : 1,
          ),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            children: [
              // Brand logo/icon
              Container(
                width: 60,
                height: 60,
                decoration: BoxDecoration(
                  color: branding.primaryColor,
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Icon(
                  icon,
                  color: Colors.white,
                  size: 32,
                ),
              ),
              
              const SizedBox(height: 12),
              
              // Brand name
              Text(
                branding.brandName,
                style: branding.headingFont.copyWith(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: isSelected ? branding.primaryColor : null,
                ),
                textAlign: TextAlign.center,
              ),
              
              const SizedBox(height: 4),
              
              // Tagline
              Text(
                branding.tagline,
                style: branding.primaryFont.copyWith(
                  fontSize: 12,
                  color: Theme.of(context).textTheme.bodyMedium?.color?.withOpacity(0.7),
                ),
                textAlign: TextAlign.center,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
              
              const SizedBox(height: 8),
              
              // Description
              Expanded(
                child: Text(
                  description,
                  style: TextStyle(
                    fontSize: 11,
                    color: Theme.of(context).textTheme.bodySmall?.color?.withOpacity(0.6),
                  ),
                  textAlign: TextAlign.center,
                  maxLines: 3,
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              
              // Selection indicator
              if (isSelected)
                Container(
                  margin: const EdgeInsets.only(top: 8),
                  child: Icon(
                    Icons.check_circle,
                    color: branding.primaryColor,
                    size: 20,
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }

  void _selectIndustry(IndustryType industry) {
    setState(() {
      selectedIndustry = industry;
      selectedSubIndustry = null; // Reset sub-industry when main industry changes
    });
  }

  Future<void> _onContinue() async {
    if (selectedIndustry == null) return;
    
    // Update the branding provider
    await ref.read(brandingProvider.notifier).updateBranding(
      selectedIndustry!,
      subIndustry: selectedSubIndustry,
    );
    
    // Navigate to dashboard or next onboarding step
    if (mounted) {
      context.go('/dashboard');
    }
  }

  void _onSkip() {
    // Keep default branding and continue
    context.go('/dashboard');
  }
}

/// Sub-industry selection screen for more specific branding
class SubIndustrySelectionScreen extends ConsumerStatefulWidget {
  final IndustryType industry;
  
  const SubIndustrySelectionScreen({
    super.key,
    required this.industry,
  });

  @override
  ConsumerState<SubIndustrySelectionScreen> createState() => _SubIndustrySelectionScreenState();
}

class _SubIndustrySelectionScreenState extends ConsumerState<SubIndustrySelectionScreen> {
  SubIndustryType? selectedSubIndustry;

  @override
  Widget build(BuildContext context) {
    final subIndustries = _getSubIndustriesForIndustry(widget.industry);
    final theme = Theme.of(context);
    
    return Scaffold(
      appBar: AppBar(
        title: Text('Choose ${widget.industry.displayName} Type'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'What type of ${widget.industry.displayName.toLowerCase()} do you run?',
              style: theme.textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'This helps us customize the experience for your specific needs',
              style: theme.textTheme.bodyLarge?.copyWith(
                color: theme.textTheme.bodyLarge?.color?.withOpacity(0.7),
              ),
            ),
            
            const SizedBox(height: 32),
            
            Expanded(
              child: ListView.separated(
                itemCount: subIndustries.length,
                separatorBuilder: (context, index) => const SizedBox(height: 12),
                itemBuilder: (context, index) {
                  final subIndustry = subIndustries[index];
                  final isSelected = selectedSubIndustry == subIndustry;
                  
                  return GestureDetector(
                    onTap: () => setState(() {
                      selectedSubIndustry = subIndustry;
                    }),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 200),
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: isSelected ? theme.colorScheme.primary.withOpacity(0.1) : null,
                        border: Border.all(
                          color: isSelected ? theme.colorScheme.primary : Colors.grey.shade300,
                          width: isSelected ? 2 : 1,
                        ),
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Row(
                        children: [
                          Expanded(
                            child: Text(
                              subIndustry.displayName,
                              style: TextStyle(
                                fontSize: 16,
                                fontWeight: isSelected ? FontWeight.w600 : FontWeight.w400,
                                color: isSelected ? theme.colorScheme.primary : null,
                              ),
                            ),
                          ),
                          if (isSelected)
                            Icon(
                              Icons.check_circle,
                              color: theme.colorScheme.primary,
                            ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
            
            const SizedBox(height: 20),
            
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: selectedSubIndustry != null ? _onContinue : null,
                style: ElevatedButton.styleFrom(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                ),
                child: const Text(
                  'Continue',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
            ),
            
            const SizedBox(height: 12),
            
            Center(
              child: TextButton(
                onPressed: () => _onSkip(),
                child: Text(
                  'Skip this step',
                  style: TextStyle(
                    color: theme.textTheme.bodyMedium?.color?.withOpacity(0.7),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  List<SubIndustryType> _getSubIndustriesForIndustry(IndustryType industry) {
    return SubIndustryType.values
        .where((subIndustry) => subIndustry.parentIndustry == industry)
        .toList();
  }

  Future<void> _onContinue() async {
    await ref.read(brandingProvider.notifier).updateBranding(
      widget.industry,
      subIndustry: selectedSubIndustry,
    );
    
    if (mounted) {
      context.go('/dashboard');
    }
  }

  void _onSkip() {
    ref.read(brandingProvider.notifier).updateBranding(widget.industry);
    context.go('/dashboard');
  }
}