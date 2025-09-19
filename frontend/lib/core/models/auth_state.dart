import 'package:freezed_annotation/freezed_annotation.dart';
import 'user.dart';

part 'auth_state.freezed.dart';

/// Authentication state using Freezed for immutable state management
@freezed
class AuthState with _$AuthState {
  /// Initial state - checking authentication status
  const factory AuthState.initial() = InitialState;
  
  /// Loading state - authentication process in progress
  const factory AuthState.loading() = LoadingState;
  
  /// Authenticated state - user is logged in
  const factory AuthState.authenticated({
    required User user,
    required String token,
  }) = AuthenticatedState;
  
  /// Unauthenticated state - user is not logged in
  const factory AuthState.unauthenticated() = UnauthenticatedState;
  
  /// Email verification required state
  const factory AuthState.emailVerificationRequired({
    required String email,
    required String message,
  }) = EmailVerificationRequiredState;
  
  /// Error state - authentication failed
  const factory AuthState.error(String message) = ErrorState;
}

/// Extension methods for AuthState
extension AuthStateExtension on AuthState {
  /// Check if user is authenticated
  bool get isAuthenticated => this is AuthenticatedState;
  
  /// Check if authentication is loading
  bool get isLoading => this is LoadingState || this is InitialState;
  
  /// Check if there's an error
  bool get hasError => this is ErrorState;
  
  /// Check if email verification is required
  bool get requiresEmailVerification => this is EmailVerificationRequiredState;
  
  /// Get current user if authenticated
  User? get user => maybeWhen(
        authenticated: (user, token) => user,
        orElse: () => null,
      );
  
  /// Get current token if authenticated
  String? get token => maybeWhen(
        authenticated: (user, token) => token,
        orElse: () => null,
      );
  
  /// Get error message if in error state
  String? get errorMessage => maybeWhen(
        error: (message) => message,
        orElse: () => null,
      );
  
  /// Get email if verification is required
  String? get verificationEmail => maybeWhen(
        emailVerificationRequired: (email, message) => email,
        orElse: () => null,
      );
  
  /// Get verification message if verification is required
  String? get verificationMessage => maybeWhen(
        emailVerificationRequired: (email, message) => message,
        orElse: () => null,
      );
}