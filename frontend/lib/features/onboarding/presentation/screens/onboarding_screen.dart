import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../shared/presentation/widgets/adaptive_layout.dart';
import '../../../../shared/presentation/widgets/responsive_form.dart';
import '../../../../core/router/app_router.dart';

/// Onboarding screen that determines user's role and business setup needs
class OnboardingScreen extends ConsumerStatefulWidget {
  const OnboardingScreen({super.key});

  @override
  ConsumerState<OnboardingScreen> createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends ConsumerState<OnboardingScreen> {
  final PageController _pageController = PageController();
  
  int _currentStep = 0;
  String _userRole = '';
  String _businessStatus = '';
  String _experienceLevel = '';
  List<String> _businessGoals = [];

  final List<Map<String, String>> _roles = [
    {
      'id': 'owner',
      'title': 'Business Owner',
      'description': 'I own and manage my business',
      'icon': 'business'
    },
    {
      'id': 'manager',
      'title': 'Manager',
      'description': 'I manage operations for a business',
      'icon': 'supervisor_account'
    },
    {
      'id': 'employee',
      'title': 'Employee',
      'description': 'I work for a business using this system',
      'icon': 'person'
    },
  ];

  final List<Map<String, String>> _businessStatuses = [
    {
      'id': 'new',
      'title': 'Just Starting',
      'description': 'I\'m launching a new business',
      'icon': 'rocket_launch'
    },
    {
      'id': 'existing',
      'title': 'Existing Business',
      'description': 'I have an established business',
      'icon': 'store'
    },
    {
      'id': 'expanding',
      'title': 'Expanding',
      'description': 'I\'m growing or adding locations',
      'icon': 'trending_up'
    },
  ];

  final List<Map<String, String>> _experienceLevels = [
    {
      'id': 'beginner',
      'title': 'Beginner',
      'description': 'New to business management software',
      'icon': 'school'
    },
    {
      'id': 'intermediate',
      'title': 'Intermediate',
      'description': 'Some experience with business tools',
      'icon': 'trending_up'
    },
    {
      'id': 'advanced',
      'title': 'Advanced',
      'description': 'Very familiar with business systems',
      'icon': 'psychology'
    },
  ];

  final List<Map<String, String>> _goals = [
    {'id': 'sales', 'title': 'Increase Sales', 'description': 'Boost revenue and customer acquisition'},
    {'id': 'efficiency', 'title': 'Improve Efficiency', 'description': 'Streamline operations and reduce waste'},
    {'id': 'customer', 'title': 'Better Customer Service', 'description': 'Enhance customer experience and satisfaction'},
    {'id': 'inventory', 'title': 'Manage Inventory', 'description': 'Track stock and reduce inventory costs'},
    {'id': 'analytics', 'title': 'Data-Driven Decisions', 'description': 'Use analytics to make better business decisions'},
    {'id': 'scaling', 'title': 'Scale Operations', 'description': 'Grow the business and manage multiple locations'},
  ];

  Future<void> _nextStep() async {
    if (_currentStep < 3) {
      setState(() => _currentStep++);
      await _pageController.nextPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    } else {
      await _completeOnboarding();
    }
  }

  Future<void> _previousStep() async {
    if (_currentStep > 0) {
      setState(() => _currentStep--);
      await _pageController.previousPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  Future<void> _completeOnboarding() async {
    // Navigate to appropriate screen based on user selections
    if (_userRole == 'owner' && _businessStatus == 'new') {
      AppRouter.navigateToNamed(RouteNames.businessSetup);
    } else {
      AppRouter.navigateToNamed(RouteNames.dashboard);
    }
  }

  bool _canProceed() {
    switch (_currentStep) {
      case 0:
        return _userRole.isNotEmpty;
      case 1:
        return _businessStatus.isNotEmpty;
      case 2:
        return _experienceLevel.isNotEmpty;
      case 3:
        return _businessGoals.isNotEmpty;
      default:
        return false;
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return AdaptiveLayout(
      child: Scaffold(
        body: SafeArea(
          child: ResponsiveForm(
            child: Column(
              children: [
                // Header with progress
                _buildHeader(theme),
                
                // Page content
                Expanded(
                  child: PageView(
                    controller: _pageController,
                    physics: const NeverScrollableScrollPhysics(),
                    children: [
                      _buildRoleSelectionStep(),
                      _buildBusinessStatusStep(),
                      _buildExperienceStep(),
                      _buildGoalsStep(),
                    ],
                  ),
                ),
                
                // Navigation
                _buildNavigationButtons(theme),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildHeader(ThemeData theme) {
    return Container(
      padding: const EdgeInsets.all(24),
      child: Column(
        children: [
          // Logo or app name
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
          
          const SizedBox(height: 16),
          
          Text(
            'Welcome to Olympus Cloud',
            style: theme.textTheme.headlineMedium?.copyWith(
              fontWeight: FontWeight.bold,
            ),
            textAlign: TextAlign.center,
          ),
          
          const SizedBox(height: 8),
          
          Text(
            'Let\'s personalize your experience',
            style: theme.textTheme.bodyLarge?.copyWith(
              color: theme.colorScheme.onSurface.withOpacity(0.7),
            ),
            textAlign: TextAlign.center,
          ),
          
          const SizedBox(height: 24),
          
          // Progress indicator
          LinearProgressIndicator(
            value: (_currentStep + 1) / 4,
            backgroundColor: theme.colorScheme.surfaceVariant,
            valueColor: AlwaysStoppedAnimation<Color>(theme.colorScheme.primary),
          ),
        ],
      ),
    );
  }

  Widget _buildRoleSelectionStep() {
    return _buildSelectionStep(
      title: 'What\'s your role?',
      subtitle: 'This helps us customize the interface for your needs',
      options: _roles,
      selectedValue: _userRole,
      onSelected: (value) {
        setState(() {
          _userRole = value;
        });
      },
    );
  }

  Widget _buildBusinessStatusStep() {
    return _buildSelectionStep(
      title: 'What\'s your business status?',
      subtitle: 'We\'ll tailor the setup process accordingly',
      options: _businessStatuses,
      selectedValue: _businessStatus,
      onSelected: (value) {
        setState(() {
          _businessStatus = value;
        });
      },
    );
  }

  Widget _buildExperienceStep() {
    return _buildSelectionStep(
      title: 'What\'s your experience level?',
      subtitle: 'We\'ll adjust the complexity of features and guidance',
      options: _experienceLevels,
      selectedValue: _experienceLevel,
      onSelected: (value) {
        setState(() {
          _experienceLevel = value;
        });
      },
    );
  }

  Widget _buildGoalsStep() {
    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'What are your main goals?',
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Select all that apply. We\'ll prioritize relevant features.',
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
              ),
            ),
            const SizedBox(height: 32),
            
            ..._goals.map((goal) {
              final isSelected = _businessGoals.contains(goal['id']);
              
              return Card(
                margin: const EdgeInsets.only(bottom: 12),
                child: InkWell(
                  onTap: () {
                    setState(() {
                      if (isSelected) {
                        _businessGoals.remove(goal['id']);
                      } else {
                        _businessGoals.add(goal['id']!);
                      }
                    });
                  },
                  borderRadius: BorderRadius.circular(12),
                  child: Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(
                        color: isSelected 
                            ? Theme.of(context).colorScheme.primary 
                            : Colors.transparent,
                        width: 2,
                      ),
                    ),
                    child: Row(
                      children: [
                        Checkbox(
                          value: isSelected,
                          onChanged: (value) {
                            setState(() {
                              if (value == true) {
                                _businessGoals.add(goal['id']!);
                              } else {
                                _businessGoals.remove(goal['id']);
                              }
                            });
                          },
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                goal['title']!,
                                style: Theme.of(context).textTheme.titleMedium?.copyWith(
                                  fontWeight: FontWeight.w600,
                                  color: isSelected 
                                      ? Theme.of(context).colorScheme.primary 
                                      : null,
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                goal['description']!,
                                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                                  color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              );
            }).toList(),
          ],
        ),
      ),
    );
  }

  Widget _buildSelectionStep({
    required String title,
    required String subtitle,
    required List<Map<String, String>> options,
    required String selectedValue,
    required ValueChanged<String> onSelected,
  }) {
    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              subtitle,
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
              ),
            ),
            const SizedBox(height: 32),
            
            ...options.map((option) {
              final isSelected = selectedValue == option['id'];
              
              return Card(
                margin: const EdgeInsets.only(bottom: 16),
                child: InkWell(
                  onTap: () => onSelected(option['id']!),
                  borderRadius: BorderRadius.circular(12),
                  child: Container(
                    padding: const EdgeInsets.all(20),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(
                        color: isSelected 
                            ? Theme.of(context).colorScheme.primary 
                            : Colors.transparent,
                        width: 2,
                      ),
                    ),
                    child: Row(
                      children: [
                        Container(
                          width: 48,
                          height: 48,
                          decoration: BoxDecoration(
                            color: isSelected 
                                ? Theme.of(context).colorScheme.primary 
                                : Theme.of(context).colorScheme.surfaceVariant,
                            borderRadius: BorderRadius.circular(12),
                          ),
                          child: Icon(
                            _getIconData(option['icon']!),
                            color: isSelected 
                                ? Theme.of(context).colorScheme.onPrimary 
                                : Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                        ),
                        const SizedBox(width: 16),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                option['title']!,
                                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                                  fontWeight: FontWeight.w600,
                                  color: isSelected 
                                      ? Theme.of(context).colorScheme.primary 
                                      : null,
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                option['description']!,
                                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                                  color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
                                ),
                              ),
                            ],
                          ),
                        ),
                        if (isSelected)
                          Icon(
                            Icons.check_circle,
                            color: Theme.of(context).colorScheme.primary,
                          ),
                      ],
                    ),
                  ),
                ),
              );
            }).toList(),
          ],
        ),
      ),
    );
  }

  IconData _getIconData(String iconName) {
    switch (iconName) {
      case 'business': return Icons.business;
      case 'supervisor_account': return Icons.supervisor_account;
      case 'person': return Icons.person;
      case 'rocket_launch': return Icons.rocket_launch;
      case 'store': return Icons.store;
      case 'trending_up': return Icons.trending_up;
      case 'school': return Icons.school;
      case 'psychology': return Icons.psychology;
      default: return Icons.circle;
    }
  }

  Widget _buildNavigationButtons(ThemeData theme) {
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Row(
        children: [
          if (_currentStep > 0)
            Expanded(
              child: OutlinedButton(
                onPressed: _previousStep,
                child: const Text('Back'),
              ),
            ),
          
          if (_currentStep > 0) const SizedBox(width: 16),
          
          Expanded(
            flex: _currentStep == 0 ? 1 : 1,
            child: ElevatedButton(
              onPressed: _canProceed() ? _nextStep : null,
              child: Text(_currentStep == 3 ? 'Get Started' : 'Next'),
            ),
          ),
        ],
      ),
    );
  }
}