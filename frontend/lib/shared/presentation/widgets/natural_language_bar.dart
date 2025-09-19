import 'package:flutter/material.dart';

/// Natural language interface bar for the application
class NaturalLanguageBar extends StatefulWidget {
  final String? placeholder;
  final Function(String)? onSubmitted;
  final bool enabled;

  const NaturalLanguageBar({
    super.key,
    this.placeholder,
    this.onSubmitted,
    this.enabled = true,
  });

  @override
  State<NaturalLanguageBar> createState() => _NaturalLanguageBarState();
}

class _NaturalLanguageBarState extends State<NaturalLanguageBar> {
  final TextEditingController _controller = TextEditingController();
  bool _isListening = false;

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _handleSubmit() {
    final text = _controller.text.trim();
    if (text.isNotEmpty && widget.onSubmitted != null) {
      widget.onSubmitted!(text);
      _controller.clear();
    }
  }

  void _toggleListening() {
    setState(() {
      _isListening = !_isListening;
    });
    
    // TODO: Implement speech recognition
    if (_isListening) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Speech recognition not yet implemented'),
          duration: Duration(seconds: 2),
        ),
      );
      setState(() {
        _isListening = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: theme.colorScheme.surface,
        border: Border(
          bottom: BorderSide(
            color: theme.dividerColor,
            width: 1,
          ),
        ),
      ),
      child: Row(
        children: [
          // Voice input button
          IconButton(
            onPressed: widget.enabled ? _toggleListening : null,
            icon: Icon(
              _isListening ? Icons.mic : Icons.mic_none,
              color: _isListening 
                  ? theme.colorScheme.primary 
                  : theme.colorScheme.onSurface.withOpacity(0.6),
            ),
            tooltip: 'Voice input',
          ),
          
          const SizedBox(width: 8),
          
          // Text input field
          Expanded(
            child: TextField(
              controller: _controller,
              enabled: widget.enabled,
              decoration: InputDecoration(
                hintText: widget.placeholder ?? 'What would you like to do?',
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(24),
                  borderSide: BorderSide.none,
                ),
                filled: true,
                fillColor: theme.colorScheme.surfaceVariant.withOpacity(0.5),
                contentPadding: const EdgeInsets.symmetric(
                  horizontal: 20,
                  vertical: 12,
                ),
                suffixIcon: _controller.text.isNotEmpty
                    ? IconButton(
                        onPressed: () {
                          _controller.clear();
                          setState(() {});
                        },
                        icon: const Icon(Icons.clear),
                      )
                    : null,
              ),
              textInputAction: TextInputAction.send,
              onSubmitted: (_) => _handleSubmit(),
              onChanged: (_) => setState(() {}),
            ),
          ),
          
          const SizedBox(width: 8),
          
          // Send button
          IconButton(
            onPressed: widget.enabled && _controller.text.trim().isNotEmpty 
                ? _handleSubmit 
                : null,
            icon: Icon(
              Icons.send,
              color: _controller.text.trim().isNotEmpty 
                  ? theme.colorScheme.primary 
                  : theme.colorScheme.onSurface.withOpacity(0.4),
            ),
            tooltip: 'Send',
          ),
        ],
      ),
    );
  }
}