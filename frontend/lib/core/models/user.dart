import 'package:freezed_annotation/freezed_annotation.dart';

part 'user.freezed.dart';
part 'user.g.dart';

/// User model representing the authenticated user
@freezed
class User with _$User {
  const factory User({
    required String id,
    required String email,
    required String firstName,
    required String lastName,
    String? profilePicture,
    required String tenantId,
    required List<String> roles,
    @Default(true) bool isActive,
    DateTime? emailVerifiedAt,
    DateTime? lastLoginAt,
    required DateTime createdAt,
    required DateTime updatedAt,
  }) = _User;

  factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);
}

/// User extension methods
extension UserExtension on User {
  /// Get full name
  String get fullName => '$firstName $lastName';

  /// Get initials for avatar
  String get initials {
    final first = firstName.isNotEmpty ? firstName[0].toUpperCase() : '';
    final last = lastName.isNotEmpty ? lastName[0].toUpperCase() : '';
    return '$first$last';
  }

  /// Check if email is verified
  bool get isEmailVerified => emailVerifiedAt != null;

  /// Check if user has specific role
  bool hasRole(String role) => roles.contains(role);

  /// Check if user has any of the specified roles
  bool hasAnyRole(List<String> roleList) {
    return roleList.any((role) => roles.contains(role));
  }

  /// Check if user is admin
  bool get isAdmin => hasRole('admin');

  /// Check if user is owner
  bool get isOwner => hasRole('owner');

  /// Check if user is manager
  bool get isManager => hasRole('manager');

  /// Check if user is employee
  bool get isEmployee => hasRole('employee');
}