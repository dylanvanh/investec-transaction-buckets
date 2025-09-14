mod bucket_classifier;
mod clients;
mod config;
mod db;

use config::settings::load_config;

use crate::bucket_classifier::BucketClassifier;
use crate::clients::InvestecClient;
use crate::clients::investec::models;

use chrono::Utc;

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

    match investec_client.get_accounts().await {
        Ok(accounts) => {
            println!("Found {} accounts:", accounts.len());

            for account in &accounts {
                let today = Utc::now().date_naive();
                let tomorrow = today + chrono::Duration::days(1);
                let from_date = today.format("2025-09-11").to_string();
                let to_date: String = tomorrow.format("%Y-%m-%d").to_string();

                println!(
                    "\nGetting recent transactions for {} ({} to {})...",
                    account.account_number, from_date, to_date
                );

                match investec_client
                    .get_transactions(&account.account_id, &from_date, &to_date)
                    .await
                {
                    Ok(transactions_response) => {
                        println!(
                            "  Found {} transactions",
                            transactions_response.transactions.len()
                        );
                        process_transactions(
                            &transactions_response.transactions,
                            &bucket_classifier,
                            &database,
                        )
                        .await;
                    }
                    Err(e) => println!("  Failed to get transactions: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to get accounts: {}", e),
    }

    Ok(())
}
async fn process_transactions(
    transactions: &[models::Transaction],
    classifier: &BucketClassifier,
    database: &db::Database,
) {
    for transaction in transactions.iter() {
        println!(
            "    - {}: {} ({})",
            transaction.description, transaction.amount, transaction.type_
        );

        if let Some(uuid) = &transaction.uuid {
            if let Ok(Some(existing_id)) =
                db::find_transaction_id_by_uuid(&database.pool, uuid).await
            {
                println!("      → Already saved (id={}), skipping", existing_id);
                continue;
            }
        }

        let bucket = match classifier
            .classify_transaction_with_fallback(transaction)
            .await
        {
            Ok(bucket) => {
                println!("      → Bucket: {}", bucket);
                bucket
            }
            Err(_) => {
                println!("      → Classification failed, skipping persist");
                continue;
            }
        };

        if let Err(e) =
            db::insert_tx_and_annotation(&database.pool, transaction, &bucket, None).await
        {
            println!("      → Failed to persist transaction + annotation: {}", e);
        }
    }
}
