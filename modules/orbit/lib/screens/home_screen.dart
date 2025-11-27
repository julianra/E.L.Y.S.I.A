// lib/screens/home_screen.dart
import 'package:flutter/material.dart';
import '../pages/home_dashboard.dart';
import '../pages/agenda_page.dart';
import '../pages/settings_page.dart';

class HomeScreen extends StatefulWidget {
  final String apiBaseUrl;

  const HomeScreen({
    super.key,
    required this.apiBaseUrl,
  });

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  int index = 0;

  final List<Widget> _pages = [];

  @override
  void initState() {
    super.initState();
    _pages.addAll([
      const HomeDashboard(),
      const AgendaPage(),
      SettingsPage(),
    ]);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _pages[index],
      bottomNavigationBar: NavigationBar(
        selectedIndex: index,
        destinations: const [
          NavigationDestination(
            icon: Icon(Icons.home_outlined),
            selectedIcon: Icon(Icons.home),
            label: "Home",
          ),
          NavigationDestination(
            icon: Icon(Icons.calendar_month_outlined),
            selectedIcon: Icon(Icons.calendar_month),
            label: "Agenda",
          ),
          NavigationDestination(
            icon: Icon(Icons.settings_outlined),
            selectedIcon: Icon(Icons.settings),
            label: "Instellingen",
          ),
        ],
        onDestinationSelected: (i) {
          setState(() => index = i);
        },
      ),
    );
  }
}
