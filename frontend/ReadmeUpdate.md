This document provides a detailed explanation of the Class Scheduler Flutter application's frontend structure. The app allows students to view courses, set preferences, and generate/manage their class schedules.



## Architecture Overview

The application follows a layered architecture with:

1. UI Screens - Flutter widgets that handle user interactions
2. API Bridge - Interface between the Flutter UI and Rust backend
3. Data Models - Structured representations of the application data



```
lib/
│
├── main.dart                     
├── api/
│   ├── api.dart                  
│   ├── bridge.dart               
│   └── models/                   
│       ├── course.dart
│       ├── preferences.dart
│       └── schedule.dart
└── screens/
    ├── dashboard_screen.dart
    ├── home_screen.dart
    ├── course_list_screen.dart
    ├── preferences_screen.dart
    └── schedule_grid_screen.dart
```



## Backend Integration Guide

### Key Integration Points for Backend Developers

1. **Rust Bridge Initialization**

   - The application initializes a connection to the Rust backend in `initRustBridge()` in `bridge.dart`.
   - This function should be implemented to establish communication between Flutter and Rust.

2. **API Functions to Implement**

   - The `Api` class in `api.dart` defines all required API functions that need to be implemented in Rust.
   - Currently, these functions return mock data, but should be connected to actual Rust implementations.

3. **Data Models**

   - The data models in the `models` directory define the structure of data exchanged between Flutter and Rust.
   - Ensure your Rust implementations match these data structures for seamless integration.

4. **Key Backend Functions**

   a. **Course Management**

   - `getAllCourses()`: Retrieves all available courses.

   b. **Student Preferences**

   - `getStudentPreferences(studentId)`: Retrieves stored preferences for a student.
   - `storeStudentPreferences(preferences)`: Saves student preferences to backend.

   c. **Schedule Management**

   - `getSemesterPlan(studentId)`: Retrieves stored schedule for a student.
   - `checkScheduleConflicts(plan)`: Analyzes a schedule for time conflicts.
   - `suggestSchedule(studentId)`: Generates a suggested schedule based on student preferences.
   - `storeSchedule(studentId, plan)`: Saves a schedule to backend.

5. **Expected Response Data**

   - All API methods should return data structured according to the models defined in the `models` directory.
   - Ensure error handling is consistent with Flutter's exception model.

6. **Hardcoded Values to Replace**

   - Current implementation uses hardcoded `studentId = 1`. This should be replaced with actual user authentication.
   - Sample course data, preferences, and schedules should be replaced with database-backed implementations.

## Implementation Notes

1. The application currently uses mocked data with artificial delays to simulate backend communication.
2. The Rust bridge is not actually initialized - this needs to be implemented.
3. The UI expects data in the exact format specified by the model classes.