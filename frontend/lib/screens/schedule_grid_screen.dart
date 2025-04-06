import 'package:flutter/material.dart';
import '../api/bridge.dart';
import '../api/models/schedule.dart';

class ScheduleGridScreen extends StatefulWidget {
  @override
  State<ScheduleGridScreen> createState() => _ScheduleGridScreenState();
}

// Store schedule entries
// Store schedule conflicts
// Loading state tracker
class _ScheduleGridScreenState extends State<ScheduleGridScreen> {
  List<ScheduleEntry> schedule = [];
  Map<String, List<String>> conflicts = {};
  bool isLoading = true;

  // Load existing schedule when screen initializes
  @override
  void initState() {
    super.initState();
    loadSemesterPlan();
  }

  // Load saved semester plan from backend
  // Get saved schedule plan
  // Check for conflicts in the schedule
  Future<void> loadSemesterPlan() async {
    try {
      setState(() => isLoading = true);
      final plan = await api.getSemesterPlan(1);
      final conflictResult = await api.checkScheduleConflicts(plan);
      setState(() {
        schedule = plan.entries;
        conflicts = conflictResult.conflicts;
        isLoading = false;
      });
    } catch (e) {
      print("Error: $e");
      setState(() => isLoading = false);
    }
  }


  // Request schedule generation from backend algorithm
  // Get suggested schedule based on preferences
  Future<void> triggerScheduleSuggestion() async {
    try {
      setState(() => isLoading = true);
      final suggestion = await api.suggestSchedule(1);
      setState(() {
        schedule = suggestion.entries;
        isLoading = false;
      });
    } catch (e) {
      print("Error: $e");
      setState(() => isLoading = false);
    }
  }

  // Save current schedule to backend
  // Create semester plan with current schedule
  // Send to backend via API bridge
  Future<void> saveSemesterPlan() async {
    try {
      final plan = SemesterPlan(
        studentId: 1,
        entries: schedule,
      );
      await api.storeSchedule(1, plan);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Schedule saved successfully.')),
      );
    } catch (e) {
      print("Error: $e");
    }
  }

  // Show loading indicator while fetching schedule
  // Display message and generate button if no schedule exists
  @override
  Widget build(BuildContext context) {
    if (isLoading) {
      return Center(child: CircularProgressIndicator(
        color: Theme.of(context).colorScheme.primary,
      ));
    }

    if (schedule.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('No schedule available. Please generate one.'),
            SizedBox(height: 16),
            ElevatedButton(
              onPressed: triggerScheduleSuggestion,
              child: Text('Generate Schedule'),
            )
          ],
        ),
      );
    }

    // Display schedule grid with actions
    return Column(
      children: [
        Expanded(
          child: GridView.count(
            crossAxisCount: 1,
            childAspectRatio: 5,
            children: schedule.map((entry) {
              final hasConflict = conflicts.containsKey(entry.courseId);
              return Card(
                margin: EdgeInsets.all(8),
                color: hasConflict ? Colors.red[50] : null,
                child: ListTile(
                  title: Text(entry.courseTitle),
                  subtitle: Text('${entry.day} ${entry.startTime}-${entry.endTime}'),
                  trailing: hasConflict
                      ? Icon(Icons.warning, color: Colors.red)
                      : Icon(Icons.check_circle, color: Colors.teal),
                ),
              );
            }).toList(),
          ),
        ),

        // Action buttons for schedule management
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: [
              ElevatedButton(
                onPressed: triggerScheduleSuggestion,
                child: Text('Generate New'),
              ),
              ElevatedButton(
                onPressed: saveSemesterPlan,
                child: Text('Save Schedule'),
              ),
            ],
          ),
        )
      ],
    );
  }
}