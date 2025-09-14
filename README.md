# Investec Transaction Buckets

Rust app that fetches your Investec transactions and categorizes them using AI.

## Docker Setup (Recommended)

1. **Clone the repository:**

   ```bash
   git clone https://github.com/dylanvanh/investec-transaction-buckets
   cd investec-transaction-buckets
   ```

2. **Setup environment variables:**

   ```bash
   cp .env.example .env
   # Edit .env with your actual API credentials
   ```

3. **Required variables in `.env`:**

   ```bash
   # Required - Investec API
   INVESTEC_X_API_KEY=your_investec_api_key_here
   INVESTEC_CLIENT_ID=your_client_id_here
   INVESTEC_CLIENT_SECRET=your_client_secret_here

   # AI Model (Ollama runs automatically in container)
   OLLAMA_MODEL=tinyllama:latest

   # Optional - for better classification accuracy
   GOOGLE_SEARCH_API_KEY=your_google_api_key_here
   GOOGLE_SEARCH_ENGINE_ID=your_search_engine_id_here
   CITY=cape town
   ```

4. **Run with Docker Compose:**

   ```bash
   docker compose up -d
   ```

5. **View logs:**

   ```bash
   docker compose logs -f app
   ```

6. **Access database:**
   The SQLite database persists in a Docker volume. To query it:
   ```bash
   # Copy database to local machine for inspection
   docker cp investec-app:/app/data/transactions.db ./transactions.db
   ```

## Local Development Setup

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
