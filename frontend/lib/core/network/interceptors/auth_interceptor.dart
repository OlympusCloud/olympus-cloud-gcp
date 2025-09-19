import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:frontend/core/auth/auth_controller.dart';
import 'package:frontend/core/storage/storage_service.dart';

class AuthInterceptor extends Interceptor {
  final AuthController authController;
  final Ref _ref;

  AuthInterceptor(this.authController, this._ref);

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) async {
    final accessToken = await _ref.read(storageServiceProvider).getAccessToken();
    if (accessToken != null) {
      options.headers['Authorization'] = 'Bearer $accessToken';
    }
    return handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.response?.statusCode == 401) {
      // If a 401 response is received, refresh the token
      try {
        final storage = _ref.read(storageServiceProvider);
        final oldRefreshToken = await storage.getRefreshToken();

        if (oldRefreshToken == null) {
          authController.logout();
          return handler.reject(err);
        }
        
        // This is a simplified refresh token logic.
        // In a real app, you would call your refresh token endpoint.
        // final newAuthResponse = await _ref.read(authApiServiceProvider).refreshToken(oldRefreshToken);
        // await storage.saveTokens(accessToken: newAuthResponse.accessToken, refreshToken: newAuthResponse.refreshToken);

        // For this example, let's assume we can't refresh and just log out.
        authController.logout();
        return handler.reject(err);

      } catch (e) {
        authController.logout();
        return handler.reject(err);
      }
    }
    return handler.next(err);
  }
}
