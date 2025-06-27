use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|_| "Hex decode error".to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut big_endian = bytes.to_vec();
    big_endian.reverse();
    big_endian
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    let bytes = num.to_le_bytes();
    let mut swapped = [0u8; 4];
    swapped.copy_from_slice(&bytes);
    swapped
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    match script {
        [0x76, 0xa9, 0x14, ..] => ScriptType::P2PKH,
        [0x00, 0x14, ..] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

// TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    if script.len() < 2 {
        return &[];
    }
    let pushdata_length = script[1] as usize;
    if script.len() < 2 + pushdata_length {
        return &[];
    }
    &script[2..2 + pushdata_length]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    if *balance >= fee {
        *balance -= fee;
    } else {
        *balance = 0; // Prevent negative balance
    }
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {txid}")
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            0x00 => Err("Invalid opcode: 0x00".to_string()),
            _ => Ok(Opcode::OpInvalid),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Clone, Debug, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo.clone() // Assuming we want to return a copy of the UTXO
}
