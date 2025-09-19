import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../api_client.dart';
import '../../auth/auth_models.dart';

/// Authentication API service for Rust auth service
class AuthApiService {
  final ApiClient _client;
  
  AuthApiService(this._client);
  
  /// Register new user
  Future<AuthResponse> register({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    String? tenantId,
  }) async {
    final response = await _client.post(
      '${ApiClient.rustAuthService}/register',
      data: {
        'email': email,
        'password': password,
        'first_name': firstName,
        'last_name': lastName,
        if (tenantId != null) 'tenant_id': tenantId,
      },
    );
    
    return AuthResponse.fromJson(response.data);
  }
  
  /// Login user
  Future<AuthResponse> login({
    required String email,
    required String password,
  }) async {
    final response = await _client.post(
      '${ApiClient.rustAuthService}/login',
      data: {
        'email': email,
        'password': password,
      },
    );
    
    return AuthResponse.fromJson(response.data);
  }
  
  /// Refresh access token
  Future<AuthResponse> refresh(String refreshToken) async {
    final response = await _client.post(
      '${ApiClient.rustAuthService}/refresh',
      data: {
        'refresh_token': refreshToken,
      },
    );
    
    return AuthResponse.fromJson(response.data);
  }
  
  /// Logout user
  Future<void> logout() async {
    await _client.post('${ApiClient.rustAuthService}/logout');
  }
  
  /// Get current user info
  Future<User> getCurrentUser() async {
    final response = await _client.get('${ApiClient.rustAuthService}/me');
    return User.fromJson(response.data);
  }
  
  /// Change password
  Future<void> changePassword({
    required String currentPassword,
    required String newPassword,
  }) async {
    await _client.post(
      '${ApiClient.rustAuthService}/change-password',
      data: {
        'current_password': currentPassword,
        'new_password': newPassword,
      },
    );
  }
  
  /// Forgot password
  Future<void> forgotPassword(String email) async {
    await _client.post(
      '${ApiClient.rustAuthService}/forgot-password',
      data: {
        'email': email,
      },
    );
  }
  
  /// Reset password
  Future<void> resetPassword({
    required String token,
    required String newPassword,
  }) async {
    await _client.post(
      '${ApiClient.rustAuthService}/reset-password',
      data: {
        'token': token,
        'new_password': newPassword,
      },
    );
  }
  
  /// Verify email
  Future<void> verifyEmail(String token) async {
    await _client.post(
      '${ApiClient.rustAuthService}/verify-email',
      data: {
        'token': token,
      },
    );
  }
}

/// Provider for AuthApiService
final authApiServiceProvider = Provider<AuthApiService>((ref) {
  final client = ref.watch(apiClientProvider);
  return AuthApiService(client);
});