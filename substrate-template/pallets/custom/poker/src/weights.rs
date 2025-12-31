//! Weights for pallet-poker

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_poker.
pub trait WeightInfo {
	fn create_game() -> Weight;
	fn join_game() -> Weight;
	fn submit_action() -> Weight;
}

/// Weights for pallet_poker using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_game() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn join_game() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_action() -> Weight {
		Weight::from_parts(50_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_game() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn join_game() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_action() -> Weight {
		Weight::from_parts(50_000, 0)
	}
}
