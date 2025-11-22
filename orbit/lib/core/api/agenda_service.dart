import 'dart:convert';

import 'package:orbit/core/api/http_client.dart';
import 'package:orbit/core/models/external_agenda_request.dart';
import 'package:orbit/core/models/external_agenda_response.dart';

class AgendaService {
  static Future<ExternalAgendaResponse> addAgendaPoint(
      ExternalAgendaRequest req) async {

    final response = await ApiClient.post(
      "/external/agenda/add",
      req.toJson(),
    );

    // Veilig decoderen
    Map<String, dynamic> data;
    try {
      data = jsonDecode(response.body);
    } catch (_) {
      throw Exception("Server returned invalid JSON: ${response.body}");
    }

    return ExternalAgendaResponse.fromJson(data);
  }
}
