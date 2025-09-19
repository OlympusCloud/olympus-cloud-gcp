import 'package:dio/dio.dart';

class ErrorInterceptor extends Interceptor {
  ErrorInterceptor();

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    // Here you can handle different types of errors,
    // show snackbars, log to a service, etc.
    // For example, you could have a global error state provider.
    
    // For now, we'll just pass the error along.
    return handler.next(err);
  }
}
