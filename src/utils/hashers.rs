use ledger_device_sdk::hash::{ripemd::Ripemd160, sha2::Sha2_256, HashInit};

use crate::{log::debug, AppSW};

pub type Hash160 = [u8; 20];

pub fn hash160(public_key: &[u8]) -> Result<[u8; 20], AppSW> {
    let mut sha256 = Sha2_256::new();
    let mut sha256_output: [u8; 32] = [0u8; 32];
    sha256
        .hash(public_key, &mut sha256_output)
        .map_err(|_| AppSW::IncorrectData)?;

    let mut ripemd160 = Ripemd160::new();
    let mut ripemd160_output: [u8; 20] = [0u8; 20];
    ripemd160
        .hash(&sha256_output, &mut ripemd160_output)
        .map_err(|_| AppSW::IncorrectData)?;

    debug!("PubKey SHA256: {:02X?}", &sha256_output);
    debug!("PubKey HASH160: {:02X?}", &ripemd160_output);

    Ok(ripemd160_output)
}
