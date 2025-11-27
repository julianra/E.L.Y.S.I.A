// lib/pages/settings_page.dart
import 'package:flutter/material.dart';
import '../storage/server_storage.dart';

class SettingsPage extends StatelessWidget {
  SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Instellingen")),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              "Server",
              style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 10),
            ElevatedButton.icon(
              onPressed: () async {
                await ServerStorage.clear();
                Navigator.pushNamedAndRemoveUntil(
                    context, "/select-server", (_) => false);
              },
              icon: const Icon(Icons.logout),
              label: const Text("Vergeet server / Log uit"),
            )
          ],
        ),
      ),
    );
  }
}
