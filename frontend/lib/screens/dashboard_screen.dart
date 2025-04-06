import 'package:flutter/material.dart';
import 'home_screen.dart';

class DashboardScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Student Dashboard')),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [

              Text('Welcome to the Scheduler App',
                  style: Theme.of(context).textTheme.titleLarge),
              SizedBox(height: 20),

              // Navigation button to main functionality
              ElevatedButton(
                onPressed: () => Navigator.push(
                    context,
                    MaterialPageRoute(builder: (_) => HomeScreen())
                ),
                child: Text('Go to Course Planner'),
              )
            ],
          ),
        ),
      ),
    );
  }
}