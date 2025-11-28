import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class TaskDetailsPage extends StatefulWidget {
  final AgendaItem item;

  const TaskDetailsPage({super.key, required this.item});

  @override
  State<TaskDetailsPage> createState() => _TaskDetailsPageState();
}

class _TaskDetailsPageState extends State<TaskDetailsPage> {
  late TextEditingController nameController;

  @override
  void initState() {
    super.initState();
    nameController = TextEditingController(text: widget.item.name);
  }

  Future<void> _delete() async {
    final api = await ElysiaService.get();
    await api.deleteTask(widget.item.id);
    Navigator.pop(context, true);
  }

  Future<void> _save() async {
    final api = await ElysiaService.get();
    await api.updateTask(widget.item.id, {
      "name": nameController.text,
    });

    Navigator.pop(context, true);
  }

  @override
  Widget build(BuildContext context) {
    final item = widget.item;

    return Scaffold(
      appBar: AppBar(
        title: const Text("Taak Details"),
        actions: [
          IconButton(
            icon: const Icon(Icons.delete),
            onPressed: _delete,
          )
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              controller: nameController,
              decoration: const InputDecoration(labelText: "Titel"),
            ),

            const SizedBox(height: 20),
            Text("Start: ${item.start}"),
            Text("Einde: ${item.end}"),
            Text("Duur: ${item.durationMinutes} min"),

            const Spacer(),
            ElevatedButton(
              onPressed: _save,
              child: const Text("Wijzigingen opslaan"),
            )
          ],
        ),
      ),
    );
  }
}
