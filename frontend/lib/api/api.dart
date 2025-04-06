import 'dart:async';
import 'models/course.dart';
import 'models/preferences.dart';
import 'models/schedule.dart';

// API interface - simplified version with required functions
class Api {
  // Test bridge connection
  Future<String> helloFromRust() async {
    await Future.delayed(Duration(milliseconds: 500));
    return "Hello from Rust!";
  }

  // Get all available courses
  Future<List<Course>> getAllCourses() async {
    await Future.delayed(Duration(seconds: 1));
    return [
      Course(id: 'CSE101', title: 'Introduction to Computer Science'),
      Course(id: 'MATH201', title: 'Calculus II'),
      Course(id: 'ENG105', title: 'Academic Writing'),
    ];
  }

  // Get student preferences
  Future<StudentPreferences> getStudentPreferences(int studentId) async {
    await Future.delayed(Duration(seconds: 1));
    return StudentPreferences(
      studentId: studentId,
      preferredTime: 'Morning',
      preferOnline: false,
    );
  }

  // Store student preferences
  Future<void> storeStudentPreferences(StudentPreferences preferences) async {
    await Future.delayed(Duration(seconds: 1));
    print("Stored preferences");
  }

  // Get semester plan
  Future<SemesterPlan> getSemesterPlan(int studentId) async {
    await Future.delayed(Duration(seconds: 1));
    return SemesterPlan(
      studentId: studentId,
      entries: [
        ScheduleEntry(
          courseId: 'CSE101',
          courseTitle: 'Introduction to Computer Science',
          day: 'Mon',
          startTime: '10:00',
          endTime: '11:30',
        ),
        ScheduleEntry(
          courseId: 'MATH201',
          courseTitle: 'Calculus II',
          day: 'Tue',
          startTime: '13:00',
          endTime: '14:30',
        ),
        ScheduleEntry(
          courseId: 'ENG105',
          courseTitle: 'Academic Writing',
          day: 'Wed',
          startTime: '09:00',
          endTime: '10:30',
        ),
      ],
    );
  }

  // Check schedule conflicts
  Future<ScheduleConflictResult> checkScheduleConflicts(SemesterPlan plan) async {
    await Future.delayed(Duration(seconds: 1));
    return ScheduleConflictResult(
      conflicts: {
        // Example conflict
        'CSE101': ['PHYS150'],
      },
    );
  }

  // Generate schedule suggestion
  Future<SemesterPlan> suggestSchedule(int studentId) async {
    await Future.delayed(Duration(seconds: 1));
    return SemesterPlan(
      studentId: studentId,
      entries: [
        ScheduleEntry(
          courseId: 'CSE101',
          courseTitle: 'Introduction to Computer Science',
          day: 'Mon',
          startTime: '10:00',
          endTime: '11:30',
        ),
        ScheduleEntry(
          courseId: 'MATH201',
          courseTitle: 'Calculus II',
          day: 'Tue',
          startTime: '13:00',
          endTime: '14:30',
        ),
        ScheduleEntry(
          courseId: 'ENG105',
          courseTitle: 'Academic Writing',
          day: 'Wed',
          startTime: '09:00',
          endTime: '10:30',
        ),
      ],
    );
  }

  // Store schedule
  Future<void> storeSchedule(int studentId, SemesterPlan plan) async {
    await Future.delayed(Duration(seconds: 1));
    print("Stored schedule");
  }
}