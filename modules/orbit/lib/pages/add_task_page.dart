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

  DateTime? exactStart;
  DateTime? exactEnd;
  DateTime? deadlineEnd;
  int? duration;

  @override
  void initState() {
    super.initState();
    exactStart = widget.prefilledStart;
  }

  String _format(DateTime? dt) {
    if (dt == null) return "Niet ingesteld";
    return "${dt.year}-${_two(dt.month)}-${_two(dt.day)} "
           "${_two(dt.hour)}:${_two(dt.minute)}";
  }

  String _iso(DateTime dt) => dt.toIso8601String();

  Future<DateTime?> _pickDateTime(DateTime initial) async {
    final date = await showDatePicker(
      context: context,
      initialDate: initial,
      firstDate: DateTime(2024),
      lastDate: DateTime(2035),
    );
    if (date == null) return null;

    final time = await showTimePicker(
      context: context,
      initialTime: TimeOfDay.fromDateTime(initial),
    );

    return DateTime(
      date.year,
      date.month,
      date.day,
      time?.hour ?? 0,
      time?.minute ?? 0,
    );
  }

  Future<void> _submit() async {
    if (titleController.text.isEmpty) return;

    final payload = {
      "name": titleController.text,
      if (exactStart != null) "exact_start": _iso(exactStart!),
      if (exactEnd != null) "exact_end": _iso(exactEnd!),
      if (deadlineEnd != null) "deadline_end": _iso(deadlineEnd!),
      if (duration != null) "duration_minutes": duration,
    };

    final api = await ElysiaService.get();
    await api.addTask(payload);

    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Nieuwe Taak"),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            controller: titleController,
            decoration: const InputDecoration(
              labelText: "Titel",
              border: OutlineInputBorder(),
            ),
          ),

          const SizedBox(height: 20),

          // START TIJD ---------------------------------------------------
          _buildTile(
            "Starttijd",
            _format(exactStart),
            onTap: () async {
              final res = await _pickDateTime(exactStart ?? DateTime.now());
              if (res != null) setState(() => exactStart = res);
            },
          ),

          const SizedBox(height: 12),

          // END TIJD -----------------------------------------------------
          _buildTile(
            "Eindtijd",
            _format(exactEnd),
            onTap: () async {
              final res = await _pickDateTime(exactEnd ?? DateTime.now());
              if (res != null) setState(() => exactEnd = res);
            },
          ),

          const SizedBox(height: 12),

          // DEADLINE -----------------------------------------------------
          _buildTile(
            "Deadline",
            _format(deadlineEnd),
            onTap: () async {
              final res = await _pickDateTime(deadlineEnd ?? DateTime.now());
              if (res != null) setState(() => deadlineEnd = res);
            },
          ),

          const SizedBox(height: 12),

          // DURATION -----------------------------------------------------
          TextField(
            decoration: const InputDecoration(
              labelText: "Duur (minuten)",
              border: OutlineInputBorder(),
            ),
            keyboardType: TextInputType.number,
            onChanged: (v) {
              duration = int.tryParse(v);
            },
          ),

          const SizedBox(height: 30),

          ElevatedButton(
            onPressed: _submit,
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.green,
              padding: const EdgeInsets.symmetric(vertical: 16),
            ),
            child: const Text("Opslaan"),
          ),
        ],
      ),
    );
  }

  Widget _buildTile(String title, String value, {required VoidCallback onTap}) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.all(14),
        decoration: BoxDecoration(
          color: Colors.white10,
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: Colors.white24),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(title, style: const TextStyle(fontSize: 16)),
            Text(value,
                style:
                    const TextStyle(fontSize: 16, color: Colors.white70)),
          ],
        ),
      ),
    );
  }

  String _two(int v) => v.toString().padLeft(2, '0');
}
