import 'dart:convert';
import 'dart:io';
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive/hive.dart';

import '../auth/auth_models.dart';
import '../storage/storage_service.dart';

/// HTTP client for communicating with Olympus Cloud backend services
/// Handles multi-service architecture: Rust auth, Python analytics, Go API gateway
class ApiClient {
  late final Dio _dio;
  final StorageService _storage;
  final String _baseUrl;
  
  // Service endpoints
  static const String rustAuthService = '/auth';
  static const String pythonAnalyticsService = '/analytics';
  static const String restaurantService = '/restaurant';
  static const String commerceService = '/commerce';
  static const String platformService = '/platform';
  
  ApiClient({
    required StorageService storage,
    String? baseUrl,
  }) : _storage = storage,
       _baseUrl = baseUrl ?? _getDefaultBaseUrl() {
    _initializeDio();
  }
  
  static String _getDefaultBaseUrl() {
    // Development endpoints - will be configurable via environment
    if (Platform.isAndroid) {
      return 'http://10.0.2.2:8080/api/v1'; // Android emulator
    } else {
      return 'http://localhost:8080/api/v1'; // iOS simulator/desktop
    }
  }
  
  void _initializeDio() {
    _dio = Dio(BaseOptions(
      baseUrl: _baseUrl,
      connectTimeout: const Duration(seconds: 30),
      receiveTimeout: const Duration(seconds: 30),
      sendTimeout: const Duration(seconds: 30),
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
    ));
    
    // Add auth interceptor
    _dio.interceptors.add(
      InterceptorsWrapper(
        onRequest: (options, handler) async {
          final token = await _storage.getAccessToken();
          if (token != null) {
            options.headers['Authorization'] = 'Bearer $token';
          }
          handler.next(options);
        },
        onError: (error, handler) async {
          if (error.response?.statusCode == 401) {
            // Try to refresh token
            final refreshed = await _refreshToken();
            if (refreshed) {
              // Retry the original request
              final options = error.requestOptions;
              final token = await _storage.getAccessToken();
              if (token != null) {
                options.headers['Authorization'] = 'Bearer $token';
              }
              try {
                final response = await _dio.fetch(options);
                handler.resolve(response);
                return;
              } catch (e) {
                // If retry fails, continue with original error
              }
            }
          }
          handler.next(error);
        },
      ),
    );
    
    // Add logging interceptor for development
    _dio.interceptors.add(
      LogInterceptor(
        requestBody: true,
        responseBody: true,
        logPrint: (obj) {
          // Use print for now, can be replaced with proper logging
          print('[API] $obj');
        },
      ),
    );
  }
  
  /// Refresh access token using refresh token
  Future<bool> _refreshToken() async {
    try {
      final refreshToken = await _storage.getRefreshToken();
      if (refreshToken == null) return false;
      
      final response = await _dio.post(
        '$rustAuthService/refresh',
        data: {'refresh_token': refreshToken},
        options: Options(
          headers: {'Authorization': null}, // Don't send bearer token for refresh
        ),
      );
      
      if (response.statusCode == 200) {
        final data = response.data;
        await _storage.saveTokens(
          accessToken: data['access_token'],
          refreshToken: data['refresh_token'],
        );
        return true;
      }
    } catch (e) {
      // Refresh failed, user needs to login again
      await _storage.clearAuth();
    }
    return false;
  }
  
  /// Generic GET request
  Future<Response<T>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    try {
      return await _dio.get<T>(
        path,
        queryParameters: queryParameters,
        options: options,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
  
  /// Generic POST request
  Future<Response<T>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    try {
      return await _dio.post<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
  
  /// Generic PUT request
  Future<Response<T>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    try {
      return await _dio.put<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
  
  /// Generic DELETE request
  Future<Response<T>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    try {
      return await _dio.delete<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
  
  /// Handle Dio errors and convert to app-specific exceptions
  ApiException _handleDioError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.receiveTimeout:
      case DioExceptionType.sendTimeout:
        return ApiException(
          message: 'Connection timeout. Please check your internet connection.',
          type: ApiExceptionType.timeout,
          statusCode: null,
        );
      
      case DioExceptionType.connectionError:
        return ApiException(
          message: 'Unable to connect to server. Please try again later.',
          type: ApiExceptionType.network,
          statusCode: null,
        );
      
      case DioExceptionType.badResponse:
        final statusCode = error.response?.statusCode;
        final message = _extractErrorMessage(error.response?.data);
        
        if (statusCode == 401) {
          return ApiException(
            message: 'Authentication failed. Please login again.',
            type: ApiExceptionType.unauthorized,
            statusCode: statusCode,
          );
        } else if (statusCode == 403) {
          return ApiException(
            message: 'Access denied. You don\'t have permission for this action.',
            type: ApiExceptionType.forbidden,
            statusCode: statusCode,
          );
        } else if (statusCode == 404) {
          return ApiException(
            message: 'Requested resource not found.',
            type: ApiExceptionType.notFound,
            statusCode: statusCode,
          );
        } else if (statusCode != null && statusCode >= 500) {
          return ApiException(
            message: 'Server error. Please try again later.',
            type: ApiExceptionType.server,
            statusCode: statusCode,
          );
        }
        
        return ApiException(
          message: message ?? 'An error occurred. Please try again.',
          type: ApiExceptionType.badRequest,
          statusCode: statusCode,
        );
      
      case DioExceptionType.cancel:
        return ApiException(
          message: 'Request was cancelled.',
          type: ApiExceptionType.cancelled,
          statusCode: null,
        );
      
      default:
        return ApiException(
          message: 'An unexpected error occurred.',
          type: ApiExceptionType.unknown,
          statusCode: null,
        );
    }
  }
  
  /// Extract error message from response data
  String? _extractErrorMessage(dynamic data) {
    if (data == null) return null;
    
    if (data is Map<String, dynamic>) {
      // Try common error message fields
      return data['error'] ?? 
             data['message'] ?? 
             data['detail'] ?? 
             data['error_description'];
    }
    
    if (data is String) {
      return data;
    }
    
    return null;
  }
  
  /// Upload file with progress callback
  Future<Response> uploadFile(
    String path,
    String filePath, {
    String fieldName = 'file',
    Map<String, dynamic>? data,
    ProgressCallback? onSendProgress,
  }) async {
    final formData = FormData.fromMap({
      if (data != null) ...data,
      fieldName: await MultipartFile.fromFile(filePath),
    });
    
    try {
      return await _dio.post(
        path,
        data: formData,
        onSendProgress: onSendProgress,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
  
  /// Download file with progress callback
  Future<Response> downloadFile(
    String urlPath,
    String savePath, {
    ProgressCallback? onReceiveProgress,
  }) async {
    try {
      return await _dio.download(
        urlPath,
        savePath,
        onReceiveProgress: onReceiveProgress,
      );
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }
}

/// Custom exception for API errors
class ApiException implements Exception {
  final String message;
  final ApiExceptionType type;
  final int? statusCode;
  
  const ApiException({
    required this.message,
    required this.type,
    this.statusCode,
  });
  
  @override
  String toString() => 'ApiException: $message (${type.name})';
}

/// Types of API exceptions
enum ApiExceptionType {
  network,
  timeout,
  unauthorized,
  forbidden,
  notFound,
  badRequest,
  server,
  cancelled,
  unknown,
}

/// Provider for ApiClient
final apiClientProvider = Provider<ApiClient>((ref) {
  final storage = ref.watch(storageServiceProvider);
  return ApiClient(storage: storage);
});