use chacha20poly1305::{
    Key,
    Nonce,
    XChaCha20Poly1305,
    XNonce,
    aead::{Aead, NewAead, AeadInPlace}
};
use rand::{rngs::OsRng, RngCore};

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
const BUFFER_SIZE: usize = 8192;
const AD: &[u8] = "Edode".as_bytes();

/// Generating a random nonce for the encryption key
fn generate_random_nonce() -> ([u8; XCHACHA20_POLY1305_NONCE_SIZE]) {
    let mut rand_generator = OsRng {};

    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);
    return nonce;
}

/// Returns an encrypted message using a key
pub fn encrypt(key: &Vec<u8>, mut data: Vec<u8>) -> Vec<u8>{
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let random_nonce = generate_random_nonce();
    let nonce = XNonce::from_slice(random_nonce.as_ref());

    cipher.encrypt_in_place(nonce, AD, &mut data)
        .expect("encryption failure!");

    let mut encrypted_data = nonce.as_slice().to_vec();
    encrypted_data.extend_from_slice(data.to_vec().as_ref());
    return encrypted_data.to_vec();
}

/// Returns a decrypted message using a key
pub fn decrypt(key: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    let mut ciphertext: Vec<u8> = Vec::new();
    {
        // edode : getting the nonce
        for i in 0..XCHACHA20_POLY1305_NONCE_SIZE {
            nonce[i] = data[i];
        }
        // edode : getting the ciphertext
        for i in XCHACHA20_POLY1305_NONCE_SIZE..data.len() {
            ciphertext.push(data[i]);
        }
    }

    let xn = XNonce::from_slice(nonce.as_slice());
    cipher.decrypt_in_place(xn, AD, &mut ciphertext)
        .expect("decryption failure!");

    return ciphertext.to_vec().to_owned();
}
