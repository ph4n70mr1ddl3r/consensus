//! Timestamp Consensus Pallet with HLC
//!
//! Provides Hybrid Logical Clock for fair timestamping.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

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
    #[pallet::getter(fn current_hlc)]
    pub type CurrentHLC<T: Config> = StorageValue<_, HLC, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        HLCUpdated { physical: u64, logical: u16 },
        TimestampVote { validator: T::AccountId, timestamp: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidTimestamp,
        DeviationTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::update_hlc())]
        pub fn update_hlc(origin: OriginFor<T>, physical: u64, logical: u16) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::submit_timestamp_vote())]
        pub fn submit_timestamp_vote(origin: OriginFor<T>, timestamp: u64) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }
    }
}

/// Hybrid Logical Clock structure
#[derive(Encode, Decode, Clone, Copy, PartialEq, RuntimeDebug)]
pub struct HLC {
    pub physical: u64,
    pub logical: u16,
    pub validator_id: u32,
}

impl HLC {
    pub fn now(validator_id: u32) -> Self {
        Self {
            physical: sp_io::timestamp::Timestamp::now().milliseconds(),
            logical: 0,
            validator_id,
        }
    }
}
