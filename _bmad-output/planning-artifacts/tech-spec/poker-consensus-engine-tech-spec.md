# Poker Consensus Engine Technical Specification

**Document:** POKER-CONSENSUS-ENGINE-TECH-SPEC
**Version:** 1.0
**Date:** 2025-12-31
**Status:** Approved for Implementation
**Classification:** Technical - Public

---

## Document Control

| Version | Date | Author | Reviewer | Changes |
|---------|------|--------|----------|---------|
| 1.0 | 2025-12-31 | Riddler | Architecture Review Board | Initial release |

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Architecture Overview](#2-system-architecture-overview)
3. [Detailed Technical Specifications](#3-detailed-technical-specifications)
4. [Interface Specifications](#4-interface-specifications)
5. [Security Specifications](#5-security-specifications)
6. [Performance Specifications](#6-performance-specifications)
7. [Deployment Specifications](#7-deployment-specifications)
8. [Testing Specifications](#8-testing-specifications)
9. [Implementation Milestones](#9-implementation-milestones)
10. [Appendices](#10-appendices)

---

## 1. Executive Summary

### 1.1 Project Overview

The Poker Consensus Engine is a decentralized, trustless mental poker coordination system that eliminates the need for trusted intermediaries in online poker games. By leveraging Byzantine Fault Tolerant (BFT) consensus, threshold cryptography, and zero-knowledge proofs, the system ensures fair gameplay, verifiable card dealing, and dispute resolution without relying on any single point of trust or failure.

The system addresses fundamental challenges in online poker:

- **Trust Elimination**: No centralized server controls game outcomes; multiple validators collectively manage game state
- **Verifiable Fairness**: Cryptographic commitments and ZK proofs prove card dealing integrity without revealing hidden cards
- **Fair Timeouts**: Threshold signatures ensure all validators agree on timeout events, preventing single-node manipulation
- **Dispute Resolution**: Event sourcing provides complete, deterministic game reconstruction for dispute resolution

### 1.2 Technical Scope

| Scope Dimension | Description |
|-----------------|-------------|
| **Consensus Model** | BABE/GRANDPA hybrid (Substrate/Polkadot) |
| **Validator Count** | 4-7 nodes (configurable, 4 for initial deployment) |
| **BFT Threshold** | f < n/3 (tolerates 1 faulty node with 4 validators) |
| **Threshold Scheme** | 3-of-4 BLS threshold signatures |
| **Cryptography** | BLS12-381 elliptic curves, Groth16/PLONK ZK proofs |
| **Network Protocol** | libp2p with custom gossip topics |
| **Block Time** | 1-2 seconds |
| **Finality** | < 5 seconds |
| **Throughput** | 1,000+ transactions per second |

### 1.3 Key Deliverables

| Deliverable | Description | Target Date |
|-------------|-------------|-------------|
| Core Consensus Layer | Production-ready Substrate node with BABE/GRANDPA | Week 8 |
| Poker Pallet | Texas Hold'em game logic with state machine | Week 16 |
| Threshold BLS System | 3-of-4 threshold signatures for fair timestamping | Week 20 |
| Privacy Layer | ZK proofs for card commitment and hand verification | Week 24 |
| API Layer | gRPC, REST, WebSocket interfaces | Week 24 |
| SDK Packages | Client libraries (JavaScript, Rust, Mobile) | Week 28 |
| Security Audit | Third-party security assessment | Week 28 |
| Production Deployment | Live network with 4 validators | Week 40 |

---

## 2. System Architecture Overview

### 2.1 High-Level Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                              POKER CONSENSUS ENGINE                                          │
├─────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐     │
│  │                                    CLIENT LAYER                                      │     │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────────┐ │     │
│  │  │   Web App   │  │ Mobile iOS  │  │ Mobile Android│ │ CLI / Game Bot Integration │ │     │
│  │  │  (React)    │  │    (Swift)  │  │   (Kotlin)   │ │      (Rust/Go SDK)         │ │     │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────────────────────┘ │     │
│  └────────────────────────────────────────────────────────────────────────────────────┘     │
│                                        │                                                     │
│                                        ▼                                                     │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐     │
│  │                               API GATEWAY LAYER                                    │     │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐│     │
│  │  │  gRPC Endpoints    │   REST API    │   WebSocket Subscriptions   │   Auth/JWT  ││     │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘│     │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐│     │
│  │  │  Rate Limiting   │   Request Validation   │   Load Balancing   │   Caching    ││     │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘│     │
│  └────────────────────────────────────────────────────────────────────────────────────┘     │
│                                        │                                                     │
│                                        ▼                                                     │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐     │
│  │                          SUBSTRATE BLOCKCHAIN LAYER                                │     │
│  │                                                                                      │     │
│  │  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────────────┐ │     │
│  │  │    VALIDATOR 1      │  │    VALIDATOR 2      │  │       VALIDATOR 3           │ │     │
│  │  │  Region: US-East    │  │  Region: EU-West    │  │      Region: Asia-East      │ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   BABE Block    ││  │  │   BABE Block    ││  │  │   BABE Block            ││ │     │
│  │  │  │   Production    ││  │  │   Production    ││  │  │   Production            ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   GRANDPA       ││  │  │   GRANDPA       ││  │  │   GRANDPA               ││ │     │
│  │  │  │   Finality      ││  │  │   Finality      ││  │  │   Finality              ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   POKER PALLET  ││  │  │   POKER PALLET  ││  │  │   POKER PALLET          ││ │     │
│  │  │  │   (Game Logic)  ││  │  │   (Game Logic)  ││  │  │   (Game Logic)          ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   BLS PALLET    ││  │  │   BLS PALLET    ││  │  │   BLS PALLET            ││ │     │
│  │  │  │   (Threshold)   ││  │  │   (Threshold)   ││  │  │   (Threshold)           ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   HSM (AWS KMS) ││  │  │   HSM (AWS KMS) ││  │  │   HSM (AWS KMS)         ││ │     │
│  │  │  │   Key Storage   ││  │  │   Key Storage   ││  │  │   Key Storage           ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  └─────────────────────┘  └─────────────────────┘  └─────────────────────────────┘ │     │
│  │                                                                                      │     │
│  │  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────────────┐ │     │
│  │  │    VALIDATOR 4      │  │    (Future) V5      │  │        (Future) V6          │ │     │
│  │  │  Region: US-West    │  │    Region: TBD      │  │        Region: TBD          │ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   BABE Block    ││  │  │   BABE Block    ││  │  │   BABE Block            ││ │     │
│  │  │  │   Production    ││  │  │   Production    ││  │  │   Production            ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   GRANDPA       ││  │  │   GRANDPA       ││  │  │   GRANDPA               ││ │     │
│  │  │  │   Finality      ││  │  │   Finality      ││  │  │   Finality              ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   POKER PALLET  ││  │  │   POKER PALLET  ││  │  │   POKER PALLET          ││ │     │
│  │  │  │   (Game Logic)  ││  │  │   (Game Logic)  ││  │  │   (Game Logic)          ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   BLS PALLET    ││  │  │   BLS PALLET    ││  │  │   BLS PALLET            ││ │     │
│  │  │  │   (Threshold)   ││  │  │   (Threshold)   ││  │  │   (Threshold)           ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  │  ┌─────────────────┐│  │  ┌─────────────────┐│  │  ┌─────────────────────────┐│ │     │
│  │  │  │   HSM (AWS KMS) ││  │  │   HSM (AWS KMS) ││  │  │   HSM (AWS KMS)         ││ │     │
│  │  │  │   Key Storage   ││  │  │   Key Storage   ││  │  │   Key Storage           ││ │     │
│  │  │  └─────────────────┘│  │  └─────────────────┘│  │  └─────────────────────────┘│ │     │
│  │  └─────────────────────┘  └─────────────────────┘  └─────────────────────────────┘ │     │
│  └────────────────────────────────────────────────────────────────────────────────────┘     │
│                                        │                                                     │
│                                        ▼                                                     │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐     │
│  │                              STORAGE & DATA LAYER                                 │     │
│  │                                                                                      │     │
│  │  ┌───────────────────┐  ┌───────────────────┐  ┌─────────────────────────────────┐ │     │
│  │  │     RocksDB       │  │   IAVL Merkle     │  │     S3/Blob Storage            │ │     │
│  │  │  (Block Storage)  │  │    Tree (State)   │  │  (Archived Games, Events)     │ │     │
│  │  └───────────────────┘  └───────────────────┘  └─────────────────────────────────┘ │     │
│  │                                                                                      │     │
│  └────────────────────────────────────────────────────────────────────────────────────┘     │
│                                                                                              │
└─────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Component Relationships

```
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                              COMPONENT INTERACTION FLOW                                     │
├─────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                              │
│  1. CLIENT REQUEST                                                                          │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────────────┐      │
│  │  Client  │───►│  Auth    │───►│  Rate    │───►│  gRPC/   │───►│  Transaction     │      │
│  │          │    │  Check   │    │  Limit   │    │  REST    │    │  Pool            │      │
│  └──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────────────┘      │
│                                                                                              │
│  2. CONSENSUS PROCESSING                                                                    │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐    ┌──────────────┐  │
│  │  BABE Block      │◄───│  Validator       │───►│  GRANDPA         │───►│  Block       │  │
│  │  Production      │    │  Communication   │    │  Finality        │    │  Commitment  │  │
│  └──────────────────┘    └──────────────────┘    └──────────────────┘    └──────────────┘  │
│                                                                                              │
│  3. STATE TRANSITION                                                                        │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐    ┌──────────────┐  │
│  │  Poker Pallet    │◄───│  Executive       │───►│  BLS Threshold   │───►│  Timestamp   │  │
│  │  (Game Logic)    │    │  (Runtime)       │    │  (Signatures)    │    │  Pallet      │  │
│  └──────────────────┘    └──────────────────┘    └──────────────────┘    └──────────────┘  │
│                                                                                              │
│  4. STORAGE UPDATE                                                                          │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐    ┌──────────────┐  │
│  │  RocksDB State   │◄───│  IAVL Merkle     │───►│  Event Log       │───►│  Merkle      │  │
│  │  Update          │    │  Root Update     │    │  (Append)        │    │  Proof Gen   │  │
│  └──────────────────┘    └──────────────────┘    └──────────────────┘    └──────────────┘  │
│                                                                                              │
│  5. EVENT NOTIFICATION                                                                     │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐    ┌──────────────┐  │
│  │  WebSocket       │◄───│  Event Emitter   │◄───│  Block Finalized │◄───│  GRANDPA     │  │
│  │  Broadcast       │    │                  │    │                  │    │  Vote        │  │
│  └──────────────────┘    └──────────────────┘    └──────────────────┘    └──────────────┘  │
│                                                                                              │
└─────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 2.3 Technology Stack Summary

| Layer | Component | Technology | Version | Purpose |
|-------|-----------|------------|---------|---------|
| **Core** | Runtime Framework | Substrate | 4.0+ | Blockchain runtime |
| **Language** | Implementation | Rust | 1.75+ | System programming |
| **Consensus** | Block Production | BABE | Native | Slot-based production |
| **Consensus** | Finality Gadget | GRANDPA | Native | Absolute finality |
| **Networking** | P2P Network | libp2p | 0.51+ | Peer discovery/communication |
| **Storage** | State Database | RocksDB | 8.x | Persistent storage |
| **Merkle** | State Commitment | IavlTree | Substrate | Merkle proofs |
| **Crypto** | Elliptic Curves | ark-bls12-381 | 0.4+ | BLS signatures |
| **Threshold** | Threshold Crypto | kzen/threshold | 0.3+ | DKG and threshold sigs |
| **ZK Proofs** | Zero-Knowledge | halo2 | Latest | Privacy proofs |
| **RPC** | gRPC Framework | tonic | 0.10+ | API implementation |
| **Encoding** | Serialization | parity-scale-codec | 4.x | SCALE encoding |

---

## 3. Detailed Technical Specifications

### 3.1 Substrate Node Configuration

#### 3.1.1 Node Types and Roles

| Node Type | Quantity | Role | Requirements |
|-----------|----------|------|--------------|
| Validator | 4-7 | Block production, finality voting | Full node + HSM |
| RPC Node | 2-4 | Public API endpoint | Full state access |
| Archive Node | 1-2 | Historical data, light client support | Full history |
| Light Client | Unlimited | State verification | Bootstrap peers only |

#### 3.1.2 Runtime Configuration

```rust
// src/runtime.rs

/// Poker Consensus Engine Runtime Configuration
#[frame_support::runtime]
mod runtime {
    use super::*;

    // System components
    type System = frame_system::Pallet<Self>;

    // Consensus components
    type Babe = pallet_babe::Pallet<Self>;
    type Grandpa = pallet_grandpa::Pallet<Self>;
    type Authorship = pallet_authorship::Pallet<Self, IronbaesFor<Self>>;

    // Gaming components
    type Poker = poker_pallet::Pallet<Self>;
    type Timestamp = pallet_timestamp::Pallet<Self>;
    type Dkg = dkg_pallet::Pallet<Self>;
    type Bls = bls_pallet::Pallet<Self>;

    // Balances and staking
    type Balances = pallet_balances::Pallet<Self>;
    type Staking = pallet_staking::Pallet<Self>;

    // Runtime API
    #[runtime::pallet_index(0)]
    pub type RuntimeCall = RuntimeCall;

    #[runtime::pallet_index(0)]
    pub type RuntimeEvent = RuntimeEvent;

    #[runtime::pallet_index(0)]
    pub type Origin = Origin;
}

/// Consensus constants
pub const SLOT_DURATION: MillisPerSecond = 1000; // 1 second slots
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 2400; // ~1 hour
pub const GRANDPA_VOTING_PERIOD: BlockNumber = 300; // ~5 minutes
```

#### 3.1.3 Transaction Weight Configuration

```rust
// src/runtime/weights.rs

/// Weights for poker pallet operations
pub struct PokerWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for PokerWeight<T> {
    // Game creation
    fn create_game() -> Weight {
        Weight::from_parts(50_000_000, 0)
    }

    fn join_game() -> Weight {
        Weight::from_parts(30_000_000, 0)
    }

    fn player_action() -> Weight {
        Weight::from_parts(25_000_000, 0)
    }

    fn submit_cards() -> Weight {
        Weight::from_parts(100_000_000, 0) // ZK proof verification
    }

    fn timeout_action() -> Weight {
        Weight::from_parts(15_000_000, 0)
    }

    fn distribute_pot() -> Weight {
        Weight::from_parts(40_000_000, 0)
    }

    // BLS operations
    fn threshold_sign() -> Weight {
        Weight::from_parts(5_000_000, 0)
    }

    fn aggregate_signatures() -> Weight {
        Weight::from_parts(10_000_000, 0)
    }

    fn verify_threshold_sig() -> Weight {
        Weight::from_parts(15_000_000, 0)
    }
}
```

#### 3.1.4 Benchmark Requirements

| Operation | Target Time | Max Time | Throughput |
|-----------|-------------|----------|------------|
| Game Creation | 100ms | 200ms | 10/sec |
| Player Action | 50ms | 100ms | 50/sec |
| ZK Proof Verify | 200ms | 500ms | 5/sec |
| BLS Threshold Sign | 10ms | 50ms | 100/sec |
| Block Production | 1s | 2s | 1 block/s |
| Finality Vote | 100ms | 200ms | 10/sec |

### 3.2 Pallet Specifications

#### 3.2.1 Poker Pallet - Storage Items

```rust
// src/pallets/poker/src/lib.rs

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Game identifier (hash of creation parameters)
    #[pallet::storage]
    #[pallet::getter(fn games)]
    pub type Games<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        GameId,
        Game<T>,
        OptionQuery,
    >;

    /// Player actions for each game (event sourcing)
    #[pallet::storage]
    #[pallet::getter(fn player_actions)]
    pub type GameActions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        GameId,
        Blake2_128Concat,
        ActionIndex,
        PlayerActionRecord<T>,
        OptionQuery,
    >;

    /// Pending timeouts awaiting threshold consensus
    #[pallet::storage]
    #[pallet::getter(fn pending_timeouts)]
    pub type PendingTimeouts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        GameId,
        TimeoutRequest<T>,
        OptionQuery,
    >;

    /// Game configurations (templates)
    #[pallet::storage]
    #[pallet::getter(fn game_configs)]
    pub type GameConfigs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ConfigId,
        GameConfig,
        OptionQuery,
    >;

    /// Next game ID counter
    #[pallet::storage]
    #[pallet::getter(fn next_game_id)]
    pub type NextGameId<T: Config> = StorageValue<_, GameId, ValueQuery>;
}

/// Game state structure
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Game<T: Config> {
    /// Unique game identifier
    pub game_id: GameId,
    /// Current game state
    pub state: GameState,
    /// Players in the game
    pub players: BoundedVec<Player<T>, T::MaxPlayers>,
    /// Community cards (committed, revealed progressively)
    pub community_cards: BoundedVec<CardCommitment, 5>,
    /// Cards dealt to each player (revealed at showdown)
    pub hole_cards: BoundedVec<BoundedVec<CardCommitment, 2>, T::MaxPlayers>,
    /// Current pot size
    pub pot: BalanceOf<T>,
    /// Current bet amount (for call/raise tracking)
    pub current_bet: BalanceOf<T>,
    /// Bets made by each player this round
    pub player_bets: BoundedVec<BalanceOf<T>, T::MaxPlayers>,
    /// Game configuration
    pub config: GameConfig,
    /// Blinds configuration
    pub blind_levels: Vec<BlindLevel>,
    /// Current blind level index
    pub blind_index: u8,
    /// Block number when game created
    pub created_block: BlockNumberOf<T>,
    /// Block number of last action
    pub last_action_block: BlockNumberOf<T>,
    /// Timeout configuration
    pub timeout_config: TimeoutConfig,
    /// Dealer/button position
    pub button_position: u32,
    /// Current actor to act
    pub actor_position: u32,
    /// Betting round
    pub betting_round: BettingRound,
    /// Hand number within session (tournament)
    pub hand_number: u32,
}

/// Game states (state machine)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum GameState {
    /// Lobby: Waiting for players
    Lobby,
    /// Pre-flop: Cards being dealt
    PreFlop,
    /// Flop: Three community cards
    Flop,
    /// Turn: Fourth community card
    Turn,
    /// River: Fifth community card
    River,
    /// Showdown: Determining winner
    Showdown,
    /// Completed: Game finished
    Completed,
    /// Cancelled: Game aborted
    Cancelled(Error),
}

/// Player action types
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum PlayerAction {
    /// Fold: Forfeit hand
    Fold,
    /// Check: Bet 0 when no bet pending
    Check,
    /// Call: Match current bet
    Call,
    /// Raise: Increase bet by amount
    Raise(Balance),
    /// All-in: Bet all remaining chips
    AllIn,
    /// Timeout request
    RequestTimeout,
}
```

#### 3.2.2 Poker Pallet - Dispatchables

```rust
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::create_game())]
impl<T: Config> Pallet<T> {
    /// Create a new poker game
    #[pallet::call_index(0)]
    #[pallet::weight(Weight::from_parts(50_000_000, 0))]
    pub fn create_game(
        origin: OriginFor<T>,
        min_players: u8,
        max_players: u8,
        buy_in: BalanceOf<T>,
        config_id: ConfigId,
        timeout_blocks: BlockNumberOf<T>,
    ) -> DispatchResult {
        let creator = ensure_signed(origin)?;
        
        // Validate parameters
        ensure!(min_players >= 2, Error::<T>::InvalidMinPlayers);
        ensure!(max_players <= T::MaxPlayers::get(), Error::<T>::TooManyPlayers);
        ensure!(min_players <= max_players, Error::<T>::InvalidPlayerRange);
        
        // Get game config
        let config = GameConfigs::<T>::get(config_id)
            .ok_or(Error::<T>::InvalidConfig)?;
        
        // Create game
        let game_id = Self::do_create_game(
            creator,
            min_players,
            max_players,
            buy_in,
            config,
            timeout_blocks,
        )?;
        
        // Emit event
        Self::deposit_event(Event::GameCreated {
            game_id,
            creator,
            min_players,
            max_players,
            buy_in,
        });
        
        Ok(())
    }

    /// Join an existing game
    #[pallet::call_index(1)]
    #[pallet::weight(Weight::from_parts(30_000_000, 0))]
    pub fn join_game(
        origin: OriginFor<T>,
        game_id: GameId,
        buy_in: BalanceOf<T>,
    ) -> DispatchResult {
        let player = ensure_signed(origin)?;
        Self::do_join_game(player, game_id, buy_in)
    }

    /// Submit a player action
    #[pallet::call_index(2)]
    #[pallet::weight(Weight::from_parts(25_000_000, 0))]
    pub fn player_action(
        origin: OriginFor<T>,
        game_id: GameId,
        action: PlayerAction,
        amount: Option<BalanceOf<T>>,
        proof: Option<ZKProof>,
    ) -> DispatchResult {
        let player = ensure_signed(origin)?;
        Self::do_player_action(player, game_id, action, amount, proof)
    }

    /// Request timeout for a game (triggers threshold consensus)
    #[pallet::call_index(3)]
    #[pallet::weight(Weight::from_parts(15_000_000, 0))]
    pub fn request_timeout(
        origin: OriginFor<T>,
        game_id: GameId,
    ) -> DispatchResult {
        let requester = ensure_signed(origin)?;
        Self::do_request_timeout(requester, game_id)
    }

    /// Submit hole cards (encrypted for showdown)
    #[pallet::call_index(4)]
    #[pallet::weight(Weight::from_parts(100_000_000, 0))]
    pub fn submit_hole_cards(
        origin: OriginFor<T>,
        game_id: GameId,
        encrypted_cards: EncryptedCards,
        commitment: CardCommitment,
    ) -> DispatchResult {
        let player = ensure_signed(origin)?;
        Self::do_submit_hole_cards(player, game_id, encrypted_cards, commitment)
    }
}

/// Events emitted by the poker pallet
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// A new game has been created
    GameCreated {
        game_id: GameId,
        creator: T::AccountId,
        min_players: u8,
        max_players: u8,
        buy_in: BalanceOf<T>,
    },
    /// A player joined a game
    PlayerJoined {
        game_id: GameId,
        player: T::AccountId,
        chip_count: BalanceOf<T>,
        player_number: u8,
    },
    /// A player performed an action
    PlayerAction {
        game_id: GameId,
        player: T::AccountId,
        action: PlayerAction,
        amount: Option<BalanceOf<T>>,
        new_bet: BalanceOf<T>,
    },
    /// Cards were dealt
    CardsDealt {
        game_id: GameId,
        cards: Vec<CardCommitment>,
        revealed: bool,
    },
    /// A timeout was triggered
    TimeoutTriggered {
        game_id: GameId,
        reason: TimeoutReason,
        timestamp: T::Timestamp,
    },
    /// Game reached showdown
    Showdown {
        game_id: GameId,
        winners: Vec<(T::AccountId, BalanceOf<T>)>,
        hand_descriptions: Vec<String>,
    },
    /// Game completed
    GameCompleted {
        game_id: GameId,
        pot_distribution: Vec<(T::AccountId, BalanceOf<T>)>,
    },
}

/// Errors for the poker pallet
#[pallet::error]
pub enum Error<T> {
    GameNotFound,
    PlayerNotInGame,
    InvalidGameState,
    NotPlayersTurn,
    InvalidAction,
    BetTooLow,
    InsufficientChips,
    TooManyPlayers,
    NotEnoughPlayers,
    InvalidMinPlayers,
    InvalidPlayerRange,
    InvalidConfig,
    TimeoutPending,
    InvalidProof,
    CommitmentMismatch,
    GameAlreadyStarted,
    InvalidBlindLevel,
    TournamentNotConfigured,
}
```

#### 3.2.3 Timestamp Pallet (HLC Implementation)

```rust
// src/pallets/timestamp/src/lib.rs

/// Hybrid Logical Clock implementation for fair timestamping
#[derive(Debug, Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct HybridLogicalClock {
    /// Physical time component (milliseconds since Unix epoch)
    pub physical_time: u64,
    /// Logical time component
    pub logical_counter: u32,
    /// Validator ID who last updated this clock
    pub source_validator: ValidatorId,
}

/// Timestamp state stored on-chain
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TimestampState<T: Config> {
    /// Current HLC
    pub current_hlc: HybridLogicalClock,
    /// Validator signatures for current HLC
    pub validator_signatures: Vec<(ValidatorId, BlsSignature)>,
    /// Number of valid signatures collected
    pub signature_count: u32,
    /// Whether threshold has been reached
    pub threshold_reached: bool,
}

/// DKG state for threshold key management
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct DKGState {
    /// Current epoch
    pub epoch: u32,
    /// Group public key (if generated)
    pub group_public_key: Option<BlsPublicKey>,
    /// Key generation phase
    pub phase: DKGPhase,
    /// Validator key shares
    pub key_shares: Vec<(ValidatorId, BlsSecretKeyShare)>,
}

/// DKG phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum DKGPhase {
    /// No key generation in progress
    Idle,
    /// Key generation in progress
    KeyGeneration,
    /// Key resharing in progress
    Resharing,
    /// Key ready for signing
    Ready,
}

/// Threshold configuration
pub const THRESHOLD_NUMERATOR: u32 = 3;  // 3-of-4
pub const THRESHOLD_DENOMINATOR: u32 = 4;
```

#### 3.2.4 DKG Pallet

```rust
// src/pallets/dkg/src/lib.rs

/// Distributed Key Generation for threshold BLS keys
#[pallet::storage]
#[pallet::getter(fn dkg_state)]
pub type DKGStateStorage<T: Config> = StorageValue<_, DKGState<T>, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn key_shares)]
pub type KeyShares<T: Config> = StorageMap<
    _,
    Twox64Concat,
    T::ValidatorId,
    BlsSecretKeyShare,
    OptionQuery,
>;

#[pallet::storage]
#[pallet::getter(fn public_key)]
pub type GroupPublicKey<T: Config> = StorageValue<_, BlsPublicKey, OptionQuery>;

/// DKG Protocol States
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DKGProtocolState {
    /// Idle - no protocol running
    Idle,
    /// Key Generation in progress
    KeyGeneration(KeyGenerationRound),
    /// Resharing in progress
    Resharing(ResharingRound),
    /// Key is ready for use
    Ready { epoch: u32 },
}

/// Key Generation Round states
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum KeyGenerationRound {
    /// Waiting for dealers
    DealPhase,
    /// Computing shares
    SharePhase,
    /// Verifying shares
    VerifyPhase,
    /// Finalizing key
    FinalizePhase,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Start a new DKG ceremony
    #[pallet::call_index(0)]
    #[pallet::weight(Weight::from_parts(50_000_000, 0))]
    pub fn start_key_generation(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        let caller = ensure_root(origin)?;
        
        // Check all validators are online
        let validators = ValidatorSet::<T>::get();
        ensure!(validators.len() >= 4, Error::<T>::InsufficientValidators);
        
        // Initialize DKG state
        DKGStateStorage::<T>::put(DKGState {
            protocol: DKGProtocolState::KeyGeneration(KeyGenerationRound::DealPhase),
            validators: validators.clone(),
            ..Default::default()
        });
        
        // Emit event
        Self::deposit_event(Event::DKGStarted {
            epoch: CurrentEpoch::<T>::get() + 1,
            validator_count: validators.len() as u32,
        });
        
        Ok(())
    }

    /// Submit dealer's key share
    #[pallet::call_index(1)]
    #[pallet::weight(Weight::from_parts(20_000_000, 0))]
    pub fn submit_dealer_share(
        origin: OriginFor<T>,
        dealer_id: ValidatorId,
        encrypted_share: EncryptedShare,
        proof_of_knowledge: BlsSignature,
    ) -> DispatchResult {
        let validator = ensure_signed(origin)?;
        
        // Verify dealer
        ensure!(dealer_id == validator, Error::<T>::InvalidDealer);
        
        let mut state = DKGStateStorage::<T>::get();
        match &mut state.protocol {
            DKGProtocolState::KeyGeneration(round) => {
                match round {
                    KeyGenerationRound::DealPhase => {
                        // Store share and proof
                        DealerShares::<T>::insert(dealer_id, (encrypted_share, proof_of_knowledge));
                    }
                    _ => return Err(Error::<T>::WrongRound.into()),
                }
            }
            _ => return Err(Error::<T>::NotInProgress.into()),
        }
        
        DKGStateStorage::<T>::put(state);
        Ok(())
    }

    /// Complete DKG and publish public key
    #[pallet::call_index(2)]
    #[pallet::weight(Weight::from_parts(30_000_000, 0))]
    pub fn finalize_key_generation(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        let _ = ensure_root(origin)?;
        
        // Verify all shares received
        let validators = ValidatorSet::<T>::get();
        for validator in &validators {
            ensure!(DealerShares::<T>::contains_key(validator), Error::<T>::MissingShare);
        }
        
        // Compute group public key
        let shares: Vec<_> = validators
            .iter()
            .map(|v| KeyShares::<T>::get(v).unwrap())
            .collect();
        
        let group_public_key = compute_group_public_key(&shares)?;
        GroupPublicKey::<T>::put(group_public_key);
        
        // Update state
        DKGStateStorage::<T>::put(DKGState {
            protocol: DKGProtocolState::Ready {
                epoch: CurrentEpoch::<T>::get(),
            },
            ..Default::default()
        });
        
        // Emit event
        Self::deposit_event(Event::DKGCompleted {
            epoch: CurrentEpoch::<T>::get(),
            public_key: group_public_key,
        });
        
        Ok(())
    }
}
```

#### 3.2.5 BLS Pallet

```rust
// src/pallets/bls/src/lib.rs

/// BLS signature aggregation and verification
#[pallet::storage]
#[pallet::getter(fn threshold_signatures)]
pub type ThresholdSignatures<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    SignatureRequestId,
    AggregatedSignature,
    OptionQuery,
>;

/// Pending partial signatures
#[pallet::storage]
#[pallet::getter(fn partial_signatures)]
pub type PartialSignatures<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    SignatureRequestId,
    Twox64Concat,
    ValidatorId,
    PartialSignature,
    OptionQuery,
>;

/// Signature request status
#[pallet::storage]
#[pallet::getter(fn signature_requests)]
pub type SignatureRequests<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    SignatureRequestId,
    SignatureRequest<T>,
    OptionQuery,
>;

/// Next signature request ID
#[pallet::storage]
#[pallet::getter(fn next_request_id)]
pub type NextRequestId<T: Config> = StorageValue<_, SignatureRequestId, ValueQuery>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Submit a partial signature for threshold signing
    #[pallet::call_index(0)]
    #[pallet::weight(Weight::from_parts(5_000_000, 0))]
    pub fn submit_partial_signature(
        origin: OriginFor<T>,
        request_id: SignatureRequestId,
        partial_sig: BlsSignature,
    ) -> DispatchResult {
        let validator = ensure_signed(origin)?;
        
        // Get signature request
        let request = SignatureRequests::<T>::get(request_id)
            .ok_or(Error::<T>::RequestNotFound)?;
        
        // Verify request is active
        ensure!(request.status == SignatureRequestStatus::Active, Error::<T>::RequestClosed);
        
        // Verify validator is in the required set
        ensure!(request.required_signers.contains(&validator), Error::<T>::NotAuthorized);
        
        // Store partial signature
        PartialSignatures::<T>::insert(request_id, &validator, partial_sig);
        
        // Check if threshold reached
        let sig_count = PartialSignatures::<T>::iter_prefix(request_id).count() as u32;
        if sig_count >= request.threshold {
            Self::aggregate_signatures(request_id)?;
        }
        
        Ok(())
    }

    /// Aggregate partial signatures into threshold signature
    fn aggregate_signatures(request_id: SignatureRequestId) -> Result<(), DispatchError> {
        let request = SignatureRequests::<T>::get(request_id)
            .ok_or(Error::<T>::RequestNotFound)?;
        
        // Collect partial signatures
        let partials: Vec<_> = PartialSignatures::<T>::iter_prefix(request_id)
            .map(|(_, sig)| sig)
            .collect();
        
        // Aggregate using BLS threshold library
        let aggregated = kzen_networks_threshold::aggregate(&partials)
            .map_err(|_| Error::<T>::AggregationFailed)?;
        
        // Store aggregated signature
        ThresholdSignatures::<T>::insert(
            request_id,
            AggregatedSignature {
                signature: aggregated,
                signers: partials.len() as u32,
                message: request.message.clone(),
            },
        );
        
        // Update request status
        SignatureRequests::<T>::mutate(request_id, |req| {
            if let Some(ref mut r) = req {
                r.status = SignatureRequestStatus::Completed;
            }
        });
        
        // Emit event
        Self::deposit_event(Event::ThresholdSignatureCreated {
            request_id,
            aggregator: request.required_signers[0],
            signers: partials.len() as u32,
        });
        
        Ok(())
    }

    /// Verify a threshold signature
    #[pallet::call_index(1)]
    #[pallet::weight(Weight::from_parts(15_000_000, 0))]
    pub fn verify_threshold_signature(
        origin: OriginFor<T>,
        request_id: SignatureRequestId,
    ) -> DispatchResultWithPostInfo {
        let _ = ensure_signed(origin)?;
        
        let signature = ThresholdSignatures::<T>::get(request_id)
            .ok_or(Error::<T>::SignatureNotFound)?;
        
        let request = SignatureRequests::<T>::get(request_id)
            .ok_or(Error::<T>::RequestNotFound)?;
        
        // Verify against group public key
        let public_key = GroupPublicKey::<T>::get()
            .ok_or(Error::<T>::NoPublicKey)?;
        
        let valid = verify_threshold_signature(
            &signature.signature,
            &request.message,
            &public_key,
        ).map_err(|_| Error::<T>::VerificationFailed)?;
        
        ensure!(valid, Error::<T>::InvalidSignature);
        
        Ok(Some(signature).into())
    }
}
```

### 3.3 Cryptographic Specifications

#### 3.3.1 BLS12-381 Parameter Definitions

```rust
// src/crypto/bls381.rs

/// BLS12-381 curve parameters for the Poker Consensus Engine
pub struct Bls381Parameters;

impl ark_bn254::Parameters for Bls381Parameters {
    /// Base field modulus (p)
    const P: ark_bn254::FqParams::BigInt = ark_bn254::Fq::MODULUS;
    
    /// Scalar field modulus (r)
    const Q: ark_bn254::FrParams::BigInt = ark_bn254::Fr::MODULUS;
    
    /// Generator point on G1
    const G1_GENERATOR: ark_bn254::G1Affine = ark_bn254::G1Affine::new(
        ark_bn254::g1::COFACTOR_G1_X,
        ark_bn254::g1::COFACTOR_G1_Y,
    );
    
    /// Generator point on G2
    const G2_GENERATOR: ark_bn254::G2Affine = ark_bn254::G2Affine::new(
        ark_bn254::g2::COFACTOR_G2_X_C0,
        ark_bn254::g2::COFACTOR_G2_X_C1,
        ark_bn254::g2::COFACTOR_G2_Y_C0,
        ark_bn254::g2::COFACTOR_G2_Y_C1,
    );
}

/// BLS12-381 key sizes
pub struct BlsKeySizes;

impl BlsKeySizes {
    /// Private key size (scalar): 32 bytes
    pub const PRIVATE_KEY_SIZE: usize = 32;
    
    /// Public key size (G2): 96 bytes
    pub const PUBLIC_KEY_SIZE: usize = 96;
    
    /// Signature size (G1): 48 bytes
    pub const SIGNATURE_SIZE: usize = 48;
    
    /// Message hash size: 48 bytes (for hashing to G1)
    pub const HASH_SIZE: usize = 48;
    
    /// Threshold share size: 32 bytes
    pub const SHARE_SIZE: usize = 32;
}

/// Threshold scheme configuration
pub const THRESHOLD_CONFIG: ThresholdConfig = ThresholdConfig {
    /// Total number of validators
    total_participants: 4,
    
    /// Minimum signatures required
    threshold: 3,
    
    /// Security level (bits)
    security_level: 128,
    
    /// Maximum signature size (bytes)
    max_signature_size: 48,
};
```

#### 3.3.2 Threshold Scheme (3-of-4 Setup)

```rust
// src/crypto/threshold.rs

/// 3-of-4 Threshold BLS Scheme
///
/// This scheme uses Shamir's Secret Sharing combined with BLS signatures.
/// Each validator holds a key share; any 3 validators can produce a valid
/// threshold signature that verifies against the group public key.

use kzen_networks_threshold::prelude::*;

/// Threshold BLS Key Generation
pub async fn generate_threshold_keys(
    participants: &[ValidatorId],
    threshold: usize,
) -> Result<ThresholdKeyPair, CryptoError> {
    // Create DKG protocol instance
    let dkg = DistributedKeyGeneration::<Bls381Parameters>::new(
        participants.len(),
        threshold,
    );
    
    // Phase 1: Deal shares
    let deals: Vec<_> = participants
        .iter()
        .enumerate()
        .map(|(i, participant)| {
            let (deal, share) = dkg.create_deal(i);
            (participant.clone(), deal, share)
        })
        .collect();
    
    // Phase 2: Process deals and generate responses
    let mut responses = Vec::new();
    for (participant, deal, _) in &deals {
        let response = dkg.process_deal(deal, participant.clone())?;
        responses.push((participant.clone(), response));
    }
    
    // Phase 3: Finalize DKG and extract key shares
    let (group_public_key, secret_shares) = dkg.finalize(&responses)?;
    
    // Create key pair for each participant
    let key_pairs: HashMap<_, _> = participants
        .iter()
        .enumerate()
        .map(|(i, participant)| {
            (
                participant.clone(),
                ThresholdKeyPair {
                    public_key: group_public_key.clone(),
                    secret_share: secret_shares[i].clone(),
                    participant_index: i,
                },
            )
        })
        .collect();
    
    Ok(ThresholdKeyPair {
        group_public_key,
        participant_key_shares: key_pairs,
    })
}

/// Partial signature generation
pub fn create_partial_signature(
    key_share: &BlsSecretKeyShare,
    message: &[u8],
) -> Result<PartialBlsSignature, CryptoError> {
    // Hash message to the scalar field
    let message_hash = hash_to_scalar(message)?;
    
    // Sign with secret share
    let signature = key_share.sign(message_hash);
    
    Ok(PartialBlsSignature {
        share: signature,
        signer_index: key_share.index(),
    })
}

/// Partial signature aggregation
pub fn aggregate_partial_signatures(
    partials: &[PartialBlsSignature],
    threshold: usize,
    group_public_key: &BlsPublicKey,
) -> Result<ThresholdSignature, CryptoError> {
    ensure!(
        partials.len() >= threshold,
        CryptoError::InsufficientSignatures {
            have: partials.len(),
            required: threshold,
        }
    );
    
    // Use Lagrange interpolation to combine shares
    let signature = kzen_networks_threshold::aggregate(partials)
        .map_err(|e| CryptoError::AggregationError(e.to_string()))?;
    
    Ok(ThresholdSignature {
        signature,
        signers: partials.len(),
        group_public_key: group_public_key.clone(),
    })
}

/// Threshold signature verification
pub fn verify_threshold_signature(
    signature: &ThresholdSignature,
    message: &[u8],
) -> Result<bool, CryptoError> {
    // Hash message to G1
    let message_hash = hash_to_g1(message)?;
    
    // Verify pairing
    let valid = Bls12_381::check(
        &signature.signature,
        &signature.group_public_key,
        &message_hash,
    );
    
    Ok(valid)
}
```

#### 3.3.3 ZK Proof Circuit Specifications

```rust
// src/crypto/zk/mod.rs

/// Zero-knowledge proof circuits for poker operations
/// Using halo2 for recursive zk-SNARKs

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, Selector},
    poly::Rotation,
};

/// Card commitment circuit parameters
pub const NUM_CARDS: usize = 52;
pub const MAX_HAND_SIZE: usize = 7; // 2 hole cards + 5 community
pub const BLINDING_FACTOR_SIZE: usize = 32;

/// Card representation in circuit
#[derive(Clone, Debug)]
pub struct Card {
    /// Card suit (0-3)
    suit: u8,
    /// Card rank (2-14, where 11=J, 12=Q, 13=K, 14=A)
    rank: u8,
}

/// Card commitment using Pedersen commitment
pub struct CardCommitmentCircuit {
    /// Card values (committed)
    pub cards: [Value<u8>; MAX_HAND_SIZE],
    /// Blinding factors
    pub blinding_factors: [Value<Fr>; MAX_HAND_SIZE],
    /// Commitment hash output
    pub commitment: Value<Fr>,
}

/// Card commitment proof parameters
pub struct CardCommitmentParams {
    /// Generator for card value
    pub g_card: Point,
    /// Generator for blinding factor
    pub g_blind: Point,
    /// Elliptic curve generator
    pub g_base: Point,
}

/// Verify card dealing fairness
pub fn verify_card_fairness(
    commitment: &CardCommitment,
    revealed_cards: &[Card],
    blinding_factors: &[Fr],
    params: &CardCommitmentParams,
) -> bool {
    // Recompute commitment from revealed cards
    let mut computed = Point::identity();
    
    for (card, blind) in revealed_cards.iter().zip(blinding_factors) {
        let card_point = encode_card_to_point(card, params);
        let blind_point = params.g_blind * blind;
        computed = computed + card_point + blind_point;
    }
    
    commitment.point == computed
}

/// Hand evaluation ZK circuit
pub struct HandEvaluationCircuit {
    /// Hole cards (hidden from proof)
    pub hole_cards: [Value<Card>; 2],
    /// Community cards (public)
    pub community_cards: [Value<Card>; 5],
    /// Claimed hand rank
    pub claimed_rank: Value<u8>,
    /// Hand ranking public key
    pub ranking_public_key: Point,
}

/// Hand evaluation proof
pub struct HandEvaluationProof {
    /// ZK-SNARK proof
    pub proof: Vec<u8>,
    /// Claimed hand strength
    pub hand_rank: u8,
    /// Public inputs commitment
    public_inputs: HandEvaluationPublicInputs,
}

/// Public inputs for hand evaluation
#[derive(Clone, Debug)]
pub struct HandEvaluationPublicInputs {
    /// Commitment to hole cards
    pub hole_card_commitment: Fr,
    /// Commitment to community cards
    pub community_card_commitment: Fr,
    /// Hand ranking system output
    pub hand_rank: u8,
    /// Nonce for randomness
    pub nonce: Fr,
}
```

#### 3.3.4 Card Commitment Scheme

```rust
// src/crypto/commitments.rs

/// Card commitment scheme using Pedersen commitments with ZK proofs
/// Allows proving card values without revealing them

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{Field, PrimeField};

/// Commitment to a card with optional blinding
#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
pub struct CardCommitment {
    /// Commitment point on curve
    pub point: CompressedPoint,
    /// Card suit (0-3) encrypted
    pub encrypted_suit: EncryptedValue,
    /// Card rank (2-14) encrypted
    pub encrypted_rank: EncryptedValue,
    /// Blinding factor commitment
    pub blinding_commitment: Fr,
}

/// Encrypted card value (for showdown)
#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
pub struct EncryptedCard {
    /// Encrypted card data
    pub ciphertext: Ciphertext,
    /// Encryption nonce
    pub nonce: Nonce,
    /// Commitment to encryption key
    pub key_commitment: Fr,
}

/// Card shuffle verification proof
pub struct ShuffleProof {
    /// ZK proof of valid shuffle
    pub proof: Vec<u8>,
    /// Initial commitment
    pub initial_commitment: CardCommitment,
    /// Final commitment
    pub final_commitment: CardCommitment,
    /// Permutation used
    pub permutation: Vec<u8>,
}

/// Create card commitment
pub fn create_card_commitment(
    card: &Card,
    blinding: Fr,
    params: &CardCommitmentParams,
) -> Result<CardCommitment, CryptoError> {
    // Encode card to curve point
    let card_point = encode_card_to_point(card, params);
    
    // Create Pedersen commitment: C = G_value * card + G_blind * blinding
    let commitment = card_point + params.g_blind * blinding;
    
    // Encrypt card values for showdown
    let encrypted_suit = encrypt_value(card.suit as u64, params)?;
    let encrypted_rank = encrypt_value(card.rank as u64, params)?;
    
    Ok(CardCommitment {
        point: commitment.compress(),
        encrypted_suit,
        encrypted_rank,
        blinding_commitment: blinding,
    })
}

/// Reveal card with proof
pub fn reveal_card(
    commitment: &CardCommitment,
    card: &Card,
    blinding: Fr,
    params: &CardCommitmentParams,
) -> Result<CardRevealProof, CryptoError> {
    // Verify commitment matches card
    let recomputed = create_card_commitment(card, blinding, params)?;
    ensure!(
        commitment.point == recomputed.point,
        CryptoError::CommitmentMismatch
    );
    
    // Create ZK proof of knowledge
    let proof = create_range_proof(card, blinding, params)?;
    
    Ok(CardRevealProof {
        card: card.clone(),
        blinding,
        proof,
    })
}

/// Encode card to curve point
fn encode_card_to_point(card: &Card, params: &CardCommitmentParams) -> Point {
    // Map card (suit, rank) to curve point using deterministic encoding
    // suit in {0,1,2,3}, rank in {2,3,...,14}
    let card_value = (card.suit as u64) * 16 + (card.rank as u64);
    
    // Hash to curve (simplified - in production use proper hash-to-curve)
    params.g_card * Fr::from(card_value)
}
```

### 3.4 Network Specifications

#### 3.4.1 libp2p Protocol Configuration

```rust
// src/network/protocol.rs

/// libp2p protocol configuration for Poker Consensus Engine
pub struct PokerNetworkProtocol;

impl ProtocolName for PokerNetworkProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/poker-consensus/1.0.0".as_bytes()
    }
}

/// Gossip topics for different message types
#[derive(Debug, Clone)]
pub enum GossipTopic {
    /// Block production and propagation
    Consensus,
    /// Finality votes
    Finality,
    /// Game state updates
    GameUpdates,
    /// Transaction propagation
    Transactions,
    /// Threshold signature requests
    ThresholdSignatures,
    /// Timeout coordination
    Timeouts,
}

impl GossipTopic {
    /// Get topic string for libp2p
    pub fn topic_string(&self) -> String {
        match self {
            GossipTopic::Consensus => "/poker/consensus/1.0.0".to_string(),
            GossipTopic::Finality => "/poker/finality/1.0.0".to_string(),
            GossipTopic::GameUpdates => "/poker/games/1.0.0".to_string(),
            GossipTopic::Transactions => "/poker/txs/1.0.0".to_string(),
            GossipTopic::ThresholdSignatures => "/poker/threshold/1.0.0".to_string(),
            GossipTopic::Timeouts => "/poker/timeouts/1.0.0".to_string(),
        }
    }
}

/// Network configuration
#[derive(Clone, Debug)]
pub struct NetworkConfig {
    /// Listen addresses
    pub listen_addresses: Vec<Multiaddr>,
    /// Bootstrap nodes
    pub boot_nodes: Vec<Multiaddr>,
    /// Maximum number of peers
    pub max_peers: u32,
    /// Minimum number of healthy peers
    pub min_peers: u32,
    /// Inbound connection limit
    pub max_inbound: u32,
    /// Outbound connection limit
    pub max_outbound: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Ping interval
    pub ping_interval: Duration,
    /// Enable gossipsub
    pub gossipsub_enable: bool,
    /// Gossip mesh size
    pub gossipsub_mesh_size: usize,
    /// Gossip history length
    pub gossipsub_history_length: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addresses: vec![
                "/ip4/0.0.0.0/tcp/30333".parse().unwrap(),
                "/ip4/0.0.0.0/tcp/30334/ws".parse().unwrap(),
            ],
            boot_nodes: vec![],
            max_peers: 50,
            min_peers: 3,
            max_inbound: 25,
            max_outbound: 25,
            connection_timeout: Duration::from_secs(30),
            ping_interval: Duration::from_secs(15),
            gossipsub_enable: true,
            gossipsub_mesh_size: 6,
            gossipsub_history_length: 12,
        }
    }
}

/// Default network configuration for 4 validators
pub fn validator_network_config() -> NetworkConfig {
    NetworkConfig {
        listen_addresses: vec![
            "/ip4/0.0.0.0/tcp/30333".parse().unwrap(),
            "/ip4/0.0.0.0/tcp/30334/ws".parse().unwrap(),
            "/ip6/::/tcp/30333".parse().unwrap(),
        ],
        max_peers: 10,  // Limited for validator mesh
        min_peers: 3,
        max_inbound: 5,
        max_outbound: 5,
        gossipsub_mesh_size: 4,  // Full mesh for 4 validators
        ..Default::default()
    }
}
```

#### 3.4.2 Message Formats

```rust
// src/network/messages.rs

/// Network message types for Poker Consensus Engine
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum NetworkMessage {
    /// Consensus messages
    Consensus(ConsensusMessage),
    /// Game state messages
    Game(GameMessage),
    /// Transaction messages
    Transaction(TransactionMessage),
    /// Threshold signature messages
    Threshold(ThresholdMessage),
    /// Timeout coordination messages
    Timeout(TimeoutMessage),
}

/// Consensus message variants
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum ConsensusMessage {
    /// Block proposal
    BlockProposal {
        block: Block,
        sender: ValidatorId,
    },
    /// Vote on block
    Vote {
        block_hash: Hash,
        vote_type: VoteType,
        signature: BlsSignature,
        validator: ValidatorId,
    },
    /// Finality vote (GRANDPA)
    FinalityVote {
        block_hash: Hash,
        block_number: BlockNumber,
        signature: BlsSignature,
        validator: ValidatorId,
    },
}

/// Game message variants
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameMessage {
    /// New game created
    GameCreated {
        game_id: GameId,
        creator: AccountId,
        config: GameConfig,
    },
    /// Player joined
    PlayerJoined {
        game_id: GameId,
        player: AccountId,
        chip_count: Balance,
    },
    /// Player action
    PlayerAction {
        game_id: GameId,
        player: AccountId,
        action: PlayerAction,
        amount: Option<Balance>,
        proof: Option<ZKProof>,
    },
    /// Cards dealt
    CardsDealt {
        game_id: GameId,
        cards: Vec<CardCommitment>,
        revealed: bool,
    },
    /// Game state update
    StateUpdate {
        game_id: GameId,
        new_state: GameState,
        metadata: GameMetadata,
    },
}

/// Threshold signature message variants
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum ThresholdMessage {
    /// Signature request
    SignatureRequest {
        request_id: SignatureRequestId,
        message: Vec<u8>,
        deadline: BlockNumber,
    },
    /// Partial signature submission
    PartialSignature {
        request_id: SignatureRequestId,
        validator: ValidatorId,
        partial_sig: BlsSignature,
    },
    /// Signature ready notification
    SignatureReady {
        request_id: SignatureRequestId,
        aggregated_sig: BlsSignature,
        signers: Vec<ValidatorId>,
    },
}

/// Timeout coordination messages
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum TimeoutMessage {
    /// Timeout request
    TimeoutRequest {
        game_id: GameId,
        requester: AccountId,
        reason: TimeoutReason,
        timestamp: u64,
    },
    /// Timeout vote
    TimeoutVote {
        game_id: GameId,
        validator: ValidatorId,
        vote: TimeoutVoteType,
        signature: BlsSignature,
    },
    /// Timeout consensus reached
    TimeoutConsensus {
        game_id: GameId,
        decision: TimeoutDecision,
        threshold_signature: BlsSignature,
    },
}
```

#### 3.4.3 Connection Requirements

```rust
// src/network/connections.rs

/// Connection requirements for validator network
pub struct ValidatorConnectionRequirements;

impl ValidatorConnectionRequirements {
    /// Minimum connections for healthy validator
    pub fn min_connections() -> usize {
        3 // n-1 for n=4 validators
    }

    /// Maximum connections
    pub fn max_connections() -> u32 {
        10
    }

    /// Connection timeout
    pub fn connection_timeout() -> Duration {
        Duration::from_secs(30)
    }

    /// Heartbeat interval
    pub fn heartbeat_interval() -> Duration {
        Duration::from_secs(60)
    }

    /// Dial backoff
    pub fn dial_backoff() -> Vec<Duration> {
        vec![
            Duration::from_secs(1),
            Duration::from_secs(2),
            Duration::from_secs(5),
            Duration::from_secs(10),
            Duration::from_secs(30),
        ]
    }

    /// Keep-alive timeout
    pub fn keep_alive_timeout() -> Duration {
        Duration::from_secs(90)
    }

    /// Authenticated encryption
    pub fn encryption() -> EncryptionConfig {
        EncryptionConfig {
            protocol: NoiseProtocol::IK,
            handshake_timeout: Duration::from_secs(10),
        }
    }
}

/// Validator network topology
#[derive(Debug)]
pub struct ValidatorTopology {
    /// Full mesh for 4 validators
    pub topology: NetworkTopology,
    /// Peer exchange enabled
    pub peer_exchange: bool,
    /// Connection pool size
    pub pool_size: usize,
}

impl Default for ValidatorTopology {
    fn default() -> Self {
        Self {
            topology: NetworkTopology::FullMesh,
            peer_exchange: false,
            pool_size: 4,
        }
    }
}
```

### 3.5 API Specifications

#### 3.5.1 RPC Methods and Parameters

| Method | Parameters | Result | Description |
|--------|------------|--------|-------------|
| poker_createGame | configId, minPlayers, maxPlayers, buyIn, timeoutBlocks | GameId | Create new game |
| poker_joinGame | gameId, buyIn | JoinResult | Join existing game |
| poker_submitAction | gameId, action, amount, proof | ActionResult | Submit player action |
| poker_getGameState | gameId, includeCards | GameState | Get game state |
| poker_getGameHistory | gameId | GameHistory | Get complete history |
| poker_commitCards | gameId, commitments, encryptionKey | CommitResult | Commit to cards |
| poker_requestTimeout | gameId | TimeoutResult | Request timeout |
| poker_getTimeoutStatus | gameId | TimeoutStatus | Get timeout status |
| poker_listGames | state, limit, offset | GameList | List games |
| poker_getBalance | accountId | Balance | Get chip balance |

#### 3.5.2 WebSocket Event Schemas

```rust
// src/api/websocket/events.rs

/// WebSocket event types for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
#[serde(tag = "event_type")]
pub enum WebSocketEvent {
    #[serde(rename = "game_created")]
    GameCreated(GameCreatedEvent),
    
    #[serde(rename = "player_joined")]
    PlayerJoined(PlayerJoinedEvent),
    
    #[serde(rename = "player_action")]
    PlayerAction(PlayerActionEvent),
    
    #[serde(rename = "cards_dealt")]
    CardsDealt(CardsDealtEvent),
    
    #[serde(rename = "state_changed")]
    StateChanged(StateChangedEvent),
    
    #[serde(rename = "timeout")]
    Timeout(TimeoutEvent),
    
    #[serde(rename = "showdown")]
    Showdown(ShowdownEvent),
    
    #[serde(rename = "game_completed")]
    GameCompleted(GameCompletedEvent),
    
    #[serde(rename = "block_finalized")]
    BlockFinalized(BlockFinalizedEvent),
}

/// Game created event
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
pub struct GameCreatedEvent {
    pub game_id: String,
    pub creator: String,
    pub min_players: u8,
    pub max_players: u8,
    pub buy_in: String,
    pub config_id: String,
    pub block_number: u32,
    pub timestamp: u64,
}

/// Player action event
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
pub struct PlayerActionEvent {
    pub game_id: String,
    pub player: String,
    pub position: u32,
    pub action: String,
    pub amount: Option<String>,
    pub current_bet: String,
    pub chip_stack: String,
    pub block_number: u32,
    pub transaction_hash: String,
}

/// Timeout event
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
pub struct TimeoutEvent {
    pub game_id: String,
    pub reason: String,
    pub requester: Option<String>,
    pub status: String,
    pub votes: u32,
    pub required: u32,
    pub decision: Option<String>,
    pub signature: Option<String>,
}

/// WebSocket message format
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
pub struct WebSocketMessage<T> {
    pub data: T,
    pub subscription_id: String,
    pub timestamp: u64,
}
```

#### 3.5.3 Error Codes and Responses

```rust
// src/api/errors.rs

/// API error codes for Poker Consensus Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TypeInfo)]
#[repr(u32)]
pub enum ApiErrorCode {
    // Success (0-99)
    Success = 0,
    
    // Client errors (100-999)
    InvalidRequest = 100,
    NotFound = 101,
    Unauthorized = 102,
    Forbidden = 103,
    RateLimited = 104,
    InvalidParams = 105,
    
    // Game errors (1000-1999)
    GameNotFound = 1000,
    GameAlreadyStarted = 1001,
    GameFull = 1002,
    PlayerNotInGame = 1003,
    NotPlayersTurn = 1004,
    InvalidAction = 1005,
    InsufficientChips = 1006,
    BetTooLow = 1007,
    InvalidBlindLevel = 1008,
    
    // Cryptographic errors (2000-2999)
    InvalidSignature = 2000,
    SignatureExpired = 2001,
    InvalidCommitment = 2002,
    InvalidProof = 2003,
    CommitmentMismatch = 2004,
    
    // Consensus errors (3000-3999)
    ConsensusNotReached = 3000,
    ValidatorNotActive = 3001,
    ThresholdNotMet = 3002,
    DoubleSignDetected = 3003,
    
    // System errors (5000-5999)
    InternalError = 5000,
    DatabaseError = 5001,
    NetworkError = 5002,
    Timeout = 5003,
    ShuttingDown = 5004,
}

/// API error response
#[derive(Debug, Clone, Serialize, Deserialize, TypeInfo)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
    pub timestamp: u64,
}

impl ApiError {
    pub fn new(
        code: ApiErrorCode,
        message: String,
        details: Option<serde_json::Value>,
        request_id: String,
    ) -> Self {
        Self {
            code: code as u32,
            message,
            details,
            request_id,
            timestamp: unix_timestamp_ms(),
        }
    }

    pub fn bad_request(message: String, request_id: String) -> Self {
        Self::new(ApiErrorCode::InvalidRequest, message, None, request_id)
    }

    pub fn not_found(resource: String, request_id: String) -> Self {
        Self::new(
            ApiErrorCode::NotFound,
            format!("{} not found", resource),
            None,
            request_id,
        )
    }

    pub fn unauthorized(request_id: String) -> Self {
        Self::new(
            ApiErrorCode::Unauthorized,
            "Authentication required".to_string(),
            None,
            request_id,
        )
    }

    pub fn rate_limited(retry_after: u64, request_id: String) -> Self {
        Self::new(
            ApiErrorCode::RateLimited,
            "Rate limit exceeded".to_string(),
            Some(json!({ "retry_after": retry_after })),
            request_id,
        )
    }

    pub fn internal(message: String, request_id: String) -> Self {
        Self::new(
            ApiErrorCode::InternalError,
            message,
            None,
            request_id,
        )
    }
}

/// Result type for API calls
pub type ApiResult<T> = Result<T, ApiError>;
```

### 3.6 Storage Specifications

#### 3.6.1 On-Chain Storage Schema

| Storage Type | Key Type | Value Type | Purpose |
|--------------|----------|------------|---------|
| Games | GameId | Game<T> | Active game states |
| GameActions | (GameId, PlayerId, ActionIndex) | PlayerActionRecord<T> | Event sourcing |
| PendingTimeouts | GameId | TimeoutRequest<T> | Timeout coordination |
| GameConfigs | ConfigId | GameConfig | Game templates |
| NextGameId | - | GameId | ID counter |
| ValidatorPerformance | ValidatorId | ValidatorStats | Performance tracking |
| DKGState | - | DKGState | Threshold key state |
| GroupPublicKey | - | BlsPublicKey | Threshold public key |
| SignatureRequests | SignatureRequestId | SignatureRequest<T> | BLS signature requests |

#### 3.6.2 Off-Chain Storage Design

| Column | Data Type | Purpose | Retention |
|--------|-----------|---------|-----------|
| Blocks | Block data | Historical blocks | 90 days |
| Games | Game archives | Completed games | 7 years |
| Events | Event logs | Complete audit trail | 7 years |
| Proofs | Merkle proofs | State verification | 1 year |
| Compressed | Compressed history | Analytics | Indefinite |
| Indexes | Search indexes | Query optimization | Hot + Warm |
| Metadata | System metadata | Configuration | Indefinite |

#### 3.6.3 Data Retention Policies

| Data Type | Hot Tier | Warm Tier | Cold Tier |
|-----------|----------|-----------|-----------|
| Block Data | 0-30 days | 30-90 days | 90+ days (S3) |
| Game Archives | All | All | All (7 years) |
| Event Logs | All | All | All (7 years) |
| Merkle Proofs | 0-30 days | 30-365 days | 365+ days |
| Metrics | 0-7 days | 7-90 days | Archived |

---

## 4. Interface Specifications

### 4.1 Pallet Extrinsics Complete List

| Extrinsic | Parameters | Returns | Weight | Description |
|-----------|------------|---------|--------|-------------|
| **Poker Pallet** |
| create_game | min_players, max_players, buy_in, config_id, timeout_blocks | GameId | 50M | Create new game |
| join_game | game_id, buy_in | () | 30M | Join existing game |
| player_action | game_id, action, amount, proof | () | 25M | Submit action |
| request_timeout | game_id | () | 15M | Request timeout |
| submit_hole_cards | game_id, encrypted_cards, commitment | () | 100M | Submit hidden cards |
| **Timestamp Pallet** |
| update_timestamp | proposed_hlc, signature | () | 10M | Submit timestamp vote |
| get_consensus_timestamp | () | HLC | 1M | Get consensus timestamp |
| **DKG Pallet** |
| start_key_generation | () | () | 50M | Start DKG ceremony |
| submit_dealer_share | dealer_id, encrypted_share, pok | () | 20M | Submit key share |
| finalize_key_generation | () | () | 30M | Complete DKG |
| **BLS Pallet** |
| submit_partial_signature | request_id, partial_sig | () | 5M | Submit signature share |
| verify_threshold_signature | request_id | Signature | 15M | Verify aggregated sig |

### 4.2 Events and Errors

#### 4.2.1 Poker Pallet Events

| Event | Fields | Description |
|-------|--------|-------------|
| GameCreated | game_id, creator, min_players, max_players, buy_in | New game created |
| PlayerJoined | game_id, player, chip_count, player_number | Player joined game |
| PlayerAction | game_id, player, action, amount, new_bet | Player took action |
| CardsDealt | game_id, cards[], revealed | Cards dealt to players |
| StateChanged | game_id, old_state, new_state | Game state changed |
| TimeoutTriggered | game_id, reason, timestamp | Timeout occurred |
| TimeoutVoted | game_id, validator, vote | Validator voted on timeout |
| TimeoutConsensus | game_id, decision, signature | Timeout consensus reached |
| Showdown | game_id, winners[], hand_descriptions[] | Showdown reached |
| GameCompleted | game_id, pot_distribution[] | Game finished |
| GameCancelled | game_id, reason | Game aborted |

#### 4.2.2 Poker Pallet Errors

| Error | Code | Description |
|-------|------|-------------|
| GameNotFound | 1000 | Game ID doesn't exist |
| PlayerNotInGame | 1003 | Player not in game |
| InvalidGameState | 1004 | Operation invalid for state |
| NotPlayersTurn | 1005 | Not this player's action |
| InvalidAction | 1006 | Action not valid |
| BetTooLow | 1007 | Bet below minimum |
| InsufficientChips | 1008 | Not enough chips |
| TooManyPlayers | 1002 | Game is full |
| InvalidConfig | 1009 | Invalid game config |
| TimeoutPending | 1010 | Timeout already requested |
| InvalidProof | 2003 | ZK proof invalid |

### 4.3 RPC API

JSON-RPC 2.0 methods available include:
- `poker_createGame` - Create a new poker game
- `poker_joinGame` - Join an existing poker game
- `poker_submitAction` - Submit a player action
- `poker_getGameState` - Get current game state
- `poker_getGameHistory` - Get complete game history
- `poker_commitCards` - Commit to card values
- `poker_requestTimeout` - Request game timeout
- `poker_getTimeoutStatus` - Get timeout consensus status
- `poker_listGames` - List games by state
- `poker_getBalance` - Get player's chip balance

### 4.4 Client SDK Interfaces

#### JavaScript/TypeScript SDK

```typescript
export class PokerClient {
  private api: ApiPromise;
  private keyring: Keyring;

  constructor(endpoint: string) {
    this.wsProvider = new WsProvider(endpoint);
    this.keyring = new Keyring({ type: 'sr25519' });
  }

  async connect(): Promise<void> {
    this.api = await ApiPromise.create({
      provider: this.wsProvider,
      types: {
        GameId: 'Hash',
        GameState: { _enum: ['Lobby', 'PreFlop', 'Flop', 'Turn', 'River', 'Showdown', 'Completed', 'Cancelled'] },
        PlayerAction: { _enum: ['Fold', 'Check', 'Call', 'Raise', 'AllIn', 'RequestTimeout'] },
        Balance: 'u128',
      }
    });
  }

  async createGame(mnemonic: string, configId: string, minPlayers: number, maxPlayers: number, buyIn: bigint, timeoutBlocks: number): Promise<string> {
    const account = this.keyring.addFromMnemonic(mnemonic);
    const tx = this.api.tx.poker.createGame(minPlayers, maxPlayers, buyIn, configId, timeoutBlocks);
    const result = await tx.signAndSend(account);
    return result.toHex();
  }

  async joinGame(mnemonic: string, gameId: string, buyIn: bigint): Promise<boolean> {
    const account = this.keyring.addFromMnemonic(mnemonic);
    const tx = this.api.tx.poker.joinGame(gameId, buyIn);
    const result = await tx.signAndSend(account);
    return result.status.isFinalized;
  }

  async submitAction(mnemonic: string, gameId: string, action: string, amount?: bigint): Promise<string> {
    const account = this.keyring.addFromMnemonic(mnemonic);
    const tx = this.api.tx.poker.playerAction(gameId, action, amount || null, null);
    const result = await tx.signAndSend(account);
    return result.toHex();
  }

  async getGameState(gameId: string): Promise<GameState> {
    const state = await this.api.query.poker.games(gameId);
    return state.toJSON() as GameState;
  }

  async subscribeGameEvents(gameId: string, callback: (event: GameEvent) => void): Promise<() => void> {
    return await this.api.query.poker.events(gameId, (events) => {
      for (const event of events) {
        callback(event.toJSON() as GameEvent);
      }
    });
  }
}
```

---

## 5. Security Specifications

### 5.1 Slashing Conditions and Penalties

| Condition | Slash Percentage | Trigger |
|-----------|------------------|---------|
| DoubleSign | 100% | Signed two blocks at same height |
| Unavailability | 10% | Extended offline (>24 hours) |
| InvalidTimestamp | 5% | Deviation > threshold |
| GameManipulation | 100% | Any game cheating |
| Equivocation | 100% | Contradictory finality votes |

### 5.2 Key Security Requirements

| Key Type | Security Level | Storage | Rotation |
|----------|----------------|---------|----------|
| Validator Consensus | HSM Required | AWS KMS / CloudHSM | 90 days |
| BLS Key Share | HSM Required | AWS KMS / CloudHSM | 30 days |
| TLS Certificate | Encrypted at rest | Certificate Manager | 365 days |
| Session Key | Software | Memory | Per session |

### 5.3 HSM Integration

Supported HSM providers:
- AWS CloudHSM
- AWS KMS
- Azure Key Vault
- Google Cloud HSM
- Cloudflare KV

### 5.4 Attack Mitigation Strategies

| Attack Type | Mitigation |
|-------------|------------|
| Sybil Attack | Identity verification, stake requirements |
| Eclipse Attack | Multi-subnet connections, peer rotation |
| DoS | Rate limiting, connection limits |
| Transaction Replay | Nonce tracking, chain ID |
| Front-Running | Threshold signatures for ordering |

---

## 6. Performance Specifications

### 6.1 Target Block Time

| Metric | Target | Threshold |
|--------|--------|-----------|
| Block Time | 1.0 - 2.0 seconds | < 3.0 seconds |
| Block Production Rate | 0.5 - 1.0 blocks/second | > 0.3 blocks/second |
| Empty Block Rate | < 5% | < 10% |

### 6.2 Transaction Throughput

| Transaction Type | Target TPS | Burst TPS |
|-----------------|------------|-----------|
| Player Action | 500 | 1,000 |
| Game Creation | 10 | 50 |
| Card Commitment | 200 | 500 |
| Timeout Request | 100 | 200 |
| Balance Transfer | 1,000 | 2,000 |
| **Total System** | **1,000 - 2,000** | **5,000** |

### 6.3 Latency Requirements

| Operation | P50 | P90 | P99 |
|-----------|------|------|------|
| Transaction Submit | 50ms | 150ms | 300ms |
| Block Inclusion | 1.5s | 2.0s | 3.0s |
| Finality | 3.0s | 5.0s | 8.0s |
| Game Action | 100ms | 250ms | 500ms |
| Timeout Consensus | 500ms | 1.0s | 2.0s |
| ZK Proof Verify | 150ms | 300ms | 500ms |
| State Query | 10ms | 50ms | 100ms |

### 6.4 Storage Growth Projections

| Data Type | Daily Growth | 1 Year | 5 Years |
|-----------|--------------|--------|---------|
| Block Data | 1 GB | 365 GB | 1.8 TB |
| Game State | 100 MB | 36 GB | 180 GB |
| Event Log | 500 MB | 182 GB | 912 GB |
| Merkle Proofs | 50 MB | 18 GB | 91 GB |
| **On-Chain Total** | **~1.6 GB** | **584 GB** | **2.9 TB** |
| Game Archives | 200 MB | 73 GB | 365 GB |
| **Off-Chain Total** | **~300 MB** | **110 GB** | **548 GB** |

---

## 7. Deployment Specifications

### 7.1 Validator Configuration

**Hardware Requirements per Validator:**
- CPU: 4 cores (8 cores recommended)
- Memory: 16 GB (32 GB recommended)
- Storage: 500 GB SSD (1 TB recommended)
- Network: 1 Gbps, latency < 100ms inter-validator

**HSM Requirements:**
- AWS KMS or CloudHSM
- Dedicated key per validator
- Automatic key rotation enabled

### 7.2 Network Topology

**Geographic Distribution:**
| Region | Validator | RPC Node | Purpose |
|--------|-----------|----------|---------|
| us-east-1 | Validator 1 | RPC Node 1 | US primary |
| eu-west-1 | Validator 2 | RPC Node 2 | EU primary |
| ap-southeast-1 | Validator 3 | RPC Node 3 | Asia primary |
| us-west-2 | Validator 4 | RPC Node 4 | US secondary |

### 7.3 Monitoring Requirements

**Critical Alerts:**
| Alert | Condition | Severity |
|-------|-----------|----------|
| BlockProductionStalled | < 0.1 blocks/second for 5m | Critical |
| ValidatorParticipationLow | < 75% participation for 2m | Warning |
| FinalityDelay | > 10 seconds for 1m | Critical |
| PeerCountLow | < 3 peers for 2m | Warning |
| NetworkErrorsHigh | > 0.1 errors/second | Warning |

---

## 8. Testing Specifications

### 8.1 Unit Test Requirements

| Component | Coverage Target | Key Tests |
|-----------|-----------------|-----------|
| Poker Pallet | > 90% | State machine transitions, action validation |
| BLS Pallet | > 95% | Signature aggregation, verification |
| DKG Pallet | > 90% | Key generation, resharing |
| Timestamp Pallet | > 90% | HLC updates, consensus |
| Network | > 85% | Message propagation, gossip |
| Storage | > 90% | CRUD operations, pruning |

### 8.2 Integration Test Scenarios

| Scenario | Description | Pass Criteria |
|----------|-------------|---------------|
| Full Game Flow | Create game, join, play, showdown | All actions execute correctly |
| Timeout Consensus | Timeout request through finality | All validators agree |
| DKG Ceremony | New key generation | All shares generated, public key published |
| BLS Threshold | Partial signatures -> aggregate | Valid threshold signature produced |
| Network Partition | Simulate partition | Safety maintained, liveness recovers |
| Validator Recovery | Validator goes offline then back | State syncs correctly |

### 8.3 Performance Benchmarks

| Benchmark | Target | Pass Criteria |
|-----------|--------|---------------|
| Block Production | 1 block/second | P99 latency < 2s |
| Transaction Throughput | 1000 TPS | No dropped transactions |
| Finality | < 5 seconds | P99 < 8s |
| Game Creation | 100ms | P99 < 500ms |
| ZK Proof Verify | 200ms | P99 < 500ms |

### 8.4 Security Testing Requirements

| Test Type | Frequency | Scope |
|-----------|-----------|-------|
| Penetration Testing | Quarterly | Full stack |
| Key Ceremony Audit | Per ceremony | DKG process |
| Slashing Simulation | Monthly | All conditions |
| Formal Verification | Per major release | Critical contracts |
| Chaos Engineering | Monthly | Network partitions |

---

## 9. Implementation Milestones

### Phase Breakdown

| Phase | Duration | Deliverables | Dependencies |
|-------|----------|--------------|--------------|
| **Phase 1: Foundation** | Weeks 1-8 | Substrate node setup, validator configuration, basic monitoring | None |
| **Phase 2: Game Framework** | Weeks 9-16 | Poker pallet MVP, event sourcing, game state machine | Phase 1 |
| **Phase 3: Cryptography** | Weeks 17-20 | BLS DKG, threshold signatures, HSM integration | Phase 2 |
| **Phase 4: Privacy** | Weeks 21-24 | ZK proofs, card commitments, hand verification | Phase 3 |
| **Phase 5: API & SDK** | Weeks 25-28 | gRPC/REST/WebSocket APIs, client SDKs | Phase 4 |
| **Phase 6: Hardening** | Weeks 29-32 | Security audit, load testing, bug bounty | Phase 5 |
| **Phase 7: Staging** | Weeks 33-36 | Staging deployment, user testing, documentation | Phase 6 |
| **Phase 8: Production** | Weeks 37-40 | Production launch, monitoring, community growth | Phase 7 |

### Key Milestones

| Milestone | Target Week | Criteria |
|-----------|-------------|----------|
| First Block Produced | Week 4 | Network produces blocks |
| Basic Poker Game Works | Week 12 | Full game flow functional |
| DKG Complete | Week 18 | Threshold keys generated |
| ZK Proofs Working | Week 22 | Card commitments verified |
| API Stable | Week 26 | SDKs published |
| Security Audit Passed | Week 30 | No critical findings |
| Production Launch | Week 40 | Live network |

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
| HSM | Hardware Security Module - secure key storage |
| ZK | Zero-Knowledge - proof without revealing information |

### 10.2 Reference Documents

1. Substrate Documentation: https://docs.substrate.io
2. Polkadot Wiki: https://wiki.polkadot.network
3. libp2p Documentation: https://docs.libp2p.io
4. arkworks Cryptography: https://arkworks.rs
5. BLS Threshold Library: https://github.com/KZen-networks/threshold
6. Halo2 ZK Proofs: https://zcash.github.io/halo2/
7. AWS KMS Documentation: https://docs.aws.amazon.com/kms/

### 10.3 Version History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-12-31 | Riddler | Initial release |

---

**Document Version:** 1.0
**Status:** Approved for Implementation
**Next Review:** Upon Phase 6 completion
**Classification:** Technical - Public
