use crate::{
    ApiError, ApiResult, Constraint, Course, DegreeRequirement, Enrollment, Major, Professor,
    ScheduledCourse, SemesterPlan, Student, StudentPreference, TimeSlot, load_json_file
};
use flutter_rust_bridge::frb;
use std::collections::HashMap;

// Base paths for data files
const DATA_DIR: &str = "data";

// Test function to verify bridge connection
#[frb]
pub fn hello_from_rust() -> String {
    "Hello from Rust!".to_string()
}

// Load and return all courses
#[frb]
pub fn get_all_courses() -> ApiResult<Vec<Course>> {
    let file_path = format!("{}/courses.json", DATA_DIR);
    load_json_file::<Vec<Course>>(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })
}

// Load student preferences by student ID
#[frb]
pub fn get_student_preferences(student_id: String) -> ApiResult<StudentPreference> {
    let file_path = format!("{}/student_preferences.json", DATA_DIR);
    let all_preferences: Vec<StudentPreference> = load_json_file(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })?;
    
    all_preferences
        .into_iter()
        .find(|pref| pref.student_id == student_id)
        .ok_or(ApiError {
            error: format!("No preferences found for student ID: {}", student_id),
            code: 404,
        })
}

// Load degree requirements by major ID
#[frb]
pub fn get_degree_requirements(major_id: u32) -> ApiResult<Vec<DegreeRequirement>> {
    let file_path = format!("{}/degree_requirements.json", DATA_DIR);
    let all_requirements: Vec<DegreeRequirement> = load_json_file(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })?;
    
    let major_requirements: Vec<DegreeRequirement> = all_requirements
        .into_iter()
        .filter(|req| req.major_id == major_id)
        .collect();
    
    if major_requirements.is_empty() {
        return Err(ApiError {
            error: format!("No degree requirements found for major ID: {}", major_id),
            code: 404,
        });
    }
    
    Ok(major_requirements)
}

// Load a student's semester plan
#[frb]
pub fn get_semester_plan(student_id: String) -> ApiResult<SemesterPlan> {
    let file_path = format!("{}/semester_plans.json", DATA_DIR);
    let all_plans: Vec<SemesterPlan> = load_json_file(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })?;
    
    all_plans
        .into_iter()
        .find(|plan| plan.student_id == student_id)
        .ok_or(ApiError {
            error: format!("No semester plan found for student ID: {}", student_id),
            code: 404,
        })
}

// Suggest a schedule for a student based on preferences, requirements, and constraints
#[frb]
pub fn suggest_schedule(student_id: String) -> ApiResult<SemesterPlan> {
    // 1. Get student information
    let student = get_student_by_id(student_id.clone())?;
    
    // 2. Get student preferences
    let preferences = get_student_preferences(student_id.clone())?;
    
    // 3. Get degree requirements for student's major
    let requirements = get_degree_requirements(student.major_id)?;
    
    // 4. Get constraints
    let constraints = get_student_constraints(student_id.clone())?;
    
    // 5. Get all available courses
    let courses = get_all_courses()?;
    
    // 6. Get existing enrollments to avoid duplicates
    let enrollments = get_student_enrollments(student_id.clone())?;
    let enrolled_course_ids: Vec<u32> = enrollments.iter().map(|e| e.course_id).collect();
    
    // 7. Filter courses based on prerequisites and already taken courses
    let available_courses: Vec<Course> = courses
        .into_iter()
        .filter(|course| !enrolled_course_ids.contains(&course.course_id))
        .filter(|course| {
            course.prerequisites.iter().all(|prereq_id| {
                enrolled_course_ids.contains(prereq_id)
            })
        })
        .collect();
    
    // 8. Create a plan based on preferences and requirements
    let mut suggested_courses: Vec<ScheduledCourse> = Vec::new();
    
    // First prioritize courses from student preferences
    for &preferred_course_id in &preferences.preferred_times {
        if suggested_courses.len() >= 5 {
            break; // Limit to 5 courses per semester
        }
        
        if let Some(course) = available_courses.iter().find(|c| preferences.preferred_courses.contains(&c.course_id)) {
            // Check if this course is in the degree requirements
            let is_required = requirements.iter().any(|req| req.course_id == course.course_id);
            
            // Consider credit limits from constraints
            let mut max_credits = 20; // Default max credits
            for constraint in &constraints {
                if constraint.constraint_type == "credit_limit" {
                    if let Some(credits) = constraint.value.max_credits {
                        max_credits = credits;
                    }
                }
            }
            
            // Calculate current total credits
            let current_credits: u32 = suggested_courses
                .iter()
                .filter_map(|sc| {
                    available_courses
                        .iter()
                        .find(|c| c.course_id == sc.course_id)
                        .map(|c| c.credits)
                })
                .sum();
            
            // Add course if it doesn't exceed credit limit and is either preferred or required
            if (is_required || preferences.preferred_courses.contains(&course.course_id)) 
                && current_credits + course.credits <= max_credits {
                
                // Use preferred time if available
                let time_slot = if !preferences.preferred_times.is_empty() {
                    preferences.preferred_times[0].clone()
                } else {
                    TimeSlot {
                        start_time: "9:00".to_string(),
                        end_time: "10:00".to_string(),
                    }
                };
                
                let scheduled_course = ScheduledCourse {
                    course_id: course.course_id,
                    time_slot,
                    day_of_week: "Monday".to_string(), // Default day
                    room: "TBD".to_string(),
                };
                
                suggested_courses.push(scheduled_course);
            }
        }
    }
    
    // Then add required courses for the degree
    for requirement in requirements {
        if suggested_courses.len() >= 5 {
            break; // Limit to 5 courses per semester
        }
        
        // Skip if course is already in the plan
        if suggested_courses.iter().any(|sc| sc.course_id == requirement.course_id) {
            continue;
        }
        
        if let Some(course) = available_courses.iter().find(|c| c.course_id == requirement.course_id) {
            // Calculate current total credits
            let current_credits: u32 = suggested_courses
                .iter()
                .filter_map(|sc| {
                    available_courses
                        .iter()
                        .find(|c| c.course_id == sc.course_id)
                        .map(|c| c.credits)
                })
                .sum();
            
            let max_credits = 20; // Default max credits
            
            // Add course if it doesn't exceed credit limit
            if current_credits + course.credits <= max_credits {
                // Use preferred time if available
                let time_slot = if !preferences.preferred_times.is_empty() {
                    preferences.preferred_times[0].clone()
                } else {
                    TimeSlot {
                        start_time: "9:00".to_string(),
                        end_time: "10:00".to_string(),
                    }
                };
                
                let scheduled_course = ScheduledCourse {
                    course_id: course.course_id,
                    time_slot,
                    day_of_week: "Monday".to_string(), // Default day
                    room: "TBD".to_string(),
                };
                
                suggested_courses.push(scheduled_course);
            }
        }
    }
    
    // Create and return the semester plan
    let plan = SemesterPlan {
        plan_id: format!("suggested-{}", student_id),
        student_id,
        semester: "Spring 2025".to_string(), // Default to next semester
        courses: suggested_courses,
    };
    
    Ok(plan)
}

// Check for conflicts in a semester plan
#[frb]
pub fn check_schedule_conflicts(plan: SemesterPlan) -> ApiResult<bool> {
    // Time conflict detection
    let mut has_conflicts = false;
    let courses = &plan.courses;
    
    for (i, course1) in courses.iter().enumerate() {
        for course2 in courses.iter().skip(i + 1) {
            // Check if courses are on the same day
            if course1.day_of_week == course2.day_of_week {
                // Parse times (assuming format like "9:00" or "14:30")
                let time1_start = parse_time(&course1.time_slot.start_time);
                let time1_end = parse_time(&course1.time_slot.end_time);
                let time2_start = parse_time(&course2.time_slot.start_time);
                let time2_end = parse_time(&course2.time_slot.end_time);
                
                // Check if time ranges overlap
                if (time1_start <= time2_end && time1_end >= time2_start) {
                    has_conflicts = true;
                    break;
                }
            }
        }
        
        if has_conflicts {
            break;
        }
    }
    
    Ok(has_conflicts)
}

// Save a new schedule plan
#[frb]
pub fn store_schedule(student_id: String, plan: SemesterPlan) -> ApiResult<bool> {
    // In a real implementation, this would write to the JSON file or database
    // For now, we just validate the plan and return success
    
    let conflict_check = check_schedule_conflicts(plan.clone())?;
    if conflict_check {
        return Err(ApiError {
            error: "Cannot store schedule with time conflicts".to_string(),
            code: 400,
        });
    }
    
    // In a real implementation, we would persist the data here
    // For now, just return success
    Ok(true)
}

// Get list of professors
#[frb]
pub fn get_professors() -> ApiResult<Vec<Professor>> {
    let file_path = format!("{}/professors.json", DATA_DIR);
    load_json_file::<Vec<Professor>>(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })
}

// Get scheduling constraints
#[frb]
pub fn get_constraints() -> ApiResult<Vec<Constraint>> {
    let file_path = format!("{}/constraints.json", DATA_DIR);
    load_json_file::<Vec<Constraint>>(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })
}

// Get constraints for a specific student
#[frb]
pub fn get_student_constraints(student_id: String) -> ApiResult<Vec<Constraint>> {
    let all_constraints = get_constraints()?;
    
    let student_constraints: Vec<Constraint> = all_constraints
        .into_iter()
        .filter(|c| c.student_id == student_id)
        .collect();
    
    Ok(student_constraints)
}

// Validate a schedule against constraints
#[frb]
pub fn validate_schedule_against_constraints(plan: SemesterPlan) -> ApiResult<bool> {
    // Check for time conflicts
    let conflicts = check_schedule_conflicts(plan.clone())?;
    if conflicts {
        return Ok(false);
    }
    
    // Get student constraints
    let constraints = get_student_constraints(plan.student_id.clone())?;
    
    // Get all courses for credit calculations
    let courses = get_all_courses()?;
    
    // Calculate total credits
    let total_credits: u32 = plan.courses
        .iter()
        .filter_map(|sc| {
            courses
                .iter()
                .find(|c| c.course_id == sc.course_id)
                .map(|c| c.credits)
        })
        .sum();
    
    // Check credit limit constraints
    for constraint in &constraints {
        if constraint.constraint_type == "credit_limit" {
            if let Some(max_credits) = constraint.value.max_credits {
                if total_credits > max_credits {
                    return Ok(false);
                }
            }
        }
    }
    
    // All constraints satisfied
    Ok(true)
}

// Helper functions

// Get a student by ID
fn get_student_by_id(student_id: String) -> ApiResult<Student> {
    let file_path = format!("{}/students.json", DATA_DIR);
    let students: Vec<Student> = load_json_file(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })?;
    
    students
        .into_iter()
        .find(|s| s.student_id == student_id)
        .ok_or(ApiError {
            error: format!("Student not found with ID: {}", student_id),
            code: 404,
        })
}

// Get enrollments for a student
fn get_student_enrollments(student_id: String) -> ApiResult<Vec<Enrollment>> {
    let file_path = format!("{}/enrollments.json", DATA_DIR);
    let enrollments: Vec<Enrollment> = load_json_file(&file_path).map_err(|e| {
        ApiError {
            error: e,
            code: 500,
        }
    })?;
    
    let student_enrollments: Vec<Enrollment> = enrollments
        .into_iter()
        .filter(|e| e.student_id == student_id)
        .collect();
    
    Ok(student_enrollments)
}

// Helper function to parse time string (e.g., "9:00" -> 900)
fn parse_time(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return 0; // Default if invalid format
    }
    
    let hour: u32 = parts[0].parse().unwrap_or(0);
    let minute: u32 = parts[1].parse().unwrap_or(0);
    
    hour * 100 + minute
}