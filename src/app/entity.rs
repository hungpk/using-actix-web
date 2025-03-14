use std::sync::Mutex;
pub struct AppData {
    pub app_name: String,
    pub request_count: Mutex<u32>,
    pub pool: sqlx::PgPool,
}
