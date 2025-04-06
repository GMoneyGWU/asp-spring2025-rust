// Schedule entry data model
class ScheduleEntry {
  final String courseId;
  final String courseTitle;

  // Day of the week
  final String day;

  // Start time (HH:MM format)
  final String startTime;

  // End time (HH:MM format)
  final String endTime;

  ScheduleEntry({
    required this.courseId,
    required this.courseTitle,
    required this.day,
    required this.startTime,
    required this.endTime,
  });
}

// Semester plan data model
class SemesterPlan {
  final int studentId;
  final List<ScheduleEntry> entries;

  SemesterPlan({
    required this.studentId,
    required this.entries,
  });
}

// Schedule conflict result data model
class ScheduleConflictResult {
  final Map<String, List<String>> conflicts;

  ScheduleConflictResult({required this.conflicts});
}