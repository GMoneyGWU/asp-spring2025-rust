use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub credits: i32,
    pub description: Option<String>,
    pub prerequisites: Option<Vec<String>>,
    #[serde(default)]
    pub available_times: Vec<TimeSlot>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub major: Option<String>,
    pub year: String,
    pub completed_courses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preference {
    pub student_id: String,
    pub preferred_courses: Vec<String>,
    pub preferred_times: Vec<TimeSlot>,
    pub max_credits: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeSlot {
    pub day: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schedule {
    pub id: Option<String>,
    pub student_id: String,
    pub semester: String,
    pub year: i32,
    pub courses: Vec<ScheduledCourse>,
    pub total_credits: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledCourse {
    pub course_id: String,
    pub time_slot: TimeSlot,
    pub room: String,
    pub professor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DegreeRequirement {
    pub major: String,
    pub required_courses: Vec<String>,
    pub elective_courses: Option<Vec<String>>,
    pub min_credits: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub student_id: String,
    pub unavailable_times: Vec<TimeSlot>,
    pub max_credits_per_semester: Option<i32>,
    pub preferred_campus_locations: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}