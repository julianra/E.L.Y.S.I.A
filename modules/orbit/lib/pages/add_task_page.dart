//lib/pages/add_task_page.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';

class AddTaskPage extends StatefulWidget {
  final DateTime? prefilledStart;

  const AddTaskPage({super.key, this.prefilledStart});

  @override
  State<AddTaskPage> createState() => _AddTaskPageState();
}

class _AddTaskPageState extends State<AddTaskPage> {
  final titleController = TextEditingController();
  DateTime? start;
  DateTime? end;

  @override
  void initState() {
    super.initState();
    start = widget.prefilledStart;
  }

  Future<void> _submit() async {
    if (titleController.text.isEmpty) return;

    final api = await ElysiaService.get();
await api.addTask({
  "name": titleController.text,
  if (start != null) "exact_start": "${start!.toIso8601String()}Z",
  if (end != null) "exact_end": "${end!.toIso8601String()}Z",
});


    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Nieuwe taak")),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            TextField(
              controller: titleController,
              decoration: const InputDecoration(labelText: "Titel (verplicht)"),
            ),

            const SizedBox(height: 15),
            Row(
              children: [
                ElevatedButton(
                  onPressed: () async {
                    final picked = await showDatePicker(
                      context: context,
                      initialDate: start ?? DateTime.now(),
                      firstDate: DateTime(2024),
                      lastDate: DateTime(2030),
                    );
                    if (picked == null) return;

                    final time = await showTimePicker(
                      context: context,
                      initialTime: const TimeOfDay(hour: 12, minute: 0),
                    );

                    setState(() {
                      start = DateTime(
                        picked.year,
                        picked.month,
                        picked.day,
                        time?.hour ?? 0,
                        time?.minute ?? 0,
                      );
                    });
                  },
                  style: ElevatedButton.styleFrom(backgroundColor: Colors.blue),
                  child: const Text("Kies start"),
                ),

                const SizedBox(width: 12),

                Text(start == null ? "Geen start" : start.toString()),
              ],
            ),

            const SizedBox(height: 15),
            ElevatedButton(
              onPressed: _submit,
              style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
              child: const Text("Opslaan"),
            )
          ],
        ),
      ),
    );
  }
}
