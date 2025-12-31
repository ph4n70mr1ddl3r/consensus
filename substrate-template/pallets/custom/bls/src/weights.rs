//! Weights for pallet-bls

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_bls.
pub trait WeightInfo {
	fn initiate_ceremony() -> Weight;
	fn set_public_key() -> Weight;
	fn submit_partial_signature() -> Weight;
	fn produce_threshold_signature() -> Weight;
}

/// Weights for pallet_bls using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn initiate_ceremony() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn set_public_key() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_partial_signature() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn produce_threshold_signature() -> Weight {
		Weight::from_parts(200_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn initiate_ceremony() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn set_public_key() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn submit_partial_signature() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn produce_threshold_signature() -> Weight {
		Weight::from_parts(200_000, 0)
	}
}
