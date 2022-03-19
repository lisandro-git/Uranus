use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use crate::blockchain::block;
use crate::blockchain::block::Block_Data;
use crate::blockchain::blockchain;
use crate::blockchain::hash;


fn main() -> std::io::Result<()> {
    let version: [u8; 4] = [6, 9, 6, 9];
    let prev_hash: [u8; 32] = [0; 32];
    let timestamp: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    let x = vec![
        block::Block_Header::new(version, prev_hash, timestamp)
    ];

    let y = vec![
        block::Block_Data::new(15)
    ];

    /*let serialized: Vec<u8> = match serialize(&y) {
        Ok(y) => y,
        Err(e) => panic!("{}", e)
    };
    println!("Serialized into {} bytes", serialized.len());
    
    let mut file = File::create("foo.txt")?;
    file.write_all(serialized.as_slice())?;

    let deserialized: Vec<Block_Data> = match deserialize(&serialized) {
        Ok(y) => y,
        Err(e) => panic!("{}", e)
    };
    println!("Serialized into {} bytes", deserialized.len());
*/
    Ok(())
}
