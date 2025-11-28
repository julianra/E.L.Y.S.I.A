// lib/pages/agenda_views/week_view.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class WeekView extends StatelessWidget {
  final Function(DateTime)? onCreateAtDay;

  const WeekView({super.key, this.onCreateAtDay});

  Future<List<AgendaItem>> _loadWeek(DateTime weekStart) async {
    final api = await ElysiaService.get();
    final raw = await api.getTasks();
    return raw.map((e) => AgendaItem.fromJson(e)).toList();
  }

  @override
  Widget build(BuildContext context) {
    final now = DateTime.now();
    final monday = now.subtract(Duration(days: now.weekday - 1)); // maandag = start week

    return FutureBuilder(
      future: _loadWeek(monday),
      builder: (context, snap) {
        if (!snap.hasData) {
          return const Center(child: CircularProgressIndicator());
        }

        final tasks = snap.data!;
        final days = List.generate(7, (i) => monday.add(Duration(days: i)));

        return SingleChildScrollView(
          padding: const EdgeInsets.all(12),
          child: Column(
            children: [
              // 🔵 Titel van de week
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Text(
                  "Week ${_weekNumber(monday)} - ${monday.year}",
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),

              // 🔵 4 dagen bovenaan
              Row(
                children: _buildDayCards(context, tasks, days.sublist(0, 4)),
              ),
              const SizedBox(height: 12),

              // 🔵 3 dagen onderaan
              Row(
                children: _buildDayCards(context, tasks, days.sublist(4, 7)),
              ),
            ],
          ),
        );
      },
    );
  }

  // -----------------------------------------------------------
  // Build day cards (flex 4 in row)
  // -----------------------------------------------------------
  List<Widget> _buildDayCards(
    BuildContext context,
    List<AgendaItem> tasks,
    List<DateTime> days,
  ) {
    return days.map((day) {
      final dayTasks = tasks.where((t) =>
        t.start.year == day.year &&
        t.start.month == day.month &&
        t.start.day == day.day
      ).toList();

      return Expanded(
        child: GestureDetector(
          onTap: () => onCreateAtDay?.call(day),
          child: Container(
            height: 140,
            margin: const EdgeInsets.symmetric(horizontal: 6),
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: Colors.blueGrey.shade900,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.blueGrey.shade700),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.2),
                  blurRadius: 6,
                  offset: const Offset(0, 2),
                ),
              ],
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // 🔵 Dag titel (bv: MA 28)
                Text(
                  _weekdayShort(day.weekday) + " ${day.day}",
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    fontSize: 16,
                  ),
                ),

                const SizedBox(height: 6),

                // 🔵 Toon max 3 taken
                for (var t in dayTasks.take(3))
                  Container(
                    margin: const EdgeInsets.only(bottom: 4),
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    decoration: BoxDecoration(
                      color: Colors.blue.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: Text(
                      t.name,
                      style: const TextStyle(fontSize: 11),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),

                // 🔵 Extra label (meer taken)
                if (dayTasks.length > 3)
                  Text(
                    "+${dayTasks.length - 3} extra",
                    style: TextStyle(
                      fontSize: 11,
                      color: Colors.grey.shade400,
                    ),
                  ),
              ],
            ),
          ),
        ),
      );
    }).toList();
  }

  // -----------------------------------------------------------
  // Helpers
  // -----------------------------------------------------------
  String _weekdayShort(int weekday) {
    const days = ["MA", "DI", "WO", "DO", "VR", "ZA", "ZO"];
    return days[weekday - 1];
  }

  int _weekNumber(DateTime date) {
    final firstDay = DateTime(date.year, 1, 1);
    final diff = date.difference(firstDay).inDays;
    return ((diff) / 7).floor() + 1;
  }
}
