use openssl::rsa::{Rsa, Padding};
use std::fs::File;
use std::io::{Read, Write};
use openssl::pkey::{Private, Public};

fn read_public_key() -> Rsa<Public> {
    let mut public_key = File::open("../key_files/c2_bot_public.key").unwrap();
    let mut file_content = String::new();
    public_key.read_to_string(&mut file_content);

    return Rsa::public_key_from_pem(&file_content.as_bytes()).unwrap();
}

pub fn encrypt_message_rsa(serialized_data: Vec<u8>) -> Vec<u8> {
    let public_key = read_public_key();
    let mut buf: Vec<u8> = vec![0; public_key.size() as usize];
    let _ = public_key.public_encrypt(serialized_data.as_slice(), &mut buf, Padding::PKCS1).unwrap();

    return buf;
}

fn read_private_key() -> Rsa<Private> {
    let mut private_key = File::open("../key_files/c2_bot_private.key").unwrap();
    let mut file_content = String::new();
    private_key.read_to_string(&mut file_content);

    return Rsa::private_key_from_pem(file_content.as_bytes()).unwrap();
}

pub fn decrypt_message_rsa(data: Vec<u8>) -> Vec<u8> {
    let private_key = read_private_key();
    let mut buf: Vec<u8> = vec![0; private_key.size() as usize];
    let _ = private_key.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();

    return buf.to_owned();
}
