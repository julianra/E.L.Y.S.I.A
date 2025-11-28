// lib/pages/add_task_quick.dart

import 'package:flutter/material.dart';
import 'package:orbit/api/elysia_service.dart';
import 'package:speech_to_text/speech_to_text.dart' as stt;
import 'package:permission_handler/permission_handler.dart';

class AddTaskQuickPage extends StatefulWidget {
  const AddTaskQuickPage({super.key});

  @override
  State<AddTaskQuickPage> createState() => _AddTaskQuickPageState();
}

class _AddTaskQuickPageState extends State<AddTaskQuickPage> {
  final textController = TextEditingController();
  bool loading = false;

  late stt.SpeechToText speech;
  bool isListening = false;

  @override
  void initState() {
    super.initState();
    speech = stt.SpeechToText();
  }

  // ----------------------------------------------------------
  // CHECK PERMISSION
  // ----------------------------------------------------------
  Future<bool> _checkPermission() async {
    var status = await Permission.microphone.status;

    if (status.isDenied) {
      status = await Permission.microphone.request();
    }

    if (!status.isGranted) {
      print("❌ Microphone permission NOT granted");
      return false;
    }

    print("🎤 Microphone permission granted");
    return true;
  }

  // ----------------------------------------------------------
  // START LISTENING
  // ----------------------------------------------------------
  Future<void> _startListening() async {
    if (!await _checkPermission()) return;

    bool available = await speech.initialize(
      onStatus: (status) => print("STATUS: $status"),
      onError: (e) => print("ERROR: $e"),
    );

    if (!available) {
      print("❌ Speech not available");
      return;
    }

    setState(() => isListening = true);

    speech.listen(
      localeId: "nl_BE",
      onResult: (result) {
        setState(() {
          textController.text = result.recognizedWords;
        });
      },
    );

    print("🎙️ Listening started...");
  }

  // ----------------------------------------------------------
  // STOP LISTENING
  // ----------------------------------------------------------
  Future<void> _stopListening() async {
    await speech.stop();
    setState(() => isListening = false);
    print("🛑 Listening stopped");
  }

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

  // ----------------------------------------------------------
  // UI BUILD
  // ----------------------------------------------------------
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

            // HOLD MIC
            GestureDetector(
              onLongPressStart: (_) => _startListening(),
              onLongPressEnd: (_) => _stopListening(),
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
