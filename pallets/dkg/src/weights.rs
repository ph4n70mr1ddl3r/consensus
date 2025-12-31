//! Weights for pallet-dkg

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn start_ceremony() -> Weight;
    fn submit_share() -> Weight;
    fn complete_ceremony() -> Weight;
}

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
