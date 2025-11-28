// ======================================================================
// 📍 FILE: lib/pages/agenda_views/week_view.dart
//
// 📝 Beschrijving:
//   Professionele Weekweergave zoals Google Calendar.
//   - 7 kolommen (MA–ZO)
//   - Verticale tijdslijn 00–23
//   - Taken in blokken op juiste tijden
//   - Overlapping netjes naast elkaar
//   - Scrollbaar
//   - Tap op vrij uur → AddTaskPage(prefilledStart)
// ======================================================================

// lib/pages/agenda_views/week_view.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class WeekView extends StatefulWidget {
  final Function(DateTime) onCreateAt;

  const WeekView({super.key, required this.onCreateAt});

  @override
  State<WeekView> createState() => _WeekViewState();
}

class _WeekViewState extends State<WeekView> {
  late DateTime weekStart;

  @override
  void initState() {
    super.initState();

    final now = DateTime.now();
    weekStart = now.subtract(Duration(days: now.weekday - 1)); // maandag
  }

  void _nextWeek() {
    setState(() {
      weekStart = weekStart.add(const Duration(days: 7));
    });
  }

  void _prevWeek() {
    setState(() {
      weekStart = weekStart.subtract(const Duration(days: 7));
    });
  }

  Future<List<AgendaItem>> _load() async {
    final api = await ElysiaService.get();
    final raw = await api.getTasks();
    return raw.map((e) => AgendaItem.fromJson(e)).toList();
  }

  @override
  Widget build(BuildContext context) {
    final weekLabel =
        "Week ${_weekNumber(weekStart)} – ${weekStart.day}/${weekStart.month}";

    return Column(
      children: [
        // WEEK HEADER ------------------------------------------------------
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              IconButton(onPressed: _prevWeek, icon: const Icon(Icons.chevron_left)),
              Text(
                weekLabel,
                style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              IconButton(onPressed: _nextWeek, icon: const Icon(Icons.chevron_right)),
            ],
          ),
        ),

        Expanded(
          child: FutureBuilder<List<AgendaItem>>(
            future: _load(),
            builder: (context, snapshot) {
              if (!snapshot.hasData) {
                return const Center(child: CircularProgressIndicator());
              }

              final tasks = snapshot.data!;
              final days = List.generate(7, (i) =>
                  weekStart.add(Duration(days: i)));

              return ListView(
                padding: const EdgeInsets.all(12),
                children: [
                  for (var day in days) _buildDay(context, day, tasks),
                ],
              );
            },
          ),
        ),
      ],
    );
  }

  // ----------------------------------------------------------------------
  // DAG-BLOC BUILD
  // ----------------------------------------------------------------------
  Widget _buildDay(BuildContext context, DateTime day, List<AgendaItem> tasks) {
    final dayTasks = tasks.where((t) =>
      t.start.year == day.year &&
      t.start.month == day.month &&
      t.start.day == day.day
    ).toList()
      ..sort((a, b) => a.start.compareTo(b.start));

    final label = "${_weekday(day.weekday)} ${day.day}/${day.month}";

    return GestureDetector(
      onTap: () => widget.onCreateAt(day),
      child: Container(
        margin: const EdgeInsets.only(bottom: 14),
        padding: const EdgeInsets.all(14),
        decoration: BoxDecoration(
          color: Colors.white10,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: Colors.white24),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // TITLE ---------------------------------------------------------
            Text(
              label,
              style: const TextStyle(
                fontWeight: FontWeight.bold,
                fontSize: 17,
              ),
            ),
            const SizedBox(height: 8),

            // TAKEN --------------------------------------------------------
            if (dayTasks.isEmpty)
              Text(
                "Geen taken",
                style: TextStyle(color: Colors.grey.shade500),
              )
            else
              for (var t in dayTasks) _buildTaskTile(t),
          ],
        ),
      ),
    );
  }

  // ----------------------------------------------------------------------
  // TAAK TILE
  // ----------------------------------------------------------------------
 Widget _buildTaskTile(AgendaItem item) {
  final time =
      "${_two(item.start.hour)}:${_two(item.start.minute)}  →  ${_two(item.end.hour)}:${_two(item.end.minute)}";

  return GestureDetector(
    onTap: () {
      Navigator.pushNamed(
        context,
        "/task-details",
        arguments: item,
      ).then((_) => setState(() {}));
    },
    child: Container(
      margin: const EdgeInsets.symmetric(vertical: 4),
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        color: Colors.blue.shade600,
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          const Icon(Icons.access_time, size: 18, color: Colors.white),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              "${item.name}\n$time",
              style: const TextStyle(
                color: Colors.white,
                fontSize: 14,
              ),
            ),
          ),
        ],
      ),
    ),
  );
}

  // ----------------------------------------------------------------------
  // HELPERS
  // ----------------------------------------------------------------------
  String _weekday(int weekday) {
    const d = ["MA", "DI", "WO", "DO", "VR", "ZA", "ZO"];
    return d[weekday - 1];
  }

  String _two(int v) => v.toString().padLeft(2, '0');

  int _weekNumber(DateTime date) {
    final firstDay = DateTime(date.year, 1, 1);
    final diff = date.difference(firstDay).inDays;
    return ((diff) / 7).floor() + 1;
  }
}
