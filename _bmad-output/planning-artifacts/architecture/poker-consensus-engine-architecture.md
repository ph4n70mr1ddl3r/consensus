# Poker Consensus Engine Architecture

**Date:** 2025-12-31
**Version:** 1.0
**Status:** Approved for Implementation

---

## 1. Executive Summary

The Poker Consensus Engine is a trustless, decentralized mental poker coordination system built on Byzantine Fault Tolerant (BFT) consensus principles. The system enables fair, verifiable poker games without requiring a trusted dealer or centralized authority. All game state transitions, card dealing, and timeout coordination are handled through distributed consensus with cryptographic guarantees.

### 1.1 Project Overview

The Poker Consensus Engine addresses the fundamental challenge of conducting fair card games in a trustless environment. Traditional online poker relies on centralized servers that players must trust to deal fairly, manage timeouts, and handle disputes. This architecture eliminates that trust requirement by:

- **Decentralizing Game Coordination**: Multiple validators collectively manage game state, ensuring no single entity can manipulate outcomes
- **Verifiable Card Dealing**: Cryptographic commitments and zero-knowledge proofs verify card dealing fairness without revealing hole cards
- **Fair Timeout Coordination**: Threshold signatures ensure all validators agree on timeout events
- **Dispute Resolution**: Complete event sourcing enables deterministic reconstruction of any game state for dispute resolution

### 1.2 Technical Scope

| Component | Technology | Purpose |
|-----------|------------|---------|
| Consensus Engine | Substrate (Rust) | BABE/GRANDPA consensus with validator rotation |
| Game Logic | Custom Pallet | Poker rules, state machine, action validation |
| Cryptography | BLS12-381 + arkworks | Threshold signatures, ZK proofs |
| Networking | libp2p | Validator communication, peer discovery |
| Storage | RocksDB + IavlTree | Persistent state, Merkle proofs |

### 1.3 Key Deliverables

1. **Core Consensus Layer**: Production-ready Substrate-based BFT consensus with 4-7 validator nodes
2. **Poker Pallet**: Complete Texas Hold'em implementation with extensible plugin architecture
3. **Threshold BLS Integration**: 3-of-4 threshold scheme for fair timestamping and multi-party authorization
4. **Privacy Layer**: ZK proofs for card commitment verification and hand evaluation
5. **API Layer**: gRPC, REST, and WebSocket interfaces for client integration

---

## 2. System Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         POKER CONSENSUS ENGINE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         CLIENT LAYER                                  │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │   │
│  │  │ Web Client  │  │ Mobile SDK  │  │ CLI Wallet  │  │ Game Bot    │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│                                      ▼                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         API GATEWAY                                   │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────────────┐  │   │
│  │  │   gRPC    │  │  REST     │  │ WebSocket │  │ Authentication    │  │   │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────────────────┘  │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│                                      ▼                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                      SUBSTRATE NODE CLUSTER                           │   │
│  │                                                                       │   │
│  │  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐ │   │
│  │  │   Validator 1     │  │   Validator 2     │  │   Validator 3     │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ BABE Block  │  │  │  │ BABE Block  │  │  │  │ BABE Block  │  │ │   │
│  │  │  │ Production  │  │  │  │ Production  │  │  │  │ Production  │  │ │   │
│  │  │  └─────────────┘  │  │  └─────────────┘  │  │  └─────────────┘  │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ GRANDPA     │  │  │  │ GRANDPA     │  │  │  │ GRANDPA     │  │ │   │
│  │  │  │ Finality    │  │  │  │ Finality    │  │  │  │ Finality    │  │ │   │
│  │  │  └─────────────┘  │  │  └─────────────┘  │  │  └─────────────┘  │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ Poker Pallet│  │  │  │ Poker Pallet│  │  │  │ Poker Pallet│  │ │   │
│  │  │  │ (Game Logic)│  │  │  │ (Game Logic)│  │  │  │ (Game Logic)│  │ │   │
│  │  │  └─────────────┘  │  │  └─────────────┘  │  │  └─────────────┘  │ │   │
│  │  └───────────────────┘  └───────────────────┘  └───────────────────┘ │   │
│  │                                                                       │   │
│  │  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐ │   │
│  │  │   Validator 4     │  │   (Future)        │  │   (Future)        │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ BABE Block  │  │  │  │ BABE Block  │  │  │  │ BABE Block  │  │ │   │
│  │  │  │ Production  │  │  │  │ Production  │  │  │  │ Production  │  │ │   │
│  │  │  └─────────────┐  │  │  └─────────────┐  │  │  └─────────────┘  │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ GRANDPA     │  │  │  │ GRANDPA     │  │  │  │ GRANDPA     │  │ │   │
│  │  │  │ Finality    │  │  │  │ Finality    │  │  │  │ Finality    │  │ │   │
│  │  │  └─────────────┘  │  │  └─────────────┘  │  │  └─────────────┘  │ │   │
│  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │  │  ┌─────────────┐  │ │   │
│  │  │  │ Poker Pallet│  │  │  │ Poker Pallet│  │  │  │ Poker Pallet│  │ │   │
│  │  │  │ (Game Logic)│  │  │  │ (Game Logic)│  │  │  │ (Game Logic)│  │ │   │
│  │  │  └─────────────┘  │  │  └─────────────┘  │  │  └─────────────┘  │ │   │
│  │  └───────────────────┘  └───────────────────┘  └───────────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│                                      ▼                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                      OFF-CHAIN STORAGE                                │   │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐  │   │
│  │  │   RocksDB     │  │   Event Log   │  │   Encrypted Game Data     │  │   │
│  │  │   (State)     │  │   (Archive)   │  │   (IPFS/Sharded)          │  │   │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Component Relationships

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        COMPONENT INTERACTION DIAGRAM                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Client                                                                  Client│
│     │                                                                        │
│     │ (1) Submit Action                                                     │
│     ▼                                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                        API Gateway                                   │     │
│  │  - Authentication  - Rate Limiting  - Request Validation            │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│     │                                                                        │
│     │ (2) Forward Transaction                                              │
│     ▼                                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                    Transaction Pool                                  │     │
│  │  - Pending Transactions  - Gossip Propagation  - Validation         │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│     │                                                                        │
│     │ (3) Block Production (BABE)                                          │
│     ▼                                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                   Executive (Runtime)                                │     │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │     │
│  │  │  Poker      │  │  Timestamp  │  │     DKG     │  │   BLS      │  │     │
│  │  │  Pallet     │  │  Pallet     │  │  Pallet     │  │  Pallet    │  │     │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └────────────┘  │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│     │                                                                        │
│     │ (4) State Transition                                                 │
│     ▼                                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                    State Storage (RocksDB)                           │     │
│  │  - Current State  - Merkle Trie  - Historical Blocks                │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│     │                                                                        │
│     │ (5) Finality Gadget (GRANDPA)                                       │
│     ▼                                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │               Validator Network (libp2p)                             │     │
│  │  - Block Votes  - Finality Signals  - Gossip Topics                  │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.3 Technology Stack Summary

| Layer | Technology | Version | Purpose |
|-------|------------|---------|---------|
| **Runtime** | Substrate | 4.0+ | Blockchain runtime framework |
| **Language** | Rust | 1.75+ | Implementation language |
| **Consensus** | BABE/GRANDPA | Native | Block production and finality |
| **Networking** | libp2p | 0.51+ | Peer-to-peer communication |
| **Storage** | RocksDB | 8.x | Persistent key-value storage |
| **Merkle Tree** | IavlTree | Substrate | State commitment proofs |
| **Crypto** | arkworks | 0.4+ | Elliptic curve cryptography |
| **BLS** | kzen/threshold | 0.3+ | Threshold signature schemes |
| **ZK Proofs** | halo2 | Latest | Zero-knowledge proof generation |
| **API** | tonic | 0.10+ | gRPC implementation |
| **Serialization** | parity-scale-codec | 4.x | SCALE encoding |

---

## 3. Detailed Component Architecture

### 3.1 Consensus Layer (BABE/GRANDPA)

#### 3.1.1 Block Production (BABE)

BABE (Blind Assignment for Blockchain Extension) provides block production through a verifiable random function (VRF) based lottery system.

**Key Characteristics:**
- **Slot-Based Production**: Each slot (approx. 1 second) produces zero or one blocks
- **VRF-Based Leader Selection**: Validators prove their right to produce through VRF outputs
- **Probabilistic Finality**: Multiple validators can produce blocks in the same slot

**Configuration:**
```rust
// BABE Configuration
pub struct BabeConfig {
    /// Slot duration in milliseconds
    pub slot_duration: MillisPerSecond,
    /// Maximum block authors per slot
    pub epoch_length: EpochLength,
    /// Validator weights
    pub validator_count: ValidatorCount,
    /// Minimum 3 validators for BFT safety
    pub minimum_validator_count: u32,
}
```

#### 3.1.2 Finality Gadget (GRANDPA)

GRANDPA (GHOST-based Recursive Ancestor Deriving Prefix Agreement) provides absolute finality through a multi-round voting process.

**Key Characteristics:**
- **Absolute Finality**: Blocks cannot be reverted once finalized
- **GHOST-based Selection**: Uses Greedy Heaviest Observed Sub-Tree for fork resolution
- **Voting Rounds**: Validators vote on block hashes, not individual blocks

**Configuration:**
```rust
// GRANDPA Configuration
pub struct GrandpaConfig {
    /// Number of validators in the set
    pub validators: Vec<AuthorityId>,
    /// Voting weight per validator
    pub authority_set: AuthoritySet,
    /// Finality voting period
    pub voting_period: BlockNumber,
}
```

### 3.2 Poker Pallet Architecture

The Poker Pallet implements game logic as a Substrate FRAME pallet with the following structure:

```
┌─────────────────────────────────────────────────────────────────┐
│                      POKER PALLET LAYERS                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    GAME MANAGER                            │  │
│  │  - Game Creation    - Player Management    - Game Lifecycle│  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                 STATE MACHINE (Texas Hold'em)              │  │
│  │  - Pre-flop → Flop → Turn → River → Showdown              │  │
│  │  - Betting Rounds    - Pot Management    - Hand Evaluation │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  CRYPTOGRAPHIC SERVICES                    │  │
│  │  - Card Commitments    - ZK Proofs    - Shuffle Verification│ │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  TIMEOUT COORDINATION                      │  │
│  │  - Action Windows    - Timeout Consensus    - Auto-muck   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### 3.2.1 Game States

```rust
/// Represents the current state of a poker game
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum GameState {
    /// Game is being set up, players joining
    Lobby,
    /// Blinds posted, cards being dealt
    PreFlop,
    /// Three community cards dealt
    Flop,
    /// Fourth community card dealt
    Turn,
    /// Fifth and final community card dealt
    River,
    /// All betting complete, determining winner
    Showdown,
    /// Game completed, winnings distributed
    Completed,
    /// Game cancelled due to timeout or consensus failure
    Cancelled,
}
```

#### 3.2.2 Player Actions

```rust
/// Actions a player can take during a poker game
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum PlayerAction {
    /// Player folds, forfeiting hand
    Fold,
    /// Player calls the current bet
    Call,
    /// Player raises by specified amount
    Raise(Balance),
    /// Player goes all-in
    AllIn,
    /// Player checks (bet 0 when no bet pending)
    Check,
    /// Player requests timeout extension
    RequestTimeout,
}
```

### 3.3 Threshold Cryptography Architecture

The threshold cryptography system enables collective decision-making without single-node trust:

```
┌─────────────────────────────────────────────────────────────────┐
│              THRESHOLD CRYPTO ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              DISTRIBUTED KEY GENERATION                   │  │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌─────────┐│  │
│  │  │ Validator │  │ Validator │  │ Validator │  │ Validator││  │
│  │  │    1      │  │    2      │  │    3      │  │    4    ││  │
│  │  │   Share   │  │   Share   │  │   Share   │  │   Share ││  │
│  │  └───────────┘  └───────────┘  └───────────┘  └─────────┘│  │
│  │        │              │              │              │      │  │
│  │        └──────────────┴──────────────┴──────────────┘      │  │
│  │                         │                                    │  │
│  │                         ▼                                    │  │
│  │              ┌─────────────────────┐                         │  │
│  │              │  GROUP PUBLIC KEY   │                         │  │
│  │              │   (Published on     │                         │  │
│  │              │     Chain)          │                         │  │
│  │              └─────────────────────┘                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                    │
│                              ▼                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              THRESHOLD SIGNING (3-of-4)                   │  │
│  │                                                            │  │
│  │  Partial  ──┐                                              │  │
│  │  Signature  │     ┌─────────────────────┐   Aggregated    │  │
│  │  from V1    ─────►│                     │   Signature     │  │
│  │              │     │   SIGNATURE         │─────────────────┼──►
│  │  Partial  ───┤     │   AGGREGATOR        │   Valid Against │  │
│  │  Signature  │     │                     │   Group Key     │  │
│  │  from V2  ───┤     └─────────────────────┘                 │  │
│  │              │                                              │  │
│  │  Partial  ───┤     (Any 3 of 4 partials can               │  │
│  │  Signature  │      reconstruct valid signature)            │  │
│  │  from V3    ─────►                                          │  │
│  │              │                                              │  │
│  │  Partial  ───┘                                              │  │
│  │  Signature                                                  │  │
│  │  from V4                                                    │  │
│  │                                                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 3.4 Timestamp Pallet (HLC)

The Timestamp Pallet implements Hybrid Logical Clocks (HLC) for fair transaction ordering:

```rust
/// Hybrid Logical Clock combining physical and logical time
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct HybridLogicalClock {
    /// Physical timestamp (milliseconds since epoch)
    pub physical_time: u64,
    /// Logical clock counter
    pub logical_counter: u32,
    /// Last validator to update
    pub source_validator: ValidatorId,
}

/// HLC Update Rules:
/// 1. On receiving timestamp (p, l, v):
///    - new_physical = max(current_physical, p)
///    - if new_physical == p:
///      new_logical = max(current_logical, l) + 1
///    - else:
///      new_logical = 0
/// 2. On local event:
///    - new_physical = max(current_physical, local_clock)
///    - new_logical = current_logical + 1
```

---

## 4. Network Architecture

### 4.1 Validator Network Topology

The validator network uses a full-mesh topology for the 4-7 validator initial deployment:

```
┌─────────────────────────────────────────────────────────────────┐
│              VALIDATOR NETWORK TOPOLOGY                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│      ┌─────────────┐                                           │
│      │ Validator 1 │◄──────────────────────────────────────┐   │
│      │   (US-East) │                                       │   │
│      └──────┬──────┘                                       │   │
│             │                                              │   │
│             │ Direct Connection                            │   │
│             │ (TCP/TLS)                                    │   │
│             │                                              │   │
│      ┌──────┴──────┐     Direct Connection      ┌──────────┴───┐
│      │ Validator 2 │◄───────────────────────────►│ Validator 3  │
│      │  (EU-West)  │                             │   (Asia-East)│
│      └──────┬──────┘                             └──────┬──────┘
│             │                                            │
│             │ Direct Connection                          │
│             │ (TCP/TLS)                                  │
│             │                                            │
│      ┌──────┴──────┐                             ┌────────▼──────┐
│      │ Validator 4 │◄────────────────────────────│ Validator 5   │
│      │  (US-West)  │                             │  (Optional)   │
│      └─────────────┘                             └───────────────┘
│                                                                 │
│  Full mesh: n(n-1)/2 connections                               │
│  With 4 validators: 6 direct connections                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 libp2p Configuration

**Gossip Topics:**

| Topic | Purpose | Message Type | Frequency |
|-------|---------|--------------|-----------|
| `/poker/consensus/1` | Block proposals | Block | Per block |
| `/poker/votes/1` | Finality votes | Vote | Per block |
| `/poker/games/1` | Game state updates | GameEvent | Per action |
| `/poker/txs/1` | Transaction propagation | SignedTx | Per transaction |
| `/poker/timestamp/1` | Timestamp consensus | TimestampVote | Per update |

### 4.3 Connection Requirements

```rust
// Network Configuration
pub struct NetworkConfig {
    /// Maximum number of peers
    pub max_peers: u32,
    /// Minimum number of peers for healthy network
    pub min_peers: u32,
    /// Inbound/outbound connection limits
    pub connection_limits: ConnectionLimits,
    /// Gossip configuration
    pub gossip_config: GossipConfig,
    /// Discovery configuration
    pub discovery_config: DiscoveryConfig,
}

/// Connection Specifications
const MIN_VALIDATOR_CONNECTIONS: usize = 3;  // n-1 for n=4
const MAX_INBOUND_CONNECTIONS: u32 = 100;
const MAX_OUTBOUND_CONNECTIONS: u32 = 100;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
```

---

## 5. Storage Architecture

### 5.1 On-Chain Storage Schema

```
┌─────────────────────────────────────────────────────────────────┐
│                 ON-CHAIN STORAGE SCHEMA                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Storage Map: Games                                              │
│  ├─ Key: GameId (Hash)                                          │
│  └─ Value: GameData                                             │
│      ├─ game_state: GameState                                   │
│      ├─ players: Vec<Player>                                    │
│      ├─ community_cards: Vec<CardCommitment>                    │
│      ├─ pot: Balance                                            │
│      ├─ current_bet: Balance                                    │
│      ├─ blind_levels: Vec<BlindLevel>                           │
│      ├─ created_block: BlockNumber                              │
│      └─ updated_block: BlockNumber                              │
│                                                                 │
│  Storage Map: PlayerActions                                      │
│  ├─ Key: (GameId, PlayerId, ActionIndex)                       │
│  └─ Value: PlayerAction                                         │
│      ├─ action_type: PlayerAction                               │
│      ├─ amount: Option<Balance>                                 │
│      ├─ timestamp: HybridLogicalClock                           │
│      ├─ signature: BlsSignature                                  │
│      └─ zk_proof: Option<ZKProof>                               │
│                                                                 │
│  Storage Map: ValidatorSet                                       │
│  ├─ Key: ValidatorId                                            │
│  └─ Value: ValidatorMetadata                                    │
│      ├─ public_key: BlsPublicKey                                │
│      ├─ stake: Balance                                          │
│      ├─ last_active_block: BlockNumber                          │
│      └─ performance_score: u8                                   │
│                                                                 │
│  Storage Value: TimestampState                                   │
│  ├─ current_hlc: HybridLogicalClock                             │
│  ├─ validator_signatures: Vec<BlsSignature>                     │
│  └─ threshold_reached: bool                                     │
│                                                                 │
│  Storage Value: DKGState                                         │
│  ├─ group_public_key: BlsPublicKey                              │
│  ├─ key_generation_epoch: u32                                   │
│  └─ resharing_in_progress: bool                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Off-Chain Storage Design

**Indexed by:**

| Storage Type | Technology | Purpose | Retention |
|--------------|------------|---------|-----------|
| Event Log | RocksDB | Event sourcing | Indefinite |
| Game Archives | RocksDB | Completed games | 7 years |
| Merkle Proofs | IavlTree | Light client verification | Current era |
| Compressed History | Parquet/S3 | Analytics | Indefinite |

---

## 6. Security Architecture

### 6.1 Slashing Conditions

```rust
/// Slashing conditions for validator misbehavior
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SlashingCondition {
    /// Validator signed two conflicting blocks
    DoubleSign {
        /// First block hash
        block1: Hash,
        /// Second block hash
        block2: Hash,
        /// Evidence block height
        evidence_height: BlockNumber,
    },
    /// Validator was unavailable for extended period
    Unavailability {
        /// Number of missed heartbeats
        missed_heartbeats: u32,
        /// Offline duration
        offline_blocks: BlockNumber,
    },
    /// Validator submitted invalid timestamp
    InvalidTimestamp {
        /// Deviation from consensus (ms)
        deviation_ms: i64,
        /// Threshold exceeded
        threshold_ms: u64,
    },
    /// Validator participated in game manipulation
    GameManipulation {
        /// Game ID involved
        game_id: GameId,
        /// Evidence type
        evidence_type: ManipulationEvidence,
    },
}

/// Slash amounts (percentage of stake)
const SLASH_DOUBLE_SIGN: Perbill = Perbill::from_percent(100);
const SLASH_UNAVAILABILITY: Perbill = Perbill::from_percent(10);
const SLASH_INVALID_TIMESTAMP: Perbill = Perbill::from_percent(5);
const SLASH_GAME_MANIPULATION: Perbill = Perbill::from_percent(100);
```

### 6.2 HSM Integration

```
┌─────────────────────────────────────────────────────────────────┐
│                    HSM INTEGRATION ARCHITECTURE                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    VALIDATOR NODE                         │  │
│  │                                                           │  │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │  │
│  │  │   Software  │    │   Software  │    │   Software  │   │  │
│  │  │   Key       │    │   Key       │    │   Key       │   │  │
│  │  │   (Temp)    │    │   (Temp)    │    │   (Temp)    │   │  │
│  │  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘   │  │
│  │         │                  │                  │          │  │
│  │         └────────┬─────────┴────────┬─────────┘          │  │
│  │                  │                  │                     │  │
│  └──────────────────┼──────────────────┼─────────────────────┘  │
│                     │                  │                         │
│                     ▼                  ▼                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              HSM INTERFACE LAYER                          │  │
│  │  - Key derivation requests  - Signing operations         │  │
│  │  - Key ceremony protocols  - Certificate management      │  │
│  └───────────────────────────────────────────────────────────┘  │
│                     │                  │                         │
│                     ▼                  ▼                         │
│  ┌──────────────────┴──────────────────┴─────────────────────┐  │
│  │                    HSM DEVICES                            │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │  │
│  │  │  Validator  │  │  Validator  │  │  Validator  │        │  │
│  │  │     HSM     │  │     HSM     │  │     HSM     │        │  │
│  │  │  (AWS KMS/  │  │  (AWS KMS/  │  │  (AWS KMS/  │        │  │
│  │  │  CloudHSM)  │  │  CloudHSM)  │  │  CloudHSM)  │        │  │
│  │  │  Ed25519    │  │  Ed25519    │  │  Ed25519    │        │  │
│  │  │  Signing    │  │  Signing    │  │  Signing    │        │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Keys stored in HSM:                                            │
│  - Validator consensus key (Ed25519)                            │
│  - BLS key shares for threshold signatures                      │
│  - TLS certificates for node-to-node encryption                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. Performance Specifications

| Metric | Target | Threshold | Measurement |
|--------|--------|-----------|-------------|
| Block Time | 1-2 seconds | < 3 seconds | BABE slot |
| Finality | < 5 seconds | < 10 seconds | GRANDPA round |
| Throughput | 1,000 TPS | 500 TPS | Peak load |
| Action Latency | < 500ms | < 1 second | End-to-end |
| Game Creation | < 2 seconds | < 5 seconds | Block inclusion |
| Timeout Consensus | < 1 second | < 2 seconds | HLC update |

---

## 8. Deployment Architecture

### 8.1 Validator Node Configuration

```yaml
# Validator Node Specifications
validator:
  resources:
    cpu: "4 cores"
    memory: "16 GiB"
    storage: "500 GiB SSD"
  network:
    bandwidth: "1 Gbps"
    latency: "< 100ms"  # inter-validator
  deployment:
    regions:
      - us-east-1
      - eu-west-1
      - ap-southeast-1
    replicas: 4
```

### 8.2 Monitoring Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                    MONITORING ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   METRICS COLLECTION                      │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────────────┐ │  │
│  │  │  Prometheus │  │  Custom     │  │  Application       │ │  │
│  │  │  (System)   │  │  (Game)     │  │  Metrics           │ │  │
│  │  └─────────────┘  └─────────────┘  └────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   AGGREGATION & STORAGE                   │  │
│  │  ┌─────────────────────────────────────────────────────┐  │  │
│  │  │                    Thanos/Cortex                     │  │  │
│  │  └─────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   VISUALIZATION & ALERTING                │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────────────┐ │  │
│  │  │   Grafana   │  │ Alertmanager │  │  PagerDuty        │ │  │
│  │  │  (Dashboards)│  │ (Alerts)    │  │  (On-call)        │ │  │
│  │  └─────────────┘  └─────────────┘  └────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 Key Metrics

| Category | Metric | Alert Threshold |
|----------|--------|-----------------|
| Consensus | Validator Participation Rate | < 95% |
| Consensus | Block Production Rate | < 95% |
| Network | Peer Count | < 3 |
| Network | Inbound/Outbound Errors | > 1% |
| Storage | Database Size | > 80% capacity |
| Application | Game Action Latency | > 1s p99 |
| Security | Slashing Events | Any |

---

## 9. Implementation Roadmap

| Phase | Duration | Deliverables | Dependencies |
|-------|----------|--------------|--------------|
| **Phase 1** | Weeks 1-8 | Substrate node, validator setup, basic monitoring | None |
| **Phase 2** | Weeks 9-16 | Poker pallet MVP, event sourcing, threshold BLS DKG | Phase 1 |
| **Phase 3** | Weeks 17-24 | ZK proof integration, API layer, SDKs | Phase 2 |
| **Phase 4** | Weeks 25-32 | Security audit, load testing, production hardening | Phase 3 |
| **Phase 5** | Weeks 33-40 | Staging deployment, user testing, documentation | Phase 4 |
| **Phase 6** | Weeks 41-48 | Production launch, community growth | Phase 5 |

---

## 10. Appendices

### 10.1 Glossary

| Term | Definition |
|------|------------|
| BFT | Byzantine Fault Tolerance - ability to tolerate arbitrary/malicious node behavior |
| BABE | Blind Assignment for Blockchain Extension - block production mechanism |
| GRANDPA | GHOST-based Recursive Ancestor Deriving Prefix Agreement - finality gadget |
| BLS | Boneh-Lynn-Shacham - threshold signature scheme |
| DKG | Distributed Key Generation - protocol for creating threshold keys |
| HLC | Hybrid Logical Clock - combining physical and logical time |
| VRF | Verifiable Random Function - cryptographic proof of random selection |
| SCALE | Simple Concatenated Aggregate Little-Endian - Substrate encoding |

### 10.2 Reference Documents

1. Substrate Documentation: https://docs.substrate.io
2. Polkadot Wiki: https://wiki.polkadot.network
3. libp2p Documentation: https://docs.libp2p.io
4. arkworks Cryptography: https://arkworks.rs
5. BLS Threshold Library: https://github.com/KZen-networks/threshold

### 10.3 Version History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-12-31 | Riddler | Initial architecture document |

---

**Document Version:** 1.0
**Status:** Approved
**Next Review:** Upon implementation completion
