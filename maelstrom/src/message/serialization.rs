use serde::Serialize;
use crate::communication::c2::Bot;
use rmp_serde::{Deserializer, Serializer};

pub fn serialize_bincode<T: serde::Serialize>(B: &T) -> Vec<u8> {
    return bincode::serialize(&B).unwrap();
}

pub fn deserialize_bincode(B: &Vec<u8>) -> Bot {
    return bincode::deserialize(&B).unwrap();
}

pub fn serialize_rmp<T: Serialize>(B: &T) -> Vec<u8>{
    return rmp_serde::to_vec(&B).unwrap();
}

pub fn deserialize_rmp(data: Vec<u8>) -> Bot {
    return rmp_serde::from_read(data.as_slice()).unwrap();
}
