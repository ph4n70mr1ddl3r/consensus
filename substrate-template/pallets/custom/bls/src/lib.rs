//! BLS Threshold Signature Pallet
//!
//! Provides BLS12-381 threshold signature capabilities.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

pub mod weights;

pub use pallet::*;

/// BLS12-381 Public Key
pub type PublicKey = [u8; 48];

/// BLS12-381 Signature
pub type Signature = [u8; 96];

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
    #[pallet::getter(fn public_key)]
    pub type CurrentPublicKey<T: Config> = StorageValue<_, PublicKey, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PublicKeyUpdated { epoch: u64, public_key: PublicKey },
        DKGInitiated { epoch: u64 },
        PartialSignatureReceived { validator: T::AccountId },
        ThresholdSignatureProduced { epoch: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        PublicKeyNotInitialized,
        InvalidSignatureShare,
        InsufficientShares,
        KeyEpochMismatch,
        CeremonyInProgress,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::initiate_ceremony())]
        pub fn initiate_ceremony(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::set_public_key())]
        pub fn set_public_key(origin: OriginFor<T>, public_key: PublicKey) -> DispatchResult {
            ensure_root(origin)?;
            CurrentPublicKey::<T>::put(public_key);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::submit_partial_signature())]
        pub fn submit_partial_signature(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::produce_threshold_signature())]
        pub fn produce_threshold_signature(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }
    }
}
