// lib/pages/home_dashboard.dart
import 'package:flutter/material.dart';

class HomeDashboard extends StatelessWidget {
  const HomeDashboard({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Orbit Dashboard"),
        centerTitle: true,
      ),
      body: ListView(
        padding: const EdgeInsets.all(20),
        children: [
          const Text(
            "Welkom terug, Julian 👋",
            style: TextStyle(fontSize: 26, fontWeight: FontWeight.bold),
          ),

          const SizedBox(height: 20),

          // STATUS CARD
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: const [
                  Icon(Icons.cloud_done, size: 40, color: Colors.greenAccent),
                  SizedBox(width: 20),
                  Expanded(
                    child: Text(
                      "Elysia is actief en verbonden",
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
                    ),
                  )
                ],
              ),
            ),
          ),

          const SizedBox(height: 20),

          // AGENDA PREVIEW
          const Text(
            "Je volgende taken",
            style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
          ),

          const SizedBox(height: 10),

          Card(
            child: ListTile(
              title: const Text("🧹 Huiskamer opruimen"),
              subtitle: const Text("Vandaag 18:00 – 30 min"),
              trailing: const Icon(Icons.chevron_right),
            ),
          ),

          Card(
            child: ListTile(
              title: const Text("🏋️‍♂️ Fitness met Marthe"),
              subtitle: const Text("Morgen 10:00"),
              trailing: const Icon(Icons.chevron_right),
            ),
          ),

          const SizedBox(height: 30),
        ],
      ),
    );
  }
}
