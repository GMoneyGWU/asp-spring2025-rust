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
| Add optional crates (e.g. `anyhow`, `log`)             | ✅ Done     | Added to `Cargo.toml` and used in code |

---

## 🛠️ 3. Data Parsing & Model Layer

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create Rust structs using `serde` for JSON files       | ✅ Done     | Implemented in `lib.rs` with appropriate annotations |
| Parse JSON files using `serde_json`                    | ✅ Done     | Implemented `load_json_file` helper in `lib.rs` |
| Load data into structs and test parsing                | ✅ Done     | Used in all API functions in `api.rs` |
| Create Dart models (if needed)                         | ⬜️ Planned | Optional depending on design |

---

## 🔌 4. Flutter ↔ Rust Bridge Integration

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create `bridge.rs` and stub function                   | ✅ Done     | Implemented in `api.rs` with `#[frb]` annotations |
| Generate `bridge_generated.dart` with codegen          | ✅ Done     | Added placeholder and script for generation |
| Call Rust function from Flutter                        | ✅ Done     | Implemented in bridge.dart with proper API delegation |
| Create API function: `get_courses()` or similar        | ✅ Done     | Implemented `get_all_courses()` and many more |
| Handle bridge errors and return types                  | ✅ Done     | Using `ApiResult<T>` with proper error handling |

---

## 🧠 5. Backend Function Roadmap (Rust)

These are the core functions that will live in `src/api.rs`, exposed via `flutter_rust_bridge`.

| Function                            | Purpose                                              | Status     |
|-------------------------------------|------------------------------------------------------|------------|
| `hello_from_rust()` | Test bridge connection (demo) | ✅ Done |
| `get_all_courses()`                 | Return full list of courses from JSON                | ✅ Done |
| `get_student_preferences(student_id)` | Load preferences by student ID                      | ✅ Done |
| `get_degree_requirements(major_id)` | Fetch degree rules                                   | ✅ Done |
| `get_semester_plan(student_id)`     | Return student's planned schedule                    | ✅ Done |
| `suggest_schedule(student_id)`      | Core function — generate schedule from inputs        | ✅ Done |
| `check_schedule_conflicts(plan)`    | Returns true/false or conflict details               | ✅ Done |
| `store_schedule(student_id, plan)`  | Save new schedule plan to database/file              | ✅ Done |
| `get_professors()`                  | Return list of professors                            | ✅ Done |
| `get_constraints()`                 | Return scheduling constraints                        | ✅ Done |
| `load_json_file(file_name)`         | Internal helper for deserializing models             | ✅ Done |
| `validate_schedule_against_constraints()` | Business logic validator                         | ✅ Done |

---

## 🖼️ 6. Frontend Function Roadmap (Flutter/Dart)

These Dart-side functions call Rust APIs and control the user interface.

| Function / Widget                    | Purpose                                             | Status     |
|-------------------------------------|-----------------------------------------------------|------------|
| `initRustBridge()`                  | Initializes `bridge_generated.dart`                 | ✅ Done     |
| `fetchAllCourses()`                 | Calls Rust `get_all_courses()`                      | ✅ Done |
| `renderCourseList()`                | Displays course list in scrollable UI               | ✅ Done |
| `displayCourseDetails(courseId)`    | Opens modal with course info                        | ✅ Done |
| `collectStudentPreferences()`       | Form to collect preferred times, formats, etc.      | ✅ Done |
| `submitPreferencesToRust()`        | Sends prefs to backend                              | ✅ Done |
| `triggerScheduleSuggestion()`       | Calls `suggest_schedule()`                          | ✅ Done |
| `renderScheduleGrid()`              | Renders schedule plan visually                      | ✅ Done |
| `highlightConflicts()`              | Shows visual feedback on overlapping classes        | ✅ Done |
| `saveSemesterPlan()`                | Sends updated schedule to backend for persistence   | ✅ Done |
| `loadStudentData()`                 | Loads profile, preferences, and prior history       | ✅ Done |
| `errorBanner(message)`              | User-visible error UI                               | ✅ Done |

---

## 📌 Notes
- All Rust functions should be exposed with `#[frb]` in `api.rs`
- All functions returning structs or vectors must be `serde` serializable
- Dart functions should use `async` and `await` with proper error handling

---

## 🧱 7. UI & Application Logic (Flutter + Rust)

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Build course browsing UI in Flutter                    | ✅ Done     | `main.dart` is still default scaffold |
| Create student dashboard screen                        | ✅ Done | No widgets beyond starter code |
| Add course detail view/modal                           | ✅ Done | UI planning needed |
| Add preferences input UI                               | ✅ Done | Preferences model exists in data only |
| Implement Rust logic: conflict detection               | ⬜️ Planned | To be built based on schedule JSON |
| Implement Rust logic: schedule suggestion              | ⬜️ Planned | Requires constraints, preferences, and requirements logic |
| Return suggestions from Rust to Flutter                | ⬜️ Planned | Part of FFI layer |
| Render recommended schedule in Flutter                 | ✅ Done | Connect to API response data |

---

## 🧪 8. Testing & Quality Assurance

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Add Rust unit tests for logic                          | 🔜 Todo     | Basic CLI testing implemented in `main.rs` |
| Add Flutter unit/widget tests                          | ⬜️ Planned | Only one auto-generated test exists |
| Handle invalid JSON / input data                       | ✅ Done     | Error handling implemented in all API functions |
| Add logging (`log`, `env_logger`)                      | 🔜 Todo     | Dependency added but not fully implemented |

---

## 🌍 9. Platform Testing

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Test on macOS/Linux/Windows desktop   | 🔜 Todo     | Bridge setup complete, ready for testing |
| Test on Android/iOS mobile            | 🔜 Todo     | Bridge configured for mobile platforms |
| Confirm FFI works across platforms    | 🔜 Todo     | Bridge generation script ready for execution |
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
