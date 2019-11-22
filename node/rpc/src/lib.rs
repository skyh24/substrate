// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! A collection of node-specific RPC methods.
//!
//! Since `substrate` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `substrate-rpc` crate.
//! It means that `core/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what `SRML` modules
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding `SRML` RPC definitions.

#![warn(missing_docs)]

use std::sync::Arc;

use node_primitives::{Block, AccountId, Index, Balance};
use node_runtime::UncheckedExtrinsic;
use sr_primitives::traits::ProvideRuntimeApi;
use txpool_api::TransactionPool;

/// Instantiate all RPC extensions for full node.
pub fn create_full<C, P, M>(client: Arc<C>, pool: Arc<P>) -> jsonrpc_core::IoHandler<M> where
	C: ProvideRuntimeApi,
	C: client::blockchain::HeaderBackend<Block>,
	C: Send + Sync + 'static,
	C::Api: srml_system_rpc::AccountNonceApi<Block, AccountId, Index>,
	C::Api: srml_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance>,
	C::Api: srml_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance, UncheckedExtrinsic>,
	P: TransactionPool + 'static,
	M: jsonrpc_core::Metadata + Default,
{
	use srml_system_rpc::{FullSystem, SystemApi};
	use srml_contracts_rpc::{Contracts, ContractsApi};
	use srml_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool))
	);
	io.extend_with(
		ContractsApi::to_delegate(Contracts::new(client.clone()))
	);
	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client))
	);
	io
}

/// Instantiate all RPC extensions for light node.
pub fn create_light<C, P, M, F>(
	client: Arc<C>,
	remote_blockchain: Arc<dyn client::light::blockchain::RemoteBlockchain<Block>>,
	fetcher: Arc<F>,
	pool: Arc<P>,
) -> jsonrpc_core::IoHandler<M>
	where
		C: ProvideRuntimeApi,
		C: client::blockchain::HeaderBackend<Block>,
		C: Send + Sync + 'static,
		C::Api: srml_system_rpc::AccountNonceApi<Block, AccountId, Index>,
		P: TransactionPool + 'static,
		M: jsonrpc_core::Metadata + Default,
		F: client::light::fetcher::Fetcher<Block> + 'static,
{
	use srml_system_rpc::{LightSystem, SystemApi};

	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		SystemApi::<AccountId, Index>::to_delegate(LightSystem::new(client, remote_blockchain, fetcher, pool))
	);
	io
}
