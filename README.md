# GWU Class Scheduler App

This project is a comprehensive application for managing GWU class schedules, student preferences, and semester planning.

It uses:
- 📄 **HTML/CSS/JavaScript** frontend
- 🦀 **Rust** backend server
- 🗃️ **JSON** files for data storage

---

## 📁 Project Structure

```plaintext
frontend/
  ├── index.html      # Main web page
  ├── styles.css      # CSS styles
  ├── script.js       # JavaScript frontend logic
backend/
  ├── Cargo.toml      # Rust project configuration
  ├── src/
      ├── main.rs     # Main Rust server logic
      ├── models.rs   # Data models/structures
      └── utils.rs    # Utility functions
data/
  ├── courses.json           # GWU Computer Science courses
  ├── students.json          # Student profiles
  ├── preferences.json       # Student course preferences
  ├── semester_plans.json    # Generated schedules
  └── constraints.json       # Student scheduling constraints
```

## 🚀 Running the Application

### Backend Setup

1. Make sure you have Rust installed (https://www.rust-lang.org/tools/install)
2. Navigate to the backend directory:
   ```
   cd backend
   ```
3. Build and run the backend server:
   ```
   cargo run
   ```
4. The backend server will start on http://localhost:8080

### Frontend Setup

#### Option 1: Open directly in browser
1. Navigate to the frontend directory
2. Open `index.html` in your web browser

#### Option 2: Use a local server
1. Navigate to the frontend directory:
   ```
   cd frontend
   ```
2. Start a simple HTTP server (if you have Python installed):
   ```
   python -m http.server 8000   # Python 3
   # OR
   python -m SimpleHTTPServer 8000   # Python 2
   ```
3. Open your browser and go to http://localhost:8000

## 📚 Features

- **Course Browsing**: View available GWU Computer Science courses
- **Schedule Management**: Create and visualize weekly class schedules
- **Preference Setting**: Specify preferred courses and unavailable times
- **Automatic Scheduling**: Generate optimal schedules based on preferences
- **Conflict Detection**: Identify and prevent time conflicts

## 💾 Data

The application uses real GWU Computer Science course information including:
- CSCI6221 - Advanced Software Paradigms
- CSCI6212 - Algorithms
- CSCI6461 - Computer Architecture
- And more graduate CS courses

## 🧪 Testing

To verify the system is working correctly:
1. Start the backend server
2. Check the health endpoint at http://localhost:8080/health
3. Open the frontend and verify courses are displayed
4. Test creating schedules and setting preferences
