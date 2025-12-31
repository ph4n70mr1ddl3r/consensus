//! Weights for pallet-timestamp-consensus

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn update_hlc() -> Weight;
    fn submit_timestamp_vote() -> Weight;
}

impl WeightInfo for () {
    fn update_hlc() -> Weight {
        Weight::from_parts(50_000, 0)
    }

    fn submit_timestamp_vote() -> Weight {
        Weight::from_parts(50_000, 0)
    }
}
