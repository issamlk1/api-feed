# API Feeder

A small Rust API service that fetches JSON from an external URL on a schedule, stores each response in SQLite, and exposes the stored records through protected HTTP endpoints.

The app is built with Axum, Tokio, SQLx, SQLite, and Docker.

## Features

- Scheduled JSON fetching with cron expressions
- Multiple fetch schedules through `CRON_SCHEDULES`
- Manual fetch trigger endpoint
- SQLite persistence
- API key protection for data and fetch routes
- Docker-first setup
- Runtime healthcheck endpoint
- Configurable app port with `APP_PORT`

## Requirements

- Docker
- Docker Compose

No local Rust installation is required for the Docker workflow.

## Quick Start

Create your environment file:

```sh
cp .env.example .env
```

Edit `.env`:

```env
APP_NAME=api-feeder
APP_PORT=3000
DATABASE_URL=sqlite:///app/data/data.db?mode=rwc
API_KEY=replace-this-with-a-secret
FETCH_URL=https://dummyjson.com/test
CRON_SCHEDULES="0 0 14 * * MON-FRI,0 30 21 * * MON-FRI"
```

Start the app:

```sh
docker compose up -d --build
```

Check status:

```sh
docker compose ps
docker compose logs -f app
```

The app will be available at:

```text
http://localhost:3000
```

If you change `APP_PORT`, use that port instead.

## Environment Variables

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `APP_NAME` | No | none | Human-readable app name. |
| `APP_PORT` | No | `3000` | Port the app listens on inside Docker and on the host. |
| `DATABASE_URL` | Yes | none | SQLite connection string. Use `?mode=rwc` so SQLite can create the file. |
| `API_KEY` | Yes | none | Required value for the `x-api-key` request header. |
| `FETCH_URL` | Yes | none | External URL fetched by the scheduler and `/fetch`. |
| `CRON_SCHEDULES` | Yes | none | Comma-separated cron expressions. |

## Cron Format

This project uses `tokio-cron-scheduler`, which expects a seconds field.

Examples:

```env
# Every minute
CRON_SCHEDULES="0 * * * * *"

# Monday-Friday at 14:00 and 21:30
CRON_SCHEDULES="0 0 14 * * MON-FRI,0 30 21 * * MON-FRI"
```

Do not define `CRON_SCHEDULES` more than once. Environment variables with the same name overwrite each other.

## API

### Healthcheck

```http
GET /health
```

No API key required.

Example:

```sh
curl http://localhost:3000/health
```

### Get Records

```http
GET /data
GET /data?date=YYYY-MM-DD
```

Requires:

```http
x-api-key: your-secret
```

Example:

```sh
curl \
  -H "x-api-key: replace-this-with-a-secret" \
  "http://localhost:3000/data"
```

Response shape:

```json
{
  "date": "2026-06-25",
  "count": 1,
  "records": [
    {
      "id": "uuid",
      "created_at": "2026-06-25",
      "data": {}
    }
  ]
}
```

### Trigger Fetch Manually

```http
POST /fetch
```

Requires:

```http
x-api-key: your-secret
```

Example:

```sh
curl \
  -X POST \
  -H "x-api-key: replace-this-with-a-secret" \
  "http://localhost:3000/fetch"
```

## SQLite Data

The database is stored through the Compose bind mount:

```yaml
./data:/app/data
```

Use this database URL in Docker:

```env
DATABASE_URL=sqlite:///app/data/data.db?mode=rwc
```

The `mode=rwc` part means read/write/create. Without it, SQLite may fail with `unable to open database file` when the database file does not exist yet.

## Docker Notes

Build and start:

```sh
docker compose up -d --build
```

Restart after changing only `.env`:

```sh
docker compose up -d --force-recreate app
```

Stop:

```sh
docker compose down
```

The Dockerfile uses Cargo cache mounts to make repeated Rust builds faster.

## Security Notes

- Do not commit your real `.env`.
- Rotate `API_KEY` if it is ever exposed.
- Put the service behind HTTPS before exposing it publicly.
- Back up the `data` directory if the stored records matter.

## License

MIT
