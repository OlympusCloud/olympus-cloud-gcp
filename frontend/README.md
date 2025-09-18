# Flutter Universal App - GitHub Copilot Agent

## Overview
This is the Flutter frontend application that provides a universal UI across iOS, Android, Web, Desktop (Windows/Mac/Linux), and Watch platforms.

## Owner
**GitHub Copilot** - Responsible for Flutter UI/UX implementation

## Features
- Universal app for all platforms
- Adaptive UI based on screen size
- Natural language interface
- Real-time updates via WebSocket
- Offline-first architecture
- Multi-tenant branding system

## Quick Start

```bash
# Check Flutter installation
flutter doctor

# Get dependencies
flutter pub get

# Run on different platforms
flutter run -d chrome        # Web
flutter run -d macos         # macOS
flutter run -d ios           # iOS Simulator
flutter run -d android       # Android Emulator

# Build for production
flutter build web --release
flutter build ios --release
flutter build apk --release
```

## Supported Platforms
- 📱 iOS (iPhone & iPad)
- 🤖 Android (Phone & Tablet)
- 🌐 Web (Responsive)
- 💻 Desktop (Windows, macOS, Linux)
- ⌚ Watch (Apple Watch, Wear OS, Garmin)

## Directory Structure
```
frontend/
├── lib/
│   ├── main.dart            # Entry point
│   ├── app/                # App configuration
│   ├── core/               # Core utilities
│   │   ├── constants/      # App constants
│   │   ├── theme/         # Theme & styling
│   │   └── utils/         # Utilities
│   ├── data/              # Data layer
│   │   ├── models/        # Data models
│   │   ├── repositories/  # API repositories
│   │   └── providers/     # Riverpod providers
│   ├── presentation/      # UI layer
│   │   ├── screens/       # App screens
│   │   ├── widgets/       # Reusable widgets
│   │   └── controllers/   # State controllers
│   └── services/          # Services
│       ├── api/          # API client
│       ├── auth/         # Authentication
│       └── websocket/    # Real-time updates
├── test/                  # Test files
├── assets/               # Images, fonts
├── web/                  # Web-specific files
├── ios/                  # iOS-specific files
├── android/              # Android-specific files
├── windows/              # Windows-specific files
├── macos/                # macOS-specific files
├── linux/                # Linux-specific files
└── pubspec.yaml          # Dependencies
```

## Key Dependencies
```yaml
dependencies:
  flutter:
    sdk: flutter

  # State Management
  flutter_riverpod: ^2.4.0

  # Navigation
  go_router: ^12.0.0

  # HTTP & WebSocket
  dio: ^5.3.0
  web_socket_channel: ^2.4.0

  # Storage
  hive: ^2.2.3
  shared_preferences: ^2.2.0

  # UI Components
  flutter_animate: ^4.2.0
  cached_network_image: ^3.3.0
```

## Environment Configuration
```dart
// lib/core/config/environment.dart
class Environment {
  static const String apiUrl = String.fromEnvironment(
    'API_URL',
    defaultValue: 'http://localhost:8080',
  );

  static const String wsUrl = String.fromEnvironment(
    'WS_URL',
    defaultValue: 'ws://localhost:8080/ws',
  );
}
```

## Integration Points
- **Go API Gateway**: Connect to port 8080 for all API calls
- **WebSocket**: Real-time updates from Go service
- **Authentication**: JWT tokens from auth endpoints

## Design System
- **Material 3**: Latest Material Design
- **Adaptive Layouts**: Responsive to screen size
- **Dark Mode**: System-aware theming
- **Accessibility**: WCAG 2.1 AA compliant

## Next Steps for GitHub Copilot
1. Initialize Flutter project with all platforms
2. Set up Riverpod for state management
3. Create authentication flow UI
4. Implement adaptive layouts
5. Build reusable component library
6. Set up API client with Dio
7. Implement WebSocket connection