mod bucket_classifier;
mod clients;
mod config;

use config::settings::load_config;

use crate::bucket_classifier::BucketClassifier;
use crate::clients::investec::investec::InvestecClient;
use crate::clients::investec::models;
use crate::clients::ollama::OllamaClient;

use chrono::Utc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config();

    let client = InvestecClient::new(config.clone())?;

    let bucket_classifier = match check_ollama_availability(&config).await {
        true => {
            println!("Ollama model detected: {}", config.ollama_model);
            BucketClassifier::new(config.ollama_model.clone(), &config)
        }
        false => {
            eprintln!("❌ ERROR: Ollama is required for transaction classification");
            eprintln!("   Please install Ollama and run:");
            eprintln!("   ollama pull {}", config.ollama_model);
            eprintln!("   Then restart this application.");
            std::process::exit(1);
        }
    };

    match client.get_accounts().await {
        Ok(accounts) => {
            println!("Found {} accounts:", accounts.len());
            for account in &accounts {
                println!("- {} ({})", account.account_name, account.account_id);

                match client.get_balance(&account.account_id).await {
                    Ok(balance) => {
                        println!(
                            "  Balance: {} {}",
                            balance.current_balance, balance.currency
                        );
                    }
                    Err(e) => println!("  Failed to get balance: {}", e),
                }
            }

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
                            &bucket_classifier,
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

async fn process_transactions(transactions: &[models::Transaction], classifier: &BucketClassifier) {
    for transaction in transactions.iter() {
        println!(
            "    - {}: {} ({})",
            transaction.description, transaction.amount, transaction.type_
        );

        if let Err(e) = classify_transaction_with_fallback(classifier, transaction).await {
            println!("      → Classification failed: {}", e);
        }
    }
}

async fn classify_transaction_with_fallback(
    classifier: &BucketClassifier,
    transaction: &models::Transaction,
) -> anyhow::Result<()> {
    match classifier
        .classify_with_search(&transaction.description, transaction.amount)
        .await
    {
        Ok(bucket) => {
            println!("      → Bucket: {}", bucket);
            Ok(())
        }
        Err(e) => {
            println!("      → Search classification failed: {}", e);
            match classifier
                .classify_transaction_without_search(&transaction.description, transaction.amount)
                .await
            {
                Ok(bucket) => {
                    println!("      → Fallback bucket: {}", bucket);
                    Ok(())
                }
                Err(e2) => {
                    println!("      → Fallback also failed: {}", e2);
                    Err(anyhow::anyhow!(
                        "Both classification methods failed: {} and {}",
                        e,
                        e2
                    ))
                }
            }
        }
    }
}

async fn check_ollama_availability(config: &config::settings::Config) -> bool {
    let ollama_client = OllamaClient::new(config.ollama_model.clone());
    ollama_client.is_available().await
}
