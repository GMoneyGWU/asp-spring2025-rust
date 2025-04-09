use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use open;

// Represents a course with all its details
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Course {
    id: u32,
    dept_code: String,
    course_number: String,
    instructor: String,
    description: String,
    location: String,
    meeting_time: String,  // Format like "M 08:00-15:00"
}

// Data for creating a new course (no ID since it's assigned automatically)
#[derive(Deserialize, Debug)]
struct NewCourse {
    dept_code: String,
    course_number: String,
    instructor: String,
    description: String,
    location: String,
    meeting_time: String,
}

// For updating an existing course, includes the ID
#[derive(Deserialize, Debug)]
struct UpdateCourse {
    id: u32,
    dept_code: String,
    course_number: String,
    instructor: String,
    description: String,
    location: String,
    meeting_time: String,
}

// A course that's been scheduled with additional info
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ScheduledCourse {
    course_id: u32,
    notes: String,
    slot: String,
}

// Holds the user's schedule
#[derive(Serialize, Deserialize, Default)]
struct Schedule {
    courses: Vec<ScheduledCourse>,
}

// User info for login and roles
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
    role: String,  // Either "admin" or "student"
}

// Login request data
#[derive(Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

// Response after successful login
#[derive(Serialize)]
struct LoginResponse {
    role: String,
}

// Tracks the current session
#[derive(Serialize, Deserialize, Debug)]
struct Session {
    username: String,
}

// Error message structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Application state shared across requests
struct AppState {
    courses: Mutex<Vec<Course>>,
    schedule: Mutex<Schedule>,
    users: Mutex<Vec<User>>,
    session: Mutex<Option<Session>>,
}

// Loads courses from file or initializes if not found
fn load_courses() -> Vec<Course> {
    if let Ok(data) = fs::read_to_string("courses.json") {
        
        serde_json::from_str(&data).unwrap_or_else(|e| {
            println!("Failed to parse courses.json: {:?}", e);
            initialize_courses()
        })
    } else {
        println!("No courses.json found, initializing with defaults");
        initialize_courses()
    }
}

// Creates an empty course list as a fallback
fn initialize_courses() -> Vec<Course> {
    Vec::new()
}

// Saves courses to file
fn save_courses(courses: &Vec<Course>) -> Result<(), std::io::Error> {
    let data = serde_json::to_string_pretty(courses)?;
    
    fs::write("courses.json", data)?;
    Ok(())
}

// Loads the schedule from file
fn load_schedule() -> Schedule {
    fs::read_to_string("schedule.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

// Saves the current schedule
fn save_schedule(schedule: &Schedule) -> Result<(), std::io::Error> {
    let data = serde_json::to_string_pretty(schedule)?;
    fs::write("schedule.json", data)?;
    
    Ok(())
}

// Loads users from file or initializes defaults
fn load_users() -> Vec<User> {
    if let Ok(data) = fs::read_to_string("users.json") {
        serde_json::from_str(&data).unwrap_or_else(|e| {
            println!("Failed to parse users.json: {:?}", e);
            initialize_users()
        })
    } else {
        println!("No users.json found, initializing with defaults");
        initialize_users()
    }
}

// Sets up default users (admin account)
fn initialize_users() -> Vec<User> {
    let initial_users = vec![
        User {
            username: "admin".to_string(),
            password: "admin".to_string(),
            role: "admin".to_string(),
        }
    ];
    
    if let Err(e) = save_users(&initial_users) {
        println!("Failed to initialize users.json: {:?}", e);
    }
    initial_users
}

// Saves user list to file
fn save_users(users: &Vec<User>) -> Result<(), std::io::Error> {
    let data = serde_json::to_string_pretty(users)?;
    fs::write("users.json", data)?;
    Ok(())
}

// Loads current session info
fn load_session() -> Option<Session> {
    fs::read_to_string("session.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
}

// Saves or clears session data
fn save_session(session: &Option<Session>) -> Result<(), std::io::Error> {
    if let Some(session) = session {
        let data = serde_json::to_string_pretty(session)?;
        fs::write("session.json", data)?;
    } else {
        fs::remove_file("session.json").unwrap_or(());
    }
    
    Ok(())
}

// Handles user login
async fn login(login: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    let mut session = state.session.lock().unwrap();
    
    if session.is_some() {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Another user is already logged in".to_string() 
        });
    }
    
    if let Some(user) = users.iter().find(|u| u.username == login.username && u.password == login.password) {
        *session = Some(Session { username: user.username.clone() });
        
        if let Err(e) = save_session(&*session) {
            return HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save session: {}", e) 
            });
        }
        HttpResponse::Ok().json(LoginResponse { role: user.role.clone() })
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse { 
            error: "Invalid credentials".to_string() 
        })
    }
}

// Logs out the current user
async fn logout(state: web::Data<AppState>) -> impl Responder {
    let mut session = state.session.lock().unwrap();
    *session = None;
    
    match save_session(&*session) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Logged out"})),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
            error: format!("Failed to clear session: {}", e) 
        })
    }
}

// Adds a new student user (admin only)
async fn add_user(user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can add users".to_string() 
        });
    }
    
    let mut users = state.users.lock().unwrap();
    if users.iter().any(|u| u.username == user.username) {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Username already exists".to_string() 
        });
    }
    
    let new_user = User {
        username: user.username.clone(),
        password: user.password.clone(),
        role: "student".to_string(),
    };
    
    users.push(new_user);
    match save_users(&users) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Student added"})),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
            error: format!("Failed to save users: {}", e) 
        }),
    }
}

// Deletes a student user (admin only)
async fn delete_user(username: web::Json<String>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can delete users".to_string() 
        });
    }
    
    let mut users = state.users.lock().unwrap();
    let username = username.into_inner();
    
    if username == "admin" {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Cannot delete admin account".to_string() 
        });
    }
    
    if let Some(index) = users.iter().position(|u| u.username == username) {
        users.remove(index);
        match save_users(&users) {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Student deleted"})),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save users: {}", e) 
            }),
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse { 
            error: "User not found".to_string() 
        })
    }
}

// Returns list of student users (admin only)
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can view users".to_string() 
        });
    }
    
    let users = state.users.lock().unwrap();
    let student_users: Vec<&User> = users.iter().filter(|u| u.role == "student").collect();
    HttpResponse::Ok().json(student_users)
}

// Adds a new course (admin only)
async fn add_course(course: web::Json<NewCourse>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can add courses".to_string() 
        });
    }
    
    let mut courses = state.courses.lock().unwrap();
    let new_id = courses.len() as u32 + 1;
    
    let new_course = Course {
        id: new_id,
        dept_code: course.dept_code.clone(),
        course_number: course.course_number.clone(),
        instructor: course.instructor.clone(),
        description: course.description.clone(),
        location: course.location.clone(),
        meeting_time: course.meeting_time.clone(),
    };
    
    if new_course.dept_code != "CS" {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Department code must be 'CS'".to_string() 
        });
    }
    
    let slots = new_course.meeting_time.split(", ");
    for slot in slots {
        let parts: Vec<&str> = slot.split(' ').collect();
        if parts.len() != 2 {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Invalid meeting time format (e.g., 'M 08:00-15:00')".to_string() 
            });
        }
        
        let times: Vec<&str> = parts[1].split('-').collect();
        if times.len() != 2 {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Invalid time range".to_string() 
            });
        }
        
        let start: Vec<u32> = times[0].split(':').map(|x| x.parse().unwrap()).collect();
        let end: Vec<u32> = times[1].split(':').map(|x| x.parse().unwrap()).collect();
        let start_hour = start[0];
        let end_hour = end[0];
        
        if start_hour < 8 || end_hour > 21 || (end_hour == 21 && end[1] > 0) {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Classes must be between 08:00 and 21:00".to_string() 
            });
        }
        
        if start_hour > end_hour || (start_hour == end_hour && start[1] >= end[1]) {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "End time must be after start time".to_string() 
            });
        }
    }
    
    courses.push(new_course.clone());
    match save_courses(&courses) {
        Ok(_) => HttpResponse::Ok().json(new_course),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
            error: format!("Failed to save courses: {}", e) 
        }),
    }
}

// Updates an existing course (admin only)
async fn update_course(course: web::Json<UpdateCourse>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can update courses".to_string() 
        });
    }
    
    let mut courses = state.courses.lock().unwrap();
    let mut schedule = state.schedule.lock().unwrap();
    let course_data = course.into_inner();
    
    //dbg, didn't get to finish doing non c
    if course_data.dept_code != "CS" {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Department code must be 'CS'".to_string() 
        });
    }
    
    let slots = course_data.meeting_time.split(", ");
    for slot in slots {
        let parts: Vec<&str> = slot.split(' ').collect();
        if parts.len() != 2 {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Invalid meeting time format (e.g., 'M 08:00-15:00')".to_string() 
            });
        }
        
        let times: Vec<&str> = parts[1].split('-').collect();
        if times.len() != 2 {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Invalid time range".to_string() 
            });
        }
        
        let start: Vec<u32> = times[0].split(':').map(|x| x.parse().unwrap()).collect();
        let end: Vec<u32> = times[1].split(':').map(|x| x.parse().unwrap()).collect();
        let start_hour = start[0];
        let end_hour = end[0];
        
        if start_hour < 8 || end_hour > 21 || (end_hour == 21 && end[1] > 0) {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "Classes must be between 08:00 and 21:00".to_string() 
            });
        }
        
        if start_hour > end_hour || (start_hour == end_hour && start[1] >= end[1]) {
            return HttpResponse::BadRequest().json(ErrorResponse { 
                error: "End time must be after start time".to_string() 
            });
        }
    }
    
    if let Some(index) = courses.iter().position(|c| c.id == course_data.id) {
        courses[index] = Course {
            id: course_data.id,
            dept_code: course_data.dept_code,
            course_number: course_data.course_number,
            instructor: course_data.instructor,
            description: course_data.description,
            location: course_data.location,
            meeting_time: course_data.meeting_time,
        };
        
        schedule.courses.retain(|sc| sc.course_id != course_data.id);
        match save_courses(&courses) {
            Ok(_) => match save_schedule(&schedule) {
                Ok(_) => HttpResponse::Ok().json(&courses[index]),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
                    error: format!("Failed to save schedule: {}", e) 
                }),
            },
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save courses: {}", e) 
            }),
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Course not found".to_string() 
        })
    }
}

// Deletes a course (admin only)
async fn delete_course(course_id: web::Json<u32>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username != "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only admin can delete courses".to_string() 
        });
    }
    
    let mut courses = state.courses.lock().unwrap();
    let mut schedule = state.schedule.lock().unwrap();
    let course_id = course_id.into_inner();
    
    if let Some(index) = courses.iter().position(|c| c.id == course_id) {
        courses.remove(index);
        schedule.courses.retain(|sc| sc.course_id != course_id);
        
        match save_courses(&courses) {
            Ok(_) => match save_schedule(&schedule) {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Course deleted"})),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
                    error: format!("Failed to save schedule: {}", e) 
                }),
            },
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save courses: {}", e) 
            }),
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Course not found".to_string() 
        })
    }
}

// Returns all available courses
async fn get_courses(state: web::Data<AppState>) -> impl Responder {
    let courses = state.courses.lock().unwrap();
    HttpResponse::Ok().json(&*courses)
}

// Adds a course to the student’s schedule
async fn add_to_schedule(course_id: web::Json<u32>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username == "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only students can modify schedule".to_string() 
        });
    }
    
    let courses = state.courses.lock().unwrap();
    let course_id = course_id.into_inner();
    let course = courses.iter().find(|c| c.id == course_id);
    
    if course.is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Course not found".to_string() 
        });
    }
    
    let course = course.unwrap();
    let mut schedule = state.schedule.lock().unwrap();
    
    for scheduled in &schedule.courses {
        let existing_course = courses.iter().find(|c| c.id == scheduled.course_id).unwrap();
        if has_time_conflict(&course.meeting_time, &existing_course.meeting_time) {
            return HttpResponse::Conflict().json(ErrorResponse { 
                error: "Time conflict with existing course".to_string() 
            });
        }
    }
    
    if !schedule.courses.iter().any(|c| c.course_id == course_id) {
        schedule.courses.push(ScheduledCourse {
            course_id,
            notes: String::new(),
            slot: String::new(),
        });
        
        if let Err(e) = save_schedule(&schedule) {
            return HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save schedule: {}", e) 
            });
        }
    }
    
    HttpResponse::Ok().json(&*schedule)
}

// Updates a schedule entry (notes or slot)
async fn update_schedule_entry(entry: web::Json<ScheduledCourse>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username == "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only students can modify schedule".to_string() 
        });
    }
    
    let mut schedule = state.schedule.lock().unwrap();
    let entry = entry.into_inner();
    
    if let Some(index) = schedule.courses.iter().position(|c| c.course_id == entry.course_id) {
        schedule.courses[index] = entry;
        if let Err(e) = save_schedule(&schedule) {
            return HttpResponse::InternalServerError().json(ErrorResponse { 
                error: format!("Failed to save schedule: {}", e) 
            });
        }
        HttpResponse::Ok().json(&schedule.courses[index])
    } else {
        HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Course not in schedule".to_string() 
        })
    }
}

// Removes a course from the schedule
async fn drop_from_schedule(course_id: web::Json<u32>, state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username == "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only students can modify schedule".to_string() 
        });
    }
    
    let courses = state.courses.lock().unwrap();
    let course_id = course_id.into_inner();
    
    if !courses.iter().any(|c| c.id == course_id) {
        return HttpResponse::BadRequest().json(ErrorResponse { 
            error: "Course not found in course list".to_string() 
        });
    }
    
    let mut schedule = state.schedule.lock().unwrap();
    let initial_len = schedule.courses.len();
    schedule.courses.retain(|c| c.course_id != course_id);
    
    if schedule.courses.len() == initial_len {
        return HttpResponse::Ok().json(&*schedule);
    }
    
    match save_schedule(&schedule) {
        Ok(_) => HttpResponse::Ok().json(&*schedule),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { 
            error: format!("Failed to save schedule: {}", e) 
        }),
    }
}

// Returns the student’s current schedule
async fn get_schedule(state: web::Data<AppState>) -> impl Responder {
    let session = state.session.lock().unwrap();
    if session.as_ref().map_or(true, |s| s.username == "admin") {
        return HttpResponse::Forbidden().json(ErrorResponse { 
            error: "Only students can view schedule".to_string() 
        });
    }
    
    let schedule = state.schedule.lock().unwrap();
    let courses = state.courses.lock().unwrap();
    
    let scheduled_courses: Vec<(ScheduledCourse, &Course)> = schedule
        .courses
        .iter()
        .filter_map(|sc| courses.iter().find(|c| c.id == sc.course_id).map(|c| (sc.clone(), c)))
        .collect();
    
    HttpResponse::Ok().json(scheduled_courses)
}

// Checks if two meeting times conflict
fn has_time_conflict(time1: &str, time2: &str) -> bool {
    let parse_time = |time: &str| -> Vec<(String, u32, u32, u32, u32)> {
        time.split(", ").map(|slot| {
            let parts: Vec<&str> = slot.split(' ').collect();
            let day = parts[0].to_string();
            let times: Vec<&str> = parts[1].split('-').collect();
            let start: Vec<u32> = times[0].split(':').map(|x| x.parse().unwrap()).collect();
            let end: Vec<u32> = times[1].split(':').map(|x| x.parse().unwrap()).collect();
            (day, start[0], start[1], end[0], end[1])
        }).collect()
    };

    let slots1 = parse_time(time1);
    let slots2 = parse_time(time2);
    
    for (day1, start_h1, start_m1, end_h1, end_m1) in &slots1 {
        let start1 = start_h1 * 60 + start_m1;
        let end1 = end_h1 * 60 + end_m1;
        
        for (day2, start_h2, start_m2, end_h2, end_m2) in &slots2 {
            let start2 = start_h2 * 60 + start_m2;
            let end2 = end_h2 * 60 + end_m2;
            
            if day1 == day2 && !(end1 <= start2 || start1 >= end2) {
                return true;
            }
        }
    }
    false
}

// Main entry point for the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fs::remove_file("session.json").unwrap_or(());
    
    let state = web::Data::new(AppState {
        courses: Mutex::new(load_courses()),
        schedule: Mutex::new(load_schedule()),
        users: Mutex::new(load_users()),
        session: Mutex::new(None),
    });

    println!("Server running at http://localhost:8080");
    open::that("http://localhost:8080").expect("Failed to open browser");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .route("/api/admin/add_user", web::post().to(add_user))
            .route("/api/admin/delete_user", web::post().to(delete_user))
            .route("/api/admin/get_users", web::get().to(get_users))
            .route("/api/admin/add_course", web::post().to(add_course))
            .route("/api/admin/update_course", web::post().to(update_course))
            .route("/api/admin/delete_course", web::post().to(delete_course))
            .route("/api/courses", web::get().to(get_courses))
            .route("/api/student/add_to_schedule", web::post().to(add_to_schedule))
            .route("/api/student/update_schedule_entry", web::post().to(update_schedule_entry))
            .route("/api/student/drop_from_schedule", web::post().to(drop_from_schedule))
            .route("/api/student/schedule", web::get().to(get_schedule))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}