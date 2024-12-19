use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<Mutex<SqlitePool>>,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = SqlitePool::connect(database_url)
            .await
            .expect("Failed to connect to the database");

        // Apply schema or migrations
        sqlx::query(include_str!("../migrations/schema.sql"))
            .execute(&pool)
            .await
            .expect("Failed to initialize database schema");

        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }
}
