# String Analysis API

A REST API service that analyses strings and stores their computed properties including length, palindrome status, character frequency, and more. Built with Rust, Axum, PostgreSQL, and Redis.

## Features

- String analysis (length, palindrome check, unique characters, word count, SHA-256 hash, character frequency)
- CRUD operations for analysed strings
- Query filtering (by length, palindrome status, word count, character presence)
- Natural language query support
- Redis caching with automatic invalidation
- Rate limiting (20 requests/minute per IP)
- OpenAPI/Swagger documentation
- Input validation and sanitization

## Prerequisites

- Rust
- PostgreSQL
- Redis
- SQLx CLI

## Dependencies Installation

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

### Install SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

## Setup Instructions

### 1. Clone the repository
```bash
git clone <repository-url>
cd string_analysis_api
```

### 2. Configure environment variables
Create a `.env` file in the project root:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/string_analysis
REDIS_URL=redis://localhost:6379
PORT=3000
RATE_LIMIT_PER_MINUTE=60
LOG_LEVEL=info
```

**Environment Variables:**
- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string
- `PORT`: HTTP server port (default: 3000)
- `RATE_LIMIT_PER_MINUTE`: Rate limit per IP (default: 60)
- `LOG_LEVEL`: Logging level (info/debug/warn/error)

### 3. Create and setup database
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

### Production mode
```bash
cargo run --release
```

The server will start on `http://localhost:3000`

## API Documentation

Interactive Swagger UI documentation is available at:
```
http://localhost:8000/swagger-ui
```

OpenAPI JSON specification:
```
http://localhost:8000/api-docs/openapi.json
```

## API Endpoints

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