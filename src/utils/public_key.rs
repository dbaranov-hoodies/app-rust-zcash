use alloc::string::String;

use ledger_device_sdk::hash::{ripemd::Ripemd160, sha2::Sha2_256, HashInit};

use crate::{
    log::{debug, error},
    utils::{
        bip32_path::Bip32Path,
        hashers::{hash160, Hash160},
        output_script_is_op_return, output_script_is_regular, public_key,
    },
    AppSW,
};

pub type CompressedPublicKey = [u8; 33];

pub const TRANSPARENT_ADDRESS_B58_LEN: usize = 35;

#[derive(Clone)]
pub struct PubKeyWithCC {
    pub public_key: [u8; 65],
    pub public_key_len: usize,
    pub chain_code: [u8; 32],
}

impl PubKeyWithCC {
    pub fn public_key_slice(&self) -> &[u8] {
        &self.public_key[..self.public_key_len]
    }

    pub fn compressed_public_key(&self) -> Result<CompressedPublicKey, AppSW> {
        let public_key = self.public_key_slice();
        if public_key.len() != 65 {
            return Err(AppSW::IncorrectData);
        }
        let mut compressed_pk = [0u8; 33];
        compressed_pk[0] = if public_key[64] & 1 == 1 { 0x03 } else { 0x02 };
        compressed_pk[1..33].copy_from_slice(&public_key[1..33]);
        Ok(compressed_pk)
    }

    pub fn public_key_hash160(self) -> Result<Hash160, AppSW> {
        let mut sha256 = Sha2_256::new();
        let mut sha256_output: [u8; 32] = [0u8; 32];
        sha256
            .hash(self.public_key_slice(), &mut sha256_output)
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
}

impl TryFrom<&Bip32Path> for PubKeyWithCC {
    type Error = AppSW;

    fn try_from(path: &Bip32Path) -> Result<Self, Self::Error> {
        use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
        let (k, cc) = Secp256k1::derive_from(path.as_slice());

        let pk = k.public_key().map_err(|_| AppSW::IncorrectData)?;
        let code = cc.ok_or(AppSW::IncorrectData)?;

        let public_key = pk.pubkey;
        let public_key_len = pk.keylength;
        let chain_code = code.value;
        Ok(Self {
            public_key,
            public_key_len,
            chain_code,
        })
    }
}

// pub fn get_address_from_output_script(script: &[u8]) -> Result<String, AppSW> {
//     const COIN_P2PKH_VERSION: u16 = 7352;
//     const ADDRESS_OFFSET: usize = 3;
//     const VERSION_SIZE: usize = 2;
//     const ADDRESS_SIZE: usize = 22;

//     if output_script_is_op_return(script) {
//         error!("Unsupported OP_RETURN script");
//         return Err(AppSW::IncorrectData);
//     }

//     if !output_script_is_regular(script) {
//         error!("Unsupported script type");
//         return Err(AppSW::IncorrectData);
//     }

//     let mut address = [0u8; ADDRESS_SIZE];
//     let version = COIN_P2PKH_VERSION.to_be_bytes();

//     address[..VERSION_SIZE].copy_from_slice(&version);
//     address[VERSION_SIZE..].copy_from_slice(&script[ADDRESS_OFFSET..ADDRESS_OFFSET + 20]);

//     let bytes: [u8; TRANSPARENT_ADDRESS_B58_LEN] = public_key_to_address_base58(&address, true)?;
//     debug!("address_bytes: {:?}", &bytes);
//     let address_base58 = str::from_utf8(&bytes)
//         .map_err(|_| AppSW::ExecutionError)?
//         .into();
//     debug!("address_string: {}", &address_base58);

//     Ok(address_base58)
// }

// pub fn public_key_to_address_base58(
//     public_key: &[u8],
//     is_hashed: bool,
// ) -> Result<[u8; TRANSPARENT_ADDRESS_B58_LEN], AppSW> {
//     // buffer for deriving address
//     // PREFIX(2 bytes) + HASH160 of Public_key(20 bytes)+ checksum (4 bytes)
//     let mut buf = [0u8; 26];

//     // For Zcash, the address is the HASH160 of the public key
//     if is_hashed {
//         buf[0..22].copy_from_slice(&public_key[0..22]);
//     } else {
//         debug!("To hash: {:02X?}", &public_key);
//         let pubkey_hash160 = hash160(public_key)?;
//         buf[0] = P2PKH_PREFIX[0];
//         buf[1] = P2PKH_PREFIX[1];
//         buf[2..22].copy_from_slice(&pubkey_hash160);
//     }

//     let checksum = compute_checksum(&buf[0..22]);
//     buf[22..26].copy_from_slice(&checksum);

//     let mut address_base58 = [0u8; TRANSPARENT_ADDRESS_B58_LEN];
//     let _written = bs58::encode(&buf[..26])
//         .onto(&mut address_base58[..])
//         .map_err(|_| AppSW::IncorrectData)?;

//     //transparent addresses begin with "t" and are followed by 34 alphanumeric characters
//     debug!("address_base58 {:?}", &address_base58);
//     Ok(address_base58)
// }
