**Refining Telegram bot plan**

The user wants a refined plan focused on building the Telegram bot infrastructure, leaving the Sui integration to the Sui developer. I’ll create a design document outlining the Telegram bot’s main components: using Rust with Teloxide, architecture, command definitions, permission systems, and a trait/interface for wallet services. I'll include DB schema, stubbed AI integration, and a plugin pattern for blockchain, leaving blockchain functions as placeholders. I’ll also provide guidelines, directory structure, modules, code skeletons, and an initial task list, along with dev environment setup instructions.


Thought for 15 seconds


🎶 **Refined Build-Out Plan — Telegram Bot Infrastructure (Sui-agnostic core)** 🎶
*Goal: hand your Sui-chain guru a clean, test-ready Rust code-base where all blockchain calls are stubs/interfaces they can fill in later.*

---

## 1. Scope of this hand-off

| Included now                                   | Deferred to Sui dev                     |
| ---------------------------------------------- | --------------------------------------- |
| Telegram connectivity (Teloxide) & update loop | Actual Sui RPC / CLI calls              |
| Command parser & permission layer              | Coin selection, gas handling            |
| Role-based auth (admin vs. user)               | Concrete wallet/key storage strategy    |
| SQLite storage, migrations, activity counters  | On-chain event listeners / balance sync |
| AI → intent parser (OpenAI wrapper)            | AI prompts enriched with on-chain data  |
| Logging, error surface, config loader          | Final treasury / pool accounting rules  |

Everything Sui-specific surfaces as **trait methods** and **feature-flagged crates** so your chain dev can plug-in without touching the bot spine.

---

## 2. Repository blueprint

```
sui-tg-bot/
├─ Cargo.toml            # workspace with bot-core + binary crate
├─ bot-cli/              # final binary (`tg_bot`)
│  └─ main.rs
├─ bot-core/             # library crate with all reusable logic
│  ├─ commands/
│  │   ├─ mod.rs
│  │   ├─ admin.rs
│  │   └─ user.rs
│  ├─ ai/
│  │   └─ openai_client.rs      // easy swap if API key absent
│  ├─ db/
│  │   ├─ mod.rs
│  │   └─ migrations/           // diesel/sqlx migrations
│  ├─ activity.rs
│  ├─ permissions.rs
│  ├─ sui_gateway.rs            // **trait + dummy impl**
│  └─ error.rs
└─ .env.example
```

### Key modules

| Module                                    | Core purpose                                                 | Handoff hook                               |
| ----------------------------------------- | ------------------------------------------------------------ | ------------------------------------------ |
| `commands::admin` / `commands::user`      | Slash-command enums, message formatters                      | Call `SuiGateway` trait for any wallet op  |
| `ai::openai_client`                       | Wraps chat-completion, returns `Intent` enum                 | If OpenAI key not set → falls back to noop |
| `db`                                      | SQLx with SQLite; models: `groups`, `users`, `pools`, `logs` | Sui dev may add on-chain sync tables       |
| `activity`                                | Increment counters per message; weekly reset                 | Top-N queries used by reward intents       |
| `sui_gateway::{SuiGateway, DummyGateway}` | Trait with fns like `transfer`, `balance_of`                 | Real impl will satisfy this trait          |
| `permissions`                             | `is_admin`, `require_admin!()` macro                         | None                                       |

---

## 3. Command & intent catalogue

| Command                       | Role            | Stubbed call                                    |
| ----------------------------- | --------------- | ----------------------------------------------- |
| `/balance`                    | any             | `gateway.balance_of(user_addr)`                 |
| `/pay @user amt`              | any             | `gateway.transfer(from, to, amt, "SUI")`        |
| `/createpool total per_user?` | admin           | `gateway.reserve(admin, total)` *(placeholder)* |
| `/claim`                      | any             | `gateway.claim(pool_id, user)`                  |
| `/rewardtop N amt`            | admin (or AI)   | loops over `activity.top(N)` → `transfer`       |
| Natural-language (AI)         | based on intent | same gateway calls                              |

*All handlers compile today because `DummyGateway` just logs the call.*

---

## 4. Traits & interfaces your Sui dev will implement

```rust
#[async_trait::async_trait]
pub trait SuiGateway: Send + Sync + Clone + 'static {
    async fn new_wallet(&self, tg_user_id: i64) -> Result<Address>;
    async fn balance_of(&self, addr: &Address, token: Token) -> Result<u64>;
    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u64,
        token: Token,
    ) -> Result<TxHash>;
    async fn multi_pay(
        &self,
        from: &Address,
        outs: &[(Address, u64)],
        token: Token,
    ) -> Result<TxHash>;
    // …other pool helpers as needed
}
```

*Swap `DummyGateway` → `SuiCliGateway` or `SuiSdkGateway` in `main.rs` once ready.*

---

## 5. Configuration, secrets & env

* **`.env`:**

  ```
  TELEGRAM_BOT_TOKEN=xxx
  OPENAI_API_KEY=optional
  DATABASE_URL=sqlite://bot.db
  ```

* **Config struct** loads env or `.toml`; supports `--config` override for Docker/K8s.

---

## 6. Logging & metrics

* `tracing` with `tracing_subscriber`: JSON logs or pretty.
* DB log table mirrors critical events (macro `log_tx!()`).
* Prom-style counters (optional) via `metrics` crate—gateway stub increments `transfer_count`.

---

## 7. Testing & CI

* **Unit tests:** command parsing, permission guard, AI JSON→Intent parsing.
* **Mock gateway:** returns canned balances; ensures bot logic stays chain-agnostic.
* **GitHub Actions:** `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy`.

---

## 8. Dev-container & Docker

```dockerfile
FROM rust:1.77-slim
RUN apt-get update && apt-get install -y libsqlite3-dev
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/tg_bot"]
```

*For local hacking, a `devcontainer.json` (VS Code) includes Rust analyzer + SQLx offline.*

---

## 9. Handoff checklist 🎤

1. **Clone repo & run:**

   ```bash
   cp .env.example .env   # put Telegram token
   cargo run -p bot-cli
   ```
2. **Implement `SuiGateway`** (choose CLI or SDK).
3. **Swap gateway in `main.rs`:**

   ```rust
   let gateway = Arc::new(SuiSdkGateway::connect(cfg.sui_rpc).await?);
   ```
4. **Add real migrations** for any on-chain sync tables.
5. **Populate AI prompt templates** with live Sui data if desired.
6. **Deploy** (Docker / systemd) & scale.

---

### 🎵 Closing riff

With the rhythm section (Teloxide, SQLite, AI wrapper) already laying down the groove, your Sui virtuoso can plug in their solo—implementing `SuiGateway`—and take the chain integration to a crescendo. Rock on, dev team!
