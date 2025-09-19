import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/core/providers/auth_provider.dart';
import 'package:frontend/core/models/auth_state.dart';

void main() {
  group('Auth Provider Tests', () {
    late ProviderContainer container;

    setUp(() {
      container = ProviderContainer();
    });

    tearDown(() {
      container.dispose();
    });

    test('AuthNotifier initializes with initial state', () {
      final authState = container.read(authStateProvider);
      
      expect(authState, const AuthState.initial());
    });

    test('AuthNotifier provides current user correctly', () {
      final currentUser = container.read(currentUserProvider);
      expect(currentUser, isNull);
    });

    test('AuthNotifier provides authentication status correctly', () {
      final isAuthenticated = container.read(isAuthenticatedProvider);
      expect(isAuthenticated, isFalse);
    });

    test('AuthNotifier provides loading status correctly', () {
      final isLoading = container.read(isAuthLoadingProvider);
      expect(isLoading, isFalse);
    });
  });
}