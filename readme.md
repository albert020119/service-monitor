# Service Health Monitor

A small Rust service that periodically checks your services (HTTP, TCP, DNS, SSL) and exposes a simple real-time dashboard.

## Features

- **Multiple checks per service** (one `url`, many checks)
- **Dashboard**: `GET /` (auto-refreshes every 5s)
- **JSON API**: `GET /api/status`

## Quick start

1. Edit `config.json` in the repo root.
2. Run:

```bash
cargo run --release
```

3. Open `http://localhost:3000`.

Notes:
- The config path is currently **hard-coded** to `config.json` in the current working directory (see `src/main.rs`).
- The dashboard binds to **`0.0.0.0:3000`** (accessible from other machines on your network if your firewall allows it).

## Configuration

The config file is a JSON object with a `services` array. Each service has a `url` and a list of `checks`.

### Schema

Each service:

- **`name`**: display name
- **`url`**: input for checks (see notes per check type below)
- **`checks`**: list of checks for this service

Each check:

- **`check_type`**: one of `Http`, `Tcp`, `Dns`, `Ssl` (case-sensitive)
- **`interval_seconds`**: how often to run this check
- **`timeout_ms`**: timeout for a single check

### Example `config.json`

```json
{
  "services": [
    {
      "name": "Example",
      "url": "https://example.com",
      "checks": [
        { "check_type": "Http", "interval_seconds": 30, "timeout_ms": 5000 },
        { "check_type": "Ssl", "interval_seconds": 60, "timeout_ms": 5000 },
        { "check_type": "Dns", "interval_seconds": 300, "timeout_ms": 3000 }
      ]
    }
  ]
}
```

For more detail and troubleshooting, see `CONFIGURATION.md`.

## Dashboard + API

- **`GET /`**: dashboard HTML
- **`GET /api/status`**: returns:
  - `{"services":[ ... ]}`

Each service entry includes:
- overall fields: `status`, `last_check`, `response_time_ms`, `uptime_percentage`, `total_checks`, `successful_checks`, `message`
- `checks`: an array of per-check statuses (each with its own status/uptime/response/next-check interval)

## Project layout

- `src/monitor/`: check implementations
- `src/dashboard/`: Axum routes for `/` and `/api/status`
- `src/state.rs`: in-memory service + per-check status storage
- `config.json`: default configuration

## Current limitations / TODOs

- **Alerting**: `src/alert/` contains helper modules, but alerting is **not currently wired into the monitoring loop**.
- **Incidents**: `src/models/incident.rs` exists but is not currently used.
- **Dashboard HTML**: the dashboard is embedded in `src/dashboard/routes.rs`. The `public/status_page/` directory is currently unused.
- **CLI/config path**: no CLI flags yet; config path and bind address/port are hard-coded.

## License

MIT
