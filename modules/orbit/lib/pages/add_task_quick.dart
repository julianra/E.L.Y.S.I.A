//lib/pages/add_task_quick.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';

class AddTaskQuickPage extends StatefulWidget {
  const AddTaskQuickPage({super.key});

  @override
  State<AddTaskQuickPage> createState() => _AddTaskQuickPageState();
}

class _AddTaskQuickPageState extends State<AddTaskQuickPage> {
  final textController = TextEditingController();
  bool loading = false;

  bool isListening = false;

  @override
  void initState() {
    super.initState();
  }

  // ----------------------------------------------------------
  // START / STOP SPEECH
  // ----------------------------------------------------------
 
  // ----------------------------------------------------------
  // SUBMIT TASK
  // ----------------------------------------------------------
  Future<void> _submit() async {
    if (textController.text.trim().isEmpty) return;

    setState(() => loading = true);

    final api = await ElysiaService.get();
    await api.addTask({
      "name": textController.text.trim(),
    });

    setState(() => loading = false);
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Snel taak toevoegen"),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            TextField(
              controller: textController,
              decoration: const InputDecoration(
                labelText: "Taak omschrijving",
                border: OutlineInputBorder(),
              ),
              maxLines: 3,
            ),

            const SizedBox(height: 20),

            // ------------------------------
            // MICROPHONE BUTTON
            // ------------------------------
            GestureDetector(
              child: Container(
                padding: const EdgeInsets.all(18),
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: isListening ? Colors.red : Colors.blue,
                ),
                child: Icon(
                  isListening ? Icons.mic : Icons.mic_none,
                  size: 36,
                  color: Colors.white,
                ),
              ),
            ),

            const SizedBox(height: 20),

            // ------------------------------
            // SAVE BUTTON
            // ------------------------------
            ElevatedButton(
              onPressed: loading ? null : _submit,
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.green,
                minimumSize: const Size(double.infinity, 48),
              ),
              child: loading
                  ? const CircularProgressIndicator(color: Colors.white)
                  : const Text("Opslaan", style: TextStyle(fontSize: 18)),
            )
          ],
        ),
      ),
    );
  }
}
