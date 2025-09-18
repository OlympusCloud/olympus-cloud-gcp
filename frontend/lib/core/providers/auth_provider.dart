import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/user.dart';
import '../models/auth_state.dart';
import '../services/api_service.dart';
import '../services/storage_service.dart';
import '../constants/app_constants.dart';

/// Authentication state provider
final authStateProvider = StateNotifierProvider<AuthNotifier, AuthState>((ref) {
  return AuthNotifier();
});

/// Authentication notifier that manages auth state
class AuthNotifier extends StateNotifier<AuthState> {
  AuthNotifier() : super(const AuthState.initial()) {
    _checkAuthStatus();
  }

  /// Check if user is already authenticated on app start
  Future<void> _checkAuthStatus() async {
    state = const AuthState.loading();
    
    try {
      final token = StorageService.getUserData<String>(AppConstants.accessTokenKey);
      final userData = StorageService.getUserData<Map<String, dynamic>>(AppConstants.userDataKey);
      
      if (token != null && token.isNotEmpty && userData != null) {
        final user = User.fromJson(userData);
        state = AuthState.authenticated(user: user, token: token);
      } else {
        state = const AuthState.unauthenticated();
      }
    } catch (e) {
      state = AuthState.error(e.toString());
    }
  }

  /// Login with email and password
  Future<void> login({
    required String email,
    required String password,
  }) async {
    state = const AuthState.loading();
    
    try {
      final response = await ApiService.post('/auth/login', data: {
        'email': email,
        'password': password,
      });
      
      if (response.statusCode == 200) {
        final data = response.data;
        final token = data['access_token'];
        final refreshToken = data['refresh_token'];
        final user = User.fromJson(data['user']);
        
        // Store tokens and user data
        await StorageService.saveUserData(AppConstants.accessTokenKey, token);
        await StorageService.saveUserData(AppConstants.refreshTokenKey, refreshToken);
        await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
        
        state = AuthState.authenticated(user: user, token: token);
      } else {
        state = AuthState.error('Login failed: ${response.data['message']}');
      }
    } catch (e) {
      state = AuthState.error('Login failed: ${e.toString()}');
    }
  }

  /// Register new user
  Future<void> register({
    required String firstName,
    required String lastName,
    required String email,
    required String password,
    required String businessName,
    required String businessType,
  }) async {
    state = const AuthState.loading();
    
    try {
      final response = await ApiService.post('/auth/register', data: {
        'first_name': firstName,
        'last_name': lastName,
        'email': email,
        'password': password,
        'business_name': businessName,
        'business_type': businessType,
      });
      
      if (response.statusCode == 201) {
        final data = response.data;
        final token = data['access_token'];
        final refreshToken = data['refresh_token'];
        final user = User.fromJson(data['user']);
        
        // Store tokens and user data
        await StorageService.saveUserData(AppConstants.accessTokenKey, token);
        await StorageService.saveUserData(AppConstants.refreshTokenKey, refreshToken);
        await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
        
        state = AuthState.authenticated(user: user, token: token);
      } else {
        state = AuthState.error('Registration failed: ${response.data['message']}');
      }
    } catch (e) {
      state = AuthState.error('Registration failed: ${e.toString()}');
    }
  }

  /// Logout user
  Future<void> logout() async {
    try {
      // Call logout API
      await ApiService.post('/auth/logout');
    } catch (e) {
      // Continue with local logout even if API call fails
    } finally {
      // Clear local storage
      await StorageService.removeUserData(AppConstants.accessTokenKey);
      await StorageService.removeUserData(AppConstants.refreshTokenKey);
      await StorageService.removeUserData(AppConstants.userDataKey);
      
      state = const AuthState.unauthenticated();
    }
  }

  /// Refresh authentication token
  Future<void> refreshToken() async {
    try {
      final refreshToken = StorageService.getUserData<String>(AppConstants.refreshTokenKey);
      
      if (refreshToken == null) {
        state = const AuthState.unauthenticated();
        return;
      }
      
      final response = await ApiService.post('/auth/refresh', data: {
        'refresh_token': refreshToken,
      });
      
      if (response.statusCode == 200) {
        final data = response.data;
        final newToken = data['access_token'];
        final user = User.fromJson(data['user']);
        
        await StorageService.saveUserData(AppConstants.accessTokenKey, newToken);
        
        state = AuthState.authenticated(user: user, token: newToken);
      } else {
        await logout();
      }
    } catch (e) {
      await logout();
    }
  }

  /// Update user profile
  Future<void> updateProfile(User updatedUser) async {
    try {
      final response = await ApiService.put('/auth/profile', data: updatedUser.toJson());
      
      if (response.statusCode == 200) {
        final user = User.fromJson(response.data['user']);
        await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
        
        if (state is AuthenticatedState) {
          final currentState = state as AuthenticatedState;
          state = AuthState.authenticated(user: user, token: currentState.token);
        }
      }
    } catch (e) {
      // Handle error silently or show notification
    }
  }
}

/// Current user provider - extracted from auth state
final currentUserProvider = Provider<User?>((ref) {
  final authState = ref.watch(authStateProvider);
  return authState.maybeWhen(
    authenticated: (user, token) => user,
    orElse: () => null,
  );
});

/// Is authenticated provider
final isAuthenticatedProvider = Provider<bool>((ref) {
  final authState = ref.watch(authStateProvider);
  return authState.maybeWhen(
    authenticated: (user, token) => true,
    orElse: () => false,
  );
});

/// Auth loading provider
final isAuthLoadingProvider = Provider<bool>((ref) {
  final authState = ref.watch(authStateProvider);
  return authState.maybeWhen(
    loading: () => true,
    orElse: () => false,
  );
});