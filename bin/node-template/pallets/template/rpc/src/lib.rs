// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use template_rpc_runtime_api::TemplateModuleApi as TemplateModuleRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::H256;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use pallet_template::MyStruct;

#[rpc]
pub trait TemplateModuleApi<BlockHash> {
  #[rpc(name = "templateModule_getDetails")]
  fn get_details(
      &self,
      id: Vec<u8>,
  ) -> Result<MyStruct<AccountId>>;
}

/// A struct that implements the ModuleApi.
pub struct TemplateModule<C, M> {
  client: Arc<C>,
  _marker: std::marker::PhantomData<M>,
}

impl<C, M> TemplateModule<C, M> {
  /// Create new `TemplateModule` instance with the given reference to the client.
  pub fn new(client: Arc<C>) -> Self {
    Self {
      client,
      _marker: Default::default(),
    }
  }
}

use sp_runtime::traits::{IdentifyAccount, Verify};
/// Error type of this RPC api.
use sp_runtime::MultiSignature;
pub type Signature = MultiSignature;
pub type AccountId =
  <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl<C, Block> TemplateModuleApi<<Block as BlockT>::Hash>
  for TemplateModule<C, Block>
// TODO: if you have more generics, no need to add <M, N, P, ..>, just do KYCModule<C, (tuple_of_all_useless_phantom_generics)>
where
  Block: BlockT,
  C: Send + Sync + 'static,
  C: ProvideRuntimeApi<Block>,
  C: HeaderBackend<Block>,
  // NOTE: this is always generic over block, even if you don't define it. Macro crap.
  // if you add more generics, then it becomes <Block, Foo, Bar, Baz>
  C::Api: TemplateModuleRuntimeApi<Block>,
{
  fn get_details(
    &self,
    //    at: Option<<Block as BlockT>::Hash>,
    id:  Vec<u8>,
  ) -> Result<MyStruct<AccountId>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(self.client.info().best_hash);

    // Our actual call to template-runtime-api
    let runtime_api_result = api.get_details_runtime(&at, id);

    runtime_api_result.map_err(|e| RpcError {
      code: ErrorCode::ServerError(9876), // No real reason for this value
      message: "Failed while getting details of MyStruct".into(),
      data: Some(format!("{:?}", e).into()),
    })
  }
}
