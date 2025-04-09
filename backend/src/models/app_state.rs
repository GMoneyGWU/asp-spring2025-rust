use std::sync::Mutex;
use crate::models::models::{Course, Student, Schedule};

// State for the application
pub struct AppState {
    pub courses: Mutex<Vec<Course>>,
    pub students: Mutex<Vec<Student>>,
    pub schedules: Mutex<Vec<Schedule>>,
}