# Worker Cleaner

MongoDB collection cleaner with scheduled cron jobs.

## Setup

1. Copy environment file:
```bash
cp .env.example .env
```

2. Configure `.env`:
```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=duck_db
COLLECTIONS=collection1,collection2,collection3
CRON_SCHEDULE=0 0 0 */3 * *
```

3. Run:
```bash
cargo run
```

## Features

- Deletes all documents from specified collections
- Runs on cron schedule (default: every 3 days at midnight)
- Executes once immediately on startup
- Logs deletion counts per collection

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `MONGODB_URI` | MongoDB connection string | `mongodb://localhost:27017` |
| `DATABASE_NAME` | Database name | `duck_db` |
| `COLLECTIONS` | Comma-separated collection names | `logs,users,temp` |
| `CRON_SCHEDULE` | Cron expression (optional) | `0 0 0 */3 * *` |

## Cron Schedule Format

```
sec min hour day-of-month month day-of-week
```

Examples:
- `0 0 0 */3 * *` - Every 3 days at midnight
- `0 0 2 * * *` - Daily at 2 AM
- `0 0 0 * * 0` - Every Sunday at midnight
