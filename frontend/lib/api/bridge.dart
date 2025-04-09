import 'dart:async';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'api.dart';
import 'generated/bridge_generated.dart';

// Global API instance accessible throughout the app
late final Api api;

// Initialize the Rust bridge for backend communication
Future<void> initRustBridge() async {
  // This path varies depending on your specific configuration
  final rustLibraryPath = _getRustLibraryPath();
  
  // Initialize Flutter Rust Bridge
  final nativeApi = await loadNativeApi(rustLibraryPath);
  
  // Create and assign the API instance with the native implementation
  api = RustApi(nativeApi);

  // Test connection to Rust backend
  try {
    final greeting = await api.helloFromRust();
    print("Rust bridge initialized: $greeting");
  } catch (e) {
    print("Error initializing Rust bridge: $e");
  }
}

/// Get the appropriate Rust library path based on the platform
String _getRustLibraryPath() {
  if (Platform.isAndroid) {
    return 'libbackend.so';
  } else if (Platform.isIOS) {
    return 'backend.framework/backend';
  } else if (Platform.isMacOS) {
    return 'libbackend.dylib';
  } else if (Platform.isWindows) {
    return 'backend.dll';
  } else if (Platform.isLinux) {
    return 'libbackend.so';
  } else {
    throw UnsupportedError('Unsupported platform');
  }
}

/// Implementation of the API that delegates to the Rust backend
class RustApi implements Api {
  final NativeApi _nativeApi;
  
  RustApi(this._nativeApi);
  
  @override
  Future<String> helloFromRust() async {
    return await _nativeApi.helloFromRust();
  }
  
  @override
  Future<List<Course>> getAllCourses() async {
    final rustCourses = await _nativeApi.getAllCourses();
    return rustCourses.map((c) => Course(
      id: c.courseId.toString(),
      title: c.courseName,
    )).toList();
  }
  
  @override
  Future<StudentPreferences> getStudentPreferences(int studentId) async {
    final rustPrefs = await _nativeApi.getStudentPreferences(studentId.toString());
    
    // Determine preferred time based on most common time slot
    String preferredTime = "Afternoon";
    if (rustPrefs.preferredTimes.isNotEmpty) {
      final slot = rustPrefs.preferredTimes.first;
      final hour = int.tryParse(slot.startTime.split(':').first) ?? 12;
      preferredTime = hour < 12 ? "Morning" : "Afternoon";
    }
    
    return StudentPreferences(
      studentId: studentId,
      preferredTime: preferredTime,
      preferOnline: false, // This information is not in our Rust model
    );
  }
  
  @override
  Future<void> storeStudentPreferences(StudentPreferences preferences) async {
    // This would require additional implementation in the Rust backend
    // For now, just print a message
    print("Stored preferences (not implemented in Rust backend yet)");
  }
  
  @override
  Future<SemesterPlan> getSemesterPlan(int studentId) async {
    final rustPlan = await _nativeApi.getSemesterPlan(studentId.toString());
    
    return SemesterPlan(
      studentId: studentId,
      entries: rustPlan.courses.map((course) => ScheduleEntry(
        courseId: course.courseId.toString(),
        courseTitle: "Course ${course.courseId}", // We would need to fetch the course title
        day: course.dayOfWeek,
        startTime: course.timeSlot.startTime,
        endTime: course.timeSlot.endTime,
      )).toList(),
    );
  }
  
  @override
  Future<ScheduleConflictResult> checkScheduleConflicts(SemesterPlan plan) async {
    // Convert Flutter plan to Rust plan format
    final rustPlan = await _convertToRustPlan(plan);
    
    // Check conflicts with Rust backend
    final hasConflicts = await _nativeApi.checkScheduleConflicts(rustPlan);
    
    // Simple implementation - just reports if there are conflicts
    final conflicts = <String, List<String>>{};
    if (hasConflicts) {
      // In a real implementation, we would have more detailed conflict information
      if (plan.entries.length >= 2) {
        conflicts[plan.entries[0].courseId] = [plan.entries[1].courseId];
      }
    }
    
    return ScheduleConflictResult(conflicts: conflicts);
  }
  
  @override
  Future<SemesterPlan> suggestSchedule(int studentId) async {
    final rustPlan = await _nativeApi.suggestSchedule(studentId.toString());
    
    return SemesterPlan(
      studentId: studentId,
      entries: rustPlan.courses.map((course) => ScheduleEntry(
        courseId: course.courseId.toString(),
        courseTitle: "Course ${course.courseId}", // We would need to fetch the course title
        day: course.dayOfWeek,
        startTime: course.timeSlot.startTime,
        endTime: course.timeSlot.endTime,
      )).toList(),
    );
  }
  
  @override
  Future<void> storeSchedule(int studentId, SemesterPlan plan) async {
    // Convert Flutter plan to Rust plan format
    final rustPlan = await _convertToRustPlan(plan);
    
    // Store the plan with Rust backend
    await _nativeApi.storeSchedule(studentId.toString(), rustPlan);
  }
  
  // Helper method to convert Flutter SemesterPlan to Rust SemesterPlan
  Future<SemesterPlan> _convertToRustPlan(SemesterPlan plan) async {
    // In a real implementation, this would convert the Flutter plan to a Rust plan format
    // For now, this is a simplified version
    return SemesterPlan(
      studentId: plan.studentId,
      entries: plan.entries,
    );
  }
}