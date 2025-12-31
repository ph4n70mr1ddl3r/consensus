//! Weights for pallet-timestamp-consensus

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_timestamp_consensus.
pub trait WeightInfo {
	fn update_hlc() -> Weight;
	fn submit_timestamp_vote() -> Weight;
}

/// Weights for pallet_timestamp_consensus using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn update_hlc() -> Weight {
		Weight::from_parts(50_000, 0)
	}

	fn submit_timestamp_vote() -> Weight {
		Weight::from_parts(50_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn update_hlc() -> Weight {
		Weight::from_parts(50_000, 0)
	}

	fn submit_timestamp_vote() -> Weight {
		Weight::from_parts(50_000, 0)
	}
}
