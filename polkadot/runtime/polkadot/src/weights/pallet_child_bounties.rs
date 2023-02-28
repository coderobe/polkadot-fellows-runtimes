// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_child_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_child_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_child_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_child_bounties::WeightInfo for WeightInfo<T> {
	/// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBountyCount (r:1 w:1)
	/// Proof: ChildBounties ChildBountyCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	/// Proof: ChildBounties ChildBountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:0 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 16384]`.
	fn add_child_bounty(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `609`
		//  Estimated: `10848`
		// Minimum execution time: 52_822 nanoseconds.
		Weight::from_ref_time(53_830_173)
			.saturating_add(Weight::from_proof_size(10848))
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(816).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	/// Proof: ChildBounties ChildrenCuratorFees (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn propose_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `663`
		//  Estimated: `7775`
		// Minimum execution time: 20_288 nanoseconds.
		Weight::from_ref_time(20_516_000)
			.saturating_add(Weight::from_proof_size(7775))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn accept_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841`
		//  Estimated: `7875`
		// Minimum execution time: 28_881 nanoseconds.
		Weight::from_ref_time(29_223_000)
			.saturating_add(Weight::from_proof_size(7875))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn unassign_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841`
		//  Estimated: `7875`
		// Minimum execution time: 41_165 nanoseconds.
		Weight::from_ref_time(41_719_000)
			.saturating_add(Weight::from_proof_size(7875))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	fn award_child_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `706`
		//  Estimated: `5272`
		// Minimum execution time: 22_428 nanoseconds.
		Weight::from_ref_time(22_898_000)
			.saturating_add(Weight::from_proof_size(5272))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	/// Proof: ChildBounties ChildBountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn claim_child_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `744`
		//  Estimated: `12920`
		// Minimum execution time: 72_832 nanoseconds.
		Weight::from_ref_time(73_776_000)
			.saturating_add(Weight::from_proof_size(12920))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	/// Proof: ChildBounties ChildrenCuratorFees (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	/// Proof: ChildBounties ChildBountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn close_child_bounty_added() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `973`
		//  Estimated: `15472`
		// Minimum execution time: 55_400 nanoseconds.
		Weight::from_ref_time(55_912_000)
			.saturating_add(Weight::from_proof_size(15472))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Bounties Bounties (r:1 w:0)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBounties (r:1 w:1)
	/// Proof: ChildBounties ChildBounties (max_values: None, max_size: Some(145), added: 2620, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	/// Proof: ChildBounties ChildrenCuratorFees (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:1)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildBountyDescriptions (r:0 w:1)
	/// Proof: ChildBounties ChildBountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn close_child_bounty_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1192`
		//  Estimated: `18075`
		// Minimum execution time: 67_548 nanoseconds.
		Weight::from_ref_time(69_397_000)
			.saturating_add(Weight::from_proof_size(18075))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
