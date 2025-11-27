import 'package:flutter/material.dart';
import '../api/elysia_api.dart';
import '../widgets/status_card.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  // 🔥 Zet dit IP juist (van je PC)
  final api = ElysiaApi(baseUrl: "http://192.168.0.118:3000");

  String kernelStatus = "Unknown";
  bool loading = false;
  final taskController = TextEditingController();

  void checkHealth() async {
    setState(() => loading = true);

    try {
      final status = await api.health();
      setState(() => kernelStatus = status);
    } catch (e) {
      setState(() => kernelStatus = "Error");
    }

    setState(() => loading = false);
  }

  void addTask() async {
    final name = taskController.text.trim();
    if (name.isEmpty) return;

    try {
      await api.addTask(name);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text("Task \"$name\" added to Marthe.")),
      );
      taskController.clear();
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text("Error: $e")),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Orbit")),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            StatusCard(status: kernelStatus, loading: loading),

            const SizedBox(height: 20),

            TextField(
              controller: taskController,
              decoration: const InputDecoration(
                labelText: "New task name",
                border: OutlineInputBorder(),
              ),
            ),

            const SizedBox(height: 10),

            ElevatedButton(
              onPressed: addTask,
              child: const Text("Add Task to Marthe"),
            ),

            const SizedBox(height: 20),

            ElevatedButton(
              onPressed: checkHealth,
              child: const Text("Check Kernel Status"),
            ),
          ],
        ),
      ),
    );
  }
}
