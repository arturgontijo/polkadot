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
//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_referenda
// --chain=kusama-dev
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	// Storage: FellowshipCollective Members (r:1 w:0)
	// Storage: FellowshipReferenda ReferendumCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: FellowshipReferenda ReferendumInfoFor (r:0 w:1)
	fn submit() -> Weight {
		Weight::from_ref_time(32_438_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_preparing() -> Weight {
		Weight::from_ref_time(48_476_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_queued() -> Weight {
		Weight::from_ref_time(94_751_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_not_queued() -> Weight {
		Weight::from_ref_time(94_975_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_passing() -> Weight {
		Weight::from_ref_time(194_931_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	fn place_decision_deposit_failing() -> Weight {
		Weight::from_ref_time(42_888_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	fn refund_decision_deposit() -> Weight {
		Weight::from_ref_time(30_949_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn cancel() -> Weight {
		Weight::from_ref_time(38_960_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn kill() -> Weight {
		Weight::from_ref_time(67_836_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda TrackQueue (r:1 w:0)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	fn one_fewer_deciding_queue_empty() -> Weight {
		Weight::from_ref_time(11_204_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_failing() -> Weight {
		Weight::from_ref_time(224_465_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_passing() -> Weight {
		Weight::from_ref_time(224_764_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_insertion() -> Weight {
		Weight::from_ref_time(91_727_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_slide() -> Weight {
		Weight::from_ref_time(91_868_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_queued() -> Weight {
		Weight::from_ref_time(94_520_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_not_queued() -> Weight {
		Weight::from_ref_time(93_483_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_no_deposit() -> Weight {
		Weight::from_ref_time(29_294_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_preparing() -> Weight {
		Weight::from_ref_time(30_363_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	fn nudge_referendum_timed_out() -> Weight {
		Weight::from_ref_time(22_703_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		Weight::from_ref_time(40_760_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		Weight::from_ref_time(91_300_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_confirming() -> Weight {
		Weight::from_ref_time(165_577_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_end_confirming() -> Weight {
		Weight::from_ref_time(166_188_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		Weight::from_ref_time(159_324_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_confirming() -> Weight {
		Weight::from_ref_time(82_615_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn nudge_referendum_approved() -> Weight {
		Weight::from_ref_time(185_354_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: FellowshipCollective MemberCount (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_rejected() -> Weight {
		Weight::from_ref_time(165_963_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
