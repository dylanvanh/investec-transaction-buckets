use std::sync::Arc;

use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::bucket_classifier::BucketClassifier;
use crate::clients::InvestecClient;
use crate::clients::investec::models;
use crate::db;

pub async fn start_hourly(
    client: Arc<InvestecClient>,
    classifier: Arc<BucketClassifier>,
    database_url: String,
) -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    let db_url = database_url.clone();

    scheduler
        .add(Job::new_async("0 0 * * * *", move |_uuid, _l| {
            let client = Arc::clone(&client);
            let classifier = Arc::clone(&classifier);
            let db_url = db_url.clone();
            Box::pin(async move {
                match db::Database::initialize(&db_url).await {
                    Ok(database) => {
                        run_sync(client.as_ref(), classifier.as_ref(), &database).await;
                    }
                    Err(e) => eprintln!("Failed to init DB for scheduled job: {}", e),
                }
            })
        })?)
        .await?;

    scheduler.start().await?;
    Ok(scheduler)
}

pub async fn run_sync(
    client: &InvestecClient,
    classifier: &BucketClassifier,
    database: &db::Database,
) {
    match client.get_accounts().await {
        Ok(accounts) => {
            println!("Found {} accounts:", accounts.len());

            for account in &accounts {
                let today = Utc::now().date_naive();
                let tomorrow = today + chrono::Duration::days(1);
                let from_date = today.format("%Y-%m-%d").to_string();
                let to_date: String = tomorrow.format("%Y-%m-%d").to_string();

                println!(
                    "\nGetting recent transactions for {} ({} to {})...",
                    account.account_number, from_date, to_date
                );

                match client
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
                            classifier,
                            database,
                        )
                        .await;
                    }
                    Err(e) => println!("  Failed to get transactions: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to get accounts: {}", e),
    }
}

pub async fn process_transactions(
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
