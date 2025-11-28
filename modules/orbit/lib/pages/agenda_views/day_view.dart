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
  DateTime currentDate = DateTime.now();

  Future<List<AgendaItem>> _load() async {
    final api = await ElysiaService.get();
    final raw = await api.getTasks();
    return raw.map((e) => AgendaItem.fromJson(e)).toList();
  }

  void _nextDay() => setState(() => currentDate = currentDate.add(const Duration(days: 1)));
  void _previousDay() => setState(() => currentDate = currentDate.subtract(const Duration(days: 1)));

  @override
  Widget build(BuildContext context) {
    final dateLabel = "${currentDate.day}/${currentDate.month}/${currentDate.year}";

    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            IconButton(onPressed: _previousDay, icon: const Icon(Icons.chevron_left)),
            Text("Dagweergave – $dateLabel", style: const TextStyle(fontSize: 17, fontWeight: FontWeight.bold)),
            IconButton(onPressed: _nextDay, icon: const Icon(Icons.chevron_right)),
          ],
        ),

        Expanded(
          child: FutureBuilder<List<AgendaItem>>(
            future: _load(),
            builder: (context, snapshot) {
              if (!snapshot.hasData) {
                return const Center(child: CircularProgressIndicator());
              }

              final tasks = snapshot.data!.where((t) =>
                t.start.year == currentDate.year &&
                t.start.month == currentDate.month &&
                t.start.day == currentDate.day
              ).toList();

              return ListView.builder(
                itemCount: 24,
                itemBuilder: (context, hour) {
                  final hourTasks = tasks.where((t) => t.start.hour == hour).toList();

                  return GestureDetector(
                    onTap: () {
                      final dt = DateTime(
                        currentDate.year,
                        currentDate.month,
                        currentDate.day,
                        hour,
                      );
                      widget.onCreateAtHour(dt);
                    },
                    child: Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        border: Border(
                          bottom: BorderSide(color: Colors.grey.shade300),
                        ),
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text("$hour:00", style: const TextStyle(fontWeight: FontWeight.bold)),
                          for (var task in hourTasks)
                            Container(
                              margin: const EdgeInsets.only(top: 6),
                              padding: const EdgeInsets.all(8),
                              decoration: BoxDecoration(
                                color: Colors.blue.shade100,
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Text(task.name),
                            ),
                        ],
                      ),
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
