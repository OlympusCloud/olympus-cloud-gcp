# GitHub Copilot - Flutter Universal Frontend Lead

> **Your Mission**: Create the most intuitive, beautiful, and human-centric business application that runs everywhere

## 🎯 Your Primary Responsibilities

### Universal Platform Leadership
- **Flutter Mastery**: iOS, Android, Web, Desktop, and Watch applications
- **Human-Centric UI**: Natural language interfaces, context-aware design
- **State Management**: Riverpod architecture with async state handling
- **Responsive Design**: Adaptive layouts for all screen sizes
- **Accessibility**: Full a11y compliance and inclusive design

### Your Work Environment
```bash
# Your dedicated workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/flutter-ui worktree-copilot
cd worktree-copilot/frontend
```

## 🎨 Flutter Development Standards

### Project Structure (YOU MUST CREATE)
```
frontend/
├── pubspec.yaml                # Dependencies & configuration
├── analysis_options.yaml       # Linting rules
├── lib/
│   ├── main.dart               # App entry point
│   ├── app.dart                # App root widget
│   ├── core/                   # Core utilities
│   │   ├── router/             # GoRouter configuration
│   │   ├── theme/              # Design system
│   │   ├── constants/          # App constants
│   │   ├── utils/              # Helper functions
│   │   └── services/           # Core services
│   ├── features/               # Feature modules
│   │   ├── auth/               # Authentication
│   │   │   ├── data/           # Repositories & models
│   │   │   ├── domain/         # Business logic
│   │   │   └── presentation/   # UI & state management
│   │   ├── dashboard/          # Main dashboard
│   │   ├── orders/             # Order management
│   │   ├── products/           # Product catalog
│   │   ├── customers/          # Customer management
│   │   └── analytics/          # Analytics & reports
│   ├── shared/                 # Shared widgets & utilities
│   │   ├── widgets/            # Reusable widgets
│   │   ├── models/             # Data models
│   │   └── providers/          # Riverpod providers
│   └── l10n/                   # Internationalization
├── test/                       # Unit & widget tests
├── integration_test/           # Integration tests
├── ios/                        # iOS specific code
├── android/                    # Android specific code
├── web/                        # Web specific code
├── macos/                      # macOS specific code
├── windows/                    # Windows specific code
├── linux/                      # Linux specific code
└── assets/                     # Images, fonts, etc.
    ├── images/
    ├── fonts/
    └── icons/
```

### Required Dependencies (ADD TO pubspec.yaml)
```yaml
name: olympus_app
description: Olympus Cloud - Next Generation Business AI OS
version: 1.0.0+1

environment:
  sdk: '>=3.1.0 <4.0.0'
  flutter: ">=3.16.0"

dependencies:
  flutter:
    sdk: flutter
  flutter_localizations:
    sdk: flutter

  # State Management
  flutter_riverpod: ^2.4.9
  riverpod_annotation: ^2.3.3

  # Routing & Navigation
  go_router: ^12.1.3

  # HTTP & API
  dio: ^5.4.0
  retrofit: ^4.0.3
  json_annotation: ^4.8.1

  # Local Storage & Caching
  hive: ^2.2.3
  hive_flutter: ^1.1.0
  shared_preferences: ^2.2.2

  # UI & Design
  google_fonts: ^6.1.0
  flutter_svg: ^2.0.9
  cached_network_image: ^3.3.1
  shimmer: ^3.0.0
  animations: ^2.0.11
  lottie: ^2.7.0

  # Responsive Design
  flutter_screenutil: ^5.9.0
  responsive_framework: ^1.1.1

  # Forms & Validation
  reactive_forms: ^16.1.1
  mask_text_input_formatter: ^2.9.0

  # Charts & Analytics
  fl_chart: ^0.66.0
  syncfusion_flutter_charts: ^24.1.41

  # Device Features
  permission_handler: ^11.1.0
  image_picker: ^1.0.7
  file_picker: ^6.1.1
  url_launcher: ^6.2.2

  # Security
  flutter_secure_storage: ^9.0.0
  crypto: ^3.0.3

  # Utilities
  intl: ^0.18.1
  uuid: ^4.2.1
  equatable: ^2.0.5
  freezed_annotation: ^2.4.1

  # Development
  flutter_lints: ^3.0.1

dev_dependencies:
  flutter_test:
    sdk: flutter

  # Code Generation
  build_runner: ^2.4.7
  riverpod_generator: ^2.3.9
  retrofit_generator: ^8.0.6
  json_serializable: ^6.7.1
  freezed: ^2.4.6
  hive_generator: ^2.0.1

  # Testing
  mockito: ^5.4.4
  mocktail: ^1.0.2
  golden_toolkit: ^0.15.0

  # Tools
  flutter_launcher_icons: ^0.13.1
  flutter_native_splash: ^2.3.8
  very_good_analysis: ^5.1.0

flutter:
  uses-material-design: true
  generate: true

  assets:
    - assets/images/
    - assets/icons/
    - assets/lottie/

  fonts:
    - family: Olympus
      fonts:
        - asset: assets/fonts/Olympus-Regular.ttf
        - asset: assets/fonts/Olympus-Bold.ttf
          weight: 700
```

## 🎨 Human-Centric Design System

### Core Design Principles
```dart
// lib/core/theme/app_theme.dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class AppTheme {
  // Human-centric color system
  static const _primarySeed = Color(0xFF2196F3);
  static const _surfaceTint = Color(0xFF1976D2);
  
  static ColorScheme get lightColorScheme => ColorScheme.fromSeed(
    seedColor: _primarySeed,
    brightness: Brightness.light,
    surfaceTint: _surfaceTint,
  );
  
  static ColorScheme get darkColorScheme => ColorScheme.fromSeed(
    seedColor: _primarySeed,
    brightness: Brightness.dark,
    surfaceTint: _surfaceTint,
  );

  static ThemeData get lightTheme => ThemeData(
    useMaterial3: true,
    colorScheme: lightColorScheme,
    textTheme: GoogleFonts.interTextTheme(),
    
    // Human-readable spacing system
    cardTheme: const CardTheme(
      elevation: 2,
      margin: EdgeInsets.all(8),
    ),
    
    // Accessible button styles
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        minimumSize: const Size(120, 48), // Larger touch targets
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
      ),
    ),
    
    // Clear input fields
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      contentPadding: const EdgeInsets.all(16),
    ),
  );

  static ThemeData get darkTheme => ThemeData(
    useMaterial3: true,
    colorScheme: darkColorScheme,
    textTheme: GoogleFonts.interTextTheme(darkColorScheme.onSurface),
    // Same customizations as light theme
  );
}
```

### Adaptive Layout System
```dart
// lib/core/theme/responsive_layout.dart
import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';

class ResponsiveLayout extends StatelessWidget {
  final Widget mobile;
  final Widget? tablet;
  final Widget? desktop;
  final Widget? watch;

  const ResponsiveLayout({
    super.key,
    required this.mobile,
    this.tablet,
    this.desktop,
    this.watch,
  });

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    
    // Watch UI (Apple Watch, Wear OS)
    if (screenWidth < 250 && watch != null) {
      return watch!;
    }
    
    // Mobile UI (phones)
    if (screenWidth < 600) {
      return mobile;
    }
    
    // Tablet UI
    if (screenWidth < 1200) {
      return tablet ?? mobile;
    }
    
    // Desktop UI
    return desktop ?? tablet ?? mobile;
  }
}

// Responsive spacing and sizing
class AppSpacing {
  static double get xs => 4.w;
  static double get sm => 8.w;
  static double get md => 16.w;
  static double get lg => 24.w;
  static double get xl => 32.w;
  static double get xxl => 48.w;
}

class AppSizing {
  static double get buttonHeight => 48.h;
  static double get cardRadius => 12.r;
  static double get iconSize => 24.w;
  static double get iconSizeLarge => 32.w;
}
```

## 🧠 State Management with Riverpod

### Authentication State
```dart
// lib/features/auth/data/auth_repository.dart
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'auth_repository.g.dart';

@riverpod
AuthRepository authRepository(AuthRepositoryRef ref) {
  final dio = ref.watch(dioProvider);
  return AuthRepository(dio);
}

class AuthRepository {
  final Dio _dio;
  
  AuthRepository(this._dio);
  
  Future<TokenResponse> login(LoginRequest request) async {
    final response = await _dio.post('/auth/login', data: request.toJson());
    return TokenResponse.fromJson(response.data);
  }
  
  Future<User> getCurrentUser() async {
    final response = await _dio.get('/auth/me');
    return User.fromJson(response.data);
  }
  
  Future<void> logout() async {
    await _dio.post('/auth/logout');
  }
}

// lib/features/auth/domain/auth_state.dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'auth_state.freezed.dart';

@freezed
class AuthState with _$AuthState {
  const factory AuthState.loading() = _Loading;
  const factory AuthState.authenticated(User user) = _Authenticated;
  const factory AuthState.unauthenticated() = _Unauthenticated;
  const factory AuthState.error(String message) = _Error;
}

// lib/features/auth/presentation/auth_controller.dart
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'auth_controller.g.dart';

@riverpod
class AuthController extends _$AuthController {
  @override
  FutureOr<AuthState> build() async {
    // Check for stored token and validate
    final token = await ref.read(tokenStorageProvider).getToken();
    if (token != null) {
      try {
        final user = await ref.read(authRepositoryProvider).getCurrentUser();
        return AuthState.authenticated(user);
      } catch (e) {
        return const AuthState.unauthenticated();
      }
    }
    return const AuthState.unauthenticated();
  }
  
  Future<void> login(String email, String password, String tenant) async {
    state = const AsyncValue.loading();
    
    try {
      final request = LoginRequest(
        email: email,
        password: password,
        tenantSlug: tenant,
      );
      
      final tokenResponse = await ref.read(authRepositoryProvider).login(request);
      await ref.read(tokenStorageProvider).saveToken(tokenResponse.accessToken);
      
      final user = await ref.read(authRepositoryProvider).getCurrentUser();
      state = AsyncValue.data(AuthState.authenticated(user));
    } catch (e, stackTrace) {
      state = AsyncValue.error(e, stackTrace);
    }
  }
  
  Future<void> logout() async {
    try {
      await ref.read(authRepositoryProvider).logout();
      await ref.read(tokenStorageProvider).clearToken();
      state = const AsyncValue.data(AuthState.unauthenticated());
    } catch (e) {
      // Still clear local state even if server call fails
      await ref.read(tokenStorageProvider).clearToken();
      state = const AsyncValue.data(AuthState.unauthenticated());
    }
  }
}
```

### Natural Language Command Bar
```dart
// lib/shared/widgets/command_bar.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class CommandBar extends ConsumerStatefulWidget {
  const CommandBar({super.key});

  @override
  ConsumerState<CommandBar> createState() => _CommandBarState();
}

class _CommandBarState extends ConsumerState<CommandBar> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();
  
  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.all(AppSpacing.md),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: BorderRadius.circular(AppSizing.cardRadius),
        border: Border.all(
          color: Theme.of(context).colorScheme.outline,
        ),
      ),
      child: Row(
        children: [
          Icon(
            Icons.mic,
            size: AppSizing.iconSize,
            color: Theme.of(context).colorScheme.primary,
          ),
          SizedBox(width: AppSpacing.sm),
          Expanded(
            child: TextField(
              controller: _controller,
              focusNode: _focusNode,
              decoration: const InputDecoration(
                hintText: "What would you like to do? Try 'create new order' or 'show today's sales'",
                border: InputBorder.none,
                contentPadding: EdgeInsets.zero,
              ),
              onSubmitted: _handleCommand,
            ),
          ),
          IconButton(
            onPressed: () => _handleCommand(_controller.text),
            icon: const Icon(Icons.send),
          ),
        ],
      ),
    );
  }
  
  void _handleCommand(String command) {
    if (command.trim().isEmpty) return;
    
    // Process natural language command
    ref.read(naturalLanguageControllerProvider.notifier)
        .processCommand(command);
    
    _controller.clear();
  }
  
  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }
}
```

## 📱 Platform-Specific Implementations

### Watch App Integration
```dart
// lib/platforms/watch/watch_app.dart
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';

class WatchApp extends StatelessWidget {
  const WatchApp({super.key});

  @override
  Widget build(BuildContext context) {
    final screenSize = MediaQuery.of(context).size;
    final isAppleWatch = screenSize.width < 250;
    
    if (isAppleWatch) {
      return CupertinoApp(
        title: 'Olympus',
        theme: const CupertinoThemeData(
          primaryColor: Color(0xFF2196F3),
        ),
        home: const WatchDashboard(),
      );
    }
    
    // Wear OS or other smart watches
    return MaterialApp(
      title: 'Olympus',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF2196F3),
          brightness: Brightness.dark,
        ),
      ),
      home: const WatchDashboard(),
    );
  }
}

class WatchDashboard extends StatelessWidget {
  const WatchDashboard({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Column(
          children: [
            // Quick stats
            _buildQuickStat('Orders', '23'),
            _buildQuickStat('Revenue', '\$1,247'),
            
            // Quick actions
            const Spacer(),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _buildQuickAction(Icons.add, 'New Order'),
                _buildQuickAction(Icons.inventory, 'Stock'),
              ],
            ),
          ],
        ),
      ),
    );
  }
  
  Widget _buildQuickStat(String label, String value) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        children: [
          Text(
            value,
            style: const TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
            ),
          ),
          Text(
            label,
            style: const TextStyle(fontSize: 12),
          ),
        ],
      ),
    );
  }
  
  Widget _buildQuickAction(IconData icon, String label) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        IconButton(
          onPressed: () {
            // Handle action
          },
          icon: Icon(icon),
        ),
        Text(
          label,
          style: const TextStyle(fontSize: 10),
        ),
      ],
    );
  }
}
```

## 🔄 Real-Time Data Integration

### WebSocket Connection
```dart
// lib/core/services/websocket_service.dart
import 'dart:convert';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'websocket_service.g.dart';

@riverpod
WebSocketService webSocketService(WebSocketServiceRef ref) {
  return WebSocketService();
}

class WebSocketService {
  WebSocketChannel? _channel;
  Stream<dynamic>? _stream;
  
  void connect(String url, String token) {
    _channel = WebSocketChannel.connect(
      Uri.parse(url),
      protocols: ['Bearer', token],
    );
    
    _stream = _channel!.stream.map((data) {
      return jsonDecode(data as String);
    });
  }
  
  Stream<dynamic> get stream => _stream!;
  
  void send(Map<String, dynamic> data) {
    _channel?.sink.add(jsonEncode(data));
  }
  
  void disconnect() {
    _channel?.sink.close();
    _channel = null;
    _stream = null;
  }
}

// Real-time order updates
@riverpod
class OrderUpdatesController extends _$OrderUpdatesController {
  @override
  Stream<List<Order>> build() {
    final webSocket = ref.read(webSocketServiceProvider);
    
    return webSocket.stream
        .where((data) => data['type'] == 'order_update')
        .map((data) => Order.fromJson(data['payload']))
        .scan<List<Order>>((previous, element, index) {
      final currentOrders = previous ?? <Order>[];
      final existingIndex = currentOrders.indexWhere((o) => o.id == element.id);
      
      if (existingIndex >= 0) {
        currentOrders[existingIndex] = element;
      } else {
        currentOrders.add(element);
      }
      
      return currentOrders;
    }, <Order>[]);
  }
}
```

## 📋 Your Daily Development Workflow

### Morning Routine (MANDATORY)
```bash
# 1. Sync with main and other agents
cd worktree-copilot
git pull origin main
git merge main

# 2. Check coordination docs
cat docs/daily-status.md
cat docs/integration-points.md

# 3. Update your status in docs/daily-status.md

# 4. Start development environment
make dev-flutter
# This runs: flutter run -d chrome --web-port 3000
```

### Development Cycle
```bash
# Hot reload development
# Flutter will automatically reload on file changes

# Run tests frequently
flutter test

# Check for issues
flutter analyze

# Format code
dart format .

# Generate code (if needed)
dart run build_runner build

# Commit frequently (every 1-2 hours)
git add -p
git commit -m "copilot(ui): implement responsive login form"
```

### Evening Integration
```bash
# Build for all platforms to ensure compatibility
flutter build web
flutter build apk --debug
flutter build ios --debug (if on macOS)

# Push your work
git push origin feat/flutter-ui

# Update status in docs/daily-status.md
```

## 🎯 Week 1 Implementation Priorities

### Day 1: Project Foundation
```bash
# 1. Initialize Flutter project
flutter create --org io.olympuscloud --project-name olympus_app .

# 2. Setup project structure
# Create all directories shown above

# 3. Configure dependencies
# Add all packages from pubspec.yaml above

# 4. Setup design system
# Implement AppTheme, ResponsiveLayout, AppSpacing
```

### Day 2: Authentication UI
```dart
// Implement these screens in order:
// 1. LoginScreen - Email/password authentication
// 2. RegisterScreen - User registration
// 3. ForgotPasswordScreen - Password recovery
// 4. TenantSelectionScreen - Multi-tenant login
```

### Day 3: Navigation & State
```dart
// Implement:
// 1. GoRouter configuration
// 2. AuthController with Riverpod
// 3. Navigation guards for protected routes
// 4. Bottom navigation or sidebar
```

### Day 4: Dashboard Foundation
```dart
// Create:
// 1. Main dashboard layout
// 2. Command bar for natural language input
// 3. Quick action cards
// 4. Real-time data widgets
```

## 📊 Testing Standards (MANDATORY)

### Widget Testing
```dart
// test/features/auth/presentation/login_screen_test.dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:olympus_app/features/auth/presentation/login_screen.dart';

void main() {
  group('LoginScreen', () {
    testWidgets('displays email and password fields', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: const LoginScreen(),
          ),
        ),
      );
      
      expect(find.byType(TextFormField), findsNWidgets(2));
      expect(find.text('Email'), findsOneWidget);
      expect(find.text('Password'), findsOneWidget);
      expect(find.byType(ElevatedButton), findsOneWidget);
    });
    
    testWidgets('shows error when login fails', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            authControllerProvider.overrideWith(
              () => MockAuthController()..setError('Invalid credentials'),
            ),
          ],
          child: const MaterialApp(
            home: LoginScreen(),
          ),
        ),
      );
      
      await tester.enterText(
        find.byKey(const Key('email_field')), 
        'test@example.com',
      );
      await tester.enterText(
        find.byKey(const Key('password_field')), 
        'password',
      );
      
      await tester.tap(find.byType(ElevatedButton));
      await tester.pumpAndSettle();
      
      expect(find.text('Invalid credentials'), findsOneWidget);
    });
  });
}
```

### Integration Testing
```dart
// integration_test/app_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:olympus_app/main.dart' as app;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  
  group('App Integration Tests', () {
    testWidgets('complete login flow', (tester) async {
      app.main();
      await tester.pumpAndSettle();
      
      // Should show login screen
      expect(find.text('Welcome to Olympus'), findsOneWidget);
      
      // Enter credentials
      await tester.enterText(
        find.byKey(const Key('email_field')), 
        'test@example.com',
      );
      await tester.enterText(
        find.byKey(const Key('password_field')), 
        'password123',
      );
      await tester.enterText(
        find.byKey(const Key('tenant_field')), 
        'demo-restaurant',
      );
      
      // Login
      await tester.tap(find.text('Login'));
      await tester.pumpAndSettle(const Duration(seconds: 3));
      
      // Should navigate to dashboard
      expect(find.text('Dashboard'), findsOneWidget);
      expect(find.byKey(const Key('command_bar')), findsOneWidget);
    });
  });
}
```

## 🔗 Critical Integration Points

### API Integration
```dart
// lib/core/services/api_service.dart
import 'package:dio/dio.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'api_service.g.dart';

@riverpod
Dio dio(DioRef ref) {
  final dio = Dio(BaseOptions(
    baseUrl: 'http://localhost:8080/api/v1', // Development
    connectTimeout: const Duration(seconds: 30),
    receiveTimeout: const Duration(seconds: 30),
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
  ));
  
  // Add auth interceptor
  dio.interceptors.add(AuthInterceptor(ref));
  
  // Add logging in debug mode
  if (kDebugMode) {
    dio.interceptors.add(LogInterceptor(
      requestBody: true,
      responseBody: true,
    ));
  }
  
  return dio;
}

class AuthInterceptor extends Interceptor {
  final Ref ref;
  
  AuthInterceptor(this.ref);
  
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) async {
    final token = await ref.read(tokenStorageProvider).getToken();
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }
  
  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    if (err.response?.statusCode == 401) {
      // Token expired, redirect to login
      ref.read(authControllerProvider.notifier).logout();
    }
    handler.next(err);
  }
}
```

### Coordination with Backend Services
- **Authentication**: Integrates with Claude Code's Rust auth service
- **API Gateway**: Consumes ChatGPT's Go API endpoints
- **Real-time Updates**: WebSocket connection for live data
- **Analytics**: Displays OpenAI Codex's Python analytics data

## 🎨 Accessibility & Inclusive Design

### Accessibility Standards
```dart
// lib/core/a11y/accessibility_service.dart
import 'package:flutter/semantics.dart';
import 'package:flutter/services.dart';

class AccessibilityService {
  static void announceMessage(String message) {
    SemanticsService.announce(message, TextDirection.ltr);
  }
  
  static void hapticFeedback() {
    HapticFeedback.lightImpact();
  }
  
  static Widget makeAccessible({
    required Widget child,
    required String semanticsLabel,
    String? semanticsHint,
    bool excludeSemantics = false,
  }) {
    return Semantics(
      label: semanticsLabel,
      hint: semanticsHint,
      excludeSemantics: excludeSemantics,
      child: child,
    );
  }
}

// Usage in widgets
ElevatedButton(
  onPressed: _handleLogin,
  child: AccessibilityService.makeAccessible(
    semanticsLabel: 'Login button. Tap to sign in to your account.',
    semanticsHint: 'Double tap to activate',
    child: const Text('Login'),
  ),
)
```

## 🏁 Success Criteria

### Week 1 Deliverables
- [ ] Flutter project initialized with proper structure
- [ ] Design system and theming complete
- [ ] Authentication screens with state management
- [ ] Responsive layout working on all platforms
- [ ] Navigation system with protected routes
- [ ] Command bar for natural language input
- [ ] Basic dashboard with real-time data
- [ ] API integration with backend services
- [ ] Accessibility compliance
- [ ] Test coverage >80%

### Platform Compatibility
- [ ] iOS app builds and runs
- [ ] Android app builds and runs
- [ ] Web app works in Chrome, Safari, Firefox
- [ ] Desktop apps (Windows, macOS, Linux)
- [ ] Basic watch app functionality

### Quality Gates
- [ ] `flutter analyze` - Zero issues
- [ ] `flutter test` - All tests pass
- [ ] Accessibility scanner - No violations
- [ ] Performance - 60fps on all platforms
- [ ] Bundle size optimization

**Remember**: You're creating the human face of the entire platform. Every interaction should feel natural, intuitive, and delightful. Users will judge the entire system by your UI/UX.

**Your motto**: *"Beautiful, intuitive, accessible to all."*