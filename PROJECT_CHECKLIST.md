# ✅ Class Scheduler App — Project Checklist

This checklist reflects the **real status** of your project as of the current state of files and implementation.

---

## 📁 1. Project Setup & Scaffolding

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create project folders: `backend/`, `frontend/`, `data/` | ✅ Done     | Scaffolded and structured |
| Initialize Rust project (`cargo init backend`)         | ✅ Done     | `main.rs` contains placeholder |
| Create Flutter frontend (`flutter create frontend`)    | ✅ Done     | Generated and structured |
| Normalize JSON files                                   | ✅ Done     | 10 JSON files structured |
| Move JSON files to `backend/data/`                     | ✅ Done     | Verified present |
| Set up `.gitignore`                                    | ✅ Done     | Present and working |

---

## 🔧 2. Dependency Configuration

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Add Rust dependencies: `serde`, `serde_json`, `unqlite`, `flutter_rust_bridge` | ✅ Done | Present in `Cargo.toml` |
| Add Flutter dependency: `flutter_rust_bridge`          | ✅ Done     | Present in `pubspec.yaml` |
| Install `flutter_rust_bridge_codegen`                  | ✅ Done     | Mentioned in README |
| Add optional crates (e.g. `anyhow`, `log`)             | ⬜️ Planned | Not yet implemented |

---

## 🛠️ 3. Data Parsing & Model Layer

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create Rust structs using `serde` for JSON files       | 🔜 Todo     | No struct definitions found yet |
| Parse JSON files using `serde_json`                    | 🔜 Todo     | Not implemented in `main.rs`, `lib.rs`, or `api.rs` |
| Load data into structs and test parsing                | 🔜 Todo     | Not yet developed |
| Create Dart models (if needed)                         | ⬜️ Planned | Optional depending on design |

---

## 🔌 4. Flutter ↔ Rust Bridge Integration

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create `bridge.rs` and stub function                   | 🔴 Not Yet | `api.rs` is empty |
| Generate `bridge_generated.dart` with codegen          | 🔜 Todo     | Should be done after API exists |
| Call Rust function from Flutter                        | 🔜 Todo     | Requires a working bridge function |
| Create API function: `get_courses()` or similar        | ⬜️ Planned | Part of bridge work |
| Handle bridge errors and return types                  | ⬜️ Planned | Use `Result<T, E>` in Rust and async in Dart |

---

## 🧠 5. Backend Function Roadmap (Rust)

These are the core functions that will live in `src/api.rs`, exposed via `flutter_rust_bridge`.

| Function                            | Purpose                                              | Status     |
|-------------------------------------|------------------------------------------------------|------------|
| `hello_from_rust()` | Test bridge connection (demo) | 🔴 Not Started |
| `get_all_courses()`                 | Return full list of courses from JSON                | ⬜️ Planned |
| `get_student_preferences(student_id)` | Load preferences by student ID                      | ⬜️ Planned |
| `get_degree_requirements(major_id)` | Fetch degree rules                                   | ⬜️ Planned |
| `get_semester_plan(student_id)`     | Return student's planned schedule                    | ⬜️ Planned |
| `suggest_schedule(student_id)`      | Core function — generate schedule from inputs        | ⬜️ Planned |
| `check_schedule_conflicts(plan)`    | Returns true/false or conflict details               | ⬜️ Planned |
| `store_schedule(student_id, plan)`  | Save new schedule plan to database/file              | ⬜️ Planned |
| `get_professors()`                  | Return list of professors                            | ⬜️ Planned |
| `get_constraints()`                 | Return scheduling constraints                        | ⬜️ Planned |
| `load_json_file(file_name)`         | Internal helper for deserializing models             | ⬜️ Planned |
| `validate_schedule_against_constraints()` | Business logic validator                         | ⬜️ Planned |

---

## 🖼️ 6. Frontend Function Roadmap (Flutter/Dart)

These Dart-side functions call Rust APIs and control the user interface.

| Function / Widget                    | Purpose                                             | Status     |
|-------------------------------------|-----------------------------------------------------|------------|
| `initRustBridge()`                  | Initializes `bridge_generated.dart`                 | 🔜 Todo     |
| `fetchAllCourses()`                 | Calls Rust `get_all_courses()`                      | ⬜️ Planned |
| `renderCourseList()`                | Displays course list in scrollable UI               | ⬜️ Planned |
| `displayCourseDetails(courseId)`    | Opens modal with course info                        | ⬜️ Planned |
| `collectStudentPreferences()`       | Form to collect preferred times, formats, etc.      | ⬜️ Planned |
| `submitPreferencesToRust()`        | Sends prefs to backend                              | ⬜️ Planned |
| `triggerScheduleSuggestion()`       | Calls `suggest_schedule()`                          | ⬜️ Planned |
| `renderScheduleGrid()`              | Renders schedule plan visually                      | ⬜️ Planned |
| `highlightConflicts()`              | Shows visual feedback on overlapping classes        | ⬜️ Planned |
| `saveSemesterPlan()`                | Sends updated schedule to backend for persistence   | ⬜️ Planned |
| `loadStudentData()`                 | Loads profile, preferences, and prior history       | ⬜️ Planned |
| `errorBanner(message)`              | User-visible error UI                               | ⬜️ Planned |

---

## 📌 Notes
- All Rust functions should be exposed with `#[frb]` in `api.rs`
- All functions returning structs or vectors must be `serde` serializable
- Dart functions should use `async` and `await` with proper error handling

---

## 🧱 7. UI & Application Logic (Flutter + Rust)

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Build course browsing UI in Flutter                    | 🔜 Todo     | `main.dart` is still default scaffold |
| Create student dashboard screen                        | ⬜️ Planned | No widgets beyond starter code |
| Add course detail view/modal                           | ⬜️ Planned | UI planning needed |
| Add preferences input UI                               | ⬜️ Planned | Preferences model exists in data only |
| Implement Rust logic: conflict detection               | ⬜️ Planned | To be built based on schedule JSON |
| Implement Rust logic: schedule suggestion              | ⬜️ Planned | Requires constraints, preferences, and requirements logic |
| Return suggestions from Rust to Flutter                | ⬜️ Planned | Part of FFI layer |
| Render recommended schedule in Flutter                 | ⬜️ Planned | Connect to API response data |

---

## 🧪 8. Testing & Quality Assurance

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Add Rust unit tests for logic                          | ⬜️ Planned | Testing framework not yet initialized |
| Add Flutter unit/widget tests                          | ⬜️ Planned | Only one auto-generated test exists |
| Handle invalid JSON / input data                       | ⬜️ Planned | Add `anyhow` and input validation logic |
| Add logging (`log`, `env_logger`)                      | ⬜️ Planned | Good for debugging during parsing/testing |

---

## 🌍 9. Platform Testing

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Test on macOS/Linux/Windows desktop   | 🔜 Todo     | Should verify with `flutter run -d macos` |
| Test on Android/iOS mobile            | ⬜️ Planned | No sign of platform-specific configs yet |
| Confirm FFI works across platforms    | ⬜️ Planned | No function calls defined yet |
| Web compatibility (API/WASM)          | ⬜️ Optional | Will require API split if pursued |

---

## 📦 10. Build & Deployment

| Task                                           | Status     | Notes |
|------------------------------------------------|------------|-------|
| Build Rust backend in release mode            | ⬜️ Planned | `cargo build --release` not tested |
| Build Flutter frontend in release mode        | ⬜️ Planned | `flutter build` not tested |
| Package mobile build (signing configs)        | ⬜️ Planned | Requires key config |
| Package desktop builds (DMG, EXE, AppImage)   | ⬜️ Planned | `flutter build macos` etc. |
| Set up CI (GitHub Actions, tests, linting)    | ⬜️ Planned | Add `.github/workflows` |

---

## 🧾 11. Documentation & Dev Experience

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Finalize `README.md` with examples    | ✅ Done     | Explains architecture, structure |
| Create `PRODUCTION_CHECKLIST.md`     | ✅ Done     | Current file |
| Add `LICENSE` (MIT, Apache, etc.)    | ⬜️ Planned | Needed before publishing |
| Create `CONTRIBUTING.md`             | ⬜️ Planned | Optional for team onboarding |
| Add usage/setup guide for new devs   | ⬜️ Planned | Could live in `/docs` or `README` |

---

## 🚀 12. Release Milestone

| Task                            | Status     | Notes |
|---------------------------------|------------|-------|
| Freeze MVP scope                | ⬜️ Planned | Finalize core features for v1 |
| Tag initial release (`v1.0.0`)  | ⬜️ Planned | Create Git tag + changelog |
| Public GitHub repo setup        | ⬜️ Planned | Add License, README badges |
| Publish or distribute builds    | ⬜️ Planned | Android APK, desktop installer, etc. |

---

## ✅ Status Legend

| Emoji | Meaning         |
|--------|------------------|
| ✅     | Complete         |
| 🔜     | Todo (Immediate) |
| 🔴     | Not Started      |
| ⬜️     | Planned (Later)  |

---

_This checklist reflects the current progress and what needs to happen next. Use it for weekly planning, GitHub issues, and milestone tracking._
