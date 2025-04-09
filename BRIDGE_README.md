# Flutter-Rust Bridge Integration

This document explains how the Flutter frontend and Rust backend are integrated using `flutter_rust_bridge`.

## Overview

The integration is accomplished through the following components:

1. **Rust Backend API** (`backend/src/api.rs`): Contains all API functions annotated with `#[frb]` to expose them to Flutter.
2. **Generated Bridge Code** (`frontend/lib/api/generated/`): Contains the generated Dart code that provides type-safe access to Rust functions.
3. **Bridge Implementation** (`frontend/lib/api/bridge.dart`): Provides a clean API interface for Flutter to call Rust functions.

## Directory Structure

- `backend/src/api.rs` - Rust API functions exposed to Flutter
- `backend/src/lib.rs` - Model definitions and helper functions
- `frontend/lib/api/bridge.dart` - Bridge initialization and API implementation
- `frontend/lib/api/generated/` - Generated bridge code
  - `bridge_generated.dart` - Core bridge implementation
  - `bridge_definitions.dart` - Type definitions generated from Rust

## Generating the Bridge Code

The bridge code is generated using `flutter_rust_bridge_codegen`. To generate:

```bash
# Run the generation script
./generate_bridge.sh
```

This script will:
1. Install `flutter_rust_bridge_codegen` if not already installed
2. Generate the bridge code
3. Build the Rust library

## Usage in Flutter

The bridge is initialized in your Flutter application as follows:

```dart
import 'package:frontend/api/bridge.dart';

void main() async {
  // Initialize the bridge
  await initRustBridge();
  
  // Now you can use the global 'api' object
  final courses = await api.getAllCourses();
  print("Courses: $courses");
  
  runApp(MyApp());
}
```

## Available API Functions

The following functions are available through the bridge:

- `helloFromRust()` - Test the bridge connection
- `getAllCourses()` - Get all available courses
- `getStudentPreferences(studentId)` - Get a student's preferences
- `getSemesterPlan(studentId)` - Get a student's semester plan
- `suggestSchedule(studentId)` - Generate a suggested schedule
- `checkScheduleConflicts(plan)` - Check for conflicts in a schedule
- `storeSchedule(studentId, plan)` - Save a schedule
- `getProfessors()` - Get all professors
- `getConstraints()` - Get all constraints
- `getStudentConstraints(studentId)` - Get constraints for a student
- `validateScheduleAgainstConstraints(plan)` - Validate a schedule against constraints

## Error Handling

All Rust functions return `ApiResult<T>` which is mapped to Dart's `Future<T>`. Errors from Rust are properly propagated to Dart as exceptions.

## Platform Support

The bridge supports the following platforms:
- macOS
- Windows
- Linux
- Android
- iOS

## Troubleshooting

If you encounter issues with the bridge:

1. Ensure you have the latest version of `flutter_rust_bridge_codegen`
2. Check that your Rust API functions are properly annotated with `#[frb]`
3. Make sure the generated code is up to date by running `./generate_bridge.sh`
4. Verify that the Rust library is built for your target platform