# Sui Squad - AI-Powered Blockchain Telegram Bot

A comprehensive Rust-based Telegram bot ecosystem that integrates with the Sui blockchain, providing AI-powered wallet management, zkLogin authentication, and seamless cryptocurrency operations through natural language commands.

## 🚀 What is Sui Squad?

Sui Squad is an intelligent Telegram bot that bridges the gap between complex blockchain operations and user-friendly conversational interfaces. It leverages:

- **AI-Powered Commands**: Natural language processing for blockchain operations using OpenAI integration
- **zkLogin Authentication**: Secure Google OAuth-based authentication via Enoki API
- **Sui Blockchain Integration**: Full wallet management including balance checking, transfers, and funding
- **JWT Authentication**: Secure session management with automatic token renewal
- **Multi-Service Architecture**: Modular design with separate bot and server components

### Key Features

🤖 **AI Assistant**: Chat naturally with the bot to perform blockchain operations  
🔐 **Secure Authentication**: Google OAuth integration with zkLogin proofs  
💰 **Wallet Management**: Check balances, send funds, and manage accounts  
🔄 **Automatic Funding**: Seamless account funding through web interface  
📱 **Group Support**: Works in both private chats and Telegram groups  
⚡ **Real-time Processing**: Live conversation tracking and context management  

## 🏗️ Project Architecture

```
sui-squad/
├── sui-squad-bot/          # Telegram bot service
│   ├── src/
│   │   ├── bot_manage/     # Bot command handlers and routing
│   │   ├── middleware/     # Authentication and user management
│   │   ├── tools/          # AI function schemas and tools
│   │   └── services/       # External service integrations
│   └── Dockerfile
├── sui-squad-server/       # Web server for webhooks and API
│   ├── src/
│   │   ├── webhook/        # OAuth callback handlers
│   │   ├── fund/          # Account funding operations
│   │   └── user/          # User management endpoints
│   └── Dockerfile
├── sui-squad-core/         # Shared library and utilities
│   ├── src/
│   │   ├── ai/            # OpenAI client and conversation management
│   │   ├── helpers/       # JWT management and utilities
│   │   └── commands/      # Bot command definitions
│   └── Cargo.toml
├── contracts/              # Sui Move smart contracts
└── docker-compose.yml      # Complete deployment configuration
```

## 🛠️ Prerequisites

- **Rust 1.85+** with Cargo
- **Docker & Docker Compose** (for containerized deployment)
- **Bacon** (optional, for development hot-reloading)
- **Node.js/Bun** (for smart contract deployment)

### Required API Keys & Accounts

1. **Telegram Bot Token** - Create a bot via [@BotFather](https://t.me/BotFather)
2. **OpenAI API Key** - For AI functionality 
3. **Google OAuth Client ID** - For zkLogin authentication
4. **Enoki API Key** - For Sui zkLogin integration
5. **Sui Network Access** - Testnet/Mainnet configuration

## ⚙️ Installation & Setup

### 1. Clone and Configure

```bash
git clone <your-repo-url>
cd sui_squad
cp env.example .env
```

### 2. Environment Configuration

Edit `.env` with your API keys and configuration:

```bash
# Essential Configuration
TELOXIDE_TOKEN=your_telegram_bot_token_here
OPENAI_API_KEY=your_openai_api_key_here
GOOGLE_CLIENT_ID=your_google_client_id_here
ENOKI_API_KEY=your_enoki_api_key_here
SECRET=your_jwt_secret_key_here

# Sui Network (testnet/mainnet/devnet)
SUI_NETWORK=testnet
SUI_SQUAD_PACKAGE_ID=your_deployed_package_id

# Server Configuration
HOST=localhost:3200
SERVER_DOMAIN=localhost:3200
```

### 3. Smart Contract Deployment

```bash
cd contracts
# Deploy the Sui Move package
sui client publish --gas-budget 100000000
# Note the Package ID for your .env file
```

## 🚀 Running the Application

### Option 1: Cargo (Local Development)

**Terminal 1 - Server:**
```bash
cargo run -p sui-squad-server
```

**Terminal 2 - Bot:**
```bash
cargo run -p sui-squad-bot
```

### Option 2: Bacon (Hot Reload Development)

Install bacon if not already installed:
```bash
cargo install bacon
```

**Terminal 1 - Server with hot reload:**
```bash
bacon run sui-squad-server
```

**Terminal 2 - Bot with hot reload:**
```bash
bacon run sui-squad-bot
```

**Terminal 3 - Tests (optional):**
```bash
bacon test
```

### Option 3: Docker Compose (Production)

**Single command deployment:**
```bash
docker-compose up -d
```

**View logs:**
```bash
docker-compose logs -f
```

**Stop services:**
```bash
docker-compose down
```

## 🤖 Bot Commands

Once running, your Telegram bot supports these commands:

- `/login` - Authenticate with the bot (generates JWT token)
- `/fund` - Fund your account via Google OAuth + zkLogin
- `/p <message>` - Chat with AI assistant (short form)
- `/prompt <message>` - Chat with AI assistant (full form)
- `/help` - Display help information

### Example Usage

```
/login
✅ Successfully logged in! You can now use commands.

/p what's my balance?
🔍 Checking your wallet balance...
💰 Your current balance: 10.5 SUI

/p send 2 SUI to @username
✅ Sent 2.0 SUI to @username
Transaction: 0x123abc...
```

## 🔧 Development

### Project Structure

- **Core Library** (`sui-squad-core`): Shared utilities, JWT management, AI integration
- **Bot Service** (`sui-squad-bot`): Telegram bot with command handlers and middleware
- **Server Service** (`sui-squad-server`): Web API for OAuth callbacks and funding

### Key Technologies

- **[Teloxide](https://github.com/teloxide/teloxide)**: Telegram bot framework
- **[Axum](https://github.com/tokio-rs/axum)**: Async web framework for server
- **[Sui SDK](https://github.com/MystenLabs/sui)**: Blockchain integration
- **[Squad Connect](https://github.com/Singularity-Shift/squad_connect)**: zkLogin utilities
- **[Sled](https://github.com/spacejam/sled)**: Embedded database for state management

### Testing

```bash
# Run all tests
cargo test

# Test specific package
cargo test -p sui-squad-core

# With bacon (hot reload)
bacon test
```

### Building

```bash
# Build all packages
cargo build --release

# Build specific service
cargo build --release -p sui-squad-bot
cargo build --release -p sui-squad-server
```

## 🐳 Docker Deployment

The project includes production-ready Dockerfiles and docker-compose configuration:

```bash
# Build and run all services
docker-compose up -d

# Scale services
docker-compose up -d --scale sui-squad-bot=2

# Update and restart
docker-compose build
docker-compose up -d
```

### Service Ports

- **Bot Service**: Internal (no exposed ports)
- **Server Service**: `3200:3200`
- **Database**: Internal Sled database with persistent volumes

## 📝 Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `TELOXIDE_TOKEN` | Telegram bot token from BotFather | ✅ |
| `OPENAI_API_KEY` | OpenAI API key for AI features | ✅ |
| `GOOGLE_CLIENT_ID` | Google OAuth client ID | ✅ |
| `ENOKI_API_KEY` | Enoki API key for zkLogin | ✅ |
| `SECRET` | JWT signing secret | ✅ |
| `SUI_NETWORK` | Sui network (testnet/mainnet/devnet) | ✅ |
| `SUI_SQUAD_PACKAGE_ID` | Deployed smart contract package ID | ✅ |
| `HOST` | Server host configuration | ✅ |
| `SEED` | Admin account mnemonic (12 words) | ✅ |

## 🤝 Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Code Style

This project uses:
- **Rust 2024 Edition**
- **Cargo fmt** for formatting
- **Cargo clippy** for linting

## 📄 License

This project is dual-licensed under:

- **[GNU General Public License v3.0 (GPL-3.0)](https://www.gnu.org/licenses/gpl-3.0.en.html)** - For open source use
- **Commercial License** - For proprietary applications

For commercial licensing, contact: james@sshiftgpt.com

Copyright (c) 2025 SSHIFT GPT 