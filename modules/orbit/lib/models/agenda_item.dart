// lib/models/agenda_item.dart

class AgendaItem {
  final String id;
  final String name;
  final DateTime start;
  final DateTime end;
  final int durationMinutes;

  AgendaItem({
    required this.id,
    required this.name,
    required this.start,
    required this.end,
    required this.durationMinutes,
  });

  factory AgendaItem.fromJson(Map<String, dynamic> json) {
    return AgendaItem(
      id: json["id"],
      name: json["name"],
      start: DateTime.parse(json["exact_start"]).toLocal(),
      end: DateTime.parse(json["exact_end"]).toLocal(),
      durationMinutes: json["duration_minutes"],
    );
  }
}
