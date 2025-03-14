use super::entity::{CreateUserPayload, User};
use super::repos;
use sqlx::PgPool;
pub async fn create_user(pool: &PgPool, new_user: &CreateUserPayload) -> Result<User, sqlx::Error> {
    repos::create(pool, new_user).await
}

#[cfg(test)]
mod tests {
    use std::env::set_var;

    use sqlx::{Pool, Postgres};

    use super::*;

    fn setup() {
        let db_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/testdb".to_string());
        set_var("DATABASE_URL", db_url);
    }
    #[sqlx::test]
    async fn create_user_works(pool: Pool<Postgres>) -> sqlx::Result<()> {
        setup();
        let test_user = CreateUserPayload {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
        };

        let result = create_user(&pool, &test_user).await;
        assert!(result.is_ok());

        let created_user = result.unwrap();
        assert_eq!(created_user.first_name, test_user.first_name);
        assert_eq!(created_user.email, test_user.email);
        assert!(created_user.id > 0);
        Ok(())
    }
    #[sqlx::test]
    async fn create_duplicate_user_fails(pool: Pool<Postgres>) -> sqlx::Result<()> {
        setup();
        let test_user = CreateUserPayload {
            first_name: "Duplicate".to_string(),
            last_name: "User".to_string(),
            email: "duplicate@example.com".to_string(),
            password: "password".to_string(),
        };

        let first_result = create_user(&pool, &test_user).await;
        assert!(first_result.is_ok());

        let second_result = create_user(&pool, &test_user).await;
        assert!(matches!(second_result, Err(sqlx::Error::RowNotFound)));
        Ok(())
    }
}
