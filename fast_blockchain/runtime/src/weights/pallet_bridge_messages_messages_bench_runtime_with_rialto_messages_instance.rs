
//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `serban-ROG-Zephyrus`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_messages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./bin/millau/runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages InboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages InboundLanes (max_values: None, max_size: Some(49208), added: 51683, mode: MaxEncodedLen)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653`
		//  Estimated: `52673`
		// Minimum execution time: 38_773_000 picoseconds.
		Weight::from_parts(41_333_000, 0)
			.saturating_add(Weight::from_parts(0, 52673))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages InboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages InboundLanes (max_values: None, max_size: Some(49208), added: 51683, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 1004]`.
	/// The range of component `n` is `[1, 1004]`.
	fn receive_n_messages_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653`
		//  Estimated: `52673`
		// Minimum execution time: 39_551_000 picoseconds.
		Weight::from_parts(22_769_841, 0)
			.saturating_add(Weight::from_parts(0, 52673))
			// Standard Error: 3_937
			.saturating_add(Weight::from_parts(7_704_895, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages InboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages InboundLanes (max_values: None, max_size: Some(49208), added: 51683, mode: MaxEncodedLen)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653`
		//  Estimated: `52673`
		// Minimum execution time: 45_162_000 picoseconds.
		Weight::from_parts(48_043_000, 0)
			.saturating_add(Weight::from_parts(0, 52673))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages InboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages InboundLanes (max_values: None, max_size: Some(49208), added: 51683, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653`
		//  Estimated: `52673`
		// Minimum execution time: 38_769_000 picoseconds.
		Weight::from_parts(41_442_733, 0)
			.saturating_add(Weight::from_parts(0, 52673))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_163, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages OutboundLanes (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundMessages (r:0 w:1)
	/// Proof: BridgeDatagenMessages OutboundMessages (max_values: None, max_size: Some(65596), added: 68071, mode: MaxEncodedLen)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `3558`
		// Minimum execution time: 37_325_000 picoseconds.
		Weight::from_parts(38_593_000, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages OutboundLanes (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundMessages (r:0 w:2)
	/// Proof: BridgeDatagenMessages OutboundMessages (max_values: None, max_size: Some(65596), added: 68071, mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `3558`
		// Minimum execution time: 38_624_000 picoseconds.
		Weight::from_parts(39_981_000, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages OutboundLanes (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: BridgeRelayers RelayerRewards (r:2 w:2)
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages OutboundMessages (r:0 w:2)
	/// Proof: BridgeDatagenMessages OutboundMessages (max_values: None, max_size: Some(65596), added: 68071, mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `6126`
		// Minimum execution time: 41_596_000 picoseconds.
		Weight::from_parts(43_115_000, 0)
			.saturating_add(Weight::from_parts(0, 6126))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: BridgeDatagenMessages PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeDatagenMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenGrandpa ImportedHeaders (r:1 w:0)
	/// Proof: BridgeDatagenGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68), added: 2048, mode: MaxEncodedLen)
	/// Storage: BridgeDatagenMessages InboundLanes (r:1 w:1)
	/// Proof: BridgeDatagenMessages InboundLanes (max_values: None, max_size: Some(49208), added: 51683, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof_with_dispatch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653`
		//  Estimated: `52673`
		// Minimum execution time: 38_958_000 picoseconds.
		Weight::from_parts(40_005_000, 0)
			.saturating_add(Weight::from_parts(0, 52673))
			// Standard Error: 626
			.saturating_add(Weight::from_parts(375_467, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
