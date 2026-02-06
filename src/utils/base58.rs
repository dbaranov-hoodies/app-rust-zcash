use crate::{
    log::debug,
    utils::{
        hashers::{hash160, Hash160},
        output_script_is_op_return, output_script_is_regular,
    },
    AppSW,
};

pub const TRANSPARENT_ADDRESS_B58_LEN: usize = 35;

pub struct Base58Address {
    pub bytes: [u8; TRANSPARENT_ADDRESS_B58_LEN],
    pub len: usize,
}

fn p2pkh_payload_to_base58_bytes(payload: &[u8; 22]) -> Result<Base58Address, AppSW> {
    let mut buf = [0u8; 26];

    // payload
    buf[..22].copy_from_slice(payload);

    // checksum
    let checksum = compute_checksum(&buf[..22]);
    buf[22..26].copy_from_slice(&checksum);

    // base58
    let mut out = [0u8; TRANSPARENT_ADDRESS_B58_LEN];
    let written = bs58::encode(&buf)
        .onto(&mut out[..])
        .map_err(|_| AppSW::IncorrectData)?;

    Ok(Base58Address {
        bytes: out,
        len: written,
    })
}

fn compute_checksum(input: &[u8]) -> [u8; 4] {
    use ledger_device_sdk::hash::{sha2::Sha2_256, HashInit};

    let mut sha256 = Sha2_256::new();
    let mut sha256_output: [u8; 32] = [0u8; 32];
    sha256.hash(input, &mut sha256_output).unwrap();

    let mut sha256_2 = Sha2_256::new();
    let mut sha256_2_output: [u8; 32] = [0u8; 32];
    sha256_2.hash(&sha256_output, &mut sha256_2_output).unwrap();

    debug!("Checksum: {:02X?}", &sha256_2_output[0..4]);

    [
        sha256_2_output[0],
        sha256_2_output[1],
        sha256_2_output[2],
        sha256_2_output[3],
    ]
}

pub fn get_address_from_output_script(script: &[u8]) -> Result<Base58Address, AppSW> {
    const COIN_P2PKH_VERSION: u16 = 7352;

    let payload = output_script_to_p2pkh_payload(script, COIN_P2PKH_VERSION)?;
    p2pkh_payload_to_base58_bytes(&payload)
}

fn output_script_to_p2pkh_payload(script: &[u8], version: u16) -> Result<[u8; 22], AppSW> {
    const ADDRESS_OFFSET: usize = 3;

    if output_script_is_op_return(script) {
        return Err(AppSW::IncorrectData);
    }

    if !output_script_is_regular(script) {
        return Err(AppSW::IncorrectData);
    }

    let mut payload = [0u8; 22];
    payload[..2].copy_from_slice(&version.to_be_bytes());
    payload[2..].copy_from_slice(&script[ADDRESS_OFFSET..ADDRESS_OFFSET + 20]);

    Ok(payload)
}

// T-address P2PKH prefix (mainnet)
const P2PKH_PREFIX: [u8; 2] = [0x1C, 0xB8];
// T-address P2PKH prefix (testnet)
const _P2PKH_PREFIX: [u8; 2] = [0x1D, 0x25];

pub fn get_address_from_public_key(public_key_hash160: &Hash160) -> Result<Base58Address, AppSW> {
    // buffer for deriving address
    // PREFIX(2 bytes) + HASH160 of Public_key(20 bytes)+ checksum (4 bytes)
    let mut buf = [0u8; 22];

    debug!("To hash: {:02X?}", &public_key_hash160);
    let pubkey_hash160 = hash160(public_key_hash160)?;
    buf[0] = P2PKH_PREFIX[0];
    buf[1] = P2PKH_PREFIX[1];
    buf[2..22].copy_from_slice(&pubkey_hash160);

    p2pkh_payload_to_base58_bytes(&buf)
}
