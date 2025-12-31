//! Weights for pallet-dkg

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dkg.
pub trait WeightInfo {
	fn start_ceremony() -> Weight;
	fn submit_share() -> Weight;
	fn complete_ceremony() -> Weight;
}

/// Weights for pallet_dkg using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn start_ceremony() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_share() -> Weight {
		Weight::from_parts(200_000, 0)
	}

	fn complete_ceremony() -> Weight {
		Weight::from_parts(150_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn start_ceremony() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_share() -> Weight {
		Weight::from_parts(200_000, 0)
	}

	fn complete_ceremony() -> Weight {
		Weight::from_parts(150_000, 0)
	}
}
