# CertaBlock Architecture

This document provides an in-depth technical overview of CertaBlock's architecture, design decisions, and implementation details.

---

## Table of Contents

- [System Overview](#system-overview)
- [Layer Architecture](#layer-architecture)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Security Model](#security-model)
- [Performance Considerations](#performance-considerations)

---

## System Overview

CertaBlock is built on a **layered architecture** that separates concerns while maintaining tight integration between components. The system consists of six primary layers, each with specific responsibilities.

### Design Principles

1. **Separation of Concerns**: Each layer has a well-defined responsibility
2. **Modularity**: Components can be developed and tested independently
3. **Scalability**: Architecture supports future enhancements
4. **Security**: Multiple layers of validation and verification
5. **Developer Experience**: Clean APIs and comprehensive documentation

---

## Layer Architecture

### 1. Client Layer

**Components:**

- React Dashboard (Vite + Tailwind CSS)
- MetaMask Wallet Integration

**Responsibilities:**

- User interface and interaction
- Wallet connection and signature management
- Real-time blockchain state visualization
- Transaction submission and monitoring

**Technology Stack:**

- **Framework**: React 18 with Vite
- **Styling**: Tailwind CSS with custom glassmorphism design
- **State Management**: React hooks (useState, useEffect, useCallback)
- **Icons**: Lucide React
- **Charts**: Recharts
- **UI Components**: Radix UI primitives

### 2. API Layer (Axum)

**Components:**

- Mining Endpoints
- Certificate Endpoints
- Token Management Endpoints
- Authentication Endpoints

**Responsibilities:**

- HTTP request handling
- Request validation and sanitization
- Response formatting
- CORS configuration
- JWT authentication middleware

**Technology Stack:**

- **Framework**: Axum (async web framework)
- **Runtime**: Tokio (async runtime)
- **Serialization**: Serde (JSON)
- **CORS**: tower-http

**Key Endpoints:**

```rust
POST   /auth/nonce          // Get authentication nonce
POST   /auth/verify         // Verify wallet signature
POST   /mine                // Mine a new block
GET    /blocks              // Get all blocks
POST   /certificates/issue  // Issue certificate
GET    /certificates/verify/:id  // Verify certificate
POST   /tokens/transfer     // Transfer tokens
GET    /tokens/balance/:address  // Get token balance
GET    /stats               // Get blockchain statistics
```

### 3. Application Layer

**Components:**

- Smart Contract Execution Engine
- Mining Logic
- Certificate Manager
- Token System (MCT)

**Responsibilities:**

- Business logic implementation
- Smart contract execution
- Certificate issuance and verification
- Token transfer and balance management
- Transaction creation and validation

**Smart Contract System:**

```rust
pub struct ContractExecutor {
    // Contract state and execution environment
}

impl ContractExecutor {
    pub fn execute(&self, contract: &Contract) -> Result<ContractResult> {
        // Deterministic contract execution
    }
}
```

### 4. Consensus Layer

**Component:**

- Proof-of-Work (PoW) Algorithm

**Responsibilities:**

- Block mining with adjustable difficulty (1-6)
- Nonce discovery through SHA-256 hashing
- Difficulty adjustment based on mining time
- Block validation and acceptance

**Mining Algorithm:**

```rust
pub fn mine_block(block: &mut Block, difficulty: u32) -> Result<()> {
    let target = "0".repeat(difficulty as usize);

    loop {
        block.nonce += 1;
        block.hash = calculate_hash(block);

        if block.hash.starts_with(&target) {
            return Ok(());
        }
    }
}
```

**Difficulty Levels:**

- **Level 1**: 1 leading zero (~instant)
- **Level 2**: 2 leading zeros (~seconds)
- **Level 3**: 3 leading zeros (~10-30 seconds)
- **Level 4**: 4 leading zeros (~1-5 minutes)
- **Level 5**: 5 leading zeros (~10-30 minutes)
- **Level 6**: 6 leading zeros (~hours)

### 5. Blockchain Core

**Components:**

- Block Structure
- Chain Validation
- Transaction Management

**Responsibilities:**

- Block creation and linking
- Chain integrity verification
- Transaction validation
- Merkle tree construction (future)

**Block Structure:**

```rust
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}
```

**Chain Validation:**

- Genesis block verification
- Hash chain integrity
- Timestamp ordering
- Transaction validity
- Nonce verification

### 6. Storage Layer

**Component:**

- In-Memory Data Structures

**Responsibilities:**

- Blockchain state storage
- Pending transaction queue
- Token balance tracking
- Certificate registry
- Session management

**Data Structures:**

```rust
pub struct BlockchainState {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub token_balances: HashMap<String, u64>,
    pub certificates: HashMap<String, Certificate>,
}
```

---

## Core Components

### Authentication System

**Flow:**

1. Client requests nonce from backend
2. Backend generates unique nonce for address
3. Client signs nonce with MetaMask
4. Backend verifies signature using secp256k1
5. Backend issues JWT token for authenticated session

**Security:**

- Nonce prevents replay attacks
- Signature verification ensures wallet ownership
- JWT tokens have expiration
- Session invalidation on logout

### Certificate Management

**Issuance:**

```rust
pub struct Certificate {
    pub id: String,           // Unique identifier
    pub recipient: String,    // Recipient address
    pub issuer: String,       // Issuer address
    pub data: String,         // Certificate data
    pub timestamp: DateTime<Utc>,
    pub block_index: u64,     // Block where recorded
}
```

**Verification:**

- Check certificate exists in registry
- Verify block inclusion
- Validate issuer signature
- Check timestamp validity

### Token System (MCT)

**Features:**

- Fungible token implementation
- Transfer between addresses
- Balance tracking
- Transaction history

**Transfer Logic:**

```rust
pub fn transfer_tokens(
    from: &str,
    to: &str,
    amount: u64,
    balances: &mut HashMap<String, u64>
) -> Result<()> {
    // Validate sender has sufficient balance
    // Deduct from sender
    // Add to recipient
    // Record transaction
}
```

---

## Data Flow

### Mining Flow

```
User → Dashboard → POST /mine → Mining Logic → PoW Consensus
  ↓                                                    ↓
  ← Dashboard ← Response ← Block Added ← Chain Updated
```

### Certificate Issuance Flow

```
User → MetaMask Sign → POST /certificates/issue → Certificate Manager
  ↓                                                        ↓
  ← Dashboard ← Response ← Transaction Created ← Certificate Stored
```

### Token Transfer Flow

```
User → MetaMask Sign → POST /tokens/transfer → Token System
  ↓                                                   ↓
  ← Dashboard ← Response ← Balance Updated ← Transaction Recorded
```

---

## Security Model

### Authentication

- **Wallet-based**: No passwords, only cryptographic signatures
- **Nonce-based**: Prevents replay attacks
- **JWT tokens**: Stateless session management
- **Signature verification**: secp256k1 curve

### Authorization

- **Role-based**: Different permissions for different operations
- **Transaction signing**: All state changes require signatures
- **Address validation**: Ethereum address format validation

### Data Integrity

- **Hash chains**: Each block links to previous via hash
- **Merkle trees**: Transaction integrity (future enhancement)
- **Immutability**: Once mined, blocks cannot be altered
- **Consensus**: PoW ensures computational cost for changes

---

## Performance Considerations

### Backend Optimization

- **Async I/O**: Tokio runtime for concurrent request handling
- **In-memory storage**: Fast read/write operations
- **Efficient hashing**: SHA-256 hardware acceleration
- **Connection pooling**: Reuse HTTP connections

### Frontend Optimization

- **Code splitting**: Vite dynamic imports
- **Lazy loading**: Components loaded on demand
- **Memoization**: React.memo and useMemo
- **Virtual scrolling**: For large block lists

### Scalability

**Current Limitations:**

- In-memory storage (limited by RAM)
- Single-node architecture
- No persistence across restarts

**Future Enhancements:**

- Persistent storage (RocksDB, PostgreSQL)
- Multi-node consensus
- Sharding for horizontal scaling
- State pruning for reduced storage

---

## Technology Choices

### Why Rust?

- **Memory safety**: No null pointers, no data races
- **Performance**: Zero-cost abstractions, compiled to native code
- **Concurrency**: Fearless concurrency with ownership model
- **Ecosystem**: Rich blockchain and crypto libraries

### Why Axum?

- **Modern**: Built on Tokio, async/await
- **Type-safe**: Compile-time route validation
- **Ergonomic**: Clean API design
- **Performance**: One of the fastest Rust web frameworks

### Why React?

- **Component-based**: Reusable UI components
- **Ecosystem**: Rich library ecosystem
- **Developer experience**: Hot module replacement, dev tools
- **Community**: Large community and resources

---

## Future Architecture Enhancements

1. **Persistent Storage**: Add database layer for blockchain persistence
2. **P2P Networking**: Implement peer-to-peer node communication
3. **Consensus Upgrades**: Add Proof-of-Stake option
4. **Smart Contract VM**: Full Turing-complete contract execution
5. **Sharding**: Horizontal scaling through chain sharding
6. **State Channels**: Off-chain transaction processing
7. **WebSocket Support**: Real-time blockchain updates
8. **GraphQL API**: Alternative to REST for complex queries

---

For more details, see:

- [API Documentation](./API.md)
- [Technical Whitepaper](./WHITEPAPER.md)
- [Setup Guide](./SETUP.md)
