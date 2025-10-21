# String Analysis API

A simple REST API service that analyses strings and stores their computed properties including length, palindrome status, character frequency, and more. Built with Rust, Axum, PostgreSQL, and Redis.

## Features

- String analysis (length, palindrome check, unique characters, word count, SHA-256 hash, character frequency)
- CRUD operations for analysed strings
- Query filtering (by length, palindrome status, word count, character presence)
- Natural language query support
- Redis caching with automatic invalidation
- Rate limiting (20 requests/minute per IP)
- OpenAPI/Swagger UI documentation
- Input validation and sanitization

## Dependencies

This project uses the following Rust crates:

- **axum** - Web framework
- **tokio** - Async runtime
- **sqlx** - Database (postgresql) support
- **serde** and **serde_json** - Serialization/deserializationutilities (CORS)
- **utoipa** and **utoipa-swagger-ui** - OpenAPI/Swagger UI integration
- **chrono** - Date and time handling
- **dotenvy** - Environment variable loading
- **anyhow** - Error handling
- **tracing** and **tracing-subscriber** - Logging implementation
- **sha2** and **hex** - String hashing

## Prerequisites

- Rust/Cargo
- PostgreSQL
- Redis
- SQLx CLI

## Installation

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install PostgreSQL
**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

**macOS:**
```bash
brew install postgresql@14
brew services start postgresql@latest
```

### Install Redis
**Ubuntu/Debian:**
```bash
sudo apt install redis-server
sudo systemctl start redis
```

**macOS:**
```bash
brew install redis
brew services start redis
```

### Install SQLx CLI (with tls support)
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

## Setup Instructions

### Clone the repository
```bash
git clone <repository-url>
cd string_analyser
```

### Configure environment variables
Create a `.env` file in the project root:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/string_analysis
REDIS_URL=redis://localhost:6379
PORT=8000
HOST=0.0.0.0
RATE_LIMIT_PER_MINUTE=60
LOG_LEVEL=info
```

**Environment Variables:**
- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string
- `PORT`: HTTP server port (default: 8000)
- `HOST`: HTTP server host (default: 0.0.0.0)
- `RATE_LIMIT_PER_MINUTE`: Rate limit per IP (default: 60)
- `LOG_LEVEL`: Logging level (info/debug/warn/error)

### Create and setup database
```bash
# Create database
sqlx database create

# Run migrations
sqlx migrate run
```

### 4. Build the project
```bash
cargo build --release
```

## Running Locally

### Development mode
```bash
cargo run
```

The server will start on the configured host and port (default: http://0.0.0.0:8000)

### Production mode
```bash
cargo run --release
```

### Docker Deployment
1. Build the Docker image:
```bash
docker build -t string_analyser .
```

2. Run the Docker container:
```bash
docker run -p 8000:8000 -e RUST_LOG=info -e SERVER_HOST=0.0.0.0 -e SERVER_PORT=8000 string_analyser
```

## API Documentation

Interactive Swagger UI documentation is available at:
```
http://localhost:8000/swagger-ui
```

OpenAPI JSON specification:
```
http://localhost:8000/api-docs/openapi.json
```

## API Documentation

### Health Check
```
GET /health
```

### Create/Analyse String
```
POST /strings
Content-Type: application/json

{
  "value": "string to analyse"
}
```

### Get String by Value
```
GET /strings/{string_value}
```

### Get All Strings (with filters)
```
GET /strings?is_palindrome=true&min_length=5&max_length=20&word_count=2&contains_character=a
```

### Natural Language Filter
```
GET /strings/filter-by-natural-language?query=all%20single%20word%20palindromic%20strings
```

### Delete String
```
DELETE /strings/{string_value}
```

## Example Usage
```bash
# Create a string
curl -X POST http://localhost:3000/strings \
  -H "Content-Type: application/json" \
  -d '{"value":"racecar"}'

# Get a string
curl http://localhost:3000/strings/racecar

# Get palindromes
curl "http://localhost:3000/strings?is_palindrome=true"

# Natural language query
curl "http://localhost:3000/strings/filter-by-natural-language?query=strings%20longer%20than%2010%20characters"

# Delete a string
curl -X DELETE http://localhost:3000/strings/racecar
```

## Project Structure
```
string_analysis_api/
├── src/
│   ├── cache/          # Caching logic
│   ├── db/             # Database repository
│   ├── routes/         # Request handlers
│   ├── middleware/     # Rate limiting
│   ├── models/         # Data models
│   ├── utils/          # String analyser, NLP parser
│   ├── api.rs       
│   ├── lib.rs       
│   └── main.rs         # App entry point
├── .env               
├── Cargo.toml        
├── DESIGN.md           # Design doc       
└── README.md
```

## Rate Limiting
The API implements IP-based rate limiting to prevent abuse:

- Limit: 20 requests per minute per IP address
- Response: HTTP 429 (Too Many Requests) when limit exceeded
- Window: Fixed 60-second window aligned to minute boundaries
- Tracking: Uses redis for thread-safe concurrent access