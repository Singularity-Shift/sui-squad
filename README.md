# Sui Squad Bot

A modular, Sui-agnostic Telegram bot infrastructure in Rust, designed for easy blockchain integration and team collaboration.

## Features
- Telegram bot using [Teloxide](https://github.com/teloxide/teloxide)
- SQLite storage via [SQLx](https://github.com/launchbadge/sqlx)
- AI intent parsing (OpenAI stub)
- Role-based permissions (admin/user)
- Pluggable blockchain gateway trait (`SuiGateway`)
- Activity tracking and reward logic

## Repository Structure
```
sui-squad-bot/      # Binary crate (main entrypoint)
sui-squad-core/     # Library crate (all logic, reusable)
.env.example        # Example environment variables
```

## Quick Start
```bash
cp .env.example .env  # Add your Telegram token
cargo run -p sui-squad-bot
```

## Development Plan
- [x] Workspace and crate scaffolding
- [x] Config loader, DB, permissions, error types
- [x] SuiGateway trait and dummy implementation
- [x] Teloxide bot skeleton
- [ ] Command parsing and dispatch
- [ ] SQLx migrations and schema
- [ ] AI intent mapping
- [ ] Unit tests and CI

## Contributing
PRs and issues welcome! See the [dev-brief.md](./dev-brief.md) for the full design plan.

---

_This README will be updated as the project evolves._ 