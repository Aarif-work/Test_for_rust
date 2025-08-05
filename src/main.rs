mod models;
mod handlers;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    let pool = MySqlPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Could not connect to DB");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_students)
            .service(handlers::get_student_details)
            .service(handlers::create_student)
            .service(handlers::create_student_details)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
