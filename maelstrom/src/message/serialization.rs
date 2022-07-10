use serde::Serialize;
//use rmp_serde::{Deserializer, Serializer};
use crate::communication::c2::Bot;

/// Bincode Serialization of a given data that has a serde::Serialize trait
pub fn serialize_bincode<T: serde::Serialize>(B: &T) -> Vec<u8> {
    return bincode::serialize(&B).unwrap();
}

/// Bincode deserialization of a Vec to a Bot struct
pub fn deserialize_bincode(B: &Vec<u8>) -> Bot {
    return bincode::deserialize(&B).unwrap();
}

/// MessagePack Serialization of a given data that has a Serialize trait
pub fn serialize_rmp<T: Serialize>(B: &T) -> Vec<u8>{
    return rmp_serde::to_vec(&B).unwrap();
}

/// MessagePack deserialization of a Vec to a Bot struct
pub fn deserialize_rmp(data: Vec<u8>) -> Bot {
    return rmp_serde::from_read(data.as_slice()).unwrap();
}