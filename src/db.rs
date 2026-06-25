use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS records (
            id          TEXT PRIMARY KEY,
            payload     TEXT NOT NULL,
            created_at  TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
