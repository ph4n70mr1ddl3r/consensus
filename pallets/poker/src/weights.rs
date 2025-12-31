//! Weights for pallet-poker

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn create_game() -> Weight;
    fn join_game() -> Weight;
    fn submit_action() -> Weight;
}

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
