// lib/pages/agenda_views/day_view.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class DayView extends StatefulWidget {
  final Function(DateTime) onCreateAtHour;

  const DayView({super.key, required this.onCreateAtHour});

  @override
  State<DayView> createState() => _DayViewState();
}

class _DayViewState extends State<DayView> {
  DateTime current = DateTime.now();

  Future<List<AgendaItem>> _load() async {
    final api = await ElysiaService.get();
    final raw = await api.getTasks();
    return raw.map((e) => AgendaItem.fromJson(e)).toList();
  }

  void _nextDay() => setState(() => current = current.add(const Duration(days: 1)));
  void _previousDay() => setState(() => current = current.subtract(const Duration(days: 1)));

  @override
  Widget build(BuildContext context) {
    final title = "${_weekday(current.weekday)} ${current.day}/${current.month}";

    return Column(
      children: [
        //-------------------- HEADER --------------------
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              IconButton(onPressed: _previousDay, icon: const Icon(Icons.chevron_left)),
              Text(
                title,
                style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              IconButton(onPressed: _nextDay, icon: const Icon(Icons.chevron_right)),
            ],
          ),
        ),

        //-------------------- CONTENT --------------------
        Expanded(
          child: FutureBuilder<List<AgendaItem>>(
            future: _load(),
            builder: (context, snapshot) {
              if (!snapshot.hasData) return const Center(child: CircularProgressIndicator());

              final tasks = snapshot.data!
                  .where((t) =>
                      t.start.year == current.year &&
                      t.start.month == current.month &&
                      t.start.day == current.day)
                  .toList()
                ..sort((a, b) => a.start.compareTo(b.start));

              return ListView(
                padding: const EdgeInsets.all(16),
                children: [
                  // Geen taken → klik om toe te voegen
                  if (tasks.isEmpty)
                    GestureDetector(
                      onTap: () => widget.onCreateAtHour(current),
                      child: Container(
                        padding: const EdgeInsets.all(20),
                        decoration: BoxDecoration(
                          color: Colors.white10,
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(color: Colors.white24),
                        ),
                        child: const Center(
                          child: Text(
                            "Geen taken — tik om toe te voegen",
                            style: TextStyle(color: Colors.white70),
                          ),
                        ),
                      ),
                    ),

                  // Taken
                  for (var task in tasks) _buildTask(context, task),

                  const SizedBox(height: 40),
                ],
              );
            },
          ),
        ),
      ],
    );
  }

  //------------------- TASK TILE -------------------
  Widget _buildTask(BuildContext context, AgendaItem item) {
    final start = "${_two(item.start.hour)}:${_two(item.start.minute)}";
    final end = "${_two(item.end.hour)}:${_two(item.end.minute)}";

    return GestureDetector(
      onTap: () {
        Navigator.pushNamed(
          context,
          "/task-details",
          arguments: item,
        ).then((_) => setState(() {}));
      },
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: Colors.blue.shade600,
          borderRadius: BorderRadius.circular(10),
        ),
        child: Row(
          children: [
            const Icon(Icons.access_time, size: 20, color: Colors.white),
            const SizedBox(width: 10),
            Expanded(
              child: Text(
                "${item.name}\n$start – $end",
                style: const TextStyle(color: Colors.white, fontSize: 16),
              ),
            ),
          ],
        ),
      ),
    );
  }

  //------------------- HELPERS -------------------
  String _weekday(int w) {
    const d = ["MA", "DI", "WO", "DO", "VR", "ZA", "ZO"];
    return d[w - 1];
  }

  String _two(int v) => v.toString().padLeft(2, '0');
}
