//! DKG Pallet - Distributed Key Generation
//!
//! Handles threshold key ceremony for BLS signatures.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: weights::WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn ceremony_status)]
    pub type CeremonyStatus<T: Config> = StorageValue<_, CeremonyState, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CeremonyStarted { epoch: u64 },
        ShareReceived { validator: T::AccountId, round: u8 },
        CeremonyCompleted { epoch: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        CeremonyNotInProgress,
        InvalidRound,
        DuplicateShare,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::start_ceremony())]
        pub fn start_ceremony(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::submit_share())]
        pub fn submit_share(origin: OriginFor<T>, round: u8, share: Vec<u8>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::complete_ceremony())]
        pub fn complete_ceremony(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Ok(())
        }
    }
}

/// DKG Ceremony State
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum CeremonyState {
    Idle,
    Round1,
    Round2,
    Round3,
    Round4,
    Completed,
}
