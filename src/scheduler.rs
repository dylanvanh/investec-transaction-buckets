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
                tracing::debug!("Scheduler triggered");
                match db::Database::initialize(&db_url).await {
                    Ok(database) => {
                        run_sync(client.as_ref(), classifier.as_ref(), &database).await;
                    }
                    Err(e) => tracing::error!("Failed to init DB for scheduled job: {}", e),
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
    tracing::info!("Starting transaction sync");

    match client.get_accounts().await {
        Ok(accounts) => {
            if accounts.is_empty() {
                tracing::warn!("No accounts found");
                return;
            }

            let mut total_transactions = 0;
            let mut new_transactions = 0;

            for account in &accounts {
                let today = Utc::now().date_naive();
                let tomorrow = today + chrono::Duration::days(1);
                let from_date = today.format("%Y-%m-%d").to_string();
                let to_date: String = tomorrow.format("%Y-%m-%d").to_string();

                match client
                    .get_transactions(&account.account_id, &from_date, &to_date)
                    .await
                {
                    Ok(transactions_response) => {
                        let count = transactions_response.transactions.len();
                        total_transactions += count;

                        if count > 0 {
                            let new_count = process_transactions(
                                &transactions_response.transactions,
                                classifier,
                                database,
                            )
                            .await;
                            new_transactions += new_count;
                        }
                    }
                    Err(e) => tracing::error!(
                        account_id = %account.account_id,
                        error = %e,
                        "Failed to get transactions"
                    ),
                }
            }

            tracing::info!(
                total = total_transactions,
                new = new_transactions,
                "Sync complete"
            );
        }
        Err(e) => tracing::error!(error = %e, "Failed to get accounts"),
    }
}

pub async fn process_transactions(
    transactions: &[models::Transaction],
    classifier: &BucketClassifier,
    database: &db::Database,
) -> usize {
    let mut new_count = 0;

    for transaction in transactions.iter() {
        if let Some(uuid) = &transaction.uuid {
            match db::find_transaction_id_by_uuid(&database.pool, uuid).await {
                Ok(Some(_)) => {
                    continue;
                }
                Ok(None) => {}
                Err(_) => {
                    continue;
                }
            }
        }

        let bucket = match classifier
            .classify_transaction_with_fallback(transaction)
            .await
        {
            Ok(bucket) => bucket,
            Err(_) => {
                continue;
            }
        };

        match db::insert_tx_and_annotation(&database.pool, transaction, &bucket, None).await {
            Ok(_) => {
                new_count += 1;
            }
            Err(_) => {}
        }
    }

    new_count
}
