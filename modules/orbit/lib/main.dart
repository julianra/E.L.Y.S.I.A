import 'package:flutter/material.dart';
import 'package:orbit/pages/add_task_page.dart';
import 'dart:io';

import 'screens/server_select_screen.dart';
import 'screens/home_screen.dart';
import 'storage/server_storage.dart';
import 'pages/task_details_page.dart';
import 'package:orbit/models/agenda_item.dart';
import 'pages/add_task_quick.dart';


void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const OrbitApp());
}

class OrbitApp extends StatefulWidget {
  const OrbitApp({super.key});

  @override
  State<OrbitApp> createState() => _OrbitAppState();
}

class _OrbitAppState extends State<OrbitApp> {
  String? baseUrl;
  bool loading = true;

  @override
  void initState() {
    super.initState();
    _loadSavedServer();
  }

  Future<void> _loadSavedServer() async {
    final saved = await ServerStorage.loadServer();

    if (saved != null) {
      final ok = await _testServer(saved);
      if (ok) {
        setState(() {
          baseUrl = saved;
          loading = false;
        });
        return;
      }
    }

    setState(() => loading = false);
  }

  Future<bool> _testServer(String base) async {
    try {
      final url = Uri.parse("$base/health");

      // Android heeft een workaround nodig voor mDNS hostnames
      final client = HttpClient()
        ..connectionTimeout = const Duration(seconds: 2);

      final request = await client.getUrl(url);
      final response = await request.close();
      return response.statusCode == 200;
    } catch (_) {
      return false;
    }
  }

  void onServerChosen(String url) async {
    await ServerStorage.saveServer(url);
    setState(() => baseUrl = url);
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return const MaterialApp(
        home: Scaffold(
          body: Center(child: CircularProgressIndicator()),
        ),
      );
    }

    return MaterialApp(
  debugShowCheckedModeBanner: false,
  title: "Orbit",
  theme: ThemeData.dark(),
  routes: {
    "/select-server": (_) => ServerSelectScreen(onSelected: onServerChosen),
    "/home": (_) => HomeScreen(apiBaseUrl: baseUrl ?? ""),
"/add-quick": (_) => const AddTaskQuickPage(),

    // ➜ HIER TOEVOEGEN
    "/add-task": (context) {
      final args = ModalRoute.of(context)!.settings.arguments;
      final prefilledStart = args is DateTime ? args : null;
      return AddTaskPage(prefilledStart: prefilledStart);
    },
   "/task-details": (context) {
  final args = ModalRoute.of(context)!.settings.arguments;

  if (args == null || args is! AgendaItem) {
    return const Scaffold(
      body: Center(
        child: Text("Geen geldige taak meegegeven"),
      ),
    );
  }

  return TaskDetailsPage(item: args);
},


  },
  home: baseUrl == null
      ? ServerSelectScreen(onSelected: onServerChosen)
      : HomeScreen(apiBaseUrl: baseUrl!),
);

}}
