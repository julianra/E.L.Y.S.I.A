//lib/widgets/status_card.dart
import 'package:flutter/material.dart';

class StatusCard extends StatelessWidget {
  final String status;
  final bool loading;

  const StatusCard({
    super.key,
    required this.status,
    required this.loading,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 3,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            const Icon(Icons.memory, size: 40),
            const SizedBox(width: 16),
            Expanded(
              child: Text(
                loading ? "Checking..." : "Kernel status: $status",
                style: const TextStyle(fontSize: 18),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
