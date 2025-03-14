mod tests {
    use super::*;
    use actix_web::{
        body::to_bytes,
        http::{self, header::ContentType, StatusCode},
        test, web, App,
    };
    use sqlx::PgPool;

    use crate::{
        handlers::user_handler::{create_user, init_routes},
        model::user::User,
        services::user_service,
    };

    async fn setup() -> PgPool {
        let db_url = std::env::var("TEST_DATABASE_URL")
            .expect("DATABASE_URL must be set to run tests");
        PgPool::connect(db_url.as_str())
            .await
            .expect("Failed to connect to Postgres")
    }

    #[actix_web::test]
    async fn test_create_user_success() {
        // Setup the database connection
        let pool = setup().await;
        // create the app that contains the endpoint
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(init_routes),
        )
        .await;

        // Setup user
        let test_user = User {
            id: 0,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            active: true,
            email: "test@example.com".to_string(),
            encrypted_password: "password".to_string(),
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
            deleted_at: "2021-01-01".to_string(),
        };

        // Build the request to the server
        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(ContentType::json())
            .set_json(test_user.clone())
            .to_request();

        // send the request and get response
        let resp = test::call_service(&app, req).await;
        // check if the response is a created status
        assert_eq!(resp.status(), StatusCode::CREATED);

        // read the response body
        let body = resp.into_body();
        let body_bytes = to_bytes(body).await.unwrap();
        // Deserialize the body as User
        let created_user: User = serde_json::from_slice(&body_bytes).unwrap();

        // check if the created user has the same properties
        assert_eq!(created_user.first_name, test_user.first_name);
        assert_eq!(created_user.email, test_user.email);
        assert!(created_user.id > 0);
    }

    #[actix_web::test]
    async fn test_create_duplicate_user_fails() {
        // setup the database connection
        let pool = setup().await;
        // create app and load the route
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(init_routes),
        )
        .await;

        // setup user that will be duplicated
        let test_user = User {
            id: 0,
            first_name: "Duplicate".to_string(),
            last_name: "User".to_string(),
            active: true,
            email: "duplicate@example.com".to_string(),
            encrypted_password: "password".to_string(),
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
            deleted_at: "2021-01-01".to_string(),
        };

        // build the first request
        let req1 = test::TestRequest::post()
            .uri("/users")
            .insert_header(ContentType::json())
            .set_json(test_user.clone())
            .to_request();
        // send request and get response
        let resp1 = test::call_service(&app, req1).await;
        // the first request should be success
        assert_eq!(resp1.status(), StatusCode::CREATED);

        // build the second request to duplicate the user
        let req2 = test::TestRequest::post()
            .uri("/users")
            .insert_header(ContentType::json())
            .set_json(test_user.clone())
            .to_request();
        // send the request
        let resp2 = test::call_service(&app, req2).await;
        // second request should be a conflict
        assert_eq!(resp2.status(), StatusCode::CONFLICT);
    }
}