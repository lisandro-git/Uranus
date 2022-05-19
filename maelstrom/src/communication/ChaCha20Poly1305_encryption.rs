use rand::{rngs::OsRng, RngCore};
use crate::communication::bot::Device_stream;

use chacha20poly1305::{Key, Nonce, XChaCha20Poly1305, XNonce};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::aead::AeadInPlace;

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
const BUFFER_SIZE: usize = 8192;

fn generate_random_nonce() -> ([u8; XCHACHA20_POLY1305_NONCE_SIZE]) {
    let mut rand_generator = OsRng {};

    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);
    return nonce;
}

pub fn encrypt(key: &Vec<u8>, mut data: Vec<u8>) -> Vec<u8>{
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let n = generate_random_nonce();
    let nonce = XNonce::from_slice(n.as_ref());

    cipher.encrypt_in_place(nonce, b"Edode", &mut data)
        .expect("encryption failure!");

    let mut encrypted_data = nonce.as_slice().to_vec();
    encrypted_data.extend_from_slice(data.to_vec().as_ref());
    return encrypted_data.to_vec();
}

pub fn decrypt(key: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    let mut ciphertext: Vec<u8> = Vec::new();
    for i in 0..XCHACHA20_POLY1305_NONCE_SIZE {
        nonce[i] = data[i];
    }
    // get the rest of the data
    for i in XCHACHA20_POLY1305_NONCE_SIZE..data.len() {
        ciphertext.push(data[i]);
    }

    let nn = XNonce::from_slice(nonce.as_slice());
    cipher.decrypt_in_place(nn, b"Edode", &mut ciphertext)
        .expect("decryption failure!");

    return ciphertext.to_vec().to_owned();
}
