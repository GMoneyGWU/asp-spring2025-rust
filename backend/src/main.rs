use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use log::{info, error};

mod models;
mod utils;

use models::{Course, Student, Preference, Schedule, TimeSlot};

// State for the application
struct AppState {
    courses: Mutex<Vec<Course>>,
    students: Mutex<Vec<Student>>,
    schedules: Mutex<Vec<Schedule>>,
}

// Load data from JSON files
fn load_data(file_path: &str) -> Result<String, std::io::Error> {
    if Path::new(file_path).exists() {
        fs::read_to_string(file_path)
    } else {
        Ok("[]".to_string()) // Return empty array if file doesn't exist
    }
}

// API endpoints
#[get("/courses")]
async fn get_courses(data: web::Data<AppState>) -> impl Responder {
    let courses = data.courses.lock().unwrap();
    HttpResponse::Ok().json(&*courses)
}

#[get("/students")]
async fn get_students(data: web::Data<AppState>) -> impl Responder {
    let students = data.students.lock().unwrap();
    HttpResponse::Ok().json(&*students)
}

#[get("/schedules")]
async fn get_schedules(data: web::Data<AppState>) -> impl Responder {
    let schedules = data.schedules.lock().unwrap();
    HttpResponse::Ok().json(&*schedules)
}

#[post("/schedules")]
async fn create_schedule(data: web::Data<AppState>, schedule: web::Json<Schedule>) -> impl Responder {
    let mut schedules = data.schedules.lock().unwrap();
    schedules.push(schedule.into_inner());
    
    // Save to file
    match fs::write("../data/semester_plans.json", serde_json::to_string_pretty(&*schedules).unwrap()) {
        Ok(_) => HttpResponse::Ok().body("Schedule saved"),
        Err(e) => {
            error!("Error saving schedule: {}", e);
            HttpResponse::InternalServerError().body("Failed to save schedule")
        }
    }
}

// Health check endpoint
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server on http://localhost:8080");

    // Load initial data
    let courses_data = match load_data("../data/courses.json") {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(courses) => courses,
            Err(e) => {
                error!("Error parsing courses data: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("Error loading courses data: {}", e);
            Vec::new()
        }
    };

    let students_data = match load_data("../data/students.json") {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(students) => students,
            Err(e) => {
                error!("Error parsing students data: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("Error loading students data: {}", e);
            Vec::new()
        }
    };

    let schedules_data = match load_data("../data/semester_plans.json") {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(schedules) => schedules,
            Err(e) => {
                error!("Error parsing schedules data: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("Error loading schedules data: {}", e);
            Vec::new()
        }
    };

    // Create app state
    let app_state = web::Data::new(AppState {
        courses: Mutex::new(courses_data),
        students: Mutex::new(students_data),
        schedules: Mutex::new(schedules_data),
    });

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(health_check)
            .service(get_courses)
            .service(get_students)
            .service(get_schedules)
            .service(create_schedule)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}