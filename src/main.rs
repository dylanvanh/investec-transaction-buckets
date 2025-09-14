mod bucket_classifier;
mod clients;
mod config;
mod db;
mod scheduler;

use config::settings::load_config;

use crate::bucket_classifier::BucketClassifier;
use crate::clients::InvestecClient;

use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config();

    let investec_client = InvestecClient::new(config.clone())?;

    let ollama_available = config.is_ollama_available();
    let gemini_available = config.is_gemini_available();
    let google_search_available = config.is_google_search_available();

    let bucket_classifier = BucketClassifier::new(config.ollama.model.clone(), &config);

    if ollama_available {
        println!("ollama details present");
    }
    if gemini_available {
        println!("gemini details present");
    }
    if google_search_available {
        println!("google search details present");
    }

    let database = db::Database::initialize(&config.database.url).await?;

    scheduler::run_sync(&investec_client, &bucket_classifier, &database).await;

    let client_arc = Arc::new(investec_client);
    let classifier_arc = Arc::new(bucket_classifier);

    let scheduler =
        scheduler::start_hourly(client_arc, classifier_arc, config.database.url.clone()).await?;

    println!("Scheduler started. Sync runs every hour at :00");

    tokio::signal::ctrl_c().await?;

    drop(scheduler);

    Ok(())
}
