import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/router/app_router.dart';
import '../../../../shared/presentation/widgets/adaptive_layout.dart';
import '../../../../shared/presentation/widgets/responsive_form.dart';
import '../../../../shared/widgets/custom_form_fields.dart';
import '../../../../shared/widgets/custom_switch_field.dart';
import '../../../../shared/widgets/loading_widgets.dart';
import '../../../../shared/utils/form_validators.dart';

/// Multi-step business setup wizard for new users
class BusinessSetupWizard extends ConsumerStatefulWidget {
  const BusinessSetupWizard({super.key});

  @override
  ConsumerState<BusinessSetupWizard> createState() => _BusinessSetupWizardState();
}

class _BusinessSetupWizardState extends ConsumerState<BusinessSetupWizard> {
  final PageController _pageController = PageController();
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  
  int _currentStep = 0;
  bool _isLoading = false;

  // Business information
  final _businessNameController = TextEditingController();
  final _businessDescriptionController = TextEditingController();
  String _selectedBusinessType = 'restaurant';
  String _selectedIndustry = 'food_service';
  String _selectedBusinessSize = 'small';
  
  // Location information
  final _addressController = TextEditingController();
  final _cityController = TextEditingController();
  final _stateController = TextEditingController();
  final _zipCodeController = TextEditingController();
  final _countryController = TextEditingController(text: 'United States');
  
  // Operating hours
  Map<String, Map<String, String>> _operatingHours = {
    'monday': {'open': '09:00', 'close': '17:00', 'closed': 'false'},
    'tuesday': {'open': '09:00', 'close': '17:00', 'closed': 'false'},
    'wednesday': {'open': '09:00', 'close': '17:00', 'closed': 'false'},
    'thursday': {'open': '09:00', 'close': '17:00', 'closed': 'false'},
    'friday': {'open': '09:00', 'close': '17:00', 'closed': 'false'},
    'saturday': {'open': '10:00', 'close': '16:00', 'closed': 'false'},
    'sunday': {'open': '10:00', 'close': '16:00', 'closed': 'true'},
  };
  
  // Features and preferences
  Set<String> _selectedFeatures = {};
  bool _enableAnalytics = true;
  bool _enableNotifications = true;
  bool _enableIntegrations = false;

  @override
  void dispose() {
    _pageController.dispose();
    _businessNameController.dispose();
    _businessDescriptionController.dispose();
    _addressController.dispose();
    _cityController.dispose();
    _stateController.dispose();
    _zipCodeController.dispose();
    _countryController.dispose();
    super.dispose();
  }

  Future<void> _nextStep() async {
    if (_currentStep < 4) {
      if (_validateCurrentStep()) {
        setState(() => _currentStep++);
        await _pageController.nextPage(
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeInOut,
        );
      }
    } else {
      await _completeSetup();
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

  bool _validateCurrentStep() {
    switch (_currentStep) {
      case 0:
        return _businessNameController.text.isNotEmpty && 
               _selectedBusinessType.isNotEmpty;
      case 1:
        return _addressController.text.isNotEmpty && 
               _cityController.text.isNotEmpty;
      case 2:
        return true; // Operating hours are optional
      case 3:
        return true; // Features are optional
      case 4:
        return true; // Final confirmation
      default:
        return false;
    }
  }

  Future<void> _completeSetup() async {
    setState(() => _isLoading = true);
    
    try {
      // TODO: Save business setup data to backend
      await Future.delayed(const Duration(seconds: 2)); // Simulate API call
      
      if (mounted) {
        AppRouter.navigateToNamedAndClearStack(RouteNames.dashboard);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Setup failed: ${e.toString()}'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return AdaptiveLayout(
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Business Setup'),
          automaticallyImplyLeading: false,
          bottom: PreferredSize(
            preferredSize: const Size.fromHeight(8),
            child: LinearProgressIndicator(
              value: (_currentStep + 1) / 5,
              backgroundColor: theme.colorScheme.surfaceVariant,
              valueColor: AlwaysStoppedAnimation<Color>(theme.colorScheme.primary),
            ),
          ),
        ),
        body: LoadingOverlay(
          isLoading: _isLoading,
          message: 'Setting up your business...',
          child: ResponsiveForm(
            child: Column(
              children: [
                // Step indicator
                _buildStepIndicator(theme),
                
                const SizedBox(height: 24),
                
                // Page view
                Expanded(
                  child: PageView(
                    controller: _pageController,
                    physics: const NeverScrollableScrollPhysics(),
                    children: [
                      _buildBusinessInfoStep(),
                      _buildLocationStep(),
                      _buildOperatingHoursStep(),
                      _buildFeaturesStep(),
                      _buildConfirmationStep(),
                    ],
                  ),
                ),
                
                // Navigation buttons
                _buildNavigationButtons(theme),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepIndicator(ThemeData theme) {
    return Row(
      children: List.generate(5, (index) {
        final isActive = index == _currentStep;
        final isCompleted = index < _currentStep;
        
        return Expanded(
          child: Container(
            margin: EdgeInsets.only(right: index < 4 ? 8 : 0),
            padding: const EdgeInsets.symmetric(vertical: 12),
            decoration: BoxDecoration(
              color: isActive 
                  ? theme.colorScheme.primary.withOpacity(0.1)
                  : isCompleted 
                      ? theme.colorScheme.primary.withOpacity(0.05)
                      : Colors.transparent,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(
                color: isActive || isCompleted 
                    ? theme.colorScheme.primary 
                    : theme.colorScheme.outline,
                width: isActive ? 2 : 1,
              ),
            ),
            child: Column(
              children: [
                Icon(
                  isCompleted ? Icons.check_circle : _getStepIcon(index),
                  color: isActive || isCompleted 
                      ? theme.colorScheme.primary 
                      : theme.colorScheme.onSurfaceVariant,
                  size: 20,
                ),
                const SizedBox(height: 4),
                Text(
                  _getStepTitle(index),
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: isActive || isCompleted 
                        ? theme.colorScheme.primary 
                        : theme.colorScheme.onSurfaceVariant,
                    fontWeight: isActive ? FontWeight.w600 : FontWeight.normal,
                  ),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        );
      }),
    );
  }

  IconData _getStepIcon(int step) {
    switch (step) {
      case 0: return Icons.business;
      case 1: return Icons.location_on;
      case 2: return Icons.schedule;
      case 3: return Icons.settings;
      case 4: return Icons.check;
      default: return Icons.circle;
    }
  }

  String _getStepTitle(int step) {
    switch (step) {
      case 0: return 'Business';
      case 1: return 'Location';
      case 2: return 'Hours';
      case 3: return 'Features';
      case 4: return 'Confirm';
      default: return '';
    }
  }

  Widget _buildBusinessInfoStep() {
    return SingleChildScrollView(
      child: Form(
        key: _formKey,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Tell us about your business',
              style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'This information helps us customize the experience for your industry and business type.',
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
              ),
            ),
            const SizedBox(height: 32),
            
            CustomFormField(
              label: 'Business Name',
              hint: 'Enter your business name',
              controller: _businessNameController,
              validator: FormValidators.businessName,
              required: true,
              prefixIcon: const Icon(Icons.business),
            ),
            
            const SizedBox(height: 24),
            
            CustomFormField(
              label: 'Business Description',
              hint: 'Briefly describe what your business does',
              controller: _businessDescriptionController,
              maxLines: 3,
              validator: (value) => FormValidators.length(value, min: 10, max: 500),
              prefixIcon: const Icon(Icons.description),
            ),
            
            const SizedBox(height: 24),
            
            CustomDropdownField<String>(
              label: 'Business Type',
              value: _selectedBusinessType,
              required: true,
              prefixIcon: const Icon(Icons.category),
              items: const [
                DropdownMenuItem(value: 'restaurant', child: Text('Restaurant')),
                DropdownMenuItem(value: 'retail', child: Text('Retail Store')),
                DropdownMenuItem(value: 'salon', child: Text('Salon/Spa')),
                DropdownMenuItem(value: 'events', child: Text('Event Management')),
                DropdownMenuItem(value: 'services', child: Text('Professional Services')),
                DropdownMenuItem(value: 'other', child: Text('Other')),
              ],
              onChanged: (value) {
                setState(() {
                  _selectedBusinessType = value!;
                });
              },
            ),
            
            const SizedBox(height: 24),
            
            CustomDropdownField<String>(
              label: 'Business Size',
              value: _selectedBusinessSize,
              prefixIcon: const Icon(Icons.people),
              items: const [
                DropdownMenuItem(value: 'micro', child: Text('Just me (1 person)')),
                DropdownMenuItem(value: 'small', child: Text('Small (2-10 people)')),
                DropdownMenuItem(value: 'medium', child: Text('Medium (11-50 people)')),
                DropdownMenuItem(value: 'large', child: Text('Large (50+ people)')),
              ],
              onChanged: (value) {
                setState(() {
                  _selectedBusinessSize = value!;
                });
              },
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildLocationStep() {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Where is your business located?',
            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'This helps us provide location-specific features and comply with local regulations.',
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
            ),
          ),
          const SizedBox(height: 32),
          
          CustomFormField(
            label: 'Street Address',
            hint: 'Enter your business address',
            controller: _addressController,
            validator: FormValidators.required,
            required: true,
            prefixIcon: const Icon(Icons.location_on),
          ),
          
          const SizedBox(height: 24),
          
          Row(
            children: [
              Expanded(
                flex: 2,
                child: CustomFormField(
                  label: 'City',
                  hint: 'City',
                  controller: _cityController,
                  validator: FormValidators.required,
                  required: true,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: CustomFormField(
                  label: 'State',
                  hint: 'State',
                  controller: _stateController,
                  validator: FormValidators.required,
                  required: true,
                ),
              ),
            ],
          ),
          
          const SizedBox(height: 24),
          
          Row(
            children: [
              Expanded(
                child: CustomFormField(
                  label: 'ZIP Code',
                  hint: 'ZIP',
                  controller: _zipCodeController,
                  validator: FormValidators.required,
                  required: true,
                  keyboardType: TextInputType.number,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                flex: 2,
                child: CustomFormField(
                  label: 'Country',
                  controller: _countryController,
                  enabled: false,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildOperatingHoursStep() {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'When are you open?',
            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Set your operating hours to help customers know when you\'re available.',
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
            ),
          ),
          const SizedBox(height: 32),
          
          ..._operatingHours.entries.map((entry) {
            final day = entry.key;
            final hours = entry.value;
            final isClosed = hours['closed'] == 'true';
            
            return Card(
              margin: const EdgeInsets.only(bottom: 16),
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  children: [
                    Row(
                      children: [
                        Expanded(
                          child: Text(
                            day.substring(0, 1).toUpperCase() + day.substring(1),
                            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ),
                        Switch(
                          value: !isClosed,
                          onChanged: (value) {
                            setState(() {
                              _operatingHours[day]!['closed'] = (!value).toString();
                            });
                          },
                        ),
                      ],
                    ),
                    if (!isClosed) ...[
                      const SizedBox(height: 16),
                      Row(
                        children: [
                          Expanded(
                            child: CustomFormField(
                              label: 'Open',
                              controller: TextEditingController(text: hours['open']),
                              readOnly: true,
                              onTap: () async {
                                final time = await showTimePicker(
                                  context: context,
                                  initialTime: TimeOfDay(
                                    hour: int.parse(hours['open']!.split(':')[0]),
                                    minute: int.parse(hours['open']!.split(':')[1]),
                                  ),
                                );
                                if (time != null) {
                                  setState(() {
                                    _operatingHours[day]!['open'] = 
                                        '${time.hour.toString().padLeft(2, '0')}:${time.minute.toString().padLeft(2, '0')}';
                                  });
                                }
                              },
                            ),
                          ),
                          const SizedBox(width: 16),
                          Expanded(
                            child: CustomFormField(
                              label: 'Close',
                              controller: TextEditingController(text: hours['close']),
                              readOnly: true,
                              onTap: () async {
                                final time = await showTimePicker(
                                  context: context,
                                  initialTime: TimeOfDay(
                                    hour: int.parse(hours['close']!.split(':')[0]),
                                    minute: int.parse(hours['close']!.split(':')[1]),
                                  ),
                                );
                                if (time != null) {
                                  setState(() {
                                    _operatingHours[day]!['close'] = 
                                        '${time.hour.toString().padLeft(2, '0')}:${time.minute.toString().padLeft(2, '0')}';
                                  });
                                }
                              },
                            ),
                          ),
                        ],
                      ),
                    ],
                  ],
                ),
              ),
            );
          }).toList(),
        ],
      ),
    );
  }

  Widget _buildFeaturesStep() {
    final features = [
      {'id': 'inventory', 'name': 'Inventory Management', 'description': 'Track stock levels and supplies'},
      {'id': 'pos', 'name': 'Point of Sale', 'description': 'Process sales and payments'},
      {'id': 'appointments', 'name': 'Appointments', 'description': 'Schedule and manage bookings'},
      {'id': 'staff', 'name': 'Staff Management', 'description': 'Manage employees and schedules'},
      {'id': 'reports', 'name': 'Analytics & Reports', 'description': 'Business insights and reporting'},
      {'id': 'customers', 'name': 'Customer Management', 'description': 'Track customer information and history'},
    ];

    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Which features do you need?',
            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Select the features you\'d like to enable. You can always change these later.',
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
            ),
          ),
          const SizedBox(height: 32),
          
          ...features.map((feature) {
            final isSelected = _selectedFeatures.contains(feature['id']);
            
            return CheckboxListTile(
              title: Text(feature['name']!),
              subtitle: Text(feature['description']!),
              value: isSelected,
              onChanged: (value) {
                setState(() {
                  if (value == true) {
                    _selectedFeatures.add(feature['id']!);
                  } else {
                    _selectedFeatures.remove(feature['id']!);
                  }
                });
              },
            );
          }).toList(),
          
          const SizedBox(height: 32),
          
          Text(
            'Preferences',
            style: Theme.of(context).textTheme.titleLarge?.copyWith(
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 16),
          
          CustomSwitchField(
            label: 'Enable Analytics',
            subtitle: 'Get insights about your business performance',
            value: _enableAnalytics,
            onChanged: (value) {
              setState(() {
                _enableAnalytics = value;
              });
            },
          ),
          
          CustomSwitchField(
            label: 'Enable Notifications',
            subtitle: 'Receive alerts about important events',
            value: _enableNotifications,
            onChanged: (value) {
              setState(() {
                _enableNotifications = value;
              });
            },
          ),
          
          CustomSwitchField(
            label: 'Enable Integrations',
            subtitle: 'Connect with third-party services',
            value: _enableIntegrations,
            onChanged: (value) {
              setState(() {
                _enableIntegrations = value;
              });
            },
          ),
        ],
      ),
    );
  }

  Widget _buildConfirmationStep() {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Almost there!',
            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Review your information and click "Complete Setup" to finish.',
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: Theme.of(context).colorScheme.onSurface.withOpacity(0.7),
            ),
          ),
          const SizedBox(height: 32),
          
          Card(
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildSummarySection('Business Information', [
                    'Name: ${_businessNameController.text}',
                    'Type: ${_selectedBusinessType.substring(0, 1).toUpperCase()}${_selectedBusinessType.substring(1)}',
                    'Size: ${_selectedBusinessSize.substring(0, 1).toUpperCase()}${_selectedBusinessSize.substring(1)}',
                  ]),
                  
                  const Divider(height: 32),
                  
                  _buildSummarySection('Location', [
                    _addressController.text,
                    '${_cityController.text}, ${_stateController.text} ${_zipCodeController.text}',
                    _countryController.text,
                  ]),
                  
                  const Divider(height: 32),
                  
                  _buildSummarySection('Features', [
                    '${_selectedFeatures.length} features selected',
                    'Analytics: ${_enableAnalytics ? 'Enabled' : 'Disabled'}',
                    'Notifications: ${_enableNotifications ? 'Enabled' : 'Disabled'}',
                    'Integrations: ${_enableIntegrations ? 'Enabled' : 'Disabled'}',
                  ]),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSummarySection(String title, List<String> items) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: Theme.of(context).textTheme.titleMedium?.copyWith(
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 8),
        ...items.map((item) => Padding(
          padding: const EdgeInsets.only(bottom: 4),
          child: Text(
            item,
            style: Theme.of(context).textTheme.bodyMedium,
          ),
        )).toList(),
      ],
    );
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
            child: LoadingButton.elevated(
              isLoading: _isLoading,
              onPressed: _nextStep,
              child: Text(_currentStep == 4 ? 'Complete Setup' : 'Next'),
            ),
          ),
        ],
      ),
    );
  }
}