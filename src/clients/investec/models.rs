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
    /// A unique and immutable identifier used to identify the account resource. This identifier has no meaning to the account owner.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Enum: "CREDIT" "DEBIT"
    #[serde(rename = "type")]
    pub type_: String,
    /// Refers to the transaction type filter's value.
    #[serde(rename = "transactionType")]
    pub transaction_type: Option<String>,
    /// Enum: "POSTED" "PENDING"
    pub status: String,
    /// Unique identifier for the transaction within an servicing institution. This identifier is both unique and immutable.
    pub description: String,
    /// Unique identifier for the transaction within an servicing institution. This identifier is both unique and immutable.
    #[serde(rename = "cardNumber")]
    pub card_number: Option<String>,
    /// Unique identifier for the transaction within an servicing institution. This identifier is both unique and immutable.
    #[serde(rename = "postedOrder")]
    pub posted_order: Option<f64>,
    /// Date and time when a transaction entry is posted to an account on the account servicer's books. Usage: Booking date is the expected booking date, unless the status is booked, in which case it is the actual booking date. All dates in the JSON payloads are represented in ISO 8601 date-time format. All date-time fields in responses must include the timezone. An example is below: 2017-04-05T10:43:07+00:00
    #[serde(rename = "postingDate")]
    pub posting_date: Option<String>,
    /// Date and time at which assets become available to the account owner in case of a credit entry, or cease to be available to the account owner in case of a debit transaction entry. Usage: If transaction entry status is pending and value date is present, then the value date refers to an expected/requested value date. For transaction entries subject to availability/float and for which availability information is provided, the value date must not be used. In this case the availability component identifies the number of availability days. All dates in the JSON payloads are represented in ISO 8601 date-time format. All date-time fields in responses must include the timezone. An example is below: 2017-04-05T10:43:07+00:00
    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,
    /// Date and time at which assets become available to the account owner in case of a credit entry, or cease to be available to the account owner in case of a debit transaction entry. Usage: If transaction entry status is pending and value date is present, then the value date refers to an expected/requested value date. For transaction entries subject to availability/float and for which availability information is provided, the value date must not be used. In this case the availability component identifies the number of availability days. All dates in the JSON payloads are represented in ISO 8601 date-time format. All date-time fields in responses must include the timezone. An example is below: 2017-04-05T10:43:07+00:00
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    /// Date and time at which assets become available to the account owner in case of a credit entry, or cease to be available to the account owner in case of a debit transaction entry. Usage: If transaction entry status is pending and value date is present, then the value date refers to an expected/requested value date. For transaction entries subject to availability/float and for which availability information is provided, the value date must not be used. In this case the availability component identifies the number of availability days. All dates in the JSON payloads are represented in ISO 8601 date-time format. All date-time fields in responses must include the timezone. An example is below: 2017-04-05T10:43:07+00:00
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    /// A number of monetary units specified in an active currency where the unit of currency is explicit and compliant with ISO 4217.
    pub amount: f64,
    /// A number of monetary units specified in an active currency where the unit of currency is explicit and compliant with ISO 4217.
    #[serde(rename = "runningBalance")]
    pub running_balance: Option<f64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
}
