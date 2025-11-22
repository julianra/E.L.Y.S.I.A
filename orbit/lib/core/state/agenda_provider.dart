import 'package:flutter/material.dart';
import 'package:orbit/core/api/agenda_service.dart';
import 'package:orbit/core/models/external_agenda_request.dart';
import 'package:orbit/core/models/external_agenda_response.dart';

class AgendaProvider extends ChangeNotifier {
  bool isLoading = false;

  ExternalAgendaResponse? lastResponse;

  Future<void> addAgendaPoint(ExternalAgendaRequest req) async {
    isLoading = true;
    notifyListeners();

    lastResponse = await AgendaService.addAgendaPoint(req);

    isLoading = false;
    notifyListeners();
  }
}
