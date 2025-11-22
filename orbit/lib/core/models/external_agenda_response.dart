class ExternalAgendaResponse {
  final bool success;
  final String message;
  final String? name;
  final String? id; // Rust stuurt dit momenteel nog niet

  ExternalAgendaResponse({
    required this.success,
    required this.message,
    this.name,
    this.id,
  });

  factory ExternalAgendaResponse.fromJson(Map<String, dynamic> json) {
    return ExternalAgendaResponse(
      success: json["success"] ?? false,
      message: json["message"] ?? "",
      name: json["name"],
      id: json["id"], // blijft null tot Rust dit toevoegt
    );
  }
}
