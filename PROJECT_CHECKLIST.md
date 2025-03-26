# 🚀 Project Checklist

This document tracks all steps and objectives required to take the **Class Scheduler App** into production across desktop, mobile, and (optionally) web platforms.

---

## 🧱 1. Core Architecture Setup

| Task                                                   | Status     | Notes |
|--------------------------------------------------------|------------|-------|
| Project structure (`backend/`, `frontend/`, `data/`)   | ✅ Done     | Folder scaffolding complete |
| Flutter frontend created (`flutter create`)            | ✅ Done     | Flutter CLI used to scaffold app |
| Rust backend initialized (`cargo init`)                | ✅ Done     | Rust project initialized |
| `flutter_rust_bridge` configured                       | ✅ Done     | Installed in both Flutter and Rust |
| JSON data normalized & loaded into Rust                | ✅ Done     | Using `serde_json` |
| Dart ↔ Rust integration working                        | 🟡 In Progress | `flutter_rust_bridge_codegen` pending |
| Sample FFI function implemented (`hello_from_rust`)    | ✅ Done     | Confirmed callable |

---

## 🔌 2. Cross-Platform Compatibility

| Platform         | Ready?      | Notes |
|------------------|-------------|-------|
| 🖥️ Desktop (macOS/Windows/Linux) | ✅ Yes     | Runs via `flutter run -d macos` etc. |
| 📱 Mobile (iOS/Android)           | 🟡 Almost  | Needs emulator/device testing |
| 🌐 Web (Flutter Web)              | 🔴 Not Yet | Requires API bridge or WASM support |

---

## 🧪 3. Development Quality

| Task                              | Status     | Notes |
|-----------------------------------|------------|-------|
| `.gitignore` configured           | ✅ Done     | Covers Rust + Flutter |
| `README.md` written               | ✅ Done     | Includes setup, structure |
| Unit tests (Rust + Flutter)       | 🔴 Not Yet | Needed for logic verification |
| Logging & error handling (Rust)   | 🟡 In Progress | Add `log`, `anyhow` crates |
| Clean separation of logic/UI      | 🟡 Partial  | More modularity as app grows |

---

## 📦 4. App Build & Packaging

| Task                                 | Status     | Notes |
|--------------------------------------|------------|-------|
| Flutter desktop/mobile build         | 🟡 Soon     | Confirm `flutter build` output |
| Flutter release mode (`--release`)   | 🔴 Not Yet | Needed for production testing |
| Rust release build                   | 🔴 Not Yet | Use `cargo build --release` |
| Asset bundling + FFI verification    | 🔴 Not Yet | Ensure `bridge_generated.dart` built |

---

## 🔐 5. Security & Stability

| Task                                  | Status     | Notes |
|---------------------------------------|------------|-------|
| Input validation (Rust/Dart)          | 🔴 Not Yet | Guard inputs and schedule logic |
| Error handling (try/catch, Result)    | 🟡 Partial  | Add user-friendly error handling |
| Secure local data storage (UnQLite)   | 🔴 Not Yet | Add encryption or export protection if needed |
| API protection (for web mode)         | 🔴 Not Yet | Add CORS & rate limiting if needed |

---

## ☁️ 6. Web Support Plan (Optional)

| Task                                   | Status     | Notes |
|----------------------------------------|------------|-------|
| Design API-based backend (e.g. Axum)   | 🟡 Planned  | Convert core logic into HTTP handlers |
| Host Rust API (e.g. Railway, Render)   | ⬜️ Planned | Connect Flutter Web to backend |
| Enable Flutter web build               | ✅ Done     | `flutter config --enable-web` |
| Enable CORS and HTTPS                  | ⬜️ Planned | Needed for production access |

---

## 🚀 7. Release & Deployment

| Task                              | Status     | Notes |
|-----------------------------------|------------|-------|
| Android signing + keystore setup | ⬜️ Planned | Use `keytool`, add to `key.properties` |
| Desktop installer packaging       | ⬜️ Planned | DMG, EXE, or AppImage |
| CI/CD setup (GitHub Actions)      | ⬜️ Planned | Automate `flutter build` and `cargo test` |
| Release tagging / changelog       | ⬜️ Planned | `git tag` and `CHANGELOG.md` |
| Publish to GitHub       | ⬜️ Planned | First stable release |

---

## ✅ Status Legend

| Emoji | Meaning     |
|--------|--------------|
| ✅     | Complete     |
| 🟡     | In Progress  |
| 🔴     | Not Started  |
| ⬜️     | Planned      |

---

_Keep this file updated as the project matures. Production readiness is a process — and this checklist will keep your team_
