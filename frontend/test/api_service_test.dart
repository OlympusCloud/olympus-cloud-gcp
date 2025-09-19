import 'package:flutter_test/flutter_test.dart';
import 'package:dio/dio.dart';
import 'package:frontend/core/services/api_service.dart';
import 'package:frontend/core/constants/app_constants.dart';

void main() {
  group('API Service Tests', () {
    setUp(() {
      // Initialize the API service
      ApiService.initialize();
    });

    test('ApiService initializes correctly', () {
      expect(ApiService.baseUrl, equals(AppConstants.apiUrl));
      expect(ApiService.dio, isA<Dio>());
    });

    test('ApiService has correct base URL', () {
      expect(ApiService.baseUrl, equals('http://localhost:8080'));
    });

    test('Dio instance has correct base options', () {
      final dio = ApiService.dio;
      expect(dio.options.baseUrl, equals('http://localhost:8080'));
      expect(dio.options.connectTimeout, equals(AppConstants.networkTimeout));
      expect(dio.options.receiveTimeout, equals(AppConstants.networkTimeout));
      expect(dio.options.headers['Content-Type'], equals('application/json'));
      expect(dio.options.headers['Accept'], equals('application/json'));
    });

    test('Dio instance has interceptors', () {
      final dio = ApiService.dio;
      expect(dio.interceptors.length, greaterThan(0));
    });

    test('Constants are correctly defined', () {
      expect(AppConstants.authEndpoint, equals('/api/v1/auth'));
      expect(AppConstants.platformEndpoint, equals('/api/v1/platform'));
      expect(AppConstants.commerceEndpoint, equals('/api/v1/commerce'));
      expect(AppConstants.analyticsEndpoint, equals('/api/v1/analytics'));
      expect(AppConstants.networkTimeout, equals(const Duration(seconds: 30)));
    });
  });
}