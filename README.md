# Investec Transaction Buckets

A Rust application for interacting with the Investec Private Banking API to retrieve account information, balances, and transaction data.

## Prerequisites

- Rust 1.75 or later
- Investec Private Banking API credentials

## Setup

### 1. Clone the repository

```bash
git clone https://github.com/dylanvanh/investec-transaction-buckets
cd investec-transaction-buckets
```

### 2. Build the project

```bash
cargo build
```

### 3. Configure API credentials

Create a `.cargo/config.toml` file:

```toml
[env]
X_API_KEY = "your-x-api-key"
CLIENT_ID = "your-client-id"
CLIENT_SECRET = "your-client-secret"
```

### 4. Get API Credentials

1. Visit your [Investec Online Banking](https://login.secure.investec.com) profile
2. Navigate to API settings and create a new API key
3. Note down your Client ID, Client Secret, and X-API-Key

## Usage

```bash
cargo run
```

## API Documentation

For complete API documentation, see: [Investec Private Banking API Documentation](https://developer.investec.com/za/api-products/documentation/SA_PB_Account_Information)
