# ✅ Class Scheduler App — Sequential Project Checklist

---

## 📁 1. Project Setup & Scaffolding

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create project folders: `backend/`, `frontend/`, `data/` | ✅ Done     | Structured for Rust + Flutter separation |
| Initialize Rust project (`cargo init backend`)         | ✅ Done     | Basic `main.rs` and `Cargo.toml` created |
| Create Flutter frontend (`flutter create frontend`)    | ✅ Done     | Dart app created inside `frontend/` |
| Normalize JSON files                                   | ✅ Done     | Data structured and organized |
| Move JSON files to `backend/data/`                     | ✅ Done     | Files present but not parsed yet |
| Set up `.gitignore`                                    | ✅ Done     | Prevents build artifacts from being tracked |

---

## 🔧 2. Dependency Configuration

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Add Rust dependencies: `serde`, `serde_json`, `unqlite`, `flutter_rust_bridge` | ✅ Done | `Cargo.toml` configured |
| Add Flutter dependency: `flutter_rust_bridge`          | ✅ Done     | In `pubspec.yaml` |
| Install `flutter_rust_bridge_codegen`                  | ✅ Done     | For generating FFI bindings |
| Add other optional crates (e.g., `anyhow`, `log`)      | ⬜️ Planned | For error handling/logging |

---

## 🛠️ 3. Data Parsing & Model Layer

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create Rust structs using `serde` for JSON files       | 🔜 Todo     | Derive from normalized data |
| Parse JSON files using `serde_json`                    | 🔜 Todo     | Begin with `courses.json` |
| Load data into structs and confirm via `cargo run`     | 🔜 Todo     | Print or log parsed values |
| Create Dart models (optional if needed)                | ⬜️ Planned | For UI-side data handling |

---

## 🔌 4. Flutter ↔ Rust Bridge Integration

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Create `bridge.rs` and stub function                   | ✅ Done     | `hello_from_rust()` created |
| Generate `bridge_generated.dart` with codegen          | ✅ Done     | Connected to Flutter |
| Call Rust function from Flutter                        | ✅ Done     | Validated via console |
| Create API function: `get_courses()` or similar        | ⬜️ Planned | Expose course data via bridge |
| Handle bridge errors and return types                  | ⬜️ Planned | Use `Result` and error messages |

---

## 🧱 5. Core UI & Logic Implementation

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Build course browsing UI in Flutter                    | ⬜️ Planned | List of courses from Rust |
| Create student dashboard screen                        | ⬜️ Planned | View upcoming schedule |
| Add course detail view/modal                           | ⬜️ Planned | Show description, credits |
| Add preferences input UI (time, course prefs)          | ⬜️ Planned | Pass to backend |
| Implement Rust logic: conflict detection               | ⬜️ Planned | Compare time slots, constraints |
| Implement Rust logic: schedule suggestion              | ⬜️ Planned | Based on degree + preferences |
| Return suggestions from Rust to Flutter                | ⬜️ Planned | FFI with structured data |
| Render recommended schedule in Flutter grid/list       | ⬜️ Planned | Display based on return data |

---

## 🧪 6. Testing & Quality Assurance

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Add Rust unit tests for logic                          | ⬜️ Planned | Start with conflict checker |
| Add Flutter unit/widget tests                          | ⬜️ Planned | Schedule rendering |
| Handle invalid JSON / input data                       | ⬜️ Planned | Use `anyhow`, validate inputs |
| Add logging (`log`, `env_logger`)                      | ⬜️ Planned | Helpful during development |

---

## 🌍 7. Platform Testing

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Test on macOS/Linux/Windows desktop   | ✅ Done     | Using `flutter run -d macos` etc. |
| Test on Android/iOS mobile            | ⬜️ Planned | Emulator or device |
| Confirm FFI works across platforms    | ⬜️ Planned | Consistent bridge behavior |
| Web compatibility (API/WASM)          | ⬜️ Optional | Rust HTTP API or WASM later |

---

## 📦 8. Build & Deployment

| Task                                           | Status     | Notes |
|------------------------------------------------|------------|-------|
| Build Rust backend in release mode            | ⬜️ Planned | `cargo build --release` |
| Build Flutter frontend in release mode        | ⬜️ Planned | `flutter build` commands |
| Package mobile build (signing configs)        | ⬜️ Planned | Android/iOS release steps |
| Package desktop builds (DMG, EXE, AppImage)   | ⬜️ Planned | For public release |
| Set up CI (GitHub Actions, tests, linting)    | ⬜️ Planned | Automate builds/tests |

---

## 🧾 9. Documentation, Licensing & Cleanup

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Finalize README.md with examples      | ✅ Done     | Explains structure, build steps |
| Create `PRODUCTION_CHECKLIST.md`     | ✅ Done     | This file! |
| Add `LICENSE` (MIT, Apache, etc.)    | ⬜️ Planned | Required for open source |
| Add `CONTRIBUTING.md` (optional)     | ⬜️ Planned | Guidelines for collaborators |
| Add usage/setup guide for new devs   | ⬜️ Planned | `docs/SETUP.md` or wiki page |

---

## ✅ Status Legend

| Emoji | Meaning     |
|--------|--------------|
| ✅     | Complete     |
| 🔜     | Todo (Immediate) |
| ⬜️     | Planned (Later) |

---

_Use this file to track team progress and milestone completion. As you implement and test features, update the statuses and move forward with confidence._  
