import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'platform_info.dart';

/// Platform-specific input handling and accessibility
class PlatformInput {
  /// Configure keyboard shortcuts for the platform
  static Map<LogicalKeySet, Intent> getKeyboardShortcuts() {
    if (!PlatformInfo.supportsFeature(PlatformFeature.keyboard)) {
      return {};
    }
    
    final shortcuts = <LogicalKeySet, Intent>{};
    
    if (PlatformInfo.isMacOS) {
      // macOS-specific shortcuts
      shortcuts.addAll({
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.keyW): const CloseTabIntent(),
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.keyT): const NewTabIntent(),
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.keyN): const NewWindowIntent(),
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.keyR): const RefreshIntent(),
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.comma): const SettingsIntent(),
        LogicalKeySet(LogicalKeyboardKey.meta, LogicalKeyboardKey.keyQ): const QuitIntent(),
      });
    } else {
      // Windows/Linux shortcuts
      shortcuts.addAll({
        LogicalKeySet(LogicalKeyboardKey.control, LogicalKeyboardKey.keyW): const CloseTabIntent(),
        LogicalKeySet(LogicalKeyboardKey.control, LogicalKeyboardKey.keyT): const NewTabIntent(),
        LogicalKeySet(LogicalKeyboardKey.control, LogicalKeyboardKey.keyN): const NewWindowIntent(),
        LogicalKeySet(LogicalKeyboardKey.f5): const RefreshIntent(),
        LogicalKeySet(LogicalKeyboardKey.control, LogicalKeyboardKey.comma): const SettingsIntent(),
        LogicalKeySet(LogicalKeyboardKey.alt, LogicalKeyboardKey.f4): const QuitIntent(),
      });
    }
    
    // Common shortcuts for all platforms
    shortcuts.addAll({
      LogicalKeySet(LogicalKeyboardKey.escape): const DismissIntent(),
      LogicalKeySet(LogicalKeyboardKey.enter): const ActivateIntent(),
      LogicalKeySet(LogicalKeyboardKey.space): const ActivateIntent(),
      LogicalKeySet(LogicalKeyboardKey.f1): const HelpIntent(),
    });
    
    return shortcuts;
  }
  
  /// Get platform-specific context menu actions
  static List<ContextMenuAction> getContextMenuActions(BuildContext context) {
    if (!PlatformInfo.supportsFeature(PlatformFeature.contextMenu)) {
      return [];
    }
    
    return [
      ContextMenuAction(
        icon: Icons.copy,
        label: 'Copy',
        action: () => _handleCopy(context),
      ),
      ContextMenuAction(
        icon: Icons.paste,
        label: 'Paste',
        action: () => _handlePaste(context),
      ),
      ContextMenuAction(
        icon: Icons.cut,
        label: 'Cut',
        action: () => _handleCut(context),
      ),
      ContextMenuAction(
        icon: Icons.select_all,
        label: 'Select All',
        action: () => _handleSelectAll(context),
      ),
    ];
  }
  
  static void _handleCopy(BuildContext context) {
    // Handle copy action
  }
  
  static void _handlePaste(BuildContext context) {
    // Handle paste action
  }
  
  static void _handleCut(BuildContext context) {
    // Handle cut action
  }
  
  static void _handleSelectAll(BuildContext context) {
    // Handle select all action
  }
  
  /// Provide haptic feedback if supported
  static Future<void> hapticFeedback(HapticFeedbackType type) async {
    if (PlatformInfo.supportsFeature(PlatformFeature.touch) && 
        PlatformInfo.isMobile) {
      switch (type) {
        case HapticFeedbackType.light:
          await HapticFeedback.lightImpact();
          break;
        case HapticFeedbackType.medium:
          await HapticFeedback.mediumImpact();
          break;
        case HapticFeedbackType.heavy:
          await HapticFeedback.heavyImpact();
          break;
        case HapticFeedbackType.selection:
          await HapticFeedback.selectionClick();
          break;
        case HapticFeedbackType.vibrate:
          await HapticFeedback.vibrate();
          break;
      }
    }
  }
  
  /// Get optimized input decoration for the platform
  static InputDecoration getOptimizedInputDecoration({
    required BuildContext context,
    String? labelText,
    String? hintText,
    String? helperText,
    String? errorText,
    Widget? prefixIcon,
    Widget? suffixIcon,
    bool filled = true,
  }) {
    final theme = Theme.of(context);
    final uiConstants = PlatformInfo.getUIConstants();
    
    return InputDecoration(
      labelText: labelText,
      hintText: hintText,
      helperText: helperText,
      errorText: errorText,
      prefixIcon: prefixIcon,
      suffixIcon: suffixIcon,
      filled: filled,
      fillColor: filled ? theme.colorScheme.surface : null,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        borderSide: BorderSide(
          color: theme.colorScheme.outline,
          width: 1.0,
        ),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        borderSide: BorderSide(
          color: theme.colorScheme.outline.withOpacity(0.5),
          width: 1.0,
        ),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        borderSide: BorderSide(
          color: theme.colorScheme.primary,
          width: 2.0,
        ),
      ),
      errorBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        borderSide: BorderSide(
          color: theme.colorScheme.error,
          width: 1.0,
        ),
      ),
      focusedErrorBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        borderSide: BorderSide(
          color: theme.colorScheme.error,
          width: 2.0,
        ),
      ),
      contentPadding: EdgeInsets.symmetric(
        horizontal: uiConstants.defaultPadding,
        vertical: uiConstants.compactPadding,
      ),
    );
  }
}

/// Haptic feedback types
enum HapticFeedbackType {
  light,
  medium,
  heavy,
  selection,
  vibrate,
}

/// Context menu action
class ContextMenuAction {
  final IconData icon;
  final String label;
  final VoidCallback action;
  
  const ContextMenuAction({
    required this.icon,
    required this.label,
    required this.action,
  });
}

/// Intent classes for keyboard shortcuts
class CloseTabIntent extends Intent {
  const CloseTabIntent();
}

class NewTabIntent extends Intent {
  const NewTabIntent();
}

class NewWindowIntent extends Intent {
  const NewWindowIntent();
}

class RefreshIntent extends Intent {
  const RefreshIntent();
}

class SettingsIntent extends Intent {
  const SettingsIntent();
}

class QuitIntent extends Intent {
  const QuitIntent();
}

class HelpIntent extends Intent {
  const HelpIntent();
}

/// Platform-aware text field with optimized input handling
class PlatformTextField extends StatelessWidget {
  final TextEditingController? controller;
  final String? labelText;
  final String? hintText;
  final String? helperText;
  final String? errorText;
  final Widget? prefixIcon;
  final Widget? suffixIcon;
  final TextInputType? keyboardType;
  final TextInputAction? textInputAction;
  final bool obscureText;
  final bool enabled;
  final bool readOnly;
  final int? maxLines;
  final int? minLines;
  final int? maxLength;
  final ValueChanged<String>? onChanged;
  final VoidCallback? onTap;
  final VoidCallback? onEditingComplete;
  final ValueChanged<String>? onSubmitted;
  final List<TextInputFormatter>? inputFormatters;
  final bool autofocus;
  final FocusNode? focusNode;
  
  const PlatformTextField({
    super.key,
    this.controller,
    this.labelText,
    this.hintText,
    this.helperText,
    this.errorText,
    this.prefixIcon,
    this.suffixIcon,
    this.keyboardType,
    this.textInputAction,
    this.obscureText = false,
    this.enabled = true,
    this.readOnly = false,
    this.maxLines = 1,
    this.minLines,
    this.maxLength,
    this.onChanged,
    this.onTap,
    this.onEditingComplete,
    this.onSubmitted,
    this.inputFormatters,
    this.autofocus = false,
    this.focusNode,
  });
  
  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      decoration: PlatformInput.getOptimizedInputDecoration(
        context: context,
        labelText: labelText,
        hintText: hintText,
        helperText: helperText,
        errorText: errorText,
        prefixIcon: prefixIcon,
        suffixIcon: suffixIcon,
      ),
      keyboardType: keyboardType,
      textInputAction: textInputAction,
      obscureText: obscureText,
      enabled: enabled,
      readOnly: readOnly,
      maxLines: maxLines,
      minLines: minLines,
      maxLength: maxLength,
      onChanged: onChanged,
      onTap: () {
        if (PlatformInfo.supportsFeature(PlatformFeature.touch)) {
          PlatformInput.hapticFeedback(HapticFeedbackType.selection);
        }
        onTap?.call();
      },
      onEditingComplete: onEditingComplete,
      onSubmitted: onSubmitted,
      inputFormatters: inputFormatters,
      autofocus: autofocus,
      focusNode: focusNode,
      style: Theme.of(context).textTheme.bodyLarge,
    );
  }
}

/// Platform-aware button with optimized interactions
class PlatformButton extends StatelessWidget {
  final VoidCallback? onPressed;
  final Widget child;
  final ButtonStyle? style;
  final bool autofocus;
  final FocusNode? focusNode;
  final HapticFeedbackType hapticFeedback;
  final PlatformButtonType type;
  
  const PlatformButton({
    super.key,
    required this.onPressed,
    required this.child,
    this.style,
    this.autofocus = false,
    this.focusNode,
    this.hapticFeedback = HapticFeedbackType.light,
    this.type = PlatformButtonType.elevated,
  });
  
  @override
  Widget build(BuildContext context) {
    final uiConstants = PlatformInfo.getUIConstants();
    
    final defaultStyle = ButtonStyle(
      minimumSize: WidgetStateProperty.all(
        Size(0, uiConstants.buttonHeight),
      ),
      shape: WidgetStateProperty.all(
        RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(uiConstants.borderRadius),
        ),
      ),
    );
    
    final mergedStyle = style != null ? defaultStyle.merge(style) : defaultStyle;
    
    Widget button;
    
    switch (type) {
      case PlatformButtonType.elevated:
        button = ElevatedButton(
          onPressed: _handlePressed,
          style: mergedStyle,
          autofocus: autofocus,
          focusNode: focusNode,
          child: child,
        );
        break;
      case PlatformButtonType.outlined:
        button = OutlinedButton(
          onPressed: _handlePressed,
          style: mergedStyle,
          autofocus: autofocus,
          focusNode: focusNode,
          child: child,
        );
        break;
      case PlatformButtonType.text:
        button = TextButton(
          onPressed: _handlePressed,
          style: mergedStyle,
          autofocus: autofocus,
          focusNode: focusNode,
          child: child,
        );
        break;
    }
    
    return button;
  }
  
  void _handlePressed() {
    if (PlatformInfo.supportsFeature(PlatformFeature.touch)) {
      PlatformInput.hapticFeedback(hapticFeedback);
    }
    onPressed?.call();
  }
}

/// Platform button types
enum PlatformButtonType {
  elevated,
  outlined,
  text,
}

/// Keyboard shortcut handler widget
class KeyboardShortcutHandler extends StatelessWidget {
  final Widget child;
  final Map<LogicalKeySet, VoidCallback> shortcuts;
  
  const KeyboardShortcutHandler({
    super.key,
    required this.child,
    required this.shortcuts,
  });
  
  @override
  Widget build(BuildContext context) {
    if (!PlatformInfo.supportsFeature(PlatformFeature.keyboard)) {
      return child;
    }
    
    return Shortcuts(
      shortcuts: shortcuts.map((key, callback) => MapEntry(
        key,
        CallbackIntent(callback),
      )),
      child: Actions(
        actions: {
          CallbackIntent: CallbackAction<CallbackIntent>(
            onInvoke: (intent) => intent.callback(),
          ),
        },
        child: Focus(
          autofocus: true,
          child: child,
        ),
      ),
    );
  }
}

/// Callback intent for keyboard shortcuts
class CallbackIntent extends Intent {
  final VoidCallback callback;
  
  const CallbackIntent(this.callback);
}