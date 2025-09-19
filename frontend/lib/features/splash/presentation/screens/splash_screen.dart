import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:lottie/lottie.dart';
import '../../../../core/router/app_router.dart';
import '../../../../core/constants/app_constants.dart';
import '../../../../core/services/storage_service.dart';

/// Splash screen that shows app logo and handles initial navigation
class SplashScreen extends ConsumerStatefulWidget {
  const SplashScreen({super.key});

  @override
  ConsumerState<SplashScreen> createState() => _SplashScreenState();
}

class _SplashScreenState extends ConsumerState<SplashScreen>
    with SingleTickerProviderStateMixin {
  late AnimationController _animationController;
  late Animation<double> _fadeAnimation;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 2000),
      vsync: this,
    );

    _fadeAnimation = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(CurvedAnimation(
      parent: _animationController,
      curve: const Interval(0.0, 0.6, curve: Curves.easeIn),
    ));

    _scaleAnimation = Tween<double>(
      begin: 0.8,
      end: 1.0,
    ).animate(CurvedAnimation(
      parent: _animationController,
      curve: const Interval(0.0, 0.6, curve: Curves.elasticOut),
    ));

    _initializeApp();
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  Future<void> _initializeApp() async {
    // Start animations
    _animationController.forward();

    // Initialize services
    await _initializeServices();

    // Wait for minimum splash duration
    await Future.delayed(const Duration(milliseconds: 3000));

    // Navigate to appropriate screen
    await _navigateToNextScreen();
  }

  Future<void> _initializeServices() async {
    try {
      // Initialize storage service
      await StorageService.initialize();
      
      // Initialize other services here
      // await ApiService.initialize();
      // await WebSocketService.connect();
      
      print('Services initialized successfully');
    } catch (e) {
      print('Error initializing services: $e');
    }
  }

  Future<void> _navigateToNextScreen() async {
    if (!mounted) return;

    try {
      // Check if user is authenticated
      final accessToken = StorageService.getUserData<String>(
        AppConstants.accessTokenKey,
      );
      
      final isFirstTime = StorageService.getUserData<bool>(
        AppConstants.firstTimeKey,
      ) ?? true;

      if (accessToken != null && accessToken.isNotEmpty) {
        // User is authenticated, go to dashboard
        AppRouter.navigateToNamedAndClearStack(RouteNames.dashboard);
      } else if (isFirstTime) {
        // First time user, show onboarding/business setup
        AppRouter.navigateToNamedAndClearStack(RouteNames.businessSetup);
      } else {
        // Returning user, show login
        AppRouter.navigateToNamedAndClearStack(RouteNames.login);
      }
    } catch (e) {
      print('Navigation error: $e');
      // Fallback to login screen
      AppRouter.navigateToNamedAndClearStack(RouteNames.login);
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final size = MediaQuery.of(context).size;

    return Scaffold(
      backgroundColor: theme.colorScheme.primary,
      body: AnimatedBuilder(
        animation: _animationController,
        builder: (context, child) {
          return Center(
            child: FadeTransition(
              opacity: _fadeAnimation,
              child: ScaleTransition(
                scale: _scaleAnimation,
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    // App Logo/Animation
                    Container(
                      width: size.width * 0.3,
                      height: size.width * 0.3,
                      constraints: const BoxConstraints(
                        minWidth: 120,
                        maxWidth: 200,
                        minHeight: 120,
                        maxHeight: 200,
                      ),
                      child: _buildLogo(context),
                    ),
                    
                    const SizedBox(height: 32),
                    
                    // App Name
                    Text(
                      AppConstants.appName,
                      style: theme.textTheme.displayMedium?.copyWith(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                        letterSpacing: 1.2,
                      ),
                      textAlign: TextAlign.center,
                    ),
                    
                    const SizedBox(height: 8),
                    
                    // Tagline
                    Text(
                      'Business AI OS',
                      style: theme.textTheme.titleLarge?.copyWith(
                        color: Colors.white.withOpacity(0.9),
                        fontWeight: FontWeight.w400,
                        letterSpacing: 0.5,
                      ),
                      textAlign: TextAlign.center,
                    ),
                    
                    const SizedBox(height: 48),
                    
                    // Loading indicator
                    SizedBox(
                      width: 32,
                      height: 32,
                      child: CircularProgressIndicator(
                        valueColor: AlwaysStoppedAnimation<Color>(
                          Colors.white.withOpacity(0.8),
                        ),
                        strokeWidth: 3,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    );
  }

  Widget _buildLogo(BuildContext context) {
    // Try to load Lottie animation first, fallback to static icon
    try {
      return Lottie.asset(
        'assets/animations/olympus_logo.json',
        fit: BoxFit.contain,
        errorBuilder: (context, error, stackTrace) {
          return _buildFallbackLogo(context);
        },
      );
    } catch (e) {
      return _buildFallbackLogo(context);
    }
  }

  Widget _buildFallbackLogo(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.1),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(
          color: Colors.white.withOpacity(0.3),
          width: 2,
        ),
      ),
      child: Icon(
        Icons.business,
        size: 80,
        color: Colors.white,
      ),
    );
  }
}