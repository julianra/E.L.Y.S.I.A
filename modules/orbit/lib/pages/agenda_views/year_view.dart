import 'package:flutter/material.dart';

class YearView extends StatelessWidget {
  const YearView({super.key});

  @override
  Widget build(BuildContext context) {
    final months = [
      "Jan","Feb","Mrt","Apr","Mei","Jun",
      "Jul","Aug","Sep","Okt","Nov","Dec"
    ];

    return GridView.builder(
      padding: const EdgeInsets.all(10),
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 3,
        childAspectRatio: 1.2,
      ),
      itemCount: 12,
      itemBuilder: (_, i) {
        return Card(
          child: Column(
            children: [
              Text(
                months[i],
                style: const TextStyle(
                    fontWeight: FontWeight.bold, fontSize: 18),
              ),
              const SizedBox(height: 6),

              // Mini grid van 7×5
              Expanded(
                child: GridView.count(
                  crossAxisCount: 7,
                  physics: const NeverScrollableScrollPhysics(),
                  children: List.generate(
                    35,
                    (d) => Center(
                      child: Text(
                        ((d + i) % 30 + 1).toString(),
                        style: const TextStyle(fontSize: 10),
                      ),
                    ),
                  ),
                ),
              )
            ],
          ),
        );
      },
    );
  }
}
