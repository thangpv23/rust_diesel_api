# Rust Diesel API

## Setup

1. Install Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`
2. Set up PostgreSQL
3. Create and update .env using sample from .env.example
4. Run migrations: `diesel migration run`

## Build and Run

### Docker (recommended)

Build and start all services (API + PostgreSQL):

```bash
docker-compose up --build
```

The API will be available at `http://localhost:3030`.

To stop:

```bash
docker-compose down
```

To stop and remove volumes (full reset):

```bash
docker-compose down -v
```

### Local (without Docker)

`cargo run`

## Seeding Data

Use the provided SQL seed to quickly populate sample items.

Prerequisites:

- Run migrations first: `diesel migration run`
- PostgreSQL client (`psql`) available in your PATH

Run seed (PowerShell on Windows):

```
# Uses DATABASE_URL from .env
psql -d "postgresql://username:password@localhost/dbname" -f ".\seeds\seed.sql"
```

Quick data check:

```
psql -d "postgresql://username:password@localhost/dbname" -c "SELECT id, name, price, stock FROM items ORDER BY id;"
```

Notes:

- The seed inserts only items. Users require hashed passwords; create users via API `/users/register` instead of raw SQL.
- To reseed, you may uncomment the TRUNCATE line in `seeds/seed.sql`.

## API Reference

- Simple HTTP requests file: `api.http` (open in VS Code; each block is runnable)
  - **Recommended:** Install [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension for syntax highlighting and "Send Request" buttons

Note: Protected endpoints require header `Authorization: Bearer <token>`.
