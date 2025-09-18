import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../../features/auth/presentation/screens/login_screen.dart';
import '../../features/auth/presentation/screens/signup_screen.dart';
import '../../features/dashboard/presentation/screens/dashboard_screen.dart';
import '../../features/splash/presentation/screens/splash_screen.dart';
import '../../shared/presentation/screens/error_screen.dart';

/// App router configuration using GoRouter
class AppRouter {
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

      // Main app routes (protected)
      GoRoute(
        path: '/dashboard',
        name: 'dashboard',
        builder: (context, state) => const DashboardScreen(),
        routes: [
          // Dashboard sub-routes will be added here
          GoRoute(
            path: 'orders',
            name: 'orders',
            builder: (context, state) => const Placeholder(
              child: Text('Orders Screen'),
            ),
          ),
          GoRoute(
            path: 'inventory',
            name: 'inventory',
            builder: (context, state) => const Placeholder(
              child: Text('Inventory Screen'),
            ),
          ),
          GoRoute(
            path: 'customers',
            name: 'customers',
            builder: (context, state) => const Placeholder(
              child: Text('Customers Screen'),
            ),
          ),
          GoRoute(
            path: 'analytics',
            name: 'analytics',
            builder: (context, state) => const Placeholder(
              child: Text('Analytics Screen'),
            ),
          ),
          GoRoute(
            path: 'settings',
            name: 'settings',
            builder: (context, state) => const Placeholder(
              child: Text('Settings Screen'),
            ),
          ),
        ],
      ),

      // Business setup routes
      GoRoute(
        path: '/business-setup',
        name: 'business-setup',
        builder: (context, state) => const Placeholder(
          child: Text('Business Setup Screen'),
        ),
      ),

      // Profile routes
      GoRoute(
        path: '/profile',
        name: 'profile',
        builder: (context, state) => const Placeholder(
          child: Text('Profile Screen'),
        ),
      ),

      // Help and support routes
      GoRoute(
        path: '/help',
        name: 'help',
        builder: (context, state) => const Placeholder(
          child: Text('Help Screen'),
        ),
      ),
    ],
    redirect: (context, state) {
      // Add authentication logic here
      // For now, allow access to all routes
      return null;
    },
  );

  /// Navigate to a specific route by name
  static void navigateToNamed(String name, {Map<String, String>? pathParameters}) {
    router.pushNamed(name, pathParameters: pathParameters ?? {});
  }

  /// Navigate and replace the current route
  static void navigateToNamedAndReplace(String name, {Map<String, String>? pathParameters}) {
    router.pushReplacementNamed(name, pathParameters: pathParameters ?? {});
  }

  /// Navigate and clear the stack
  static void navigateToNamedAndClearStack(String name, {Map<String, String>? pathParameters}) {
    router.goNamed(name, pathParameters: pathParameters ?? {});
  }

  /// Go back to the previous route
  static void goBack() {
    router.pop();
  }

  /// Check if we can go back
  static bool canGoBack() {
    return router.canPop();
  }
}

/// Route names for easy reference
class RouteNames {
  static const String splash = 'splash';
  static const String login = 'login';
  static const String signup = 'signup';
  static const String dashboard = 'dashboard';
  static const String orders = 'orders';
  static const String inventory = 'inventory';
  static const String customers = 'customers';
  static const String analytics = 'analytics';
  static const String settings = 'settings';
  static const String businessSetup = 'business-setup';
  static const String profile = 'profile';
  static const String help = 'help';
}

/// Route paths for easy reference
class RoutePaths {
  static const String splash = '/';
  static const String login = '/login';
  static const String signup = '/signup';
  static const String dashboard = '/dashboard';
  static const String orders = '/dashboard/orders';
  static const String inventory = '/dashboard/inventory';
  static const String customers = '/dashboard/customers';
  static const String analytics = '/dashboard/analytics';
  static const String settings = '/dashboard/settings';
  static const String businessSetup = '/business-setup';
  static const String profile = '/profile';
  static const String help = '/help';
}