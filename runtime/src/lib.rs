//! Poker Consensus Engine Runtime
//!
//! This is the runtime module for the Poker Consensus Engine.
//! It composes all the pallets and provides the runtime API.

#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::{
    construct_runtime, debug, parameter_types,
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
};

pub use frame_system::EnsureRoot;
pub use sp_consensus_babe::AuthorityId as BabeAuthorityId;
pub use sp_consensus_grandpa::AuthorityId as GrandpaAuthorityId;
pub use sp_runtime::{create_runtime_str, generic, impl_opaque_keys, Perbill, Permill};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// Create the runtime by composing the pallets
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // System
        System: frame_system = 0,
        Timestamp: pallet_timestamp = 1,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip = 2,

        // Consensus
        Babe: pallet_babe = 3,
        Grandpa: pallet_grandpa = 4,
        Session: pallet_session = 5,
        ImOnline: pallet_im_online = 6,
        Offences: pallet_offences = 7,
        AuthorityDiscovery: pallet_authority_discovery = 8,

        // Transaction payment
        TransactionPayment: pallet_transaction_payment = 10,

        // Poker pallets (to be implemented)
        // Poker: pallet_poker = 20,
        // TimestampConsensus: pallet_timestamp_consensus = 21,
        // DKG: pallet_dkg = 22,
        // BLS: pallet_bls = 23,
    }
);

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = {
    authoring_version: 1,
    spec_version: 1_0_0,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const MaximumBlockWeight: Weight = WEIGHT_PER_SECOND.saturating_mul(2);
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type DbWeight = RocksDbWeight;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u32;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = Indices;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = ConstU32<0>;
    type FilterUncle = ();
    type EventHandler = ();
}

parameter_types! {
    pub const EpochDuration: u64 = EPOCH_LENGTH_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: Moment = SLOT_DURATION as Moment;
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type DisabledValidators = Session;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationProofSystem = ();
    type KeyOwnerIdentification = <Self as sp_runtime::traits::IdentifyAccount>::AccountId;
    type AccountId = AccountId;
    type MaxAuthorities = MaxAuthorities;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxAuthorities: u32 = 100;
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationProofSystem = ();
    type KeyOwnerIdentification = <Self as sp_runtime::traits::IdentifyAccount>::AccountId;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_session::ValidFromSession;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = ();
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = opaque::SessionKeys;
    type WeightInfo = ();
}

impl pallet_im_online::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AuthoritySet = Historical;
    type MaxKeys = MaxKeys;
    type MaxOffenders = MaxOffenders;
    type WeightInfo = ();
}

impl pallet_offences::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type IdentificationTuple = ();
    type OnOffenceHandler = ();
}

impl pallet_authority_discovery::Config for Runtime {
    type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1_000_000_000_000_000_000; // 1 unit
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type LengthToFee = IdentityFee<Balance>;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    RuntimeCall: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::AccountId, Self::Signature>>(
        call: RuntimeCall,
        account: AccountId,
        index: u32,
        signed_payload: sp_runtime::generic::SignedPayload<RuntimeCall, SignedExtra>,
    ) -> Option<(RuntimeCall, SignedExtra)> {
        Some((call, (account, index, signed_payload)))
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Signature = Signature;
    type SignedPayload = SignedPayload;
}

pub struct SignedExtra;

impl sp_runtime::traits::SignedExtension for SignedExtra {
    type AccountId = AccountId;
    type Call = RuntimeCall;
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }
}

type SignedPayload = sp_runtime::generic::SignedPayload<RuntimeCall, SignedExtra>;

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Balance type
pub type Balance = u128;

/// Account ID type
pub type AccountId = sp_core::crypto::AccountId32;

/// Index type
pub type Index = u32;

/// Block number type
pub type BlockNumber = u32;

/// Hash type
pub type Hash = H256;

/// Header type
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Opaque types
pub mod opaque {
    use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsicT;
    use sp_runtime::generic::Header as HeaderT;

    use super::*;

    /// Opaque block header type.
    pub type Header = HeaderT<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsicT>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
        }
    }
}

impl frame_system::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Call = RuntimeCall;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = sp_runtime::traits::IdentityLookup<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type MaxConsumers = ConstU32<16>;
}

// Implementation of Indices for multi-address accounts
pub struct Indices;

impl sp_runtime::traits::StaticLookup for Indices {
    type Source = AccountId;
    type Target = AccountId;
    fn lookup(a: &AccountId) -> Option<AccountId> {
        Some(a.clone())
    }
    fn reverse_lookup(a: &AccountId) -> Option<AccountId> {
        Some(a.clone())
    }
}

#[cfg(feature = "std")]
include!("../build.rs");
