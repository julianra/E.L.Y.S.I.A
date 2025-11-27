// lib/screens/server_select_screen.dart
// Screen to discover and select Elysia API servers on the local network
// Uses ElysiaDiscovery from api/discovery.dart
// Returns the selected server's base URL to the previous screen
// Usage: Navigator.push to this screen, await the result
// 
import 'package:flutter/material.dart';
import '../api/discovery.dart';

class ServerSelectScreen extends StatefulWidget {
  final Function(String) onSelected;

  const ServerSelectScreen({super.key, required this.onSelected});

  @override
  State<ServerSelectScreen> createState() => _ServerSelectScreenState();
}

class _ServerSelectScreenState extends State<ServerSelectScreen> {
  List<String> servers = [];
  bool loading = false;

  @override
  void initState() {
    super.initState();
    _discover();
  }

  Future<void> _discover() async {
    setState(() => loading = true);

    servers = await ElysiaDiscovery.discover();

    setState(() => loading = false);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Selecteer een Elysia Server")),
      body: Column(
        children: [
          if (loading) const LinearProgressIndicator(),

          if (!loading && servers.isEmpty)
            const Padding(
              padding: EdgeInsets.all(16),
              child: Text("Geen Elysia servers gevonden"),
            ),

          Expanded(
            child: ListView(
              children: servers
                  .map(
                    (s) => ListTile(
                      title: Text(s),
                      tileColor: Colors.white10,
                      onTap: () {
                        widget.onSelected(s);
                      },
                    ),
                  )
                  .toList(),
            ),
          ),

          const SizedBox(height: 10),

          ElevatedButton(
            onPressed: _discover,
            child: const Text("Opnieuw zoeken"),
          ),

          const SizedBox(height: 20),
        ],
      ),
    );
  }
}
