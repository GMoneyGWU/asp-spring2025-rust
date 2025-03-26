# 📚 Class Scheduler App

A cross-platform, high-performance **class scheduling application** that leverages a **Flutter frontend** and a **Rust backend**, integrated via `flutter_rust_bridge`. This app is designed to manage course schedules, availability, constraints, preferences, and degree planning for students, instructors, and academic institutions.

---

## 🧩 Architecture

```plaintext
Flutter (Dart) UI
│
│  via flutter_rust_bridge
▼
Rust Backend
│
├── Business Logic
├── Constraint Handling
└── Persistence Layer (UnQLite / JSON files)

## 📦 Project Structure

```plaintext
scheduler_app/
├── backend/               # Rust crate
│   ├── src/
│   ├── data/
│   └── Cargo.toml
├── frontend/              # Flutter app
│   └── pubspec.yaml
└── README.md

## 🔧 Dependencies

### Rust (`backend/Cargo.toml`)
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
flutter_rust_bridge = "1.79"
unqlite = "1.0.0"

### Flutter (`frontend/pubspec.yaml`)
```yaml
dependencies:
  flutter:
    sdk: flutter
  flutter_rust_bridge: ^2.9.0