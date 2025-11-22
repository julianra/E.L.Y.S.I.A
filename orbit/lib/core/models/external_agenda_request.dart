import 'dart:convert';

class ExternalAgendaRequest {
  final String name;

  // Tijd & planning
  final int? durationMinutes;
  final DateTime? date;
  final DateTime? exactStart;
  final DateTime? exactEnd;
  final DateTime? deadlineEnd;

  // Basiscategorisatie
  final String? priority;
  final String? taskType;
  final String? project;
  final String? location;

  // AI & intelligentie
  final int? energyCost;
  final String? category;
  final String? recurrence;
  final double? importanceScore;
  final int? predictedDuration;
  final double? confidenceScore;
  final int? emotionalLoad;

  // Metadata
  final List<String>? requiredTools;
  final List<String>? blockingRules;
  final List<String>? contextTags;
  final List<String>? linkedTasks;

  ExternalAgendaRequest({
    required this.name,
    this.durationMinutes,
    this.date,
    this.exactStart,
    this.exactEnd,
    this.deadlineEnd,
    this.priority,
    this.taskType,
    this.project,
    this.location,
    this.energyCost,
    this.category,
    this.recurrence,
    this.importanceScore,
    this.predictedDuration,
    this.confidenceScore,
    this.emotionalLoad,
    this.requiredTools,
    this.blockingRules,
    this.contextTags,
    this.linkedTasks,
  });

  Map<String, dynamic> toJson() {
    final map = {
      "name": name,
      "duration_minutes": durationMinutes,
      "date": date?.toIso8601String(),
      "exact_start": exactStart?.toIso8601String(),
      "exact_end": exactEnd?.toIso8601String(),
      "deadline_end": deadlineEnd?.toIso8601String(),
      "priority": priority,
      "type": taskType,
      "project": project,
      "location": location,
      "energy_cost": energyCost,
      "category": category,
      "recurrence": recurrence,
      "importance_score": importanceScore,
      "predicted_duration": predictedDuration,
      "confidence_score": confidenceScore,
      "emotional_load": emotionalLoad,
      "required_tools": requiredTools,
      "blocking_rules": blockingRules,
      "context_tags": contextTags,
      "linked_tasks": linkedTasks,
    };

    // 👇 verwijder null velden → Rust raakt NIET meer in paniek
    map.removeWhere((key, value) => value == null);

    // 👇 veilige debug print
    print("SENDING JSON: ${jsonEncode(map)}");

    return map;
  }
}
