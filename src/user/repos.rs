use super::entity::{CreateUserPayload, User};
use sqlx::PgPool;

pub async fn create(db: &PgPool, new_user: &CreateUserPayload) -> Result<User, sqlx::Error> {
    let user_exist = find_by_email(db, &new_user.email).await;
    if user_exist.is_ok() {
        return Err(sqlx::Error::Configuration(
            "User with that email already exists".into(),
        ));
    }
    let encrypted_password = bcrypt::hash(new_user.password.clone(), bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    let new_user_id: i32 = sqlx::query!(
        r#"
            INSERT INTO users (first_name, last_name, active, email, encrypted_password)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#,
        new_user.first_name,
        new_user.last_name,
        true,
        new_user.email,
        encrypted_password,
    )
    .fetch_one(db)
    .await?
    .id;

    let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", new_user_id)
        .fetch_one(db)
        .await?;
    Ok(user)
}

pub async fn find(db: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(db)
        .await?;
    Ok(user)
}

pub async fn find_by_email(db: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(db)
        .await?;
    Ok(user)
}
