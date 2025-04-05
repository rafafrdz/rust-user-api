# Rust Web API with Axum

This project is a web API built using Rust and the Axum framework. It includes database integration with SQLx and
PostgreSQL, environment configuration with dotenvy, and structured logging with tracing.

## Features

- Health check endpoint
- User management
- CORS support
- Structured logging
- Database connection pooling

## Prerequisites

- Rust
- Cargo
- PostgreSQL

## Getting Started

### Clone the repository

```sh
git clone https://github.com/rafafrdz/rust-user-api.git
cd rust-user-api
```

### Set up the environment

Create a .env file in the root directory and add the following environment variables:

```env
DATABASE_URL=postgres://username:password@localhost/dbname
RUST_LOG=info
```

### Run database migrations

Ensure you have a PostgreSQL database running and run the migrations:

```sh
cargo install sqlx-cli
sqlx migrate run
```

### Start the server

```sh
cargo run
```

The server will start on `http://localhost:3000`.

### Test the API

You can use tools like Postman or curl to test the API endpoints.

```sh
curl http://localhost:3000/health
```

### Debugging

To run the server in debug mode, open two shell and use the following commands:

### In the first shell, run the server:
```sh
cargo watch -q -c -w src/ -x run
```

### In the second shell, run the tests:
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```


## Endpoints

### Health Check

- GET /health

### Get Current User

- POST /me
    - Request body: { "email": "user@example.com" }

## Project Structure

```plaintext
src/main.rs: Entry point of the application
src/db.rs: Database connection and queries
src/error.rs: Custom error handling
src/models.rs: Data models
src/routes: Route handlers
migrations: SQL migration files
tests: Integration tests
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
