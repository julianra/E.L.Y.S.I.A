//lib/storage/server_storage.dart
import 'package:shared_preferences/shared_preferences.dart';

class ServerStorage {
  static const _key = "elysia_server";

  static Future<void> saveServer(String url) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_key, url);
  }

  static Future<String?> loadServer() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString(_key);
  }

  static Future<void> clear() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_key);
  }
}
