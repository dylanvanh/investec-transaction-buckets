# Investec Transaction Buckets

Rust app that fetches your Investec transactions and categorizes them using AI.

## Quick Start

1. **Install dependencies:**

   - Rust 1.85+
   - **Either** [Ollama](https://ollama.ai/) **or** Gemini API key

2. **Clone and build:**

   ```bash
   git clone https://github.com/dylanvanh/investec-transaction-buckets
   cd investec-transaction-buckets
   cargo build
   ```

3. **Setup credentials** in `.cargo/config.toml`:

   ```toml
   [env]
   # Required
   INVESTEC_X_API_KEY = "your-investec-api-key"
   INVESTEC_CLIENT_ID = "your-client-id"
   INVESTEC_CLIENT_SECRET = "your-client-secret"

   # Choose ONE AI service (required)
   # Option 1: Local AI (Ollama)
   OLLAMA_MODEL = "tinyllama:latest"

   # Option 2: Cloud AI (Gemini - includes built-in search)
   GEMINI_API_KEY = "your-gemini-api-key"
   GEMINI_MODEL = "gemini-1.5-flash"

   # Optional: Improves Ollama accuracy (not needed for Gemini)
   GOOGLE_SEARCH_API_KEY = "your-google-search-api-key"
   GOOGLE_SEARCH_ENGINE_ID = "your-search-engine-id"

   # Optional
   CITY = "cape town"
   ```

4. **Get API keys:**

   - **Investec**: Login to online banking → API settings
   - **Gemini API**: [Get a Gemini API key](https://ai.google.dev/gemini-api/docs) (if using Gemini)
   - **Google Search API**: [Create Custom Search Engine](https://developers.google.com/custom-search/v1/overview) (optional, improves Ollama)

5. **If using Ollama, start it:**

   ```bash
   ollama pull tinyllama:latest
   ollama serve
   ```

6. **Run:**
   ```bash
   cargo run
   ```

## How it works

- Fetches recent transactions from Investec API
- Uses AI (Ollama or Gemini) to classify transactions into buckets
- Gemini uses built-in Google Search, Ollama can use external search for better accuracy
- Outputs categorized transactions

## Requirements

- **Investec API credentials** (required)
- **Either Ollama running OR Gemini API key** (choose one)
- **Google Search API** (optional, improves Ollama accuracy)

## Buckets

Transactions are categorized into: Food, Transportation, Entertainment, Bills & Utilities, Healthcare, Income, Transfers, Other.
