use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod api;

// Course-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub course_id: u32,
    pub course_name: String,
    pub credits: u32,
    pub department: String,
    pub semester_offered: String,
    pub prerequisites: Vec<u32>,
}

// Student-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub student_id: String,
    pub name: String,
    pub email: String,
    pub major_id: u32,
    pub graduation_semesters: u32,
    pub student_uuid: String,
    pub advisor_email: String,
}

// Preference-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeSlot {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentPreference {
    pub preference_id: String,
    pub student_id: String,
    pub preferred_times: Vec<TimeSlot>,
    pub preferred_courses: Vec<u32>,
}

// Constraint-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstraintValue {
    #[serde(rename = "type")]
    pub constraint_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_credits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub constraint_id: String,
    pub student_id: String,
    pub constraint_type: String,
    pub value: ConstraintValue,
}

// Degree requirement-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DegreeRequirement {
    pub requirement_id: u32,
    pub major_id: u32,
    pub course_id: u32,
    pub required_credits: u32,
}

// Major-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Major {
    pub major_id: u32,
    pub major_name: String,
    pub department: String,
    pub required_credits: u32,
}

// Professor-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Professor {
    pub professor_id: String,
    pub name: String,
    pub department: String,
    pub email: String,
}

// Enrollment-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enrollment {
    pub enrollment_id: String,
    pub student_id: String,
    pub course_id: u32,
    pub semester: String,
    pub status: String,
}

// Schedule-related structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledCourse {
    pub course_id: u32,
    pub time_slot: TimeSlot,
    pub day_of_week: String,
    pub room: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemesterPlan {
    pub plan_id: String,
    pub student_id: String,
    pub semester: String,
    pub courses: Vec<ScheduledCourse>,
}

// Helper functions for loading data
pub fn load_json_file<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, String> {
    let path = Path::new(file_path);
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file {}: {}", file_path, e)),
    };

    match serde_json::from_str(&file_content) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to parse JSON from {}: {}", file_path, e)),
    }
}

// Common error type for the API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub code: u32,
}

// Types for returning API results
pub type ApiResult<T> = Result<T, ApiError>;