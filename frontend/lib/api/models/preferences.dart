// Student preferences data model
class StudentPreferences {
  final int studentId;
  // Preferred time of day (Morning/Afternoon/Evening)
  final String preferredTime;
  final bool preferOnline;

  StudentPreferences({
    required this.studentId,
    required this.preferredTime,
    required this.preferOnline,
  });
}