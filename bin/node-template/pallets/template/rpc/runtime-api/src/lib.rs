// Copyright 2020 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::prelude::*;

use sp_core::H256;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::MultiSignature;
use pallet_template::MyStruct;
#[cfg(feature = "std")]
use sp_std::prelude::Vec;
pub type Signature = MultiSignature;
pub type AccountId =
  <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
  pub trait TemplateModuleApi {
    fn get_details_runtime(id: Vec<u8>) -> MyStruct<AccountId>;
   }
}
