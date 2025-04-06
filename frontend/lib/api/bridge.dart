import 'dart:async';
import 'api.dart';

// Global API instance accessible throughout the app
late final Api api;

// Initialize the Rust bridge for backend communication
Future<void> initRustBridge() async {
  // In the actual implementation, this would initialize Rust bridge
  // Currently using mock implementation
  api = Api();

  // Test connection to Rust backend
  try {
    final greeting = await api.helloFromRust();
    print("Rust bridge initialized: $greeting");
  } catch (e) {
    print("Error initializing Rust bridge: $e");
  }
}