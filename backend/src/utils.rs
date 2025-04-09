use chrono::NaiveTime;
use std::fs;
use std::path::Path;
use log::error;

use crate::models::{Schedule, TimeSlot, Course};

// Parse time string in format "HH:MM" to NaiveTime
pub fn parse_time(time_str: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M").ok()
}

// Check if two time slots overlap
pub fn time_slots_overlap(slot1: &TimeSlot, slot2: &TimeSlot) -> bool {
    if slot1.day != slot2.day {
        return false;
    }

    let start1 = match parse_time(&slot1.start_time) {
        Some(time) => time,
        None => return false,
    };

    let end1 = match parse_time(&slot1.end_time) {
        Some(time) => time,
        None => return false,
    };

    let start2 = match parse_time(&slot2.start_time) {
        Some(time) => time,
        None => return false,
    };

    let end2 = match parse_time(&slot2.end_time) {
        Some(time) => time,
        None => return false,
    };

    // Check for overlap
    start1 < end2 && start2 < end1
}

// Check if a schedule has time conflicts
pub fn check_schedule_conflicts(schedule: &Schedule) -> Vec<(String, String)> {
    let mut conflicts = Vec::new();

    for (i, course1) in schedule.courses.iter().enumerate() {
        for course2 in schedule.courses.iter().skip(i + 1) {
            if time_slots_overlap(&course1.time_slot, &course2.time_slot) {
                conflicts.push((course1.course_id.clone(), course2.course_id.clone()));
            }
        }
    }

    conflicts
}

// Load data from JSON files
pub fn load_json_file<T: serde::de::DeserializeOwned>(file_path: &str) -> Option<T> {
    if !Path::new(file_path).exists() {
        error!("File does not exist: {}", file_path);
        return None;
    }

    match fs::read_to_string(file_path) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(parsed) => Some(parsed),
                Err(e) => {
                    error!("Error parsing JSON from {}: {}", file_path, e);
                    None
                }
            }
        },
        Err(e) => {
            error!("Error reading file {}: {}", file_path, e);
            None
        }
    }
}

// Generate a simple unique ID
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    format!("{}", now)
}