# Rust Diesel API

Run a Rust API (Warp + Diesel + PostgreSQL) with Docker. Migrations and seed data run automatically on startup.

## Quick Start (Docker)

- Prerequisite: Install Docker.
- Prepare a `.env` file at the project root. Use host `postgres` for Docker (see the comments already in `.env`). See `.env.example`.

Start the stack (API + DB):

```powershell
docker compose up -d --build
```

The API serves at `http://localhost:3030`.

Verify quickly:

```powershell
curl http://localhost:3030/items
```

Stop and reset:

```powershell
docker compose down
docker compose down -v   # full reset (drops DB volume)
```

What happens automatically:

- Diesel migrations run on API container start.
- Seed file `/app/seeds/seed.sql` is executed (inserts sample items only).

## Local Development (without Docker)

Use this path only if you are not using Docker.

Tip: Switch `DATABASE_URL` in `.env` to use host `localhost` for local runs (there is a commented example in `.env`).

1. Install Rust toolchain and Diesel CLI:

```powershell
rustup toolchain install stable
cargo install diesel_cli --no-default-features --features postgres
```

1. Start a local PostgreSQL and create a database.

1. Create or edit `.env` to use a localhost URL, e.g.:

```bash
DATABASE_URL=postgresql://postgres:postgres@localhost/rust_diesel
JWT_SECRET=dev_change_me
```

1. Run migrations:

```powershell
diesel migration run
```

1. Seed data (optional):

```powershell
psql "$env:DATABASE_URL" -f .\seeds\seed.sql
```

1. Run the API:

```powershell
cargo run
```

## API Reference

- Requests collection: `api.http` (use VS Code + REST Client extension)
- Protected endpoints require `Authorization: Bearer <token>`.

## Notes on Seeding

- Seed inserts sample items only. Users should be created via `/users/register` (passwords are hashed).
- To reseed, you can TRUNCATE items in `seeds/seed.sql` (commented line).

## Switching between Docker and Local

- Docker: `DATABASE_URL=...@postgres/<db>` (inside containers, the DB host is `postgres`).
- Local: `DATABASE_URL=...@localhost/<db>` (when running `cargo run` on your machine).
- See `.env` for commented examples; uncomment the one you need and comment the other.
