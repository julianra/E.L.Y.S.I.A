import 'dart:convert';
import 'package:http/http.dart' as http;

class ElysiaApi {
  final String baseUrl;

  ElysiaApi({required this.baseUrl});

  // GET /health
  Future<String> health() async {
    final res = await http.get(Uri.parse('$baseUrl/health'));

    if (res.statusCode != 200) {
      throw Exception("Health check failed: ${res.body}");
    }

    final data = jsonDecode(res.body);
    return data["status"] ?? "unknown";
  }

  // POST /marthe/add_task
  Future<void> addTask(String name) async {
    final res = await http.post(
      Uri.parse('$baseUrl/marthe/add_task'),
      headers: {"Content-Type": "application/json"},
      body: jsonEncode({"name": name}),
    );

    if (res.statusCode != 200) {
      throw Exception("Failed to add task: ${res.body}");
    }
  }
}
