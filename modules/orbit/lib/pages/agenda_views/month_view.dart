import 'dart:math';
import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class MonthView extends StatefulWidget {
  const MonthView({super.key});

  @override
  State<MonthView> createState() => _MonthViewState();
}

class _MonthViewState extends State<MonthView> {
  DateTime focus = DateTime.now();

  void _nextMonth() =>
      setState(() => focus = DateTime(focus.year, focus.month + 1, 1));

  void _prevMonth() =>
      setState(() => focus = DateTime(focus.year, focus.month - 1, 1));

  Future<List<AgendaItem>> _load() async {
    final api = await ElysiaService.get();
    final raw = await api.getTasks();
    return raw.map((e) => AgendaItem.fromJson(e)).toList();
  }

  @override
  Widget build(BuildContext context) {
    final label = "${focus.month}/${focus.year}";
    final daysInMonth = DateTime(focus.year, focus.month + 1, 0).day;

    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            IconButton(onPressed: _prevMonth, icon: const Icon(Icons.chevron_left)),
            Text(label, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18)),
            IconButton(onPressed: _nextMonth, icon: const Icon(Icons.chevron_right)),
          ],
        ),

        Expanded(
          child: FutureBuilder<List<AgendaItem>>(
            future: _load(),
            builder: (context, snapshot) {
              if (!snapshot.hasData) return const Center(child: CircularProgressIndicator());

              final tasks = snapshot.data!;

              return GridView.builder(
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: 7,
                ),
                itemCount: daysInMonth,
                itemBuilder: (context, index) {
                  final dayNumber = index + 1;
                  final dayTasks = tasks.where((t) =>
                    t.start.year == focus.year &&
                    t.start.month == focus.month &&
                    t.start.day == dayNumber
                  ).toList();

                  final random3 = (dayTasks.toList()..shuffle()).take(3).toList();

                  return Container(
                    margin: const EdgeInsets.all(3),
                    padding: const EdgeInsets.all(6),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(6),
                      border: Border.all(color: Colors.grey.shade300),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text("$dayNumber", style: const TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 4),
                        for (var t in random3)
                          Text(
                            "• ${t.name}",
                            style: const TextStyle(fontSize: 11),
                            overflow: TextOverflow.ellipsis,
                          ),
                      ],
                    ),
                  );
                },
              );
            },
          ),
        ),
      ],
    );
  }
}
