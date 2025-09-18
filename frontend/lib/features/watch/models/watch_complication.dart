import 'package:freezed_annotation/freezed_annotation.dart';

part 'watch_complication.freezed.dart';
part 'watch_complication.g.dart';

@freezed
class WatchComplication with _$WatchComplication {
  const factory WatchComplication({
    required String id,
    required String title,
    required WatchComplicationType type,
    required String value,
    String? subtitle,
    String? unit,
    WatchComplicationTrend? trend,
    DateTime? lastUpdated,
    Map<String, dynamic>? metadata,
    @Default(true) bool enabled,
    @Default(300) int refreshIntervalSeconds,
  }) = _WatchComplication;

  factory WatchComplication.fromJson(Map<String, dynamic> json) =>
      _$WatchComplicationFromJson(json);
}

@freezed
class WatchComplicationConfig with _$WatchComplicationConfig {
  const factory WatchComplicationConfig({
    required List<String> enabledComplications,
    required Map<String, int> refreshIntervals,
    @Default(true) bool autoUpdate,
    @Default(false) bool showTrends,
    @Default(false) bool vibrationAlerts,
    WatchComplicationDisplayMode? displayMode,
  }) = _WatchComplicationConfig;

  factory WatchComplicationConfig.fromJson(Map<String, dynamic> json) =>
      _$WatchComplicationConfigFromJson(json);
}

enum WatchComplicationType {
  @JsonValue('daily_sales')
  dailySales,
  @JsonValue('order_count')
  orderCount,
  @JsonValue('current_customers')
  currentCustomers,
  @JsonValue('inventory_alerts')
  inventoryAlerts,
  @JsonValue('staff_status')
  staffStatus,
  @JsonValue('payment_pending')
  paymentPending,
  @JsonValue('next_appointment')
  nextAppointment,
  @JsonValue('table_status')
  tableStatus,
  @JsonValue('queue_length')
  queueLength,
  @JsonValue('revenue_target')
  revenueTarget,
}

enum WatchComplicationTrend {
  @JsonValue('up')
  up,
  @JsonValue('down')
  down,
  @JsonValue('stable')
  stable,
}

enum WatchComplicationDisplayMode {
  @JsonValue('minimal')
  minimal,
  @JsonValue('detailed')
  detailed,
  @JsonValue('chart')
  chart,
}

extension WatchComplicationTypeExtension on WatchComplicationType {
  String get displayName {
    switch (this) {
      case WatchComplicationType.dailySales:
        return 'Daily Sales';
      case WatchComplicationType.orderCount:
        return 'Orders Today';
      case WatchComplicationType.currentCustomers:
        return 'Current Customers';
      case WatchComplicationType.inventoryAlerts:
        return 'Inventory Alerts';
      case WatchComplicationType.staffStatus:
        return 'Staff Status';
      case WatchComplicationType.paymentPending:
        return 'Pending Payments';
      case WatchComplicationType.nextAppointment:
        return 'Next Appointment';
      case WatchComplicationType.tableStatus:
        return 'Table Status';
      case WatchComplicationType.queueLength:
        return 'Queue Length';
      case WatchComplicationType.revenueTarget:
        return 'Revenue Target';
    }
  }

  String get shortName {
    switch (this) {
      case WatchComplicationType.dailySales:
        return 'Sales';
      case WatchComplicationType.orderCount:
        return 'Orders';
      case WatchComplicationType.currentCustomers:
        return 'Customers';
      case WatchComplicationType.inventoryAlerts:
        return 'Inventory';
      case WatchComplicationType.staffStatus:
        return 'Staff';
      case WatchComplicationType.paymentPending:
        return 'Payments';
      case WatchComplicationType.nextAppointment:
        return 'Next';
      case WatchComplicationType.tableStatus:
        return 'Tables';
      case WatchComplicationType.queueLength:
        return 'Queue';
      case WatchComplicationType.revenueTarget:
        return 'Target';
    }
  }

  String get iconEmoji {
    switch (this) {
      case WatchComplicationType.dailySales:
        return '💰';
      case WatchComplicationType.orderCount:
        return '📝';
      case WatchComplicationType.currentCustomers:
        return '👥';
      case WatchComplicationType.inventoryAlerts:
        return '📦';
      case WatchComplicationType.staffStatus:
        return '👨‍💼';
      case WatchComplicationType.paymentPending:
        return '💳';
      case WatchComplicationType.nextAppointment:
        return '📅';
      case WatchComplicationType.tableStatus:
        return '🪑';
      case WatchComplicationType.queueLength:
        return '📊';
      case WatchComplicationType.revenueTarget:
        return '🎯';
    }
  }

  String get defaultUnit {
    switch (this) {
      case WatchComplicationType.dailySales:
      case WatchComplicationType.revenueTarget:
        return '\$';
      case WatchComplicationType.orderCount:
      case WatchComplicationType.currentCustomers:
      case WatchComplicationType.inventoryAlerts:
      case WatchComplicationType.staffStatus:
      case WatchComplicationType.paymentPending:
      case WatchComplicationType.tableStatus:
      case WatchComplicationType.queueLength:
        return '';
      case WatchComplicationType.nextAppointment:
        return 'min';
    }
  }

  int get defaultRefreshInterval {
    switch (this) {
      case WatchComplicationType.dailySales:
      case WatchComplicationType.orderCount:
      case WatchComplicationType.revenueTarget:
        return 300; // 5 minutes
      case WatchComplicationType.currentCustomers:
      case WatchComplicationType.tableStatus:
      case WatchComplicationType.queueLength:
        return 60; // 1 minute
      case WatchComplicationType.inventoryAlerts:
      case WatchComplicationType.staffStatus:
        return 600; // 10 minutes
      case WatchComplicationType.paymentPending:
        return 120; // 2 minutes
      case WatchComplicationType.nextAppointment:
        return 30; // 30 seconds
    }
  }

  bool get isRealTime {
    switch (this) {
      case WatchComplicationType.currentCustomers:
      case WatchComplicationType.tableStatus:
      case WatchComplicationType.queueLength:
      case WatchComplicationType.nextAppointment:
        return true;
      default:
        return false;
    }
  }
}

extension WatchComplicationTrendExtension on WatchComplicationTrend {
  String get emoji {
    switch (this) {
      case WatchComplicationTrend.up:
        return '📈';
      case WatchComplicationTrend.down:
        return '📉';
      case WatchComplicationTrend.stable:
        return '➡️';
    }
  }

  String get symbol {
    switch (this) {
      case WatchComplicationTrend.up:
        return '↗';
      case WatchComplicationTrend.down:
        return '↘';
      case WatchComplicationTrend.stable:
        return '→';
    }
  }
}