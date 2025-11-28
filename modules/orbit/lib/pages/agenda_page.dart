// lib/pages/agenda_page.dart
import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:orbit/pages/agenda_views/day_view.dart';
import 'package:orbit/pages/agenda_views/month_view.dart';
import 'package:orbit/pages/agenda_views/week_view.dart'; // zodra file bestaat

class AgendaPage extends StatefulWidget {
  const AgendaPage({super.key});

  @override
  State<AgendaPage> createState() => _AgendaPageState();
}

class _AgendaPageState extends State<AgendaPage> with SingleTickerProviderStateMixin {
  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  void _openAddTask([DateTime? prefilledStart]) async {
    await Navigator.pushNamed(
      context,
      "/add-task",
      arguments: prefilledStart,
    );
    setState(() {}); // Refresh agenda after adding task
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Mijn Agenda"),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: "Dag"),
            Tab(text: "Week"),
            Tab(text: "Maand"),
          ],
        ),
      ),

      body: TabBarView(
        controller: _tabController,
        children: [
          DayView(onCreateAtHour: _openAddTask),
          WeekView(onCreateAtDay: _openAddTask),
          const MonthView(),
        ],
      ),

      floatingActionButton: FloatingActionButton(
        backgroundColor: Colors.blue,
        onPressed: () => _openAddTask(),
        child: const Icon(Icons.add),
      ),
    );
  }
}
