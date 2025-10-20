# String Analysis API - System Design

## Overview

A REST API service that accepts strings, computes various properties (length, palindrome check, character frequency, etc.), and stores them in PostgreSQL with Redis caching. Built with Rust/Axum.

## Architecture

### Component Stack

- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Cache**: Redis
- **Hash Algorithm**: SHA-256

### Request Flow

```
Client Request
    ↓
Rate Limiter                                                       
    ↓                                                     
Request Validation                                                   
    ↓
Cache Check       ⟶         Cache Miss
    ↓                           ↓
Cache Hit  ⟶  Response  ⟵  Database Query

       ⟶ Cache Update (async) ⟵
```

## Data Model

### Schema (Postgresql)

**Table: `analysed_strings`**

```
id                   VARCHAR(64) PRIMARY KEY
value                TEXT NOT NULL UNIQUE
length               INTEGER NOT NULL
is_palindrome        BOOLEAN NOT NULL
unique_char_count    INTEGER NOT NULL
word_count           INTEGER NOT NULL
char_frequency_map       JSONB NOT NULL
created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
```

**Indexes:**
- `idx_is_palindrome` on `is_palindrome`
- `idx_length` on `length`
- `idx_word_count` on `word_count`
- `idx_created_at` on `created_at DESC`
- `idx_char_frequency_map_gin` GIN index on `char_frequency_map` for containment queries

### Redis Keys Structure

- `string:{sha256}` ⟶ Full JSON object (TTL: 1 hour)
- `query:{hash_of_params}` ⟶ Query results array (TTL: 15 minutes)
- `ratelimit:{ip}:{minute}` ⟶ Request counter (TTL: 60 seconds)

## API Endpoints

### POST `/strings`

**Purpose**: Analyse and store a new string

**Request Body**:
```json
{
  "value": "string to analyse"
}
```

**Validation Rules**:
- `value` field must exist (400 if missing)
- `value` must be string type (422 if wrong type)
- Empty strings are not valid

**Processing Steps**:
- Compute SHA-256 hash
- Check if hash exists in cache
- If not in cache, check database
- If exists, return 409 Conflict
- Compute all properties
- Insert into database
- Store in cache (async background task)
- Return 201 with full object

**Response Codes**:
- 201: Created successfully
- 400: Missing `value` field
- 409: String already exists
- 422: `value` is not a string

### GET `/strings/{string_value}`

**Purpose**: Retrieve analysed string by exact value

**Processing Steps**:
- Check Redis cache first
- On miss, query database by `value` column
- If found, cache result (async background task) and return
- If not found, return 404

**Response Codes**:
- 200: Found
- 404: Not found

### GET `/strings`

**Purpose**: List all strings with optional filters

**Query Parameters**:
- `is_palindrome`: boolean (true/false)
- `min_length`: integer >= 0
- `max_length`: integer >= 0
- `word_count`: exact integer match
- `contains_character`: single character

**Filter Logic**:
- All filters are *AND* conditions
- `contains_character` uses JSONB containment: `char_frequency ? 'a'`
- Results ordered by `created_at DESC`

**Processing Steps**:
- Check Redis cache first
- On miss, query database, with applied filter logic
- If found, cache result (async background task) and return
- If not found, return 404

**Response Codes**:
- 200: Success (even if empty results)
- 400: Invalid parameter type or value

### GET `/strings/filter-by-natural-language`

**Purpose**: Query using plain English

**Query Parameter**:
- `query`: URL-encoded natural language string

**Natural Language Parser Rules**:

| Pattern | Maps To |
|---------|---------|
| "longer than N" / "more than N characters" | `min_length = N + 1` |
| "shorter than N" / "less than N characters" | `max_length = N - 1` |
| "exactly N characters" | `min_length = N, max_length = N` |
| "palindrome" / "palindromic" | `is_palindrome = true` |
| "N words" / "N-word" / "single word" | `word_count = N` |
| "contains X" / "containing X" / "with X" | `contains_character = X` (first char) |
| "letter X" / "character X" | `contains_character = X` |

**Parser Implementation**:
- Tokenize query (split by spaces, lowercase)
- Extract numbers using regex
- Match keywords to filter mapping
- Build filter struct from matches
- If parse fails, return 400
- If conflicting filters detected (e.g., min_length > max_length), return 422

**Response Codes**:
- 200: Successfully parsed and executed
- 400: Unable to parse any valid filters
- 422: Conflicting filters detected

### DELETE /strings/{string_value}

**Purpose**: Remove a string by exact value

**Processing Steps**:
- Delete from database by `value`
- Delete from cache (both direct key and invalidate query cache) (async)
- Return 204 if deleted, 404 if not found

**Response Codes**:
- 204: Deleted successfully
- 404: String doesn't exist

## String Analysis Logic

### Length Calculation
```
"hello" ⟶ 5
"" ⟶ 0
"hello world" ⟶ 11 (includes space)
```

### Palindrome Check
- Case-insensitive comparison
- Ignore whitespace and punctuation? **No**
```
"Racecar" ⟶ true
"race car" ⟶ false (spaces break palindrome)
"racecar" ⟶ true
```

### Unique Characters
Count distinct characters (case-insensitive)
```
"hello" ⟶ 4 (h, e, l, o)
"AAaa" ⟶ 1 (a)
```

### Word Count
Split on any whitespace, count non-empty tokens
```
"hello world" ⟶ 2
"hello  world" ⟶ 2 (multiple spaces = 1 separator)
"hello" ⟶ 1
"" ⟶ 0
```

### Character Frequency Map
Case-insensitive character counts
```
"hello" ⟶ {"h": 1, "e": 1, "l": 2, "o": 1}
"AaA" ⟶ {"a": 1}
```

## Rate Limiting

**Strategy**: Token bucket per IP address

**Implementation**:
- Redis key: `ratelimit:{ip_address}:{current_minute}`
- Limit: 60 requests per minute per IP
- Sliding window using current minute timestamp
- Return 429 Too Many Requests when exceeded
- Include headers:
  - `X-RateLimit-Limit: 60`
  - `X-RateLimit-Remaining: 47`
  - `X-RateLimit-Reset: {unix_timestamp}`

## Caching Strategy
- Cache query results using hash of all parameters as key
- Invalidate on any POST or DELETE operation

### Write Operations (POST)
- Insert to database first (source of truth)
- Background task: Write to Redis cache
- If cache write fails, log but don't fail request

### Read Operations (GET)
- Check Redis cache
- On hit: Return immediately
- On miss: Query database ⟶ Update cache ⟶ Return

### Cache Invalidation
- DELETE operation: Remove specific key + clear query cache
- POST operation: Clear query cache (list endpoints)
- Use Redis `SCAN` with pattern matching for bulk invalidation

### TTL Values
- Individual strings: 1 hour
- Query results: 15 minutes
- Rate limit counters: 60 seconds

## Performance Optimizations

### Database
- Connection pooling (10-20 connections)
- Prepared statements for all queries
- Batch inserts if processing multiple strings
- JSONB indexes for character frequency lookups

### Application
- Async I/O throughout (Tokio runtime)
- Background tasks for non-critical cache writes
- Lazy evaluation of string properties
- Reuse SHA-256 hasher instances

### Redis
- Pipeline commands when possible
- Connection pooling
- Compression for large query results (>1KB)

## Input Sanitization

### SQL Injection Prevention
- Use parameterized queries exclusively
- Never construct SQL with string concatenation
- Axum extractors provide automatic type validation

### Request Validation
- Validate JSON structure before processing
- Type checking via serde deserialization
- Reject requests with unexpected fields (strict mode)
- Sanitize string values: no special handling needed (store as-is)

## OpenAPI/Swagger Documentation

**Generation**: Use `utoipa` crate

**Included Information**:
- All endpoints with request/response schemas
- Error response schemas (400, 404, 409, 422, 429)
- Query parameter descriptions and types
- Example requests and responses
- Authentication (if added later)

**Access**: `/api-docs` (JSON) and `/swagger-ui` (interactive UI)

## Error Response Format

Standardized error structure:

```json
{
  "error": {
    "code": "INVALID_INPUT",
    "message": "Field 'value' is required",
    "details": {
      "field": "value"
    }
  }
}
```

**Error Codes**:
- `INVALID_INPUT`: 400 errors
- `NOT_FOUND`: 404 errors
- `CONFLICT`: 409 errors
- `VALIDATION_ERROR`: 422 errors
- `RATE_LIMIT_EXCEEDED`: 429 errors

## Background Tasks

**Use Cases**:
- Cache warming after database write
- Async cache invalidation
- Metrics collection

**Implementation**:
- Spawn Tokio tasks for non-blocking operations
- Don't wait for completion on request path
- Log failures but don't propagate to client

---