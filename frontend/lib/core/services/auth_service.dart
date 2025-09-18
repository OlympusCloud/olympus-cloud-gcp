import 'package:dio/dio.dart';
import '../constants/app_constants.dart';
import '../models/user.dart';
import 'api_service.dart';
import 'storage_service.dart';

/// Authentication service for handling login/logout/registration with backend
class AuthService {
  /// Login with email and password
  static Future<AuthResult> login({
    required String email,
    required String password,
    bool rememberMe = false,
  }) async {
    try {
      final response = await ApiService.post('/api/v1/auth/login', data: {
        'email': email,
        'password': password,
        'remember_me': rememberMe,
      });

      if (response.statusCode == 200 && response.data != null) {
        final data = response.data as Map<String, dynamic>;
        
        // Extract tokens
        final accessToken = data['access_token'] as String?;
        final refreshToken = data['refresh_token'] as String?;
        final user = User.fromJson(data['user'] as Map<String, dynamic>);

        if (accessToken == null) {
          throw const AuthException('Invalid response: missing access token');
        }

        // Store tokens securely
        await StorageService.saveUserData(AppConstants.accessTokenKey, accessToken);
        if (refreshToken != null) {
          await StorageService.saveUserData(AppConstants.refreshTokenKey, refreshToken);
        }
        
        // Store user data
        await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());

        return AuthResult.success(
          user: user,
          accessToken: accessToken,
          refreshToken: refreshToken,
        );
      }

      throw const AuthException('Login failed: Invalid response');
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Login failed: ${e.toString()}');
    }
  }

  /// Register a new user account
  static Future<AuthResult> register({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    String? businessName,
    String? businessType,
  }) async {
    try {
      final response = await ApiService.post('/api/v1/auth/register', data: {
        'email': email,
        'password': password,
        'first_name': firstName,
        'last_name': lastName,
        if (businessName != null) 'business_name': businessName,
        if (businessType != null) 'business_type': businessType,
      });

      if (response.statusCode == 201 && response.data != null) {
        final data = response.data as Map<String, dynamic>;
        
        // Check if email verification is required
        if (data['requires_verification'] == true) {
          return AuthResult.emailVerificationRequired(
            email: email,
            message: data['message'] as String? ?? 'Please check your email to verify your account',
          );
        }

        // Auto-login after successful registration
        final accessToken = data['access_token'] as String?;
        final refreshToken = data['refresh_token'] as String?;
        final user = User.fromJson(data['user'] as Map<String, dynamic>);

        if (accessToken != null) {
          await StorageService.saveUserData(AppConstants.accessTokenKey, accessToken);
          if (refreshToken != null) {
            await StorageService.saveUserData(AppConstants.refreshTokenKey, refreshToken);
          }
          await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());

          return AuthResult.success(
            user: user,
            accessToken: accessToken,
            refreshToken: refreshToken,
          );
        }

        return AuthResult.emailVerificationRequired(
          email: email,
          message: 'Registration successful! Please verify your email to continue.',
        );
      }

      throw const AuthException('Registration failed: Invalid response');
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Registration failed: ${e.toString()}');
    }
  }

  /// Logout user and clear stored tokens
  static Future<void> logout() async {
    try {
      final accessToken = await StorageService.getUserData(AppConstants.accessTokenKey);
      
      if (accessToken != null) {
        // Notify backend about logout (optional, backend can handle token invalidation)
        try {
          await ApiService.post('/api/v1/auth/logout');
        } catch (e) {
          // Continue with local logout even if backend call fails
        }
      }
      
      // Clear all stored authentication data
      await StorageService.removeUserData(AppConstants.accessTokenKey);
      await StorageService.removeUserData(AppConstants.refreshTokenKey);
      await StorageService.removeUserData(AppConstants.userDataKey);
      await StorageService.removeUserData(AppConstants.tenantDataKey);
      
    } catch (e) {
      throw AuthException('Logout failed: ${e.toString()}');
    }
  }

  /// Refresh access token using stored refresh token
  static Future<String?> refreshToken() async {
    try {
      final refreshToken = await StorageService.getUserData(AppConstants.refreshTokenKey);
      
      if (refreshToken == null) {
        throw const AuthException('No refresh token available');
      }

      final response = await ApiService.post('/api/v1/auth/refresh', data: {
        'refresh_token': refreshToken,
      });

      if (response.statusCode == 200 && response.data != null) {
        final data = response.data as Map<String, dynamic>;
        final newAccessToken = data['access_token'] as String?;
        final newRefreshToken = data['refresh_token'] as String?;

        if (newAccessToken != null) {
          await StorageService.saveUserData(AppConstants.accessTokenKey, newAccessToken);
          
          if (newRefreshToken != null) {
            await StorageService.saveUserData(AppConstants.refreshTokenKey, newRefreshToken);
          }
          
          return newAccessToken;
        }
      }

      throw const AuthException('Token refresh failed');
    } on ApiException catch (e) {
      // If refresh fails, clear tokens
      await logout();
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      await logout();
      throw AuthException('Token refresh failed: ${e.toString()}');
    }
  }

  /// Get current user from stored data
  static Future<User?> getCurrentUser() async {
    try {
      final userData = await StorageService.getUserData(AppConstants.userDataKey);
      if (userData != null) {
        return User.fromJson(userData as Map<String, dynamic>);
      }
      return null;
    } catch (e) {
      return null;
    }
  }

  /// Check if user is authenticated (has valid access token)
  static Future<bool> isAuthenticated() async {
    try {
      final accessToken = await StorageService.getUserData(AppConstants.accessTokenKey);
      return accessToken != null && accessToken.toString().isNotEmpty;
    } catch (e) {
      return false;
    }
  }

  /// Verify email with verification code
  static Future<AuthResult> verifyEmail({
    required String email,
    required String verificationCode,
  }) async {
    try {
      final response = await ApiService.post('/api/v1/auth/verify-email', data: {
        'email': email,
        'verification_code': verificationCode,
      });

      if (response.statusCode == 200 && response.data != null) {
        final data = response.data as Map<String, dynamic>;
        
        // Auto-login after successful verification
        final accessToken = data['access_token'] as String?;
        final refreshToken = data['refresh_token'] as String?;
        final user = User.fromJson(data['user'] as Map<String, dynamic>);

        if (accessToken != null) {
          await StorageService.saveUserData(AppConstants.accessTokenKey, accessToken);
          if (refreshToken != null) {
            await StorageService.saveUserData(AppConstants.refreshTokenKey, refreshToken);
          }
          await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());

          return AuthResult.success(
            user: user,
            accessToken: accessToken,
            refreshToken: refreshToken,
          );
        }
      }

      throw const AuthException('Email verification failed');
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Email verification failed: ${e.toString()}');
    }
  }

  /// Request password reset
  static Future<void> requestPasswordReset(String email) async {
    try {
      await ApiService.post('/api/v1/auth/forgot-password', data: {
        'email': email,
      });
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Password reset request failed: ${e.toString()}');
    }
  }

  /// Reset password with reset token
  static Future<void> resetPassword({
    required String email,
    required String resetToken,
    required String newPassword,
  }) async {
    try {
      await ApiService.post('/api/v1/auth/reset-password', data: {
        'email': email,
        'reset_token': resetToken,
        'new_password': newPassword,
      });
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Password reset failed: ${e.toString()}');
    }
  }

  /// Update user profile
  static Future<User> updateProfile({
    required String firstName,
    required String lastName,
    String? phone,
    Map<String, dynamic>? preferences,
  }) async {
    try {
      final response = await ApiService.put('/api/v1/auth/profile', data: {
        'first_name': firstName,
        'last_name': lastName,
        if (phone != null) 'phone': phone,
        if (preferences != null) 'preferences': preferences,
      });

      if (response.statusCode == 200 && response.data != null) {
        final user = User.fromJson(response.data as Map<String, dynamic>);
        
        // Update stored user data
        await StorageService.saveUserData(AppConstants.userDataKey, user.toJson());
        
        return user;
      }

      throw const AuthException('Profile update failed');
    } on ApiException catch (e) {
      throw AuthException(_mapApiErrorToAuthError(e));
    } catch (e) {
      throw AuthException('Profile update failed: ${e.toString()}');
    }
  }

  /// Map API exceptions to auth-specific error messages
  static String _mapApiErrorToAuthError(ApiException apiError) {
    switch (apiError.type) {
      case ApiExceptionType.unauthorized:
        return 'Invalid email or password';
      case ApiExceptionType.badRequest:
        return apiError.message.isNotEmpty ? apiError.message : 'Invalid request data';
      case ApiExceptionType.forbidden:
        return 'Access denied. Please verify your account.';
      case ApiExceptionType.noInternet:
        return 'No internet connection. Please check your network.';
      case ApiExceptionType.timeout:
        return 'Request timed out. Please try again.';
      case ApiExceptionType.serverError:
        return 'Server error occurred. Please try again later.';
      default:
        return apiError.message.isNotEmpty ? apiError.message : 'Authentication failed';
    }
  }
}

/// Authentication result wrapper
class AuthResult {
  final bool isSuccess;
  final User? user;
  final String? accessToken;
  final String? refreshToken;
  final bool requiresEmailVerification;
  final String? email;
  final String? message;

  const AuthResult._({
    required this.isSuccess,
    this.user,
    this.accessToken,
    this.refreshToken,
    this.requiresEmailVerification = false,
    this.email,
    this.message,
  });

  factory AuthResult.success({
    required User user,
    required String accessToken,
    String? refreshToken,
  }) {
    return AuthResult._(
      isSuccess: true,
      user: user,
      accessToken: accessToken,
      refreshToken: refreshToken,
    );
  }

  factory AuthResult.emailVerificationRequired({
    required String email,
    String? message,
  }) {
    return AuthResult._(
      isSuccess: false,
      requiresEmailVerification: true,
      email: email,
      message: message,
    );
  }
}

/// Authentication exception
class AuthException implements Exception {
  final String message;

  const AuthException(this.message);

  @override
  String toString() => 'AuthException: $message';
}