// lib/core/api/http_client.dart
// Simple HTTP client for making API requests

import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiClient {
  static const String baseUrl = "http://192.168.0.17:3000";

  static Future<http.Response> post(
      String path, Map<String, dynamic> body) async {
    final url = Uri.parse("$baseUrl$path");

    return await http.post(
      url,
      headers: {"Content-Type": "application/json"},
      body: jsonEncode(body),
    );
  }
}
