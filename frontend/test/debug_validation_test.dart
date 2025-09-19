import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:frontend/features/auth/presentation/screens/signup_screen.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  testWidgets('Debug form validation behavior', (WidgetTester tester) async {
    // Set larger window size for test to accommodate the full form
    tester.view.physicalSize = const Size(400, 1200);
    tester.view.devicePixelRatio = 1.0;

    final container = ProviderContainer();
    addTearDown(container.dispose);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: MaterialApp(
          home: const SignupScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    print('Looking for Create Account buttons...');
    final createAccountFinder = find.text('Create Account');
    print('Found ${createAccountFinder.evaluate().length} Create Account widgets');
    
    print('Trying to tap Create Account .last...');
    // Try to submit without filling fields - use same approach as original test
    await tester.tap(find.text('Create Account').last); // Get the button, not title
    await tester.pumpAndSettle();
    
    print('After tapping, looking for validation messages...');
    
    // Check exact messages the test is looking for
    final fullNameErrorFinder = find.text('Please enter your full name');
    print('Found ${fullNameErrorFinder.evaluate().length} "Please enter your full name" messages');
    
    final emailErrorFinder = find.text('Please enter your email');
    print('Found ${emailErrorFinder.evaluate().length} "Please enter your email" messages');
    
    final passwordErrorFinder = find.text('Please enter a password');
    print('Found ${passwordErrorFinder.evaluate().length} "Please enter a password" messages');
    
    final confirmPasswordErrorFinder = find.text('Please confirm your password');
    print('Found ${confirmPasswordErrorFinder.evaluate().length} "Please confirm your password" messages');
    
    // Check for any error text
    final anyErrorFinder = find.textContaining('Please enter');
    print('Found ${anyErrorFinder.evaluate().length} messages containing "Please enter"');
    
    for (var element in anyErrorFinder.evaluate()) {
      final widget = element.widget as Text;
      print('Error message found: "${widget.data}"');
    }
  });
}