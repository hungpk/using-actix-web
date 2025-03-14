use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer, Responder};
use env_logger;
use std::sync::Mutex;
use using_actix_web::app::entity::AppData;
use using_actix_web::user::handler as user_handler;
#[actix_web::get("/hello")]
async fn greet() -> impl Responder {
    format!("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;
    println!("Server started at http://localhost:{}", port);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to create pool");
    let state = web::Data::new(AppData {
        app_name: "Actix-web".to_string(),
        request_count: Mutex::new(0),
        pool: pool,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(user_handler::init_routes)
            .service(greet)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_greet_returns_hello_world() {
        let app = test::init_service(App::new().service(greet)).await;

        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, world!");
    }
}
