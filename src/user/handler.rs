use super::entity::{CreateUserPayload, UserResponse};
use super::repos;
use super::service as user_service;
use crate::app::entity::AppData;
use actix_web::{web, HttpResponse, Responder};

#[actix_web::post("/users")]
pub async fn create_user(
    user_data: actix_web::web::Json<CreateUserPayload>,
    app: web::Data<AppData>,
) -> impl Responder {
    let user_data = user_data.into_inner();
    println!("Creating user: {:?}", user_data);
    let app = app.into_inner();
    *app.request_count.lock().unwrap() += 1;
    println!("Request count: {}", *app.request_count.lock().unwrap());
    let db = &app.pool;
    let result = user_service::create_user(db, &user_data).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(&user)),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[actix_web::get("/users/{id}")]
pub async fn get_user(id: web::Path<i32>, app: web::Data<AppData>) -> impl Responder {
    let app = app.into_inner();
    let user_id = id.into_inner();
    let db = &app.pool;
    let result = repos::find(db, user_id).await;
    match result {
        Ok(user) => {
            let user_response = UserResponse::from(&user);
            HttpResponse::Ok().json(user_response)
        }
        Err(err) => HttpResponse::NotFound().json(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    println!("User routes initialized");
    cfg.service(create_user).service(get_user);
}
