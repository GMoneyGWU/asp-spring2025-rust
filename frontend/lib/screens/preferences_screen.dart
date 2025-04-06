import 'package:flutter/material.dart';
import '../api/bridge.dart';
import '../api/models/preferences.dart';

class PreferencesScreen extends StatefulWidget {
  @override
  State<PreferencesScreen> createState() => _PreferencesScreenState();
}

// Default preference values
class _PreferencesScreenState extends State<PreferencesScreen> {
  String preferredTime = 'Morning';
  bool preferOnline = false;

  @override
  // Load existing preferences when screen initializes
  void initState() {
    super.initState();
    loadStudentData();
  }

  // Load saved student preferences from backend
  Future<void> loadStudentData() async {
    try {
      final prefs = await api.getStudentPreferences(1);
      setState(() {
        preferredTime = prefs.preferredTime;
        preferOnline = prefs.preferOnline;
      });
    } catch (e) {
      print("Error: $e");
    }
  }

  // Submit updated preferences to backend
  // Create preferences object with current values
  // Send to backend via API bridge
  Future<void> submitPreferencesToRust() async {
    try {
      final preferences = StudentPreferences(
        studentId: 1,
        preferredTime: preferredTime,
        preferOnline: preferOnline,
      );
      await api.storeStudentPreferences(preferences);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Preferences saved'),
          backgroundColor: Theme.of(context).colorScheme.primary,
        ),
      );
    } catch (e) {
      print("Error: $e");
    }
  }

  // Time preference dropdown section
  // Online class preference checkbox
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Preferred Time of Day:'),
          DropdownButton<String>(
            value: preferredTime,
            onChanged: (value) => setState(() => preferredTime = value!),
            items: ['Morning', 'Afternoon', 'Evening']
                .map((e) => DropdownMenuItem(value: e, child: Text(e)))
                .toList(),
          ),
          Row(
            children: [
              Checkbox(
                value: preferOnline,
                onChanged: (value) => setState(() => preferOnline = value!),
              ),
              Text('Prefer Online Classes'),
            ],
          ),
          SizedBox(height: 16),
          ElevatedButton(
            onPressed: submitPreferencesToRust,
            child: Text('Submit Preferences'),
          ),
        ],
      ),
    );
  }
}