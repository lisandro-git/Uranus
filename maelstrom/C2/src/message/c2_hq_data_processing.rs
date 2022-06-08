use crate::communication::c2::Bot;
//serialize using bincode

pub fn serialize_bincode(B: &Bot) -> Vec<u8>{
    return bincode::serialize(&B).unwrap();
}

pub fn deserialize_bincode(B: &Vec<u8>) -> Bot{
    return bincode::deserialize(&B).unwrap();
}
