//! Utility untuk kebutuhan pembuatan dan validasi token seperti akses token.
//! 
//! 

use sodiumoxide::{crypto, randombytes};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};
use rsnowflake::SnowflakeIdGenerator;
use hex;

use std::io::Cursor;



/// Generate random token
pub fn generate_u64() -> u64 {
    // let mut idgen = SnowflakeIdGenerator::new(1);
    // idgen.generate() as u64
    let mut bytes = Cursor::new(randombytes::randombytes(8));
    bytes.read_u64::<LittleEndian>().expect("Can't generate u64")
}

/// Menggenerasikan kode unik untuk akses token pada API.
pub fn generate_access_token() -> String {
    let token_u64 = generate_u64();
    dbg!(token_u64);
    let mut wtr = vec![];
    wtr.write_u64::<BigEndian>(token_u64).unwrap();
    hex::encode(&crypto::hash::hash(wtr.as_slice()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    #[test]
    fn test_generate_access_token() {
        let _ = ::sodiumoxide::init();
        
        let access_tokens:Vec<String> = (0..10).map(|i| super::generate_access_token()).collect();
        let access_tokens2 = access_tokens.clone();

        for at1 in access_tokens {
            let mut map = HashMap::new();
            for at2 in access_tokens2.iter() {
                if &at1 == at2 {
                    if !map.contains_key(at2){
                        map.insert(at2.clone(), 1);
                    }
                    if map.get(at2) == Some(&1){
                        continue;
                    }
                }
                assert_ne!(&at1, at2);
            }
        }
    }
}