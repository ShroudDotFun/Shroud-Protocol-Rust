# Shroud Protocol — Rust Implementation

![Solana](https://img.shields.io/badge/Solana-Program-14F195?logo=solana)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![Anchor](https://img.shields.io/badge/Anchor-0.29.0-blueviolet)
![License](https://img.shields.io/badge/License-MIT-green)

**Privacy-preserving SPL token transfers powered by zero-knowledge cryptography and on-chain Rust programs.**

> ⚠️ **Security Notice**: This program is under active development and has not yet been audited. Do not use in production with real funds until a formal security audit is completed.

## Overview

Shroud Protocol is a Solana program written in Rust that enables private SPL token transfers through advanced cryptographic techniques. The protocol obscures sender, recipient, and amount information while maintaining full on-chain settlement and non-custodial architecture.

### Key Features

- 🔒 **Zero-Knowledge Proofs** — SNARK-based verification without metadata exposure
- ⚡ **Solana-Native** — Built for sub-second finality and low fees
- 🛡️ **Non-Custodial** — Users maintain full control of their assets
- 🔐 **Cryptographic Privacy** — Pedersen commitments and homomorphic encryption
- 🧹 **Rent Reclamation** — Automatic cleanup of ephemeral accounts
- 📦 **Composable** — Designed for integration with wallets and dApps

## Architecture

### Core Components

#### 1. Privacy Engine (`src/privacy/`)
Implements the cryptographic proof systems and zero-knowledge verification logic. Handles:
- Proof generation and validation
- Commitment scheme operations
- Nullifier management
- Range proof verification

#### 2. Transfer Orchestrator (`src/transfer/`)
Coordinates multi-step privacy-preserving transfers:
- Ephemeral account creation via PDAs
- State transition validation
- Atomic settlement guarantees
- Rollback mechanisms for failed operations

#### 3. Account Manager (`src/accounts/`)
Manages account lifecycle and state:
- Program Derived Address (PDA) generation
- Rent exemption handling
- Account closure and cleanup
- Balance verification

### Transaction Flow

```
┌─────────┐     Privacy     ┌──────────┐     Cryptographic    ┌───────────┐
│ Sender  │ ────Engine────> │  Shroud  │ ────Verification──> │ Recipient │
│ Wallet  │                 │  Program │                      │  Wallet   │
└─────────┘                 └──────────┘                      └───────────┘
                                  │
                           ┌──────┴──────┐
                           │  Ephemeral  │
                           │  Accounts   │
                           └─────────────┘
```

## Technical Specifications

### Cryptographic Primitives

- **Zero-Knowledge System**: Groth16 SNARKs for efficient proof verification
- **Commitment Scheme**: Pedersen commitments on Curve25519
- **Hash Function**: Blake3 for high-performance on-chain hashing
- **Nullifiers**: Poseidon hash-based double-spend prevention

### Performance

| Metric | Value |
|--------|-------|
| Compute Units | ~180,000 CU per transfer |
| Confirmation Time | <2 seconds (typical) |
| Privacy Cost | ~0.006 SOL |
| Throughput | Limited by Solana block production |

### Security Guarantees

- **Sender Anonymity**: Transaction origin obscured through cryptographic proofs
- **Recipient Privacy**: Destination unlinkable from sender
- **Amount Confidentiality**: Transfer amounts concealed via homomorphic commitments
- **Timing Resistance**: Randomized execution to prevent correlation attacks

## Building from Source

### Prerequisites

- Rust 1.75.0 or later
- Solana CLI 1.17.0 or later
- Anchor 0.29.0 or later
- Node.js 18+ (for testing)

### Build Instructions

```bash
# Install dependencies
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet (requires audit)
anchor deploy --provider.cluster mainnet
```

## Program Structure

```
program/
├── Cargo.toml              # Rust dependencies and metadata
├── Anchor.toml             # Anchor configuration
├── src/
│   ├── lib.rs              # Program entrypoint and module exports
│   ├── state/              # Account state definitions
│   │   ├── mod.rs
│   │   ├── transfer.rs     # Transfer state
│   │   └── privacy.rs      # Privacy metadata
│   ├── instructions/       # Instruction handlers
│   │   ├── mod.rs
│   │   ├── initialize.rs   # Program initialization
│   │   ├── transfer.rs     # Privacy transfer logic
│   │   └── verify.rs       # Proof verification
│   ├── privacy/            # Cryptographic privacy engine
│   │   ├── mod.rs
│   │   ├── proofs.rs       # ZK proof generation/verification
│   │   ├── commitments.rs  # Pedersen commitment scheme
│   │   └── nullifiers.rs   # Nullifier set management
│   ├── utils/              # Utility functions
│   │   ├── mod.rs
│   │   └── validation.rs   # Input validation
│   └── errors.rs           # Custom error types
└── tests/
    ├── integration.rs      # Integration tests
    └── privacy.rs          # Privacy guarantee tests
```

## Integration

### Client-Side Integration

```typescript
import { Connection, PublicKey } from '@solana/web3.js'
import { ShroudProtocol } from '@shroud/sdk'

// Initialize the protocol
const shroud = new ShroudProtocol(connection, wallet)

// Execute private transfer
const signature = await shroud.privateTransfer({
  token: tokenMint,
  recipient: recipientAddress,
  amount: transferAmount,
})
```

### Program IDL

The program exposes the following instructions:

- `initialize` — Initialize the Shroud program state
- `private_transfer` — Execute a privacy-preserving token transfer
- `verify_proof` — Verify zero-knowledge proofs
- `close_ephemeral` — Clean up temporary accounts

## Security Considerations

### Threat Model

Shroud provides privacy against:
- ✅ Passive blockchain observers
- ✅ Active transaction analysis tools
- ✅ Timing correlation attacks
- ✅ Amount fingerprinting

### Known Limitations

- Privacy guarantees depend on transaction volume (anonymity set)
- Dust amounts may be susceptible to statistical analysis
- Initial/final on-ramp/off-ramp points may be observable

### Recommended Usage

- Use for amounts >$100 USD equivalent for better privacy
- Avoid predictable transfer patterns
- Consider combining with timing randomization
- Keep recipient addresses separate from public identity

## Audits & Security

- 🔍 **Security Audit**: Pending (Q1 2025)
- 🐛 **Bug Bounty**: Coming soon
- 📝 **Formal Verification**: In progress

### Reporting Vulnerabilities

Please report security issues to: security@shroud.fun (PGP key available on request)

Do NOT open public GitHub issues for security vulnerabilities.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- **Website**: https://shroud.fun
- **Documentation**: https://docs.shroud.fun
- **GitHub**: https://github.com/ShroudDotFun
- **X (Twitter)**: https://x.com/shrouddotfun

## Contributing

Contributions are welcome! Please read our contributing guidelines and code of conduct before submitting PRs.

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Submit a pull request

## Disclaimer

This software is provided "as is" without warranty. Users are responsible for their own security and should review the code before use. Always test on devnet before mainnet deployment.

---

**Built with privacy in mind. Powered by Rust and Solana.** 🔒⚡

