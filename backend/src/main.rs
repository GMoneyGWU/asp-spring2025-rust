use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::{info, error};
use std::sync::Mutex;

mod api;
mod models;
mod utils;

use api::routes::{health_check, get_courses, get_students, get_schedules, create_schedule};
use models::app_state::AppState;
use models::models::{Course, Student, Schedule};
use utils::file_utils::load_data;

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