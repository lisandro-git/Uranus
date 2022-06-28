use openssl::{
    rsa::{Rsa, Padding},
    pkey::{Private, Public}
};
use std::{
    fs::File,
    io::{Read, Write}
};

fn read_public_key() -> Rsa<Public> {
    let mut public_key = File::open("../key_files/c2_bot_public.key").unwrap();
    let mut file_content = String::new();
    public_key.read_to_string(&mut file_content);

    return Rsa::public_key_from_pem(&file_content.as_bytes()).unwrap();
}

pub fn encrypt_message_rsa(serialized_data: Vec<u8>) -> Vec<u8> {
    let public_key = read_public_key();
    let mut ciphertext: Vec<u8> = vec![0; public_key.size() as usize];
    public_key.public_encrypt(serialized_data.as_slice(), &mut ciphertext, Padding::PKCS1).unwrap();

    return ciphertext;
}

fn read_private_key() -> Rsa<Private> {
    let mut private_key = File::open("../key_files/c2_bot_private.key").unwrap();
    let mut file_content = String::new();
    private_key.read_to_string(&mut file_content);

    return Rsa::private_key_from_pem(file_content.as_bytes()).unwrap();
}

pub fn decrypt_message_rsa(data: Vec<u8>) -> Vec<u8> {
    let private_key = read_private_key();
    let mut ciphertext: Vec<u8> = vec![0; private_key.size() as usize];
    private_key.private_decrypt(&data, &mut ciphertext, Padding::PKCS1).unwrap();

    return ciphertext;
}
