import 'package:flutter/material.dart';

/// Custom form field widget with consistent styling and validation
class CustomFormField extends StatelessWidget {
  final String? label;
  final String? hint;
  final String? initialValue;
  final TextEditingController? controller;
  final String? Function(String?)? validator;
  final void Function(String?)? onSaved;
  final void Function(String)? onChanged;
  final void Function(String)? onFieldSubmitted;
  final void Function()? onTap;
  final TextInputType? keyboardType;
  final TextInputAction? textInputAction;
  final bool obscureText;
  final bool enabled;
  final bool readOnly;
  final int? maxLines;
  final int? maxLength;
  final Widget? prefixIcon;
  final Widget? suffixIcon;
  final String? prefixText;
  final String? suffixText;
  final bool required;
  final FocusNode? focusNode;
  final TextCapitalization textCapitalization;

  const CustomFormField({
    super.key,
    this.label,
    this.hint,
    this.initialValue,
    this.controller,
    this.validator,
    this.onSaved,
    this.onChanged,
    this.onFieldSubmitted,
    this.onTap,
    this.keyboardType,
    this.textInputAction,
    this.obscureText = false,
    this.enabled = true,
    this.readOnly = false,
    this.maxLines = 1,
    this.maxLength,
    this.prefixIcon,
    this.suffixIcon,
    this.prefixText,
    this.suffixText,
    this.required = false,
    this.focusNode,
    this.textCapitalization = TextCapitalization.none,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (label != null) ...[
          Row(
            children: [
              Text(
                label!,
                style: theme.textTheme.labelLarge?.copyWith(
                  fontWeight: FontWeight.w500,
                ),
              ),
              if (required)
                Text(
                  ' *',
                  style: TextStyle(
                    color: theme.colorScheme.error,
                    fontSize: theme.textTheme.labelLarge?.fontSize,
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
        ],
        TextFormField(
          controller: controller,
          initialValue: controller == null ? initialValue : null,
          validator: validator,
          onSaved: onSaved,
          onChanged: onChanged,
          onFieldSubmitted: onFieldSubmitted,
          onTap: onTap,
          keyboardType: keyboardType,
          textInputAction: textInputAction,
          obscureText: obscureText,
          enabled: enabled,
          readOnly: readOnly,
          maxLines: maxLines,
          maxLength: maxLength,
          focusNode: focusNode,
          textCapitalization: textCapitalization,
          decoration: InputDecoration(
            hintText: hint,
            prefixIcon: prefixIcon,
            suffixIcon: suffixIcon,
            prefixText: prefixText,
            suffixText: suffixText,
            counterText: '', // Hide character counter
          ),
        ),
      ],
    );
  }
}

/// Dropdown form field with consistent styling
class CustomDropdownField<T> extends StatelessWidget {
  final String? label;
  final String? hint;
  final T? value;
  final List<DropdownMenuItem<T>> items;
  final void Function(T?)? onChanged;
  final String? Function(T?)? validator;
  final void Function(T?)? onSaved;
  final bool enabled;
  final Widget? prefixIcon;
  final bool required;

  const CustomDropdownField({
    super.key,
    this.label,
    this.hint,
    this.value,
    required this.items,
    this.onChanged,
    this.validator,
    this.onSaved,
    this.enabled = true,
    this.prefixIcon,
    this.required = false,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (label != null) ...[
          Row(
            children: [
              Text(
                label!,
                style: theme.textTheme.labelLarge?.copyWith(
                  fontWeight: FontWeight.w500,
                ),
              ),
              if (required)
                Text(
                  ' *',
                  style: TextStyle(
                    color: theme.colorScheme.error,
                    fontSize: theme.textTheme.labelLarge?.fontSize,
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
        ],
        DropdownButtonFormField<T>(
          value: value,
          items: items,
          onChanged: enabled ? onChanged : null,
          validator: validator,
          onSaved: onSaved,
          decoration: InputDecoration(
            hintText: hint,
            prefixIcon: prefixIcon,
          ),
        ),
      ],
    );
  }
}

/// Checkbox form field with consistent styling
class CustomCheckboxField extends StatelessWidget {
  final String? label;
  final String? subtitle;
  final bool? value;
  final void Function(bool?)? onChanged;
  final bool enabled;
  final Widget? leading;
  final bool required;

  const CustomCheckboxField({
    super.key,
    this.label,
    this.subtitle,
    this.value,
    this.onChanged,
    this.enabled = true,
    this.leading,
    this.required = false,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return CheckboxListTile(
      title: label != null 
          ? Row(
              children: [
                Expanded(child: Text(label!)),
                if (required)
                  Text(
                    ' *',
                    style: TextStyle(color: theme.colorScheme.error),
                  ),
              ],
            )
          : null,
      subtitle: subtitle != null ? Text(subtitle!) : null,
      value: value,
      onChanged: enabled ? onChanged : null,
      secondary: leading,
      contentPadding: EdgeInsets.zero,
      controlAffinity: ListTileControlAffinity.leading,
    );
  }
}

/// Radio group form field
class CustomRadioGroupField<T> extends StatelessWidget {
  final String? label;
  final T? value;
  final List<RadioOption<T>> options;
  final void Function(T?)? onChanged;
  final bool enabled;
  final bool required;
  final Axis direction;

  const CustomRadioGroupField({
    super.key,
    this.label,
    this.value,
    required this.options,
    this.onChanged,
    this.enabled = true,
    this.required = false,
    this.direction = Axis.vertical,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (label != null) ...[
          Row(
            children: [
              Text(
                label!,
                style: theme.textTheme.labelLarge?.copyWith(
                  fontWeight: FontWeight.w500,
                ),
              ),
              if (required)
                Text(
                  ' *',
                  style: TextStyle(
                    color: theme.colorScheme.error,
                    fontSize: theme.textTheme.labelLarge?.fontSize,
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
        ],
        if (direction == Axis.vertical)
          ...options.map((option) => RadioListTile<T>(
                title: Text(option.label),
                subtitle: option.subtitle != null ? Text(option.subtitle!) : null,
                value: option.value,
                groupValue: value,
                onChanged: enabled ? onChanged : null,
                contentPadding: EdgeInsets.zero,
              ))
        else
          Wrap(
            children: options.map((option) => Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Radio<T>(
                  value: option.value,
                  groupValue: value,
                  onChanged: enabled ? onChanged : null,
                ),
                Text(option.label),
                const SizedBox(width: 16),
              ],
            )).toList(),
          ),
      ],
    );
  }
}

/// Radio option model
class RadioOption<T> {
  final T value;
  final String label;
  final String? subtitle;

  const RadioOption({
    required this.value,
    required this.label,
    this.subtitle,
  });
}

/// Date picker form field
class CustomDateField extends StatelessWidget {
  final String? label;
  final String? hint;
  final DateTime? value;
  final void Function(DateTime?)? onChanged;
  final String? Function(DateTime?)? validator;
  final void Function(DateTime?)? onSaved;
  final bool enabled;
  final bool required;
  final DateTime? firstDate;
  final DateTime? lastDate;
  final Widget? prefixIcon;

  const CustomDateField({
    super.key,
    this.label,
    this.hint,
    this.value,
    this.onChanged,
    this.validator,
    this.onSaved,
    this.enabled = true,
    this.required = false,
    this.firstDate,
    this.lastDate,
    this.prefixIcon,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (label != null) ...[
          Row(
            children: [
              Text(
                label!,
                style: theme.textTheme.labelLarge?.copyWith(
                  fontWeight: FontWeight.w500,
                ),
              ),
              if (required)
                Text(
                  ' *',
                  style: TextStyle(
                    color: theme.colorScheme.error,
                    fontSize: theme.textTheme.labelLarge?.fontSize,
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
        ],
        TextFormField(
          readOnly: true,
          enabled: enabled,
          controller: TextEditingController(
            text: value != null 
                ? '${value!.day}/${value!.month}/${value!.year}'
                : '',
          ),
          decoration: InputDecoration(
            hintText: hint ?? 'Select date',
            prefixIcon: prefixIcon ?? const Icon(Icons.calendar_today),
            suffixIcon: const Icon(Icons.arrow_drop_down),
          ),
          onTap: enabled ? () async {
            final date = await showDatePicker(
              context: context,
              initialDate: value ?? DateTime.now(),
              firstDate: firstDate ?? DateTime(1900),
              lastDate: lastDate ?? DateTime(2100),
            );
            if (date != null) {
              onChanged?.call(date);
            }
          } : null,
          validator: (textValue) {
            return validator?.call(value);
          },
          onSaved: (textValue) {
            onSaved?.call(value);
          },
        ),
      ],
    );
  }
}