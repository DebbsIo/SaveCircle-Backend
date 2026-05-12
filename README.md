# SaveCircle-Backend

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Stellar](https://img.shields.io/badge/Stellar-Network-blue)](https://www.stellar.org/)

*Save Together. Trust the Chain.*

SaveCircle-Backend is the Rust-based backend API for SaveCircle, an open-source platform that digitizes informal savings circles (Ajo, Chama, Tanda) on the Stellar network. It eliminates fraud, automates payouts, and builds verifiable on-chain credit histories.

## Features

- **Group Management**: Create and manage savings circles with role-based access
- **Smart Contract Integration**: Seamless integration with Stellar Soroban contracts for transparent fund management
- **On-Chain Credit Scoring**: Build and track credit scores based on savings behavior
- **Microloan Module**: Enable microloans backed by savings history
- **Notifications**: SMS and push notifications for contributions and payouts
- **Multi-Signature Accounts**: Secure fund management with Stellar multi-sig accounts

## Technology Stack

- **Language**: Rust
- **Framework**: Actix-web
- **Database**: PostgreSQL with Diesel ORM
- **Blockchain**: Stellar Network (Soroban smart contracts)
- **Authentication**: Stellar Wallet + Phone OTP
- **Notifications**: Africa's Talking API (SMS) + Firebase (Push)
- **Deployment**: Docker

## Prerequisites

- Rust 1.70 or higher
- PostgreSQL
- Stellar account and Soroban CLI (for development)

## Installation

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/DebbsIo/SaveCircle-Backend.git
   cd SaveCircle-Backend
   ```

2. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

4. Set up the database:
   - Install PostgreSQL locally or use Docker
   - Create a database named `savecircle`

5. Configure environment variables:
   - Copy `.env.example` to `.env`
   - Fill in your configuration

6. Run the server:
   ```bash
   cargo run
   ```

### Using Docker

1. Clone the repository and navigate to it

2. Use Docker Compose for development:
   ```bash
   docker-compose up --build
   ```

3. For production build:
   ```bash
   docker build -t savecircle-backend .
   docker run -p 8080:8080 savecircle-backend
   ```

## API Documentation

API documentation will be available at `/docs` once the server is running.

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](.github/CONTRIBUTING.md) for details.

## Code of Conduct

Please read our [Code of Conduct](.github/CODE_OF_CONDUCT.md) before contributing.

## Security

If you discover a security vulnerability, please report it to security@savecircle.org.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- Project Website: [savecircle.org](https://savecircle.org)
- GitHub Issues: [Issues](https://github.com/DebbsIo/SaveCircle-Backend/issues)
- Discord: [Join our community](https://discord.gg/savecircle)