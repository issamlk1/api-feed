use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn fetch_and_store(pool: &SqlitePool, url: &str) {
    let response = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => { eprintln!("Fetch error: {e}"); return; }
    };

    let json: serde_json::Value = match response.json().await {
        Ok(j) => j,
        Err(e) => { eprintln!("Parse error: {e}"); return; }
    };

    let id = Uuid::new_v4().to_string();
    let payload = json.to_string();
    let created_at = Utc::now().format("%Y-%m-%d").to_string();

    match sqlx::query(
        "INSERT INTO records (id, payload, created_at) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload)
    .bind(&created_at)
    .execute(pool)
    .await
    {
        Ok(_) => println!("Stored record {id} for {created_at}"),
        Err(e) => eprintln!("DB insert error: {e}"),
    }
}