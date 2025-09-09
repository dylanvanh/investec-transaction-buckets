use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountNumber")]
    pub account_number: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "kycCompliant")]
    pub kyc_compliant: bool,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "profileName")]
    pub profile_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountsResponse {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "currentBalance")]
    pub current_balance: f64,
    #[serde(rename = "availableBalance")]
    pub available_balance: f64,
    pub currency: String,
    #[serde(rename = "budgetBalance")]
    pub budget_balance: f64,
    #[serde(rename = "straightBalance")]
    pub straight_balance: f64,
    #[serde(rename = "cashBalance")]
    pub cash_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "transactionType")]
    pub transaction_type: Option<String>,
    pub status: String,
    pub description: String,
    #[serde(rename = "cardNumber")]
    pub card_number: Option<String>,
    #[serde(rename = "postedOrder")]
    pub posted_order: Option<f64>,
    #[serde(rename = "postingDate")]
    pub posting_date: Option<String>,
    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    pub amount: f64,
    #[serde(rename = "runningBalance")]
    pub running_balance: Option<f64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
}
