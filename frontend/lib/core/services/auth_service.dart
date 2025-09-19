import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../features/auth/data/models/user.dart';
import '../constants/app_constants.dart';
import 'api_service.dart';
import 'storage_service.dart';

/// Authentication service provider
final authServiceProvider = Provider<AuthService>((ref) {
  return AuthService();
});

/// Authentication service for handling login/logout/registration with backend
class AuthService {
  /// Login with email and password
  Future<User> login(String email, String password) async {
    try {
      // Simulate API call for now - replace with actual backend integration
      await Future.delayed(const Duration(seconds: 2));
      
      // Mock successful login
      final user = User(
        id: 'user-123',
        email: email,
        name: 'Test User',
        role: 'user',
        permissions: ['read', 'write'],
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        isEmailVerified: true,
      );

      // Store user data
      await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
      await StorageService.saveUserData(AppConstants.accessTokenKey, 'mock-token-123');

      return user;
    } catch (e) {
      throw Exception('Login failed: ${e.toString()}');
    }
  }

  /// Register a new user account
  Future<User> register({
    required String email,
    required String password,
    required String name,
  }) async {
    try {
      // Simulate API call for now - replace with actual backend integration
      await Future.delayed(const Duration(seconds: 2));
      
      // Mock successful registration
      final user = User(
        id: 'user-${DateTime.now().millisecondsSinceEpoch}',
        email: email,
        name: name,
        role: 'user',
        permissions: ['read', 'write'],
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        isEmailVerified: false,
      );

      // Store user data
      await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
      await StorageService.saveUserData(AppConstants.accessTokenKey, 'mock-token-${user.id}');

      return user;
    } catch (e) {
      throw Exception('Registration failed: ${e.toString()}');
    }
  }

  /// Login with Google
  Future<User> loginWithGoogle() async {
    try {
      // Simulate Google OAuth flow
      await Future.delayed(const Duration(seconds: 2));
      
      final user = User(
        id: 'google-user-123',
        email: 'user@gmail.com',
        name: 'Google User',
        role: 'user',
        permissions: ['read', 'write'],
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        isEmailVerified: true,
      );

      await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
      await StorageService.saveUserData(AppConstants.accessTokenKey, 'google-token-123');

      return user;
    } catch (e) {
      throw Exception('Google login failed: ${e.toString()}');
    }
  }

  /// Login with Apple
  Future<User> loginWithApple() async {
    try {
      // Simulate Apple Sign In flow
      await Future.delayed(const Duration(seconds: 2));
      
      final user = User(
        id: 'apple-user-123',
        email: 'user@privaterelay.appleid.com',
        name: 'Apple User',
        role: 'user',
        permissions: ['read', 'write'],
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        isEmailVerified: true,
      );

      await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
      await StorageService.saveUserData(AppConstants.accessTokenKey, 'apple-token-123');

      return user;
    } catch (e) {
      throw Exception('Apple login failed: ${e.toString()}');
    }
  }

  /// Send password reset email
  Future<void> resetPassword(String email) async {
    try {
      // Simulate password reset API call
      await Future.delayed(const Duration(seconds: 1));
      // In real implementation, this would call the backend
    } catch (e) {
      throw Exception('Password reset failed: ${e.toString()}');
    }
  }

  /// Logout user and clear stored tokens
  Future<void> logout() async {
    try {
      // Clear all stored authentication data
      await StorageService.removeUserData(AppConstants.accessTokenKey);
      await StorageService.removeUserData(AppConstants.refreshTokenKey);
      await StorageService.removeUserData(AppConstants.userDataKey);
      await StorageService.removeUserData(AppConstants.tenantDataKey);
    } catch (e) {
      throw Exception('Logout failed: ${e.toString()}');
    }
  }

  /// Get current user from stored data
  Future<User?> getCurrentUser() async {
    try {
      final userData = await StorageService.getUserData(AppConstants.userDataKey);
      if (userData != null && userData is Map<String, dynamic>) {
        return User.fromJson(userData);
      }
      return null;
    } catch (e) {
      return null;
    }
  }

  /// Check if user is authenticated (has valid access token)
  Future<bool> isAuthenticated() async {
    try {
      final accessToken = await StorageService.getUserData(AppConstants.accessTokenKey);
      return accessToken != null && accessToken.toString().isNotEmpty;
    } catch (e) {
      return false;
    }
  }

  /// Refresh access token
  Future<String?> refreshToken() async {
    try {
      final refreshToken = await StorageService.getUserData(AppConstants.refreshTokenKey);
      
      if (refreshToken == null) {
        throw Exception('No refresh token available');
      }

      // Simulate token refresh
      await Future.delayed(const Duration(seconds: 1));
      
      const newAccessToken = 'refreshed-token-123';
      await StorageService.saveUserData(AppConstants.accessTokenKey, newAccessToken);
      
      return newAccessToken;
    } catch (e) {
      await logout();
      throw Exception('Token refresh failed: ${e.toString()}');
    }
  }

  /// Update user profile
  Future<User> updateProfile({
    required String name,
    String? phone,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      final currentUser = await getCurrentUser();
      if (currentUser == null) {
        throw Exception('No authenticated user');
      }

      // Simulate profile update
      await Future.delayed(const Duration(seconds: 1));

      final updatedUser = currentUser.copyWith(
        name: name,
        phone: phone,
        metadata: metadata,
        updatedAt: DateTime.now(),
      );

      await StorageService.saveUserData(AppConstants.userDataKey, updatedUser.toJson());
      
      return updatedUser;
    } catch (e) {
      throw Exception('Profile update failed: ${e.toString()}');
    }
  }
}