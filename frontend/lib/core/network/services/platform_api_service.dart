import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../api_client.dart';

/// Platform API service for tenant, location, and role management
class PlatformApiService {
  final ApiClient _client;
  
  PlatformApiService(this._client);
  
  /// Get tenant details
  Future<Tenant> getTenant(String tenantId) async {
    final response = await _client.get('${ApiClient.platformService}/tenants/$tenantId');
    return Tenant.fromJson(response.data);
  }
  
  /// Create new tenant
  Future<Tenant> createTenant({
    required String name,
    required String industry,
    String? logo,
    Map<String, dynamic>? settings,
  }) async {
    final response = await _client.post(
      '${ApiClient.platformService}/tenants',
      data: {
        'name': name,
        'industry': industry,
        if (logo != null) 'logo': logo,
        if (settings != null) 'settings': settings,
      },
    );
    
    return Tenant.fromJson(response.data);
  }
  
  /// Get locations for tenant
  Future<List<Location>> getLocations({
    required String tenantId,
  }) async {
    final response = await _client.get(
      '${ApiClient.platformService}/locations',
      queryParameters: {'tenant_id': tenantId},
    );
    
    return (response.data as List)
        .map((item) => Location.fromJson(item))
        .toList();
  }
  
  /// Create new location
  Future<Location> createLocation({
    required String tenantId,
    required String name,
    required String address,
    String? phone,
    String? email,
    Map<String, dynamic>? settings,
  }) async {
    final response = await _client.post(
      '${ApiClient.platformService}/locations',
      data: {
        'tenant_id': tenantId,
        'name': name,
        'address': address,
        if (phone != null) 'phone': phone,
        if (email != null) 'email': email,
        if (settings != null) 'settings': settings,
      },
    );
    
    return Location.fromJson(response.data);
  }
  
  /// Get available roles
  Future<List<Role>> getRoles() async {
    final response = await _client.get('${ApiClient.platformService}/roles');
    
    return (response.data as List)
        .map((item) => Role.fromJson(item))
        .toList();
  }
  
  /// Create new role
  Future<Role> createRole({
    required String name,
    required String description,
    required List<String> permissions,
  }) async {
    final response = await _client.post(
      '${ApiClient.platformService}/roles',
      data: {
        'name': name,
        'description': description,
        'permissions': permissions,
      },
    );
    
    return Role.fromJson(response.data);
  }
  
  /// Assign role to user
  Future<void> assignRole({
    required String userId,
    required String roleId,
  }) async {
    await _client.post('${ApiClient.platformService}/users/$userId/roles/$roleId');
  }
  
  /// Remove role from user
  Future<void> removeRole({
    required String userId,
    required String roleId,
  }) async {
    await _client.delete('${ApiClient.platformService}/users/$userId/roles/$roleId');
  }
}

/// Tenant data model
class Tenant {
  final String id;
  final String name;
  final String industry;
  final String? logo;
  final Map<String, dynamic> settings;
  final DateTime createdAt;
  final DateTime updatedAt;
  
  Tenant({
    required this.id,
    required this.name,
    required this.industry,
    this.logo,
    required this.settings,
    required this.createdAt,
    required this.updatedAt,
  });
  
  factory Tenant.fromJson(Map<String, dynamic> json) {
    return Tenant(
      id: json['id'],
      name: json['name'],
      industry: json['industry'],
      logo: json['logo'],
      settings: Map<String, dynamic>.from(json['settings'] ?? {}),
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }
}

/// Location data model
class Location {
  final String id;
  final String tenantId;
  final String name;
  final String address;
  final String? phone;
  final String? email;
  final Map<String, dynamic> settings;
  final DateTime createdAt;
  final DateTime updatedAt;
  
  Location({
    required this.id,
    required this.tenantId,
    required this.name,
    required this.address,
    this.phone,
    this.email,
    required this.settings,
    required this.createdAt,
    required this.updatedAt,
  });
  
  factory Location.fromJson(Map<String, dynamic> json) {
    return Location(
      id: json['id'],
      tenantId: json['tenant_id'],
      name: json['name'],
      address: json['address'],
      phone: json['phone'],
      email: json['email'],
      settings: Map<String, dynamic>.from(json['settings'] ?? {}),
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }
}

/// Role data model
class Role {
  final String id;
  final String name;
  final String description;
  final List<String> permissions;
  final DateTime createdAt;
  final DateTime updatedAt;
  
  Role({
    required this.id,
    required this.name,
    required this.description,
    required this.permissions,
    required this.createdAt,
    required this.updatedAt,
  });
  
  factory Role.fromJson(Map<String, dynamic> json) {
    return Role(
      id: json['id'],
      name: json['name'],
      description: json['description'],
      permissions: List<String>.from(json['permissions'] ?? []),
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }
}

/// Provider for PlatformApiService
final platformApiServiceProvider = Provider<PlatformApiService>((ref) {
  final client = ref.watch(apiClientProvider);
  return PlatformApiService(client);
});