/*****************************************************************************
 *   Ledger App Boilerplate Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

use crate::{
    app_ui::address::ui_display_pk,
    utils::{
        base58::get_address_from_public_key, bip32_path::Bip32Path, hashers::hash160,
        public_key::PubKeyWithCC,
    },
};
use crate::{log::debug, utils::public_key};

use crate::AppSW;
use ledger_device_sdk::io::Comm;

/// Handler for GET_PUBLIC_KEY APDU command.
///
/// Derives and returns the public key for a given BIP32 path, optionally
/// displaying the corresponding address on the device for user verification.
///
/// # Flow
///
/// 1. Parse BIP32 path from APDU data
/// 2. Derive public key using shared helper `get_pubkey_from_path()`
/// 3. If display requested, compute and show address on device
/// 4. Return public key and chaincode to client
///
/// # Note
///
/// This handler uses the same address derivation logic as `swap::check_address()`
/// via the shared `get_address_hash_from_pubkey()` helper, ensuring consistency.
pub fn handler_get_public_key(comm: &mut Comm, display: bool) -> Result<(), AppSW> {
    debug!("Called get public key handler");
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    let path: Bip32Path = data.try_into()?;

    debug!("path {:?}", path);
    let public_key_with_cc = PubKeyWithCC::try_from(&path)?;
    // let PubKeyWithCC {
    //     public_key,
    //     public_key_len,
    //     chain_code,
    // } = PubKeyWithCC::try_from(&path)?;
    let binding = public_key_with_cc.clone();
    let public_key = binding.public_key_slice();

    debug!("public_key {:?}", public_key);

    let compressed_key = &public_key_with_cc.clone().compressed_public_key()?;
    let h160 = hash160(compressed_key)?;

    let base58_address = get_address_from_public_key(&h160)?;
    debug!("bytes collected{:?}", &base58_address.len);
    let address_str = str::from_utf8(&base58_address.bytes).map_err(|_| AppSW::ExecutionError)?;
    debug!("address_str {:?}", address_str);
    // Display address on device if requested
    if display && !ui_display_pk(address_str)? {
        return Err(AppSW::Deny);
    }

    comm.append(&[public_key_with_cc.public_key_len as u8]);
    comm.append(public_key);

    debug!("Public Key: {:02X?}", public_key);

    let addr_len = address_str.len() as u8;
    comm.append(&[addr_len]);
    comm.append(address_str.as_bytes());

    debug!("Address: {}", address_str);

    // Don't encode chain code length, it's always 32 bytes
    comm.append(&public_key_with_cc.clone().chain_code);

    debug!("Chain Code: {:02X?}", public_key_with_cc.chain_code);

    Ok(())
}
