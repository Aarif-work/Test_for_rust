use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::{Student, NewStudent, StudentDetails, NewStudentDetails};

#[get("/GetStudents")]
pub async fn get_students(db: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(_) => HttpResponse::InternalServerError().body("Error : fetching students"),
    }
}

#[get("/GetStudents")]
pub async fn get_student_details(db: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, StudentDetails>("SELECT * FROM student_details")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(_) => HttpResponse::InternalServerError().body("Error : fetching students"),
    }
}

#[post("/CreatStudents")]
pub async fn create_student(
    db: web::Data<MySqlPool>,
    student: web::Json<NewStudent>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO students (students, math, phy, chy) VALUES (?, ?, ?, ?)",
        student.students,
        student.math,
        student.phy,
        student.chy
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("mgs : Student inserted"),
        Err(_) => HttpResponse::InternalServerError().body("Error : Insert failed"),
    }
}

#[post("/CreatStudentsDetails")]
pub async fn create_student_details(
    db: web::Data<MySqlPool>,
    student_details: web::Json<NewStudentDetails>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO student_details (age, grade, address) VALUES (?, ?, ?)",
        student_details.age,
        student_details.grade,
        student_details.address 
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("mgs : Student details inserted"),
        Err(_) => HttpResponse::InternalServerError().body("Error : Insert failed"),
    }
}