mod clients;
mod config;

use config::settings::load_config;

use crate::clients::investec::investec::InvestecClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config();

    let client = InvestecClient::new(config)?;

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
                println!(
                    "\nGetting recent transactions for {}...",
                    account.account_number
                );
                match client
                    .get_transactions(&account.account_id, "2025-09-08", "2025-09-09")
                    .await
                {
                    Ok(transactions_response) => {
                        println!(
                            "  Found {} transactions",
                            transactions_response.transactions.len()
                        );
                        for transaction in transactions_response.transactions.iter().take(3) {
                            println!(
                                "    - {}: {} ({})",
                                transaction.description, transaction.amount, transaction.type_
                            );
                        }
                    }
                    Err(e) => println!("  Failed to get transactions: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to get accounts: {}", e),
    }

    Ok(())
}
