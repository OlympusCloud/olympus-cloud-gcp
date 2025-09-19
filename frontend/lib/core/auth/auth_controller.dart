import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../storage/storage_service.dart';
import '../network/services/auth_api_service.dart';
import 'auth_models.dart';

part 'auth_controller.g.dart';

/// Authentication state
enum AuthState {
  initial,
  loading,
  authenticated,
  unauthenticated,
}

/// Authentication controller using Riverpod 2.0
@Riverpod(keepAlive: true)
class AuthController extends _$AuthController {
  late final AuthApiService _authApi;
  late final StorageService _storage;
  
  @override
  AsyncValue<AuthState> build() {
    _authApi = ref.read(authApiServiceProvider);
    _storage = ref.read(storageServiceProvider);
    
    // Check if user is already authenticated on app start
    _checkAuthStatus();
    
    return const AsyncValue.data(AuthState.initial);
  }
  
  /// Check current authentication status
  Future<void> _checkAuthStatus() async {
    state = const AsyncValue.loading();
    
    try {
      final token = await _storage.getAccessToken();
      final user = await _storage.getCurrentUser();
      
      if (token != null && user != null) {
        // Try to refresh the user data to validate token
        try {
          final currentUser = await _authApi.getCurrentUser();
          await _storage.saveCurrentUser(currentUser);
          ref.read(currentUserProvider.notifier).state = currentUser;
          state = const AsyncValue.data(AuthState.authenticated);
        } catch (e) {
          // Token is invalid, clear storage and set unauthenticated
          await _storage.clearAuth();
          ref.read(currentUserProvider.notifier).state = null;
          state = const AsyncValue.data(AuthState.unauthenticated);
        }
      } else {
        state = const AsyncValue.data(AuthState.unauthenticated);
      }
    } catch (e) {
      state = AsyncValue.error(e, StackTrace.current);
    }
  }
  
  /// Login with email and password
  Future<void> login({
    required String email,
    required String password,
  }) async {
    state = const AsyncValue.loading();
    
    try {
      final authResponse = await _authApi.login(
        email: email,
        password: password,
      );
      
      // Save tokens and user data
      await _storage.saveTokens(
        accessToken: authResponse.accessToken,
        refreshToken: authResponse.refreshToken,
      );
      await _storage.saveCurrentUser(authResponse.user);
      
      // Update providers
      ref.read(currentUserProvider.notifier).state = authResponse.user;
      
      state = const AsyncValue.data(AuthState.authenticated);
    } catch (e) {
      state = AsyncValue.error(e, StackTrace.current);
    }
  }
  
  /// Register new user
  Future<void> register({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    String? tenantId,
  }) async {
    state = const AsyncValue.loading();
    
    try {
      final authResponse = await _authApi.register(
        email: email,
        password: password,
        firstName: firstName,
        lastName: lastName,
        tenantId: tenantId,
      );
      
      // Save tokens and user data
      await _storage.saveTokens(
        accessToken: authResponse.accessToken,
        refreshToken: authResponse.refreshToken,
      );
      await _storage.saveCurrentUser(authResponse.user);
      
      // Update providers
      ref.read(currentUserProvider.notifier).state = authResponse.user;
      
      state = const AsyncValue.data(AuthState.authenticated);
    } catch (e) {
      state = AsyncValue.error(e, StackTrace.current);
    }
  }
  
  /// Logout user
  Future<void> logout() async {
    state = const AsyncValue.loading();
    
    try {
      // Call logout API to invalidate tokens on server
      await _authApi.logout();
    } catch (e) {
      // Continue with logout even if API call fails
      print('Logout API call failed: $e');
    }
    
    // Clear local storage
    await _storage.clearAuth();
    ref.read(currentUserProvider.notifier).state = null;
    
    state = const AsyncValue.data(AuthState.unauthenticated);
  }
  
  /// Change password
  Future<void> changePassword({
    required String currentPassword,
    required String newPassword,
  }) async {
    try {
      await _authApi.changePassword(
        currentPassword: currentPassword,
        newPassword: newPassword,
      );
    } catch (e) {
      rethrow;
    }
  }
  
  /// Forgot password
  Future<void> forgotPassword(String email) async {
    try {
      await _authApi.forgotPassword(email);
    } catch (e) {
      rethrow;
    }
  }
  
  /// Reset password
  Future<void> resetPassword({
    required String token,
    required String newPassword,
  }) async {
    try {
      await _authApi.resetPassword(
        token: token,
        newPassword: newPassword,
      );
    } catch (e) {
      rethrow;
    }
  }
  
  /// Verify email
  Future<void> verifyEmail(String token) async {
    try {
      await _authApi.verifyEmail(token);
      
      // Refresh user data after email verification
      final currentUser = await _authApi.getCurrentUser();
      await _storage.saveCurrentUser(currentUser);
      ref.read(currentUserProvider.notifier).state = currentUser;
    } catch (e) {
      rethrow;
    }
  }
  
  /// Refresh current user data
  Future<void> refreshUserData() async {
    try {
      final currentUser = await _authApi.getCurrentUser();
      await _storage.saveCurrentUser(currentUser);
      ref.read(currentUserProvider.notifier).state = currentUser;
    } catch (e) {
      rethrow;
    }
  }
}

/// Current user provider
final currentUserProvider = StateProvider<User?>((ref) => null);

/// Check if user is authenticated
final isAuthenticatedProvider = Provider<bool>((ref) {
  final authState = ref.watch(authControllerProvider);
  return authState.maybeWhen(
    data: (state) => state == AuthState.authenticated,
    orElse: () => false,
  );
});

/// Get current user safely
final currentUserSafeProvider = Provider<User?>((ref) {
  final isAuth = ref.watch(isAuthenticatedProvider);
  if (!isAuth) return null;
  return ref.watch(currentUserProvider);
});

/// Check if user has specific permission
final hasPermissionProvider = Provider.family<bool, String>((ref, permission) {
  final user = ref.watch(currentUserSafeProvider);
  return user?.permissions.contains(permission) ?? false;
});

/// Check if user has specific role
final hasRoleProvider = Provider.family<bool, String>((ref, roleName) {
  final user = ref.watch(currentUserSafeProvider);
  return user?.roles.any((role) => role.name == roleName) ?? false;
});

/// Get current tenant
final currentTenantProvider = Provider<String?>((ref) {
  final user = ref.watch(currentUserSafeProvider);
  return user?.tenantId;
});