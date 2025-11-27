// lib/pages/agenda_views/week_view.dart
import 'package:flutter/material.dart';

class WeekView extends StatelessWidget {
  const WeekView({super.key});

  @override
  Widget build(BuildContext context) {
    final days = ["Ma", "Di", "Wo", "Do", "Vr", "Za", "Zo"];

    // Fictieve events – dagIndex: 0 = Ma, 6 = Zo
    

    return Column(
      children: [
        // Header met dagen
        Row(
          children: days
              .map(
                (d) => Expanded(
                  child: Container(
                    height: 40,
                    alignment: Alignment.center,
                    child: Text(
                      d,
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 14,
                      ),
                    ),
                  ),
                ),
              )
              .toList(),
        ),

        const Divider(height: 1),

        // Scrollbare uren-rows
        Expanded(
          child: ListView.builder(
            itemCount: 24,
            itemBuilder: (context, hour) {
              final label = "${hour.toString().padLeft(2, '0')}:00";

              return SizedBox(
                height: 70,
                child: Row(
                  children: [
                    // optioneel: kleine tijdkolom links (zoals in Google Calendar "schedule" view)
                    SizedBox(
                      width: 40,
                      child: Align(
                        alignment: Alignment.topRight,
                        child: Text(
                          label,
                          style: const TextStyle(
                            color: Colors.grey,
                            fontSize: 10,
                          ),
                        ),
                      ),
                    ),

                    // 7 dagen kolommen
                    Expanded(
                      child: Row(
                        children: List.generate(7, (dayIndex) {
                          

                          return Expanded(
                            child: Container(
                              decoration: BoxDecoration(
                                border: Border(
                                  top: BorderSide(
                                      color: Colors.grey.shade900, width: 0.5),
                                  right: BorderSide(
                                      color: Colors.grey.shade900, width: 0.5),
                                ),
                              ),
                              padding: const EdgeInsets.all(2),
                              child: Stack(
                                
                              ),
                            ),
                          );
                        }),
                      ),
                    ),
                  ],
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}


