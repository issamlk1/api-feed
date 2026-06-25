use axum::{extract::{Query, State}, response::Json};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::{auth::ApiKey, models::DateQuery, fetcher::fetch_and_store};

pub async fn get_records(
    _: ApiKey,
    State(pool): State<SqlitePool>,
    Query(params): Query<DateQuery>,
) -> Json<Value> {
    let date = params.date
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let rows = sqlx::query_as::<_, (String, String, String)>(
        "SELECT id, payload, created_at FROM records WHERE created_at = ? ORDER BY rowid DESC"
    )
    .bind(&date)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Parse each stored JSON payload back out
    let data: Vec<Value> = rows.iter().map(|(id, payload, created_at)| {
        let inner: Value = serde_json::from_str(payload).unwrap_or(Value::Null);
        json!({ "id": id, "created_at": created_at, "data": inner })
    }).collect();

    Json(json!({ "date": date, "count": data.len(), "records": data }))
}

pub async fn trigger_fetch(
    _: ApiKey,
    State(pool): State<SqlitePool>,
) -> Json<Value> {
    let url = std::env::var("FETCH_URL").unwrap();
    fetch_and_store(&pool, &url).await;
    Json(json!({ "status": "done" }))
}