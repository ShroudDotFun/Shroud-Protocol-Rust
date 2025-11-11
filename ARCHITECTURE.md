# Shroud Protocol Architecture

## System Overview

Shroud Protocol is a privacy-preserving layer for SPL token transfers on Solana. The system combines zero-knowledge cryptography with on-chain program execution to provide transaction privacy without trusted third parties.

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Client Layer                             │
│  (Web UI, SDK, Wallet Integration)                          │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  Shroud Rust Program                         │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐  │
│  │   Privacy    │  │   Transfer   │  │    Account      │  │
│  │   Engine     │  │ Orchestrator │  │    Manager      │  │
│  └──────────────┘  └──────────────┘  └─────────────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Solana Runtime (BPF)                            │
│  • Account Model  • Transaction Processing  • Consensus     │
└─────────────────────────────────────────────────────────────┘
```

## Privacy Engine

### Zero-Knowledge Proof System

The privacy engine implements SNARK-based proofs for:

1. **Proof of Ownership**: Sender owns the tokens without revealing identity
2. **Proof of Amount**: Transfer amount is valid without revealing value
3. **Proof of Nullifier**: Prevents double-spending without exposing spent notes

### Commitment Scheme

Uses Pedersen commitments on Curve25519:
```
C = amount * G + blinding * H
```

Where:
- `G`, `H` are generator points
- `amount` is the hidden transfer value
- `blinding` is a random secret

### Nullifier System

Prevents double-spending through unique nullifiers:
```
nullifier = Hash(secret || commitment)
```

Nullifiers are stored in a Merkle tree for efficient verification.

## Transfer Orchestrator

Coordinates the multi-step privacy transfer process:

### Phase 1: Proof Generation
- Client generates ZK proof off-chain
- Creates commitment to transfer amount
- Derives nullifier from secret

### Phase 2: Validation
- Program verifies ZK proof on-chain
- Checks nullifier hasn't been spent
- Validates commitment structure

### Phase 3: Execution
- Creates ephemeral accounts (PDAs)
- Executes token transfer through privacy layer
- Updates nullifier set

### Phase 4: Cleanup
- Closes ephemeral accounts
- Reclaims rent to sender
- Finalizes transfer state

## Account Manager

### Program Derived Addresses (PDAs)

Shroud uses PDAs for deterministic account generation:

```rust
// Transfer state PDA
["transfer", sender_pubkey, nullifier] → TransferState

// Privacy state PDA
["privacy"] → PrivacyState

// Ephemeral account PDA
["ephemeral", transfer_id] → TokenAccount
```

### Account Lifecycle

1. **Creation**: Accounts created via PDAs with proper rent exemption
2. **Usage**: Active during transfer execution
3. **Closure**: Automatic cleanup with rent reclamation

## Data Structures

### TransferState

```rust
pub struct TransferState {
    pub bump: u8,
    pub amount_commitment: [u8; 32],
    pub nullifier: [u8; 32],
    pub timestamp: i64,
    pub status: TransferStatus,
}
```

### PrivacyState

```rust
pub struct PrivacyState {
    pub authority: Pubkey,
    pub merkle_root: [u8; 32],
    pub nullifier_root: [u8; 32],
    pub transfer_count: u64,
    pub last_update: i64,
}
```

## Cryptographic Primitives

### Hash Functions

- **Blake3**: Primary hash function for commitments and nullifiers
- **SHA256**: Merkle tree construction
- **Poseidon**: Zero-knowledge friendly hash (planned)

### Elliptic Curves

- **Curve25519**: Pedersen commitment base curve
- **BN254**: Groth16 proof system curve (planned)

### Random Number Generation

- Verifiable Random Function (VRF) for deterministic randomness
- Solana's slot hashes for entropy source

## Security Model

### Assumptions

1. **Honest Majority**: Majority of Solana validators are honest
2. **Cryptographic Hardness**: Discrete log problem is hard on Curve25519
3. **Hash Security**: Blake3 is collision-resistant

### Threat Protection

| Threat | Protection |
|--------|------------|
| Sender Deanonymization | ZK proofs + ephemeral accounts |
| Recipient Tracking | Commitment-based addressing |
| Amount Analysis | Homomorphic commitments |
| Timing Attacks | Randomized execution patterns |
| Double-Spending | Nullifier set verification |

### Known Limitations

- **Anonymity Set**: Privacy improves with more users
- **Chain Analysis**: Advanced ML techniques may correlate patterns
- **Side Channels**: Network-level observers may see timing

## Performance Optimization

### Compute Unit Usage

- Transfer instruction: ~180,000 CU
- Proof verification: ~80,000 CU
- Account management: ~20,000 CU

### Optimization Techniques

1. **Lazy Evaluation**: Defer expensive operations
2. **Batch Processing**: Group operations when possible
3. **Account Caching**: Minimize account deserialization
4. **Proof Compression**: Optimize proof size

## Future Enhancements

### Roadmap

- [ ] **Multi-hop Privacy**: Additional anonymity layers
- [ ] **Confidential Amounts**: Fully hidden transfer values
- [ ] **Stealth Addresses**: One-time recipient addresses
- [ ] **Decoy Transactions**: Additional privacy through noise
- [ ] **Cross-chain Privacy**: Bridge to other networks

### Research Areas

- zk-STARK integration for quantum resistance
- Recursive proof composition
- Privacy-preserving smart contract execution
- Confidential token standards

## Testing Strategy

### Unit Tests
- Individual function correctness
- Edge case handling
- Error conditions

### Integration Tests
- End-to-end transfer flows
- Multi-user scenarios
- State consistency checks

### Property Tests
- Cryptographic invariants
- Privacy guarantees
- Economic security

### Fuzzing
- Input validation robustness
- Proof parsing security
- Account state transitions

## Deployment

### Devnet Deployment

```bash
anchor build
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment

⚠️ **Requires**:
- Completed security audit
- Bug bounty program
- Community review period
- Multi-sig governance

## Monitoring & Maintenance

### Metrics to Track

- Transfer success rate
- Average confirmation time
- Compute unit usage
- Privacy guarantee violations (if any)

### Upgrade Process

1. Propose upgrade via governance
2. Security review of changes
3. Testnet deployment and testing
4. Mainnet upgrade via authority

---

**Version**: 0.1.0 (Pre-Audit)  
**Last Updated**: November 11, 2025  
**Status**: Development

