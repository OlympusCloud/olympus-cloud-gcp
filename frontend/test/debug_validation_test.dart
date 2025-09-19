import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:frontend/features/auth/presentation/screens/signup_screen.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  setUpAll(() async {
    TestWidgetsFlutterBinding.ensureInitialized();
  });

  testWidgets('Debug form validation behavior', (WidgetTester tester) async {
    // Set window size for test
    tester.view.physicalSize = const Size(400, 800);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp(
          home: const SignupScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    print('Searching for Create Account buttons...');
    final createAccountFinder = find.text('Create Account');
    print('Found ${createAccountFinder.evaluate().length} Create Account widgets');
    
    for (var element in createAccountFinder.evaluate()) {
      print('Create Account widget: ${element.widget.runtimeType}');
    }

    print('Looking for ElevatedButton or button widgets...');
    final buttonFinder = find.byType(ElevatedButton);
    print('Found ${buttonFinder.evaluate().length} ElevatedButton widgets');
    
    // Try tapping the last Create Account text
    await tester.tap(createAccountFinder.last);
    await tester.pumpAndSettle();
    
    print('After tapping, looking for validation messages...');
    final fullNameErrorFinder = find.text('Please enter your full name');
    print('Found ${fullNameErrorFinder.evaluate().length} "Please enter your full name" messages');
    
    final emailErrorFinder = find.text('Please enter your email');
    print('Found ${emailErrorFinder.evaluate().length} "Please enter your email" messages');
    
    // Let's also check if any error text exists at all
    final anyErrorFinder = find.textContaining('Please enter');
    print('Found ${anyErrorFinder.evaluate().length} messages containing "Please enter"');
    
    for (var element in anyErrorFinder.evaluate()) {
      final widget = element.widget as Text;
      print('Error message found: "${widget.data}"');
    }
  });
}