// lib/api/elysia_api.dart

import 'dart:convert';
import 'package:http/http.dart' as http;

class ElysiaApi {
  final String baseUrl;

  ElysiaApi({required this.baseUrl});

  // ---------------------------
  // GET /health
  // ---------------------------
  Future<bool> health() async {
    final res = await http.get(Uri.parse('$baseUrl/health'));
    return res.statusCode == 200;
  }

  // ---------------------------
  // POST /agenda/add
  // (correct endpoint)
  // ---------------------------
  Future<void> addTask(Map<String, dynamic> payload) async {
    final res = await http.post(
      Uri.parse('$baseUrl/agenda/add'),
      headers: {"Content-Type": "application/json"},
      body: jsonEncode(payload),
    );

    if (res.statusCode != 200) {
      throw Exception("Failed to add task: ${res.body}");
    }
  }

  // ---------------------------
  // GET /marthe/tasks
  // ---------------------------
  Future<List<dynamic>> getTasks() async {
    final res = await http.get(Uri.parse('$baseUrl/marthe/tasks'));

    if (res.statusCode != 200) {
      throw Exception("Failed to fetch tasks");
    }

    return jsonDecode(res.body) as List<dynamic>;
  }
}
