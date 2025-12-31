//! Poker Game Logic Pallet
//!
//! Implements poker game state machine and rules.

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
    #[pallet::getter(fn games)]
    pub type Games<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], Game<T::AccountId>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        GameCreated { game_id: [u8; 32] },
        PlayerJoined { game_id: [u8; 32], player: T::AccountId },
        GameStarted { game_id: [u8; 32] },
        ActionSubmitted { game_id: [u8; 32], player: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        GameNotFound,
        GameFull,
        NotPlayersTurn,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_game())]
        pub fn create_game(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::join_game())]
        pub fn join_game(origin: OriginFor<T>, game_id: [u8; 32]) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::submit_action())]
        pub fn submit_action(origin: OriginFor<T>, game_id: [u8; 32], action: u8) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }
    }
}

/// Game structure
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
pub struct Game<AccountId> {
    pub id: [u8; 32],
    pub owner: AccountId,
    pub players: Vec<AccountId>,
    pub status: u8, // 0=waiting, 1=started, 2=ended
    pub min_stake: u128,
}
