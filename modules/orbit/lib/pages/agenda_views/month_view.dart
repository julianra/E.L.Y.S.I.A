import 'package:flutter/material.dart';

class MonthView extends StatelessWidget {
  const MonthView({super.key});

  @override
  Widget build(BuildContext context) {
    final days = ["Ma", "Di", "Wo", "Do", "Vr", "Za", "Zo"];

    return Column(
      children: [
        // DAGEN HEADER
        Row(
          children: days
              .map(
                (d) => Expanded(
                  child: Container(
                    padding: const EdgeInsets.all(8),
                    alignment: Alignment.center,
                    child: Text(
                      d,
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ),
              )
              .toList(),
        ),

        // KALENDER GRID (6 weken)
        Expanded(
          child: GridView.builder(
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 7,
            ),
            itemCount: 42,
            itemBuilder: (_, i) {
              final day = (i + 27) % 30 + 1; // Fictieve dagen

              return Container(
                margin: const EdgeInsets.all(2),
                decoration: BoxDecoration(
                  border: Border.all(color: Colors.grey.shade800),
                  borderRadius: BorderRadius.circular(6),
                ),
                padding: const EdgeInsets.all(6),
                child: Text("$day"),
              );
            },
          ),
        ),
      ],
    );
  }
}
