import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/branding/branding_provider.dart';
import '../../core/branding/industry_branding.dart';

/// Widget that displays the industry logo and branding
class IndustryLogo extends ConsumerWidget {
  final double? height;
  final double? width;
  final bool showTagline;
  final bool showBrandName;
  final Color? customColor;

  const IndustryLogo({
    super.key,
    this.height,
    this.width,
    this.showTagline = false,
    this.showBrandName = true,
    this.customColor,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    final theme = Theme.of(context);

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Logo image or fallback icon
        Container(
          height: height ?? 48,
          width: width ?? 48,
          decoration: BoxDecoration(
            color: customColor ?? branding.primaryColor,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Icon(
            _getIndustryIcon(branding.industry),
            color: Colors.white,
            size: (height ?? 48) * 0.6,
          ),
        ),
        
        if (showBrandName) ...[
          const SizedBox(height: 8),
          Text(
            branding.brandName,
            style: branding.headingFont.copyWith(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: customColor ?? theme.textTheme.titleLarge?.color,
            ),
            textAlign: TextAlign.center,
          ),
        ],
        
        if (showTagline) ...[
          const SizedBox(height: 4),
          Text(
            branding.tagline,
            style: branding.primaryFont.copyWith(
              fontSize: 12,
              color: theme.textTheme.bodyMedium?.color?.withOpacity(0.7),
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ],
    );
  }

  IconData _getIndustryIcon(IndustryType industry) {
    switch (industry) {
      case IndustryType.restaurant:
        return Icons.restaurant;
      case IndustryType.retail:
        return Icons.storefront;
      case IndustryType.salon:
        return Icons.content_cut;
      case IndustryType.hospitality:
        return Icons.hotel;
      case IndustryType.events:
        return Icons.event;
      case IndustryType.other:
        return Icons.business;
    }
  }
}

/// Status indicator with industry-specific colors
class IndustryStatusIndicator extends ConsumerWidget {
  final String status;
  final double size;
  final bool showLabel;

  const IndustryStatusIndicator({
    super.key,
    required this.status,
    this.size = 12,
    this.showLabel = false,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final statusColors = ref.watch(industryStatusColorsProvider);
    final color = statusColors[status] ?? Colors.grey;

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: size,
          height: size,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
          ),
        ),
        if (showLabel) ...[
          const SizedBox(width: 8),
          Text(
            status.replaceAll('_', ' ').toUpperCase(),
            style: TextStyle(
              fontSize: size * 0.8,
              fontWeight: FontWeight.w600,
              color: color,
            ),
          ),
        ],
      ],
    );
  }
}

/// Feature icon with industry-specific styling
class IndustryFeatureIcon extends ConsumerWidget {
  final String featureKey;
  final double size;
  final Color? color;
  final bool showLabel;
  final String? customLabel;

  const IndustryFeatureIcon({
    super.key,
    required this.featureKey,
    this.size = 24,
    this.color,
    this.showLabel = false,
    this.customLabel,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final featureIcons = ref.watch(industryFeatureIconsProvider);
    final branding = ref.watch(brandingProvider);
    final icon = featureIcons[featureKey] ?? Icons.help_outline;
    final iconColor = color ?? branding.primaryColor;

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(
          icon,
          size: size,
          color: iconColor,
        ),
        if (showLabel) ...[
          const SizedBox(height: 4),
          Text(
            customLabel ?? featureKey.replaceAll('_', ' ').toUpperCase(),
            style: TextStyle(
              fontSize: size * 0.4,
              fontWeight: FontWeight.w500,
              color: iconColor,
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ],
    );
  }
}

/// Industry-branded card widget
class IndustryCard extends ConsumerWidget {
  final Widget child;
  final String? title;
  final String? subtitle;
  final List<Widget>? actions;
  final EdgeInsetsGeometry? padding;
  final Color? backgroundColor;
  final VoidCallback? onTap;

  const IndustryCard({
    super.key,
    required this.child,
    this.title,
    this.subtitle,
    this.actions,
    this.padding,
    this.backgroundColor,
    this.onTap,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    final theme = Theme.of(context);

    return Card(
      color: backgroundColor,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: padding ?? const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (title != null || actions != null)
                Row(
                  children: [
                    if (title != null)
                      Expanded(
                        child: Text(
                          title!,
                          style: branding.headingFont.copyWith(
                            fontSize: 16,
                            fontWeight: FontWeight.w600,
                            color: theme.textTheme.titleLarge?.color,
                          ),
                        ),
                      ),
                    if (actions != null) ...actions!,
                  ],
                ),
              if (subtitle != null) ...[
                const SizedBox(height: 4),
                Text(
                  subtitle!,
                  style: branding.primaryFont.copyWith(
                    fontSize: 14,
                    color: theme.textTheme.bodyMedium?.color?.withOpacity(0.7),
                  ),
                ),
              ],
              if (title != null || subtitle != null) const SizedBox(height: 12),
              child,
            ],
          ),
        ),
      ),
    );
  }
}

/// Industry-branded button
class IndustryButton extends ConsumerWidget {
  final String text;
  final VoidCallback? onPressed;
  final IconData? icon;
  final bool isOutlined;
  final bool isLoading;
  final Color? customColor;

  const IndustryButton({
    super.key,
    required this.text,
    this.onPressed,
    this.icon,
    this.isOutlined = false,
    this.isLoading = false,
    this.customColor,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);
    final buttonColor = customColor ?? branding.primaryColor;

    if (isOutlined) {
      return OutlinedButton.icon(
        onPressed: isLoading ? null : onPressed,
        icon: isLoading
            ? SizedBox(
                width: 16,
                height: 16,
                child: CircularProgressIndicator(
                  strokeWidth: 2,
                  valueColor: AlwaysStoppedAnimation<Color>(buttonColor),
                ),
              )
            : (icon != null ? Icon(icon) : const SizedBox.shrink()),
        label: Text(text),
        style: OutlinedButton.styleFrom(
          foregroundColor: buttonColor,
          side: BorderSide(color: buttonColor),
        ),
      );
    }

    return ElevatedButton.icon(
      onPressed: isLoading ? null : onPressed,
      icon: isLoading
          ? const SizedBox(
              width: 16,
              height: 16,
              child: CircularProgressIndicator(
                strokeWidth: 2,
                valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
              ),
            )
          : (icon != null ? Icon(icon) : const SizedBox.shrink()),
      label: Text(text),
      style: ElevatedButton.styleFrom(
        backgroundColor: buttonColor,
        foregroundColor: Colors.white,
      ),
    );
  }
}

/// Industry-specific app bar
class IndustryAppBar extends ConsumerWidget implements PreferredSizeWidget {
  final String? title;
  final List<Widget>? actions;
  final bool showLogo;
  final bool automaticallyImplyLeading;

  const IndustryAppBar({
    super.key,
    this.title,
    this.actions,
    this.showLogo = true,
    this.automaticallyImplyLeading = true,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final branding = ref.watch(brandingProvider);

    return AppBar(
      automaticallyImplyLeading: automaticallyImplyLeading,
      title: Row(
        children: [
          if (showLogo) ...[
            IndustryLogo(
              height: 32,
              width: 32,
              showBrandName: false,
            ),
            const SizedBox(width: 12),
          ],
          if (title != null)
            Text(
              title!,
              style: branding.headingFont.copyWith(
                fontSize: 18,
                fontWeight: FontWeight.w600,
              ),
            ),
        ],
      ),
      actions: actions,
      backgroundColor: Theme.of(context).colorScheme.surface,
      foregroundColor: Theme.of(context).colorScheme.onSurface,
      elevation: 0,
      centerTitle: false,
    );
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}