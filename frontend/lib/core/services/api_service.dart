import 'package:dio/dio.dart';
import 'package:pretty_dio_logger/pretty_dio_logger.dart';
import '../constants/app_constants.dart';
import 'storage_service.dart';

/// Service for making API calls to the Go backend
class ApiService {
  static late Dio _dio;
  static late String _baseUrl;

  /// Initialize the API service
  static void initialize() {
    _baseUrl = AppConstants.apiUrl;
    
    _dio = Dio(BaseOptions(
      baseUrl: _baseUrl,
      connectTimeout: AppConstants.networkTimeout,
      receiveTimeout: AppConstants.networkTimeout,
      sendTimeout: AppConstants.networkTimeout,
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
    ));

    // Add interceptors
    _dio.interceptors.add(_AuthInterceptor());
    
    if (AppConstants.isDebugMode) {
      _dio.interceptors.add(PrettyDioLogger(
        requestHeader: true,
        requestBody: true,
        responseBody: true,
        responseHeader: false,
        error: true,
        compact: true,
      ));
    }
  }

  static Dio get dio => _dio;
  static String get baseUrl => _baseUrl;

  // Generic GET request
  static Future<Response<T>> get<T>(
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

  // Generic POST request
  static Future<Response<T>> post<T>(
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

  // Generic PUT request
  static Future<Response<T>> put<T>(
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

  // Generic DELETE request
  static Future<Response<T>> delete<T>(
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

  // Handle Dio errors
  static ApiException _handleDioError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return ApiException(
          'Connection timeout. Please check your internet connection.',
          type: ApiExceptionType.timeout,
        );
      
      case DioExceptionType.badResponse:
        final statusCode = error.response?.statusCode;
        final message = error.response?.data?['message'] ?? 
                       error.response?.statusMessage ?? 
                       'Unknown error occurred';
        
        return ApiException(
          message,
          statusCode: statusCode,
          type: _getExceptionTypeFromStatusCode(statusCode),
        );
      
      case DioExceptionType.cancel:
        return ApiException(
          'Request was cancelled',
          type: ApiExceptionType.cancelled,
        );
      
      case DioExceptionType.connectionError:
        return ApiException(
          'No internet connection. Please check your network.',
          type: ApiExceptionType.noInternet,
        );
      
      default:
        return ApiException(
          'An unexpected error occurred: ${error.message}',
          type: ApiExceptionType.unknown,
        );
    }
  }

  static ApiExceptionType _getExceptionTypeFromStatusCode(int? statusCode) {
    switch (statusCode) {
      case 400:
        return ApiExceptionType.badRequest;
      case 401:
        return ApiExceptionType.unauthorized;
      case 403:
        return ApiExceptionType.forbidden;
      case 404:
        return ApiExceptionType.notFound;
      case 422:
        return ApiExceptionType.validationError;
      case 500:
        return ApiExceptionType.serverError;
      default:
        return ApiExceptionType.unknown;
    }
  }
}

/// Authentication interceptor to add bearer token to requests
class _AuthInterceptor extends Interceptor {
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    final token = StorageService.getUserData<String>(AppConstants.accessTokenKey);
    
    if (token != null && token.isNotEmpty) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    
    super.onRequest(options, handler);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    // Handle token refresh on 401 errors
    if (err.response?.statusCode == 401) {
      final refreshToken = StorageService.getUserData<String>(AppConstants.refreshTokenKey);
      
      if (refreshToken != null && refreshToken.isNotEmpty) {
        try {
          // Attempt to refresh token
          final response = await ApiService.post('/auth/refresh', data: {
            'refresh_token': refreshToken,
          });
          
          if (response.statusCode == 200) {
            final newToken = response.data['access_token'];
            await StorageService.saveUserData(AppConstants.accessTokenKey, newToken);
            
            // Retry the original request with new token
            final options = err.requestOptions;
            options.headers['Authorization'] = 'Bearer $newToken';
            
            final retryResponse = await ApiService.dio.fetch(options);
            handler.resolve(retryResponse);
            return;
          }
        } catch (e) {
          // Refresh failed, clear tokens and let error propagate
          await StorageService.removeUserData(AppConstants.accessTokenKey);
          await StorageService.removeUserData(AppConstants.refreshTokenKey);
        }
      }
    }
    
    super.onError(err, handler);
  }
}

/// Custom API exception class
class ApiException implements Exception {
  final String message;
  final int? statusCode;
  final ApiExceptionType type;

  ApiException(
    this.message, {
    this.statusCode,
    required this.type,
  });

  @override
  String toString() {
    return 'ApiException: $message (${statusCode ?? 'Unknown'})';
  }
}

/// Types of API exceptions
enum ApiExceptionType {
  timeout,
  noInternet,
  badRequest,
  unauthorized,
  forbidden,
  notFound,
  validationError,
  serverError,
  cancelled,
  unknown,
}