// lib/screens/server_select_screen.dart
// Screen to discover and select Elysia API servers on the local network
// Uses ElysiaDiscovery from api/discovery.dart
// Returns the selected server's base URL to the previous screen
// Usage: Navigator.push to this screen, await the result
// 
import 'package:flutter/material.dart';
import '../api/discovery.dart';

class ServerSelectScreen extends StatefulWidget {
  const ServerSelectScreen({super.key});

  @override
  State<ServerSelectScreen> createState() => _ServerSelectScreenState();
}

class _ServerSelectScreenState extends State<ServerSelectScreen> {
  List<String> servers = [];
  bool loading = true;

  @override
  void initState() {
    super.initState();
    findServers();
  }

  void findServers() async {
    final result = await ElysiaDiscovery.discover();
    setState(() {
      servers = result;
      loading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    return Scaffold(
      appBar: AppBar(title: const Text("Selecteer ELYSIA Server")),
      body: servers.isEmpty
          ? const Center(child: Text("Geen ELYSIA servers gevonden"))
          : ListView.builder(
              itemCount: servers.length,
              itemBuilder: (_, i) {
                return ListTile(
                  title: Text(servers[i]),
                  onTap: () => Navigator.pop(context, servers[i]),
                );
              },
            ),
    );
  }
}
