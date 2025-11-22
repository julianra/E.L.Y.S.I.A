// lib/features/agenda/pages/add_agenda_page.dart
// Page to add a new agenda point
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'package:orbit/core/models/external_agenda_request.dart';
import 'package:orbit/core/state/agenda_provider.dart';

class AddAgendaPage extends StatefulWidget {
  const AddAgendaPage({super.key});

  @override
  State<AddAgendaPage> createState() => _AddAgendaPageState();
}

class _AddAgendaPageState extends State<AddAgendaPage> {
  final _formKey = GlobalKey<FormState>();

  // -----------------------
  // BASIC FIELDS
  // -----------------------
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _durationController = TextEditingController();

  String _priority = "normal";

  // LOCATION
  String _location = "home";
  bool _useCustomLocation = false;
  final TextEditingController _customLocationController =
      TextEditingController();

  // TODO SYSTEM
  bool _isTodo = false;
  String _todoWhen = "today";
  DateTime? _todoCustomDate;
  DateTime? _basicDateTime;


  // -----------------------
  // ADVANCED FIELDS
  // -----------------------
  bool _showAdvanced = false;

  final TextEditingController _projectController = TextEditingController();
  final TextEditingController _taskTypeController = TextEditingController();
  final TextEditingController _categoryController = TextEditingController();

  DateTime? _exactStart;
  DateTime? _exactEnd;
  DateTime? _deadlineEnd;

  int? _energyCost;
  double? _importanceScore;
  int? _emotionalLoad;

  final TextEditingController _requiredToolsController =
      TextEditingController();
  final TextEditingController _contextTagsController = TextEditingController();
  final TextEditingController _linkedTasksController = TextEditingController();

  // -----------------------
  // DATE/TIME PICKER
  // -----------------------
  Future<void> _pickDateTime(Function(DateTime?) setter,
      {bool todoMode = false}) async {
    final now = DateTime.now();

    final firstDate = todoMode
        ? now.add(const Duration(days: 3)) // cannot pick today/tomorrow/overmorgen
        : now;

    final date = await showDatePicker(
      context: context,
      firstDate: firstDate,
      lastDate: DateTime(now.year + 5),
      initialDate: firstDate,
    );

    if (date == null) return;

    if (!todoMode) {
      final time = await showTimePicker(
        context: context,
        initialTime: TimeOfDay.now(),
      );
      if (time == null) return;

      setter(DateTime(date.year, date.month, date.day, time.hour, time.minute));
    } else {
      setter(DateTime(date.year, date.month, date.day));
    }

    setState(() {});
  }

  // -----------------------
  // BUILD
  // -----------------------
  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<AgendaProvider>(context);

    return Scaffold(
      appBar: AppBar(title: const Text("Nieuw Agendapunt")),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Form(
          key: _formKey,
          child: ListView(
            children: [
              // -----------------------
              // BASIC MODE
              // -----------------------
              TextFormField(
                controller: _nameController,
                decoration: const InputDecoration(
                  labelText: "Titel",
                  border: OutlineInputBorder(),
                ),
                validator: (v) =>
                    v == null || v.isEmpty ? "Vul een titel in" : null,
              ),
              const SizedBox(height: 20),

              TextFormField(
                controller: _durationController,
                decoration: const InputDecoration(
                  labelText: "Duur (minuten)",
                  border: OutlineInputBorder(),
                ),
                keyboardType: TextInputType.number,
              ),
              const SizedBox(height: 20),

              DropdownButtonFormField<String>(
                value: _priority,
                decoration: const InputDecoration(
                  labelText: "Prioriteit",
                  border: OutlineInputBorder(),
                ),
                items: const [
                  DropdownMenuItem(value: "low", child: Text("Laag")),
                  DropdownMenuItem(value: "normal", child: Text("Normaal")),
                  DropdownMenuItem(value: "high", child: Text("Hoog")),
                ],
                onChanged: (v) => setState(() => _priority = v!),
              ),
              const SizedBox(height: 20),

              // LOCATION DROPDOWN + CUSTOM
              DropdownButtonFormField<String>(
                value: _useCustomLocation ? "custom" : _location,
                decoration: const InputDecoration(
                  labelText: "Locatie",
                  border: OutlineInputBorder(),
                ),
                items: const [
                  DropdownMenuItem(value: "home", child: Text("Thuis")),
                  DropdownMenuItem(value: "office", child: Text("Kantoor")),
                  DropdownMenuItem(value: "outside", child: Text("Buiten")),
                  DropdownMenuItem(value: "custom", child: Text("Eigen locatie…")),
                ],
                onChanged: (v) {
                  setState(() {
                    if (v == "custom") {
                      _useCustomLocation = true;
                    } else {
                      _useCustomLocation = false;
                      _location = v!;
                    }
                  });
                },
              ),

              if (_useCustomLocation) ...[
                const SizedBox(height: 10),
                TextFormField(
                  controller: _customLocationController,
                  decoration: const InputDecoration(
                    labelText: "Eigen locatie",
                    border: OutlineInputBorder(),
                  ),
                  validator: (value) {
                    if (_useCustomLocation &&
                        (value == null || value.isEmpty)) {
                      return "Geef een locatie in";
                    }
                    return null;
                  },
                ),
              ],

              const SizedBox(height: 20),

              // TODO SYSTEM
              CheckboxListTile(
                title: const Text("Dit is een To-do taak"),
                value: _isTodo,
                onChanged: (v) => setState(() => _isTodo = v!),
              ),

              if (_isTodo) ...[
                const SizedBox(height: 10),
                DropdownButtonFormField<String>(
                  value: _todoWhen,
                  decoration: const InputDecoration(
                    labelText: "Wanneer?",
                    border: OutlineInputBorder(),
                  ),
                  items: const [
                    DropdownMenuItem(value: "today", child: Text("Vandaag")),
                    DropdownMenuItem(value: "tomorrow", child: Text("Morgen")),
                    DropdownMenuItem(
                        value: "day_after_tomorrow", child: Text("Overmorgen")),
                    DropdownMenuItem(
                        value: "next_week", child: Text("Volgende week")),
                    DropdownMenuItem(
                        value: "this_month", child: Text("Deze maand")),
                    DropdownMenuItem(
                        value: "specific", child: Text("Specifieke datum…")),
                  ],
                  onChanged: (v) => setState(() => _todoWhen = v!),
                ),

                if (_todoWhen == "specific") ...[
                  const SizedBox(height: 10),
                  ListTile(
                    title: Text(
                      _todoCustomDate == null
                          ? "Kies een datum"
                          : "Gekozen: $_todoCustomDate",
                    ),
                    trailing: const Icon(Icons.calendar_today),
                    onTap: () => _pickDateTime((d) => _todoCustomDate = d,
                        todoMode: true),
                  ),
                ],
              ],

              const SizedBox(height: 20),
              // BASIC DATE + TIME PICKER
              ListTile(
                title: Text(
                  _basicDateTime == null
                      ? "Kies datum & tijd"
                      : "Gekozen: $_basicDateTime",
                ),
                trailing: const Icon(Icons.calendar_today),
                onTap: () async {
                  final now = DateTime.now();

                  // 1. Datum kiezen
                  final date = await showDatePicker(
                    context: context,
                    firstDate: now,
                    lastDate: DateTime(now.year + 5),
                    initialDate: now,
                  );

                  if (date == null) return;

                  // 2. Tijd kiezen
                  final time = await showTimePicker(
                    context: context,
                    initialTime: TimeOfDay.now(),
                  );
                  if (time == null) return;

                  setState(() {
                    _basicDateTime = DateTime(
                      date.year,
                      date.month,
                      date.day,
                      time.hour,
                      time.minute,
                    );
                  });
                },
              ),
              const SizedBox(height: 20),

              // -----------------------
              // ADVANCED TOGGLE
              // -----------------------
              TextButton.icon(
                icon: Icon(
                    _showAdvanced ? Icons.expand_less : Icons.expand_more),
                label: Text(
                    _showAdvanced ? "Verberg geavanceerde opties" : "Toon geavanceerde opties"),
                onPressed: () => setState(() => _showAdvanced = !_showAdvanced),
              ),

              if (_showAdvanced) ...[
                const Divider(height: 30),

                // EXACT START
                ListTile(
                  title: Text(
                    _exactStart == null
                        ? "Geen exacte start"
                        : "Exacte start: $_exactStart",
                  ),
                  trailing: const Icon(Icons.schedule),
                  onTap: () =>
                      _pickDateTime((d) => _exactStart = d, todoMode: false),
                ),

                // EXACT END
                ListTile(
                  title: Text(
                    _exactEnd == null
                        ? "Geen exact einde"
                        : "Exact einde: $_exactEnd",
                  ),
                  trailing: const Icon(Icons.event),
                  onTap: () =>
                      _pickDateTime((d) => _exactEnd = d, todoMode: false),
                ),

                // DEADLINE
                ListTile(
                  title: Text(
                    _deadlineEnd == null
                        ? "Geen deadline"
                        : "Deadline: $_deadlineEnd",
                  ),
                  trailing: const Icon(Icons.warning),
                  onTap: () =>
                      _pickDateTime((d) => _deadlineEnd = d, todoMode: false),
                ),

                const SizedBox(height: 20),

                // PROJECT
                TextFormField(
                  controller: _projectController,
                  decoration: const InputDecoration(
                    labelText: "Project",
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 20),

                // TYPE
                TextFormField(
                  controller: _taskTypeController,
                  decoration: const InputDecoration(
                    labelText: "Type (bv. normal, call, focus)",
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 20),

                // CATEGORY
                TextFormField(
                  controller: _categoryController,
                  decoration: const InputDecoration(
                    labelText: "Categorie (bv. creative, admin)",
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 20),

                // ENERGY COST
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: "Energy cost (0–10)",
                    border: OutlineInputBorder(),
                  ),
                  keyboardType: TextInputType.number,
                  onChanged: (v) => _energyCost = int.tryParse(v),
                ),
                const SizedBox(height: 20),

                // IMPORTANCE
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: "Importance score (0–1)",
                    border: OutlineInputBorder(),
                  ),
                  keyboardType: TextInputType.number,
                  onChanged: (v) => _importanceScore = double.tryParse(v),
                ),
                const SizedBox(height: 20),

                // EMOTIONAL LOAD
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: "Emotional load (0–10)",
                    border: OutlineInputBorder(),
                  ),
                  keyboardType: TextInputType.number,
                  onChanged: (v) => _emotionalLoad = int.tryParse(v),
                ),
                const SizedBox(height: 20),

                // REQUIRED TOOLS
                TextFormField(
                  controller: _requiredToolsController,
                  decoration: const InputDecoration(
                    labelText: "Benodigdheden (comma separated)",
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 20),

                // CONTEXT TAGS
                TextFormField(
                  controller: _contextTagsController,
                  decoration: const InputDecoration(
                    labelText: "Context tags (comma separated)",
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 20),

                // LINKED TASKS
                TextFormField(
                  controller: _linkedTasksController,
                  decoration: const InputDecoration(
                    labelText: "Linked task IDs (comma separated)",
                    border: OutlineInputBorder(),
                  ),
                ),
              ],

              const SizedBox(height: 40),

              // -----------------------
              // SUBMIT BUTTON
              // -----------------------
              SizedBox(
                height: 55,
                child: ElevatedButton(
                  onPressed: provider.isLoading
                      ? null
                      : () async {
                          if (!_formKey.currentState!.validate()) return;

                          // -----------------------
                          // HANDLE TODO DATE
                          // -----------------------
                          DateTime? todoDate;
                          final now = DateTime.now();

                          if (_isTodo) {
                            if (_todoWhen == "today") {
                              todoDate =
                                  DateTime(now.year, now.month, now.day);
                            } else if (_todoWhen == "tomorrow") {
                              final t = now.add(const Duration(days: 1));
                              todoDate = DateTime(t.year, t.month, t.day);
                            } else if (_todoWhen == "day_after_tomorrow") {
                              final t = now.add(const Duration(days: 2));
                              todoDate = DateTime(t.year, t.month, t.day);
                            } else if (_todoWhen == "next_week") {
                              final t = now.add(const Duration(days: 7));
                              todoDate = DateTime(t.year, t.month, t.day);
                            } else if (_todoWhen == "this_month") {
                              todoDate = DateTime(now.year, now.month + 1, 1)
                                  .subtract(const Duration(days: 1));
                            } else if (_todoWhen == "specific") {
                              todoDate = _todoCustomDate;
                            }
                          }

                          final req = ExternalAgendaRequest(
                            name: _nameController.text,

                            durationMinutes:
                                int.tryParse(_durationController.text),

                            priority: _priority,

                            // LOCATION
                            location: _useCustomLocation
                                ? _customLocationController.text
                                : _location,

                            // TODO DATE
                            date: _isTodo 
                            ? todoDate 
                            : _basicDateTime, // BASIC DATETIME gestuurd

                            exactStart: _exactStart,
                            exactEnd: _exactEnd,
                            deadlineEnd: _deadlineEnd,

                            project: _projectController.text.isEmpty
                                ? null
                                : _projectController.text,
                            taskType: _taskTypeController.text.isEmpty
                                ? null
                                : _taskTypeController.text,
                            category: _categoryController.text.isEmpty
                                ? null
                                : _categoryController.text,

                            energyCost: _energyCost,
                            importanceScore: _importanceScore,
                            emotionalLoad: _emotionalLoad,

                            requiredTools:
                                _requiredToolsController.text.isEmpty
                                    ? null
                                    : _requiredToolsController.text
                                        .split(",")
                                        .map((e) => e.trim())
                                        .toList(),
                            contextTags: _contextTagsController.text.isEmpty
                                ? null
                                : _contextTagsController.text
                                    .split(",")
                                    .map((e) => e.trim())
                                    .toList(),
                            linkedTasks: _linkedTasksController.text.isEmpty
                                ? null
                                : _linkedTasksController.text
                                    .split(",")
                                    .map((e) => e.trim())
                                    .toList(),
                          );

                          await provider.addAgendaPoint(req);

                          if (provider.lastResponse?.success == true) {
                            ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(
                                content: Text(
                                    provider.lastResponse!.message),
                              ),
                            );
                            Navigator.pop(context);
                          }
                        },
                  child: provider.isLoading
                      ? const CircularProgressIndicator(color: Colors.white)
                      : const Text("Toevoegen"),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
