//! The main entry point for our backend application
//! This provides a simple CLI to test the API functions

use backend::api;
use std::env;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "hello" => {
            println!("{}", api::hello_from_rust());
        },
        "courses" => {
            match api::get_all_courses() {
                Ok(courses) => {
                    println!("Found {} courses:", courses.len());
                    for course in courses {
                        println!("  - {}: {} ({})", course.course_id, course.course_name, course.department);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        "preferences" => {
            if args.len() < 3 {
                println!("Error: Missing student_id parameter");
                print_usage();
                return;
            }
            
            let student_id = &args[2];
            match api::get_student_preferences(student_id.clone()) {
                Ok(prefs) => {
                    println!("Preferences for student {}:", student_id);
                    println!("  Preferred courses: {:?}", prefs.preferred_courses);
                    println!("  Preferred times:");
                    for time in prefs.preferred_times {
                        println!("    - {} to {}", time.start_time, time.end_time);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        "requirements" => {
            if args.len() < 3 {
                println!("Error: Missing major_id parameter");
                print_usage();
                return;
            }
            
            let major_id = args[2].parse::<u32>().unwrap_or(0);
            match api::get_degree_requirements(major_id) {
                Ok(reqs) => {
                    println!("Requirements for major {}:", major_id);
                    for req in reqs {
                        println!("  - Course {}: {} credits", req.course_id, req.required_credits);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        "suggest" => {
            if args.len() < 3 {
                println!("Error: Missing student_id parameter");
                print_usage();
                return;
            }
            
            let student_id = &args[2];
            match api::suggest_schedule(student_id.clone()) {
                Ok(plan) => {
                    println!("Suggested schedule for student {} ({})", student_id, plan.semester);
                    println!("Courses:");
                    for course in plan.courses {
                        println!("  - Course {}: {} on {}, room {}", 
                            course.course_id, 
                            format!("{} - {}", course.time_slot.start_time, course.time_slot.end_time),
                            course.day_of_week,
                            course.room);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        "professors" => {
            match api::get_professors() {
                Ok(profs) => {
                    println!("Found {} professors:", profs.len());
                    for prof in profs {
                        println!("  - {}: {} ({})", prof.professor_id, prof.name, prof.department);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        "constraints" => {
            if args.len() < 3 {
                println!("Error: Missing student_id parameter");
                print_usage();
                return;
            }
            
            let student_id = &args[2];
            match api::get_student_constraints(student_id.clone()) {
                Ok(constraints) => {
                    println!("Constraints for student {}:", student_id);
                    for constraint in constraints {
                        println!("  - {}: {:?}", constraint.constraint_type, constraint.value);
                    }
                },
                Err(e) => println!("Error: {} (code: {})", e.error, e.code),
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: cargo run -- <command> [params]");
    println!("Commands:");
    println!("  hello                   - Test bridge connection");
    println!("  courses                 - List all courses");
    println!("  preferences <student_id> - Get student preferences");
    println!("  requirements <major_id>  - Get degree requirements for a major");
    println!("  suggest <student_id>     - Suggest a schedule for a student");
    println!("  professors              - List all professors");
    println!("  constraints <student_id> - Get constraints for a student");
}
