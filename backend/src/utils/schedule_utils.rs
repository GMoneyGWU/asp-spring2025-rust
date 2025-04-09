use chrono::NaiveTime;
use crate::models::models::{Schedule, TimeSlot};

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