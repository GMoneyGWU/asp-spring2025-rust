import 'package:flutter/material.dart';
import 'course_list_screen.dart';
import 'preferences_screen.dart';
import 'schedule_grid_screen.dart';

class HomeScreen extends StatefulWidget {
  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {

  // Error message storage for application-wide errors
  String? errorMessage;

  // Track active tab index
  int selectedIndex = 0;

  // Define screens for tab navigation
  final List<Widget> screens = [
    CourseListScreen(),
    PreferencesScreen(),
    ScheduleGridScreen(),
  ];

  // Handle tab selection
  void onTabTapped(int index) {
    setState(() => selectedIndex = index);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Class Scheduler')),
      body: Column(
        children: [

          // Error message display bar
          if (errorMessage != null)
            Container(
              color: Colors.red,
              padding: EdgeInsets.all(8),
              child: Row(
                children: [
                  Icon(Icons.error, color: Colors.white),
                  SizedBox(width: 8),
                  Expanded(child: Text(errorMessage!, style: TextStyle(color: Colors.white))),
                  IconButton(
                    icon: Icon(Icons.close, color: Colors.white),
                    onPressed: () => setState(() => errorMessage = null),
                  )
                ],
              ),
            ),

          // Display active tab content
          Expanded(child: screens[selectedIndex]),
        ],
      ),

      // Bottom navigation with 3 main sections
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: selectedIndex,
        onTap: onTabTapped,
        items: [
          BottomNavigationBarItem(icon: Icon(Icons.list), label: 'Courses'),
          BottomNavigationBarItem(icon: Icon(Icons.settings), label: 'Preferences'),
          BottomNavigationBarItem(icon: Icon(Icons.schedule), label: 'Schedule'),
        ],
      ),
    );
  }
}