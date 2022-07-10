use std::str::from_utf8;
use openssl::{
    pkey::HasPublic,
    rsa::Rsa
};
use crate::{
    encoder,
    encryption as enc,
};

/// Removing trailing zeroes from a given Vec
pub fn remove_trailing_zeros(data: Vec<u8>) -> Vec<u8> {
    let mut transit: Vec<u8> = vec![];
    let mut res: Vec<u8> = vec![];
    let mut keep_push: bool = false;
    for d in data.iter().rev() {
        if *d == 0 && !keep_push{
            continue;
        } else {
            transit.push(*d);
            keep_push = true;
        }
    }
    for t in transit.iter().rev() {
        res.push(*t);
    }
    return res.to_owned();
}

/// Returns an Deobfuscated data from a given obfuscated data
pub fn deobfuscate_data(morse_code: Vec<u8>, authenticated: bool, ccp_key: &Vec<u8>) -> Vec<u8> {
    let base32_data = encoder::morse::decode(remove_trailing_zeros(morse_code));
    let encrypted_data = base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        from_utf8(base32_data.as_slice()).unwrap()
    ).unwrap();
    return if authenticated { // edode : ChaCha20Poly1305 decryption
        enc::ChaCha20Poly1305_encryption::decrypt(ccp_key, encrypted_data)
    } else { // edode : RSA decryption
        remove_trailing_zeros(enc::rsa_encryption::decrypt_message_rsa(encrypted_data))
    }
}

/// Returns an Obfuscated Vec from a given data
pub fn obfuscate_data(data: Vec<u8>, ccp_key: &Vec<u8>) -> Vec<u8> {
    let encrypted_data = enc::ChaCha20Poly1305_encryption::encrypt(ccp_key, data);
    let base32_data = base32::encode(
        base32::Alphabet::RFC4648 { padding: true },
        &encrypted_data
    );
    let morse_code = encoder::morse::encode(base32_data);
    return morse_code.into_bytes();
}