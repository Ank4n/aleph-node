//! Autogenerated weights for pallet_feature_control
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-07, STEPS: `20`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("./benchmark-chainspec.json"), DB CACHE: 1024

// Executed Command:
// target/release/aleph-node
// benchmark
// pallet
// --chain=./benchmark-chainspec.json
// --pallet=pallet_feature_control
// --extrinsic=*
// --steps=20
// --repeat=5
// --template=.maintain/pallet-weight-template.hbs
// --wasm-execution=compiled
// --output=pallets/feature-control/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_feature_control.
pub trait WeightInfo {
    /// Weight of the `enable` extrinsic.
    fn enable() -> Weight;
    /// Weight of the `disable` extrinsic.
    fn disable() -> Weight;
}

impl<I: BenchmarkInfo> WeightInfo for I {
    fn enable() -> Weight {
        <I as BenchmarkInfo>::enable()
    }
    
    fn disable() -> Weight {
        <I as BenchmarkInfo>::disable()
    }
}

/// Benchmark results for pallet_feature_control.
trait BenchmarkInfo {
    fn enable() -> Weight;
    fn disable() -> Weight;
}

/// Weights for pallet_feature_control using the Substrate node and recommended hardware.
pub struct AlephWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> BenchmarkInfo for AlephWeight<T> {
    // Storage: `FeatureControl::ActiveFeatures` (r:0 w:1)
    // Proof: `FeatureControl::ActiveFeatures` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn enable() -> Weight {
        // Minimum execution time: 7_723 nanoseconds.
        Weight::from_parts(8_817_000_u64, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    // Storage: `FeatureControl::ActiveFeatures` (r:0 w:1)
    // Proof: `FeatureControl::ActiveFeatures` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn disable() -> Weight {
        // Minimum execution time: 7_178 nanoseconds.
        Weight::from_parts(7_537_000_u64, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests
impl BenchmarkInfo for () {
    // Storage: `FeatureControl::ActiveFeatures` (r:0 w:1)
    // Proof: `FeatureControl::ActiveFeatures` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn enable() -> Weight {
        // Minimum execution time: 7_723 nanoseconds.
        Weight::from_parts(8_817_000_u64, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    // Storage: `FeatureControl::ActiveFeatures` (r:0 w:1)
    // Proof: `FeatureControl::ActiveFeatures` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn disable() -> Weight {
        // Minimum execution time: 7_178 nanoseconds.
        Weight::from_parts(7_537_000_u64, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}