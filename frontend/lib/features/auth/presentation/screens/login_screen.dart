import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/router/app_router.dart';
import '../../../../shared/presentation/widgets/adaptive_layout.dart';
import '../../../../shared/presentation/widgets/responsive_form.dart';

/// Login screen with responsive design and natural language support
class LoginScreen extends ConsumerStatefulWidget {
  const LoginScreen({super.key});

  @override
  ConsumerState<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _naturalLanguageController = TextEditingController();
  
  bool _isLoading = false;
  bool _obscurePassword = true;
  bool _showNaturalLanguageInput = false;

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    _naturalLanguageController.dispose();
    super.dispose();
  }

  Future<void> _handleLogin() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() => _isLoading = true);

    try {
      // TODO: Implement authentication with Rust backend
      // final result = await ref.read(authServiceProvider).login(
      //   email: _emailController.text.trim(),
      //   password: _passwordController.text,
      // );
      
      // Simulate login for now
      await Future.delayed(const Duration(seconds: 2));
      
      if (mounted) {
        AppRouter.navigateToNamedAndClearStack(RouteNames.dashboard);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Login failed: ${e.toString()}'),
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

  Future<void> _handleNaturalLanguageLogin() async {
    final query = _naturalLanguageController.text.trim();
    if (query.isEmpty) return;

    setState(() => _isLoading = true);

    try {
      // TODO: Implement natural language processing for login
      // This could parse queries like:
      // "Log me in as john@example.com"
      // "Sign in with my Google account"
      // "Use my fingerprint to login"
      
      await Future.delayed(const Duration(seconds: 2));
      
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Natural language login not yet implemented'),
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Login failed: ${e.toString()}'),
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
    final size = MediaQuery.of(context).size;

    return AdaptiveLayout(
      child: Scaffold(
        body: SafeArea(
          child: ResponsiveForm(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const SizedBox(height: 16), // Reduced from 32
                  
                  // Header
                  _buildHeader(theme),
                  
                  const SizedBox(height: 24), // Reduced from 48
                  
                  // Natural Language Toggle
                  _buildNaturalLanguageToggle(theme),
                  
                  const SizedBox(height: 16), // Reduced from 24
                  
                  // Login Form or Natural Language Input
                  _showNaturalLanguageInput
                      ? _buildNaturalLanguageInput(theme)
                      : _buildLoginForm(theme),
                  
                  const SizedBox(height: 16), // Reduced from 24
                  
                  // Social Login Options
                  _buildSocialLogin(theme),
                  
                  const SizedBox(height: 16), // Reduced from 32
                  
                  // Sign Up Link
                  _buildSignUpLink(theme),
                  
                  const SizedBox(height: 16), // Reduced from 32
                  
                  // Help Link
                  _buildHelpLink(theme),
                  
                  const SizedBox(height: 16), // Reduced from 32
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildHeader(ThemeData theme) {
    return Column(
      children: [
        // Logo
        Container(
          width: 80,
          height: 80,
          decoration: BoxDecoration(
            color: theme.colorScheme.primary.withOpacity(0.1),
            borderRadius: BorderRadius.circular(20),
          ),
          child: Icon(
            Icons.business,
            size: 40,
            color: theme.colorScheme.primary,
          ),
        ),
        
        const SizedBox(height: 24),
        
        Text(
          'Welcome back',
          style: theme.textTheme.displaySmall?.copyWith(
            fontWeight: FontWeight.bold,
          ),
          textAlign: TextAlign.center,
        ),
        
        const SizedBox(height: 8),
        
        Text(
          'Sign in to continue to your business dashboard',
          style: theme.textTheme.bodyLarge?.copyWith(
            color: theme.colorScheme.onBackground.withOpacity(0.7),
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }

  Widget _buildNaturalLanguageToggle(ThemeData theme) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        TextButton.icon(
          onPressed: () {
            setState(() {
              _showNaturalLanguageInput = !_showNaturalLanguageInput;
            });
          },
          icon: Icon(
            _showNaturalLanguageInput 
                ? Icons.keyboard 
                : Icons.mic,
          ),
          label: Text(
            _showNaturalLanguageInput 
                ? 'Use form instead' 
                : 'Tell me what to do',
          ),
        ),
      ],
    );
  }

  Widget _buildNaturalLanguageInput(ThemeData theme) {
    return Column(
      children: [
        TextFormField(
          controller: _naturalLanguageController,
          decoration: const InputDecoration(
            labelText: 'Tell me how you\'d like to sign in...',
            hintText: 'e.g., "Log me in as john@example.com"',
            prefixIcon: Icon(Icons.chat_bubble_outline),
          ),
          maxLines: 3,
          textInputAction: TextInputAction.done,
          onFieldSubmitted: (_) => _handleNaturalLanguageLogin(),
        ),
        
        const SizedBox(height: 24),
        
        SizedBox(
          width: double.infinity,
          child: ElevatedButton(
            onPressed: _isLoading ? null : _handleNaturalLanguageLogin,
            child: _isLoading
                ? const SizedBox(
                    height: 20,
                    width: 20,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const Text('Process Request'),
          ),
        ),
      ],
    );
  }

  Widget _buildLoginForm(ThemeData theme) {
    return Form(
      key: _formKey,
      child: Column(
        children: [
          // Email field
          TextFormField(
            controller: _emailController,
            decoration: const InputDecoration(
              labelText: 'Email',
              hintText: 'Enter your email address',
              prefixIcon: Icon(Icons.email_outlined),
            ),
            keyboardType: TextInputType.emailAddress,
            textInputAction: TextInputAction.next,
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter your email';
              }
              if (!RegExp(r'^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$').hasMatch(value)) {
                return 'Please enter a valid email';
              }
              return null;
            },
          ),
          
          const SizedBox(height: 16),
          
          // Password field
          TextFormField(
            controller: _passwordController,
            decoration: InputDecoration(
              labelText: 'Password',
              hintText: 'Enter your password',
              prefixIcon: const Icon(Icons.lock_outlined),
              suffixIcon: IconButton(
                icon: Icon(
                  _obscurePassword ? Icons.visibility : Icons.visibility_off,
                ),
                onPressed: () {
                  setState(() {
                    _obscurePassword = !_obscurePassword;
                  });
                },
              ),
            ),
            obscureText: _obscurePassword,
            textInputAction: TextInputAction.done,
            onFieldSubmitted: (_) => _handleLogin(),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter your password';
              }
              if (value.length < 6) {
                return 'Password must be at least 6 characters';
              }
              return null;
            },
          ),
          
          const SizedBox(height: 8),
          
          // Forgot password
          Align(
            alignment: Alignment.centerRight,
            child: TextButton(
              onPressed: () {
                // TODO: Implement forgot password
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Forgot password not yet implemented'),
                  ),
                );
              },
              child: const Text('Forgot password?'),
            ),
          ),
          
          const SizedBox(height: 24),
          
          // Login button
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: _isLoading ? null : _handleLogin,
              child: _isLoading
                  ? const SizedBox(
                      height: 20,
                      width: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Text('Sign In'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSocialLogin(ThemeData theme) {
    return Column(
      children: [
        Row(
          children: [
            Expanded(child: Divider(color: theme.dividerColor)),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Text(
                'Or continue with',
                style: theme.textTheme.bodySmall?.copyWith(
                  color: theme.colorScheme.onBackground.withOpacity(0.6),
                ),
              ),
            ),
            Expanded(child: Divider(color: theme.dividerColor)),
          ],
        ),
        
        const SizedBox(height: 24),
        
        Row(
          children: [
            Expanded(
              child: OutlinedButton.icon(
                onPressed: () {
                  // TODO: Implement Google login
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Google login not yet implemented'),
                    ),
                  );
                },
                icon: const Icon(Icons.g_mobiledata, size: 24),
                label: const Text('Continue with Google'),
              ),
            ),
            
            const SizedBox(width: 16),
            
            Expanded(
              child: OutlinedButton.icon(
                onPressed: () {
                  // TODO: Implement Apple login
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Apple login not yet implemented'),
                    ),
                  );
                },
                icon: const Icon(Icons.apple, size: 24),
                label: const Text('Continue with Apple'),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildSignUpLink(ThemeData theme) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          "Don't have an account? ",
          style: theme.textTheme.bodyMedium,
        ),
        TextButton(
          onPressed: () {
            AppRouter.navigateToNamed(RouteNames.signup);
          },
          child: const Text('Sign Up'),
        ),
      ],
    );
  }

  Widget _buildHelpLink(ThemeData theme) {
    return TextButton(
      onPressed: () {
        AppRouter.navigateToNamed(RouteNames.help);
      },
      child: Text(
        'Need help? Contact support',
        style: theme.textTheme.bodySmall?.copyWith(
          color: theme.colorScheme.onBackground.withOpacity(0.6),
        ),
      ),
    );
  }
}