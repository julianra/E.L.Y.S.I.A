//lib/api/elysia_service.dart

import 'package:orbit/api/elysia_api.dart';
import 'package:orbit/api/discovery.dart';
import 'package:orbit/storage/server_storage.dart';

class ElysiaService {
  static ElysiaApi? _instance;

  static Future<ElysiaApi> get() async {
    if (_instance != null) return _instance!;

    // 1 — probeer opgeslagen server
    final saved = await ServerStorage.loadServer();
    if (saved != null) {
      _instance = ElysiaApi(baseUrl: saved);
      return _instance!;
    }

    // 2 — probeer discovery via mDNS
    final servers = await ElysiaDiscovery.discover();
    if (servers.isNotEmpty) {
      await ServerStorage.saveServer(servers.first);
      _instance = ElysiaApi(baseUrl: servers.first);
      return _instance!;
    }

    throw Exception("Geen Elysia server gevonden");
  }
}
