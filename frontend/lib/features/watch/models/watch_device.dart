import 'package:freezed_annotation/freezed_annotation.dart';

part 'watch_device.freezed.dart';
part 'watch_device.g.dart';

@freezed
class WatchDevice with _$WatchDevice {
  const factory WatchDevice({
    required String id,
    required String name,
    required WatchDeviceType type,
    required WatchConnectionStatus status,
    String? model,
    String? osVersion,
    DateTime? lastConnected,
    Map<String, dynamic>? capabilities,
  }) = _WatchDevice;

  factory WatchDevice.fromJson(Map<String, dynamic> json) =>
      _$WatchDeviceFromJson(json);
}

@freezed
class WatchDeviceCapabilities with _$WatchDeviceCapabilities {
  const factory WatchDeviceCapabilities({
    @Default(false) bool supportsNotifications,
    @Default(false) bool supportsHeartRate,
    @Default(false) bool supportsGPS,
    @Default(false) bool supportsCellular,
    @Default(false) bool supportsPayments,
    @Default(false) bool supportsApps,
    @Default(false) bool supportsComplication,
    @Default(false) bool supportsHapticFeedback,
    @Default(false) bool supportsMicrophone,
    @Default(false) bool supportsSpeaker,
  }) = _WatchDeviceCapabilities;

  factory WatchDeviceCapabilities.fromJson(Map<String, dynamic> json) =>
      _$WatchDeviceCapabilitiesFromJson(json);
}

enum WatchDeviceType {
  @JsonValue('apple_watch')
  appleWatch,
  @JsonValue('wear_os')
  wearOS,
  @JsonValue('garmin')
  garmin,
  @JsonValue('fitbit')
  fitbit,
  @JsonValue('samsung_galaxy_watch')
  samsungGalaxyWatch,
  @JsonValue('other')
  other,
}

enum WatchConnectionStatus {
  @JsonValue('connected')
  connected,
  @JsonValue('disconnected')
  disconnected,
  @JsonValue('connecting')
  connecting,
  @JsonValue('pairing')
  pairing,
  @JsonValue('error')
  error,
}

extension WatchDeviceTypeExtension on WatchDeviceType {
  String get displayName {
    switch (this) {
      case WatchDeviceType.appleWatch:
        return 'Apple Watch';
      case WatchDeviceType.wearOS:
        return 'Wear OS';
      case WatchDeviceType.garmin:
        return 'Garmin';
      case WatchDeviceType.fitbit:
        return 'Fitbit';
      case WatchDeviceType.samsungGalaxyWatch:
        return 'Samsung Galaxy Watch';
      case WatchDeviceType.other:
        return 'Other';
    }
  }

  String get iconAsset {
    switch (this) {
      case WatchDeviceType.appleWatch:
        return 'assets/icons/apple_watch.svg';
      case WatchDeviceType.wearOS:
        return 'assets/icons/wear_os.svg';
      case WatchDeviceType.garmin:
        return 'assets/icons/garmin.svg';
      case WatchDeviceType.fitbit:
        return 'assets/icons/fitbit.svg';
      case WatchDeviceType.samsungGalaxyWatch:
        return 'assets/icons/samsung_watch.svg';
      case WatchDeviceType.other:
        return 'assets/icons/watch_generic.svg';
    }
  }
}

extension WatchConnectionStatusExtension on WatchConnectionStatus {
  String get displayName {
    switch (this) {
      case WatchConnectionStatus.connected:
        return 'Connected';
      case WatchConnectionStatus.disconnected:
        return 'Disconnected';
      case WatchConnectionStatus.connecting:
        return 'Connecting';
      case WatchConnectionStatus.pairing:
        return 'Pairing';
      case WatchConnectionStatus.error:
        return 'Connection Error';
    }
  }

  bool get isConnected => this == WatchConnectionStatus.connected;
  bool get isConnecting => this == WatchConnectionStatus.connecting || this == WatchConnectionStatus.pairing;
  bool get hasError => this == WatchConnectionStatus.error;
}