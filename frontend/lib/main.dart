import 'package:flutter/material.dart';
import 'api/bridge.dart';
import 'screens/dashboard_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Rust bridge for communication with backend
  await initRustBridge();
  runApp(SchedulerApp());
}

class SchedulerApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Class Scheduler',
      debugShowCheckedModeBanner: false,

      // Application-wide theme configuration
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.teal,
          brightness: Brightness.light,
        ),

        // Various component-specific theme settings
        scaffoldBackgroundColor: Colors.white,
        appBarTheme: AppBarTheme(
          backgroundColor: Colors.teal.shade100,
          foregroundColor: Colors.black87,
        ),

        // Bottom navigation theme configuration
        bottomNavigationBarTheme: BottomNavigationBarThemeData(
          backgroundColor: Colors.teal.shade100,
          selectedItemColor: Colors.teal.shade900,
          unselectedItemColor: Colors.grey[600],
        ),

        // Button style configuration
        elevatedButtonTheme: ElevatedButtonThemeData(
          style: ElevatedButton.styleFrom(
            backgroundColor: Colors.teal,
            foregroundColor: Colors.white,
            textStyle: TextStyle(fontWeight: FontWeight.bold),
          ),
        ),

        // Other theme configurations
        checkboxTheme: CheckboxThemeData(
          fillColor: MaterialStateProperty.all(Colors.teal),
        ),
        dropdownMenuTheme: DropdownMenuThemeData(
          menuStyle: MenuStyle(
            backgroundColor: MaterialStateProperty.all(Colors.white),
          ),
        ),
        snackBarTheme: SnackBarThemeData(
          backgroundColor: Colors.teal,
          contentTextStyle: TextStyle(color: Colors.white),
        ),
      ),

      // Initial screen to display
      home: DashboardScreen(),
    );
  }
}