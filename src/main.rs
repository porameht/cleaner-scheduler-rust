use anyhow::{Context, Result};
use log::{error, info};
use mongodb::{bson, Client};
use tokio_cron_scheduler::{Job, JobScheduler};

async fn clean_collections() -> Result<()> {
    let uri = std::env::var("MONGODB_URI").context("MONGODB_URI not set")?;
    let db_name = std::env::var("DATABASE_NAME").context("DATABASE_NAME not set")?;
    let collections = std::env::var("COLLECTIONS").context("COLLECTIONS not set")?;

    let client = Client::with_uri_str(&uri).await?;
    let db = client.database(&db_name);

    let mut total = 0u64;
    for name in collections.split(',').map(str::trim) {
        match db
            .collection::<bson::Document>(name)
            .delete_many(bson::doc! {}, None)
            .await
        {
            Ok(result) => {
                info!("Cleaned {}: {} documents", name, result.deleted_count);
                total += result.deleted_count;
            }
            Err(e) => error!("Failed to clean {}: {}", name, e),
        }
    }

    info!("Total deleted: {}", total);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let schedule = std::env::var("CRON_SCHEDULE").unwrap_or_else(|_| "0 0 0 */3 * *".to_string());
    info!("Starting cleaner (schedule: {})", schedule);

    let mut sched = JobScheduler::new().await?;
    sched
        .add(Job::new_async(schedule.as_str(), |_, _| {
            Box::pin(async {
                if let Err(e) = clean_collections().await {
                    error!("Task failed: {}", e);
                }
            })
        })?)
        .await?;

    // Run once on startup
    if let Err(e) = clean_collections().await {
        error!("Initial task failed: {}", e);
    }

    sched.start().await?;
    info!("Scheduler started. Press Ctrl+C to stop.");

    tokio::signal::ctrl_c().await?;
    sched.shutdown().await?;

    Ok(())
}
