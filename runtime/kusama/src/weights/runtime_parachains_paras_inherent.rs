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
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_parachains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaSessionInfo Sessions (r:1 w:0)
	// Storage: ParasDisputes Disputes (r:1 w:1)
	// Storage: ParasDisputes Included (r:1 w:1)
	// Storage: ParasDisputes SpamSlots (r:1 w:1)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		// Minimum execution time: 808_425 nanoseconds.
		Weight::from_ref_time(344_310_798 as u64)
			// Standard Error: 21_870
			.saturating_add(Weight::from_ref_time(48_449_591 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(29 as u64))
			.saturating_add(T::DbWeight::get().writes(18 as u64))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_bitfields() -> Weight {
		// Minimum execution time: 325_574 nanoseconds.
		Weight::from_ref_time(332_360_000 as u64)
			.saturating_add(T::DbWeight::get().reads(26 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		// Minimum execution time: 5_648_731 nanoseconds.
		Weight::from_ref_time(876_233_814 as u64)
			// Standard Error: 46_671
			.saturating_add(Weight::from_ref_time(48_048_616 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(29 as u64))
			.saturating_add(T::DbWeight::get().writes(16 as u64))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:0)
	// Storage: Paras UpgradeRestrictionSignal (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		// Minimum execution time: 38_109_414 nanoseconds.
		Weight::from_ref_time(38_345_799_000 as u64)
			.saturating_add(T::DbWeight::get().reads(31 as u64))
			.saturating_add(T::DbWeight::get().writes(16 as u64))
	}
}
