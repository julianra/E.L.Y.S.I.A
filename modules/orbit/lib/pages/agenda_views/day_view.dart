// lib/pages/agenda_views/day_view.dart
import 'package:flutter/material.dart';

class DayView extends StatelessWidget {
  const DayView({super.key});

  @override
  Widget build(BuildContext context) {
    // Fictieve events – later koppelen we dit aan Marthe
    final events = [
      _DayEvent("📘 Studeren", 10, 1.5),
      _DayEvent("🏋️ Fitness", 14, 1),
      _DayEvent("🧹 Opruimen", 18, 0.5),
    ];

    return ListView.builder(
      itemCount: 24,
      itemBuilder: (context, hour) {
        final label = "${hour.toString().padLeft(2, '0')}:00";

        // events die op dit uur starten
        final hourEvents = events.where((e) => e.startHour.floor() == hour);

        return SizedBox(
          height: 70,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Tijd-kolom
              SizedBox(
                width: 60,
                child: Align(
                  alignment: Alignment.topRight,
                  child: Text(
                    label,
                    style: const TextStyle(color: Colors.grey, fontSize: 12),
                  ),
                ),
              ),

              // Hoofd-kolom
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    border: Border(
                      top: BorderSide(color: Colors.grey.shade800),
                    ),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 4),
                  child: Stack(
                    children: [
                      // (later: achtergrond of current time marker)
                      ...hourEvents.map(
                        (e) => Align(
                          alignment: Alignment.topLeft,
                          child: Container(
                            height: 60 * e.durationHours, // simpele hoogte
                            decoration: BoxDecoration(
                              color: Colors.blue.withOpacity(0.7),
                              borderRadius: BorderRadius.circular(6),
                            ),
                            padding: const EdgeInsets.all(6),
                            child: Text(
                              e.title,
                              style: const TextStyle(
                                color: Colors.white,
                                fontSize: 12,
                              ),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _DayEvent {
  final String title;
  final double startHour; // bijv 10.0, 14.0
  final double durationHours;

  _DayEvent(this.title, this.startHour, this.durationHours);
}
