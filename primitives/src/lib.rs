// This file is part of Astar.

// Copyright (C) Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

//! Core Astar types.
//!
//! These core Astar types are used by the Shiden, Shibuya, Astar and Local runtime.
pub mod xcm;

/// Checked Ethereum transaction primitives.
pub mod ethereum_checked;

/// EVM primitives.
pub mod evm;

/// Precompiles
pub mod precompiles;

/// dApp staking & inflation primitives.
pub mod dapp_staking;

/// Useful primitives for testing.
pub mod testing;

/// Oracle & price primitives.
pub mod oracle;

/// Governance primitives.
pub mod governance;

/// Genesis generation helpers & primitives.
pub mod genesis;

/// Parachain related constants.
pub mod parachain;

/// Benchmark primitives
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarks;

use frame_support::migrations::{FailedMigrationHandler, FailedMigrationHandling};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
};

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = sp_runtime::MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Balance of an account.
pub type Balance = u128;
/// An index to a block.
pub type BlockNumber = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Id used for identifying assets.
///
/// AssetId allocation:
/// [1; 2^32-1]     Custom user assets (permissionless)
/// [2^32; 2^64-1]  Statemine assets (simple map)
/// [2^64; 2^128-1] Ecosystem assets
/// 2^128-1         Relay chain token (KSM)
pub type AssetId = u128;
/// Block type.
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
/// Index of a transaction in the chain.
pub type Nonce = u32;

/// Unfreeze chain on failed migration and continue with extrinsic execution.
/// Migration must be tested and make sure it doesn't fail. If it happens, we don't have other
/// choices but unfreeze chain and continue with extrinsic execution.
pub struct UnfreezeChainOnFailedMigration;
impl FailedMigrationHandler for UnfreezeChainOnFailedMigration {
    fn failed(migration: Option<u32>) -> FailedMigrationHandling {
        log::error!(target: "mbm", "Migration failed at cursor: {migration:?}");
        FailedMigrationHandling::ForceUnstuck
    }
}

/// Currently used MAX_POV_SIZE on Polkadot & Kusama is 10 MiB.
/// At the moment of adding this constant, we're using `stable2412` which still has 5MiB limit.
///
/// To prevent excessive increase in gas used by ethereum transactions, we'll define this temporary
/// constant for the correct PoV limit. After the next uplift, this should be removed & replaced with value from polkadot-sdk.
///
// For reference: https://github.com/paritytech/polkadot-sdk/blob/f6cd17e550caeaa1b8184b5f3135ca21f2cb16eb/polkadot/primitives/src/v8/mod.rs#L455
pub const MAX_POV_SIZE: u32 = 10 * 1024 * 1024;
