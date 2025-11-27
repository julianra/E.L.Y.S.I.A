// lib/main.dart
// Main entry point for the Orbit Flutter application
// Handles server selection and navigation to HomeScreen
// Uses ServerSelectScreen for server discovery
//
import 'package:flutter/material.dart';
import 'screens/home_screen.dart';
import 'screens/server_select_screen.dart';

void main() {
  runApp(const OrbitApp());
}

class OrbitApp extends StatefulWidget {
  const OrbitApp({super.key});

  @override
  State<OrbitApp> createState() => _OrbitAppState();
}

class _OrbitAppState extends State<OrbitApp> {
  String? baseUrl;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Orbit',
      theme: ThemeData.dark(),
      home: baseUrl == null
          ? Builder(builder: (context) {
              Future(() async {
                final selected = await Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (_) => const ServerSelectScreen(),
                  ),
                );
                if (selected != null) {
                  setState(() => baseUrl = selected);
                }
              });

              return const Scaffold(
                body: Center(child: CircularProgressIndicator()),
              );
            })
          : HomeScreen(apiBaseUrl: baseUrl!),
    );
  }
}
