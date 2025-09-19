// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'auth_models.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$AuthResponseImpl _$$AuthResponseImplFromJson(Map<String, dynamic> json) =>
    _$AuthResponseImpl(
      accessToken: json['access_token'] as String,
      refreshToken: json['refresh_token'] as String,
      tokenType: json['token_type'] as String,
      expiresIn: (json['expires_in'] as num).toInt(),
      user: User.fromJson(json['user'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$AuthResponseImplToJson(_$AuthResponseImpl instance) =>
    <String, dynamic>{
      'access_token': instance.accessToken,
      'refresh_token': instance.refreshToken,
      'token_type': instance.tokenType,
      'expires_in': instance.expiresIn,
      'user': instance.user,
    };

_$UserImpl _$$UserImplFromJson(Map<String, dynamic> json) => _$UserImpl(
      id: json['id'] as String,
      email: json['email'] as String,
      firstName: json['first_name'] as String,
      lastName: json['last_name'] as String,
      tenantId: json['tenant_id'] as String?,
      isEmailVerified: json['is_email_verified'] as bool? ?? false,
      isActive: json['is_active'] as bool? ?? true,
      roles: (json['roles'] as List<dynamic>?)
              ?.map((e) => UserRole.fromJson(e as Map<String, dynamic>))
              .toList() ??
          const [],
      permissions: (json['permissions'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
      createdAt: json['created_at'] == null
          ? null
          : DateTime.parse(json['created_at'] as String),
      updatedAt: json['updated_at'] == null
          ? null
          : DateTime.parse(json['updated_at'] as String),
      lastLoginAt: json['last_login_at'] == null
          ? null
          : DateTime.parse(json['last_login_at'] as String),
      avatar: json['avatar'] as String?,
      phone: json['phone'] as String?,
      preferences: json['preferences'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$$UserImplToJson(_$UserImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'email': instance.email,
      'first_name': instance.firstName,
      'last_name': instance.lastName,
      'tenant_id': instance.tenantId,
      'is_email_verified': instance.isEmailVerified,
      'is_active': instance.isActive,
      'roles': instance.roles,
      'permissions': instance.permissions,
      'created_at': instance.createdAt?.toIso8601String(),
      'updated_at': instance.updatedAt?.toIso8601String(),
      'last_login_at': instance.lastLoginAt?.toIso8601String(),
      'avatar': instance.avatar,
      'phone': instance.phone,
      'preferences': instance.preferences,
    };

_$UserRoleImpl _$$UserRoleImplFromJson(Map<String, dynamic> json) =>
    _$UserRoleImpl(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String,
      permissions: (json['permissions'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          const [],
      assignedAt: json['assigned_at'] == null
          ? null
          : DateTime.parse(json['assigned_at'] as String),
    );

Map<String, dynamic> _$$UserRoleImplToJson(_$UserRoleImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'description': instance.description,
      'permissions': instance.permissions,
      'assigned_at': instance.assignedAt?.toIso8601String(),
    };

_$LoginRequestImpl _$$LoginRequestImplFromJson(Map<String, dynamic> json) =>
    _$LoginRequestImpl(
      email: json['email'] as String,
      password: json['password'] as String,
    );

Map<String, dynamic> _$$LoginRequestImplToJson(_$LoginRequestImpl instance) =>
    <String, dynamic>{
      'email': instance.email,
      'password': instance.password,
    };

_$RegisterRequestImpl _$$RegisterRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$RegisterRequestImpl(
      email: json['email'] as String,
      password: json['password'] as String,
      firstName: json['first_name'] as String,
      lastName: json['last_name'] as String,
      tenantId: json['tenant_id'] as String?,
      phone: json['phone'] as String?,
    );

Map<String, dynamic> _$$RegisterRequestImplToJson(
        _$RegisterRequestImpl instance) =>
    <String, dynamic>{
      'email': instance.email,
      'password': instance.password,
      'first_name': instance.firstName,
      'last_name': instance.lastName,
      'tenant_id': instance.tenantId,
      'phone': instance.phone,
    };

_$ChangePasswordRequestImpl _$$ChangePasswordRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$ChangePasswordRequestImpl(
      currentPassword: json['current_password'] as String,
      newPassword: json['new_password'] as String,
    );

Map<String, dynamic> _$$ChangePasswordRequestImplToJson(
        _$ChangePasswordRequestImpl instance) =>
    <String, dynamic>{
      'current_password': instance.currentPassword,
      'new_password': instance.newPassword,
    };

_$ForgotPasswordRequestImpl _$$ForgotPasswordRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$ForgotPasswordRequestImpl(
      email: json['email'] as String,
    );

Map<String, dynamic> _$$ForgotPasswordRequestImplToJson(
        _$ForgotPasswordRequestImpl instance) =>
    <String, dynamic>{
      'email': instance.email,
    };

_$ResetPasswordRequestImpl _$$ResetPasswordRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$ResetPasswordRequestImpl(
      token: json['token'] as String,
      newPassword: json['new_password'] as String,
    );

Map<String, dynamic> _$$ResetPasswordRequestImplToJson(
        _$ResetPasswordRequestImpl instance) =>
    <String, dynamic>{
      'token': instance.token,
      'new_password': instance.newPassword,
    };

_$VerifyEmailRequestImpl _$$VerifyEmailRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$VerifyEmailRequestImpl(
      token: json['token'] as String,
    );

Map<String, dynamic> _$$VerifyEmailRequestImplToJson(
        _$VerifyEmailRequestImpl instance) =>
    <String, dynamic>{
      'token': instance.token,
    };

_$RefreshTokenRequestImpl _$$RefreshTokenRequestImplFromJson(
        Map<String, dynamic> json) =>
    _$RefreshTokenRequestImpl(
      refreshToken: json['refresh_token'] as String,
    );

Map<String, dynamic> _$$RefreshTokenRequestImplToJson(
        _$RefreshTokenRequestImpl instance) =>
    <String, dynamic>{
      'refresh_token': instance.refreshToken,
    };
