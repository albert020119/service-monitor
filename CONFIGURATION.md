# Configuration

This project reads configuration from **`config.json`** in the repo root (hard-coded in `src/main.rs`).

## Service schema

`config.json` must be:

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

Fields:

- **`name`**: Friendly name used in logs and in the dashboard.
- **`url`**: Input shared by all checks in `checks`. Each check type interprets it slightly differently (details below).
- **`checks`**: List of checks to run for this service.

Each check entry:

- **`check_type`**: One of `Http`, `Tcp`, `Dns`, `Ssl` (case-sensitive).
- **`interval_seconds`**: Sleep time between runs for this specific check.
- **`timeout_ms`**: Timeout applied to the request/connect/handshake for this specific check.

## `check_type` details

### `Http`

Implementation: `src/monitor/http_check.rs`

- Sends an HTTP GET to `url`.
- Marks success when the HTTP status is in the \(2xx\) range.
- Records `response_time_ms` as the wall-clock time for the request attempt.

Recommended `url` values:
- `https://example.com/health`
- `http://localhost:8080/readyz`

### `Tcp`

Implementation: `src/monitor/tcp_check.rs`

`url` parsing:
- If `url` includes a scheme (contains `://`), everything before and including `://` is stripped.
- Any path portion after `/` is stripped.
- If a `:port` suffix is present and is numeric, it will be used.
- Otherwise the check defaults to port **80**.

Examples:
- `db.example.com:5432` → connect to `db.example.com:5432`
- `http://example.com` → connect to `example.com:80`
- `http://example.com:8080/anything` → connect to `example.com:8080`

### `Dns`

Implementation: `src/monitor/dns_check.rs`

- Performs a DNS A/AAAA lookup for the host derived from `url`.
- This check will accept a hostname *or* a full URL and will normalize it by stripping:
  - `scheme://` if present
  - any path after `/`
  - a numeric `:port` suffix if present

Good:
- `example.com`
- `internal.service.local`
- `https://example.com`
- `https://example.com:8443/health`

Bad:
- `example.com/path` (includes path)

### `Ssl`

Implementation: `src/monitor/ssl_check.rs`

This check:
- Connects to host/port via TCP
- Attempts a TLS handshake using the host name for SNI / certificate validation

`url` parsing is identical to `Tcp`, except the default port is **443**.

Examples:
- `example.com` → connect+handshake to `example.com:443`
- `example.com:8443` → connect+handshake to `example.com:8443`
- `https://example.com` → connect+handshake to `example.com:443`

## Troubleshooting

- **Dashboard shows no services**: make sure your checks have run at least once; services appear when they are first updated in memory.
- **`Dns` always fails**: ensure `url` is only a hostname (no scheme like `https://`).
- **`Tcp`/`Ssl` always hits the wrong port**: include an explicit `:port` suffix in `url`.
- **TLS failures**: this check currently only verifies that a TLS handshake completes; failures may happen with self-signed certs, wrong SNI name, captive portals, or blocked ports.

