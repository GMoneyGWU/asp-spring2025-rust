use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs;
use std::sync::Mutex;
use log::error;

use crate::models::models::{Course, Student, Schedule};
use crate::models::app_state::AppState;

#[get("/courses")]
pub async fn get_courses(data: web::Data<AppState>) -> impl Responder {
    let courses = data.courses.lock().unwrap();
    HttpResponse::Ok().json(&*courses)
}

#[get("/students")]
pub async fn get_students(data: web::Data<AppState>) -> impl Responder {
    let students = data.students.lock().unwrap();
    HttpResponse::Ok().json(&*students)
}

#[get("/schedules")]
pub async fn get_schedules(data: web::Data<AppState>) -> impl Responder {
    let schedules = data.schedules.lock().unwrap();
    HttpResponse::Ok().json(&*schedules)
}

#[post("/schedules")]
pub async fn create_schedule(data: web::Data<AppState>, schedule: web::Json<Schedule>) -> impl Responder {
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

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}