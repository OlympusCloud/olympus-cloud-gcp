import 'package:flutter/material.dart';
import '../../../core/router/app_router.dart';

/// Error screen that displays when navigation errors occur
class ErrorScreen extends StatelessWidget {
  final String error;

  const ErrorScreen({
    super.key,
    required this.error,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Error'),
        automaticallyImplyLeading: false,
      ),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 64,
              color: theme.colorScheme.error,
            ),
            
            const SizedBox(height: 24),
            
            Text(
              'Oops! Something went wrong',
              style: theme.textTheme.headlineSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
              textAlign: TextAlign.center,
            ),
            
            const SizedBox(height: 16),
            
            Text(
              'We encountered an unexpected error. Please try again or contact support if the problem persists.',
              style: theme.textTheme.bodyLarge?.copyWith(
                color: theme.colorScheme.onBackground.withOpacity(0.7),
              ),
              textAlign: TextAlign.center,
            ),
            
            const SizedBox(height: 8),
            
            if (error.isNotEmpty) ...[
              ExpansionTile(
                title: const Text('Error Details'),
                children: [
                  Container(
                    width: double.infinity,
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: theme.colorScheme.errorContainer.withOpacity(0.3),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(
                      error,
                      style: theme.textTheme.bodySmall?.copyWith(
                        fontFamily: 'monospace',
                      ),
                    ),
                  ),
                ],
              ),
            ],
            
            const SizedBox(height: 32),
            
            Row(
              children: [
                Expanded(
                  child: OutlinedButton(
                    onPressed: () {
                      AppRouter.navigateToNamedAndClearStack(RouteNames.login);
                    },
                    child: const Text('Go to Login'),
                  ),
                ),
                
                const SizedBox(width: 16),
                
                Expanded(
                  child: ElevatedButton(
                    onPressed: () {
                      AppRouter.navigateToNamedAndClearStack(RouteNames.dashboard);
                    },
                    child: const Text('Go to Dashboard'),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 16),
            
            TextButton(
              onPressed: () {
                AppRouter.navigateToNamed(RouteNames.help);
              },
              child: const Text('Contact Support'),
            ),
          ],
        ),
      ),
    );
  }
}