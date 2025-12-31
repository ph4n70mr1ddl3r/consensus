//! Weights for pallet-bls

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn initiate_ceremony() -> Weight;
    fn set_public_key() -> Weight;
    fn submit_partial_signature() -> Weight;
    fn produce_threshold_signature() -> Weight;
}

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
