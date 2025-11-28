// lib/pages/agenda_views/month_view.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/models/agenda_item.dart';

class MonthView extends StatefulWidget {
  const MonthView({super.key});

  @override
  State<MonthView> createState() => _MonthViewState();
}

class _MonthViewState extends State<MonthView> {
  DateTime focus = DateTime(DateTime.now().year, DateTime.now().month, 1);

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
    final monthLabel = "${_monthName(focus.month)} ${focus.year}";

    return Column(
      children: [
        // HEADER -------------------------------------------------------------
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              IconButton(onPressed: _prevMonth, icon: const Icon(Icons.chevron_left)),
              Text(
                monthLabel,
                style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
              ),
              IconButton(onPressed: _nextMonth, icon: const Icon(Icons.chevron_right)),
            ],
          ),
        ),

        // WEEKDAY LABELS -----------------------------------------------------
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: const [
              Expanded(child: Center(child: Text("M"))),
              Expanded(child: Center(child: Text("D"))),
              Expanded(child: Center(child: Text("W"))),
              Expanded(child: Center(child: Text("D"))),
              Expanded(child: Center(child: Text("V"))),
              Expanded(child: Center(child: Text("Z"))),
              Expanded(child: Center(child: Text("Z"))),
            ],
          ),
        ),

        const SizedBox(height: 4),

        // CALENDAR -----------------------------------------------------------
        Expanded(
          child: FutureBuilder<List<AgendaItem>>(
            future: _load(),
            builder: (context, snapshot) {
              if (!snapshot.hasData) {
                return const Center(child: CircularProgressIndicator());
              }

              final items = snapshot.data!;
              return _buildCalendar(items);
            },
          ),
        ),
      ],
    );
  }

  // ------------------------------------------------------------------------
  // CALENDAR GRID
  // ------------------------------------------------------------------------
  Widget _buildCalendar(List<AgendaItem> all) {
    final firstDayWeekday = focus.weekday == 7 ? 0 : focus.weekday;
    final daysInMonth = DateTime(focus.year, focus.month + 1, 0).day;

    final totalCells = ((firstDayWeekday + daysInMonth) / 7).ceil() * 7;

final screenHeight = MediaQuery.of(context).size.height;

// Hoeveel rijen nodig?
final rows = (totalCells / 7).ceil();

// Bereken beschikbare hoogte
final headerHeight = 180; // top bars + padding
final availableHeight = screenHeight - headerHeight;

// Hoogte per cel (ruimte eerlijk verdelen)
final cellHeight = availableHeight / rows;

// Aspect ratio aanpassen zodat vakjes mooi vullen
final cellWidth = MediaQuery.of(context).size.width / 7;
final aspectRatio = cellWidth / cellHeight;

return GridView.builder(
  padding: const EdgeInsets.all(12),
  gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
    crossAxisCount: 7,
    childAspectRatio: aspectRatio,
  ),
  itemCount: totalCells,
  itemBuilder: (context, index) {
    final dayNumber = index - (firstDayWeekday - 1);
    if (dayNumber < 1 || dayNumber > daysInMonth) {
      return Container();
    }

    final day = DateTime(focus.year, focus.month, dayNumber);

    final dayTasks = all.where((t) =>
      t.start.year == day.year &&
      t.start.month == day.month &&
      t.start.day == day.day
    ).toList()
      ..sort((a, b) => a.start.compareTo(b.start));

    return _buildDayBox(day, dayTasks);
  },
);

  }

  // ------------------------------------------------------------------------
  // PER DAY BOX
  // ------------------------------------------------------------------------
  Widget _buildDayBox(DateTime day, List<AgendaItem> tasks) {
    return GestureDetector(
      onTap: () {
  Navigator.pushNamed(
    context,
    "/add-task",
    arguments: day,
  ).then((_) => setState(() {}));
},

      child: Container(
        margin: const EdgeInsets.all(3),
        padding: const EdgeInsets.all(6),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(8),
          color: Colors.white10,
          border: Border.all(color: Colors.white24),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // DAY NUMBER
            Text(
              "${day.day}",
              style: TextStyle(
                fontWeight: FontWeight.bold,
                color: day.day == DateTime.now().day &&
                        day.month == DateTime.now().month
                    ? Colors.blue
                    : Colors.white,
              ),
            ),

            const SizedBox(height: 4),

            // TASK PILLS
            Expanded(
              child: SingleChildScrollView(
                child: Column(
                  children: [
                    for (var t in tasks.take(6))
                      Container(
                        width: double.infinity,
                        margin: const EdgeInsets.only(bottom: 3),
                        padding: const EdgeInsets.symmetric(
                          vertical: 2,
                          horizontal: 4,
                        ),
                        decoration: BoxDecoration(
                          color: Colors.blue.withOpacity(0.25),
                          borderRadius: BorderRadius.circular(4),
                        ),
                        child: GestureDetector(
  onTap: () {
    Navigator.pushNamed(
      context,
      "/task-details",
      arguments: t,
    ).then((_) => setState(() {}));
  },
  child: Text(
    t.name,
    style: const TextStyle(fontSize: 10),
  ),
),

                      ),

                    if (tasks.length > 6)
                      Text(
                        "+${tasks.length - 6} meer",
                        style: TextStyle(fontSize: 10, color: Colors.grey.shade400),
                      ),
                  ],
                ),
              ),
            )
          ],
        ),
      ),
    );
  }

  // ------------------------------------------------------------------------
  // HELPERS
  // ------------------------------------------------------------------------
  String _monthName(int m) {
    const names = [
      "",
      "January", "February", "March", "April", "May", "June",
      "July", "August", "September", "October", "November", "December"
    ];
    return names[m];
  }
}
