// lib/pages/agenda_page.dart
import 'package:flutter/material.dart';
import 'agenda_views/day_view.dart';
import 'agenda_views/week_view.dart';
import 'agenda_views/month_view.dart';
import 'agenda_views/year_view.dart';

class AgendaPage extends StatelessWidget {
  const AgendaPage({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 4,
      child: Scaffold(
        appBar: AppBar(
          title: const Text("Agenda"),
          bottom: const TabBar(
            tabs: [
              Tab(text: "Dag"),
              Tab(text: "Week"),
              Tab(text: "Maand"),
              Tab(text: "Jaar"),
            ],
          ),
        ),
        body: const TabBarView(
          children: [
            DayView(),
            WeekView(),
            MonthView(),
            YearView(),
          ],
        ),
      ),
    );
  }
}
