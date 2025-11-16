# Service Health Monitor

Monitor service health with HTTP, TCP, DNS, and SSL checks. Includes a web dashboard for viewing real-time status.

## Quick Start

```bash
cargo build --release
cargo run --release
```

Open `http://localhost:3000` in your browser.

## Configuration

Edit `config.json` to add services:

```json
{
  "services": [
    {
      "name": "Example",
      "url": "https://example.com",
      "check_type": "Http",
      "interval_seconds": 30,
      "timeout_ms": 5000
    }
  ]
}
```

Check types: `Http`, `Tcp`, `Dns`, `Ssl`

## API

- `GET /` - Dashboard
- `GET /api/status` - JSON status of all services

## License

MIT
