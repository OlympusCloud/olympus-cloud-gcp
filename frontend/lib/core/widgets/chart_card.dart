import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

enum ChartType {
  line,
  bar,
  pie,
  area,
}

class ChartCard extends StatelessWidget {
  final String title;
  final String? subtitle;
  final ChartType chartType;
  final List<ChartData>? data;
  final Color? primaryColor;
  final Color? backgroundColor;
  final VoidCallback? onTap;
  final Widget? action;

  const ChartCard({
    super.key,
    required this.title,
    required this.chartType,
    this.subtitle,
    this.data,
    this.primaryColor,
    this.backgroundColor,
    this.onTap,
    this.action,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final defaultColor = primaryColor ?? theme.colorScheme.primary;
    
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      color: backgroundColor,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Row(
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          title,
                          style: theme.textTheme.titleLarge?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        if (subtitle != null) ...[
                          const SizedBox(height: 4),
                          Text(
                            subtitle!,
                            style: theme.textTheme.bodyMedium?.copyWith(
                              color: theme.colorScheme.onSurface.withValues(alpha: 0.7),
                            ),
                          ),
                        ],
                      ],
                    ),
                  ),
                  if (action != null) action!,
                ],
              ),
              const SizedBox(height: 16),
              // Chart
              SizedBox(
                height: 200,
                child: _buildChart(context, defaultColor),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildChart(BuildContext context, Color color) {
    // If no data provided, show placeholder
    if (data == null || data!.isEmpty) {
      return _buildPlaceholderChart(context, color);
    }
    
    switch (chartType) {
      case ChartType.line:
        return _buildLineChart(color);
      case ChartType.bar:
        return _buildBarChart(color);
      case ChartType.pie:
        return _buildPieChart(color);
      case ChartType.area:
        return _buildAreaChart(color);
    }
  }

  Widget _buildPlaceholderChart(BuildContext context, Color color) {
    final theme = Theme.of(context);
    
    return Container(
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              _getChartIcon(),
              size: 48,
              color: color.withValues(alpha: 0.5),
            ),
            const SizedBox(height: 8),
            Text(
              'Chart Preview',
              style: theme.textTheme.bodyMedium?.copyWith(
                color: color.withValues(alpha: 0.7),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildLineChart(Color color) {
    return LineChart(
      LineChartData(
        gridData: const FlGridData(show: false),
        titlesData: const FlTitlesData(show: false),
        borderData: FlBorderData(show: false),
        lineBarsData: [
          LineChartBarData(
            spots: _generateSampleLineData(),
            isCurved: true,
            color: color,
            barWidth: 3,
            dotData: const FlDotData(show: false),
            belowBarData: BarAreaData(
              show: true,
              color: color.withValues(alpha: 0.1),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBarChart(Color color) {
    return BarChart(
      BarChartData(
        gridData: const FlGridData(show: false),
        titlesData: const FlTitlesData(show: false),
        borderData: FlBorderData(show: false),
        barGroups: _generateSampleBarData(color),
      ),
    );
  }

  Widget _buildPieChart(Color color) {
    return PieChart(
      PieChartData(
        sections: _generateSamplePieData(color),
        centerSpaceRadius: 40,
        sectionsSpace: 2,
      ),
    );
  }

  Widget _buildAreaChart(Color color) {
    return LineChart(
      LineChartData(
        gridData: const FlGridData(show: false),
        titlesData: const FlTitlesData(show: false),
        borderData: FlBorderData(show: false),
        lineBarsData: [
          LineChartBarData(
            spots: _generateSampleLineData(),
            isCurved: true,
            color: color,
            barWidth: 0,
            dotData: const FlDotData(show: false),
            belowBarData: BarAreaData(
              show: true,
              color: color.withValues(alpha: 0.3),
            ),
          ),
        ],
      ),
    );
  }

  IconData _getChartIcon() {
    switch (chartType) {
      case ChartType.line:
        return Icons.show_chart;
      case ChartType.bar:
        return Icons.bar_chart;
      case ChartType.pie:
        return Icons.pie_chart;
      case ChartType.area:
        return Icons.area_chart;
    }
  }

  List<FlSpot> _generateSampleLineData() {
    return [
      const FlSpot(0, 3),
      const FlSpot(1, 4),
      const FlSpot(2, 3.5),
      const FlSpot(3, 5),
      const FlSpot(4, 4.5),
      const FlSpot(5, 6),
      const FlSpot(6, 5.5),
    ];
  }

  List<BarChartGroupData> _generateSampleBarData(Color color) {
    return [
      BarChartGroupData(x: 0, barRods: [BarChartRodData(toY: 8, color: color)]),
      BarChartGroupData(x: 1, barRods: [BarChartRodData(toY: 10, color: color)]),
      BarChartGroupData(x: 2, barRods: [BarChartRodData(toY: 6, color: color)]),
      BarChartGroupData(x: 3, barRods: [BarChartRodData(toY: 12, color: color)]),
      BarChartGroupData(x: 4, barRods: [BarChartRodData(toY: 9, color: color)]),
    ];
  }

  List<PieChartSectionData> _generateSamplePieData(Color color) {
    return [
      PieChartSectionData(
        color: color,
        value: 40,
        title: '40%',
        radius: 60,
      ),
      PieChartSectionData(
        color: color.withValues(alpha: 0.7),
        value: 30,
        title: '30%',
        radius: 55,
      ),
      PieChartSectionData(
        color: color.withValues(alpha: 0.4),
        value: 20,
        title: '20%',
        radius: 50,
      ),
      PieChartSectionData(
        color: color.withValues(alpha: 0.2),
        value: 10,
        title: '10%',
        radius: 45,
      ),
    ];
  }
}

class ChartData {
  final String label;
  final double value;
  final Color? color;

  const ChartData({
    required this.label,
    required this.value,
    this.color,
  });
}