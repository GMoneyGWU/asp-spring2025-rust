import 'package:flutter/material.dart';
import '../api/bridge.dart';
import '../api/models/course.dart';

class CourseListScreen extends StatefulWidget {
  @override
  State<CourseListScreen> createState() => _CourseListScreenState();
}

class _CourseListScreenState extends State<CourseListScreen> {

  // Store fetched courses and Loading state tracker
  List<Course> courses = [];
  bool isLoading = true;

  @override
  void initState() {
    super.initState();

    // Load courses when screen initializes
    fetchAllCourses();
  }

  // Fetch courses from backend via API bridge
  Future<void> fetchAllCourses() async {
    try {
      setState(() => isLoading = true);
      final result = await api.getAllCourses();
      setState(() {
        courses = result;
        isLoading = false;
      });
    } catch (e) {
      print("Error: $e");
      setState(() => isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {

    // Show loading indicator while fetching courses
    if (isLoading) {
      return Center(child: CircularProgressIndicator(
        color: Theme.of(context).colorScheme.primary,
      ));
    }

    // Display message if no courses are available
    if (courses.isEmpty) {
      return Center(child: Text('No courses available.'));
    }


    // Display cousrses' list
    return ListView.builder(
      itemCount: courses.length,
      itemBuilder: (context, index) {
        final course = courses[index];
        return ListTile(
          title: Text(course.title),
          subtitle: Text(course.id),
          onTap: () => displayCourseDetails(context, course),
        );
      },
    );
  }

  // Show dialog with course details when a course is tapped
  void displayCourseDetails(BuildContext context, Course course) {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: Text(course.title),
        content: Text('Course ID: ${course.id}'),
        actions: [
          TextButton(
            child: Text('Close'),
            onPressed: () => Navigator.of(context).pop(),
            style: TextButton.styleFrom(
              foregroundColor: Theme.of(context).colorScheme.primary,
            ),
          ),
        ],
      ),
    );
  }
}