import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../features/onboarding/screens/industry_selection_screen.dart';
import '../../features/restaurant/screens/restaurant_dashboard_screen.dart';
import '../../features/auth/presentation/screens/login_screen.dart';
import '../../features/auth/presentation/screens/signup_screen.dart';
import '../../features/splash/presentation/screens/splash_screen.dart';
import '../../shared/presentation/screens/error_screen.dart';

/// Temporary minimal router for testing industry branding
class TempRouter {
  static final _rootNavigatorKey = GlobalKey<NavigatorState>();

  static final GoRouter router = GoRouter(
    navigatorKey: _rootNavigatorKey,
    initialLocation: '/',
    debugLogDiagnostics: true,
    errorBuilder: (context, state) => ErrorScreen(error: state.error.toString()),
    routes: [
      // Splash route
      GoRoute(
        path: '/',
        name: 'splash',
        builder: (context, state) => const SplashScreen(),
      ),

      // Industry selection (onboarding)
      GoRoute(
        path: '/industry-selection',
        name: 'industry-selection',
        builder: (context, state) => const IndustrySelectionScreen(),
      ),

      // Authentication routes
      GoRoute(
        path: '/login',
        name: 'login',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/signup',
        name: 'signup',
        builder: (context, state) => const SignupScreen(),
      ),

      // Restaurant dashboard
      GoRoute(
        path: '/restaurant-dashboard',
        name: 'restaurant-dashboard',
        builder: (context, state) => const RestaurantDashboardScreen(),
      ),

      // Simple demo route
      GoRoute(
        path: '/demo',
        name: 'demo',
        builder: (context, state) => Scaffold(
          appBar: AppBar(title: const Text('Industry Branding Demo')),
          body: const Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text('Industry Branding System is Working!'),
                SizedBox(height: 20),
                Text('Navigate between industries to see theme changes.'),
              ],
            ),
          ),
        ),
      ),
    ],
  );
}