use std::str::from_utf8;
use rmp_serde::{Deserializer, Serializer};
use crate::communication::encryption;
use crate::morse;
use super::bot;

fn remove_trailing_zeros(data: Vec<u8>) -> Vec<u8> {
    // Used to remove the zeros at the end of the received encrypted message
    // but not inside the message (purpose of the 'keep_push' var

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
    //res.push(0)
    return res.to_owned();
}

pub fn serialize_data(B: &bot::Bot) -> Vec<u8>{
    return rmp_serde::to_vec(&B).unwrap();
}

pub fn deserialize_message(data: Vec<u8>) -> bot::Bot {
    return rmp_serde::from_read(data.as_slice()).unwrap();
}

pub fn deobfuscate_data(morse_code: Vec<u8>) -> Vec<u8> {
    let base32_encoding = base32::Alphabet::RFC4648 { padding: true };
    let base32_data = morse::morse_to_word::decode(remove_trailing_zeros(morse_code));
    let encrypted_data = base32::decode(
        base32_encoding,
        from_utf8(base32_data.as_slice()).unwrap()).unwrap();
    return remove_trailing_zeros(encryption::decrypt_message(encrypted_data));
}
