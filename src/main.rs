mod auth;
mod db;
mod fetcher;
mod models;
mod routes;

use std::sync::Arc;
use axum::{routing::{get,post}, Router};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let fetch_url    = std::env::var("FETCH_URL")?;
    let fetch_url    = Arc::new(fetch_url);

    let pool = db::init_pool(&database_url).await?;

    // --- Cron: fetch ---
    let scheduler = JobScheduler::new().await?;
    let cron_pool = pool.clone();
    let cron_url  = fetch_url.clone();

    let cron_schedule = std::env::var("CRON_SCHEDULE")?;
    scheduler.add(
        Job::new_async(cron_schedule.as_str(), move |_, _| {
            let p = cron_pool.clone();
            let u = cron_url.clone();
            Box::pin(async move {
                fetcher::fetch_and_store(&p, &u).await;
            })
        })?
    ).await?;

    scheduler.start().await?;

    fetcher::fetch_and_store(&pool, &fetch_url).await;

    // --- Router ---
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/data", get(routes::get_records))
        .route("/fetch", post(routes::trigger_fetch))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
