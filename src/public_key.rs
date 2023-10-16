use std::fmt;

use sha256::digest;
use crate::math::*;
use crate::errors::{VerificationError, PubKeyDeserializeError};

#[derive(Clone, Debug)]
pub struct PublicKey{
    n: u64, // Key Length
    e: u64
}

impl PublicKey{
    pub fn new(e: u64, n: u64) -> Self{
        Self { n: n, e: e }
    }

    pub fn encrypt(&self, plaintext: &String) -> Vec<u8>{
        let mut ciphertext: Vec<u8> = vec![];
    
        for byte in plaintext.as_bytes() {
            ciphertext.append(&mut sam(*byte as u64, self.e, self.n).to_be_bytes().to_vec());
        }

        ciphertext
    }

    pub fn verify(self, plaintext: &String, signature: Vec<u8>) -> Result<bool, VerificationError>{
        let hashed = digest(plaintext);
    
        let mut utf8_signature: Vec<u8> = vec![];
        let mut i = 0;
    
        while i < signature.len(){
            let be_bytes: [u8; 8] = signature[i..i + 8].try_into().unwrap();
    
            utf8_signature.push(sam(u64::from_be_bytes(be_bytes), self.e, self.n) as u8);
            
            i += 8;
        }

        println!("{}", signature.len());

        match String::from_utf8(utf8_signature) {
            Ok(decrypted_signature) => {
                return Ok(hashed == decrypted_signature);
            }
            Err(err) => return Err(VerificationError::new(err.to_string())),
        }
    }

    pub fn serialize(&self) -> Vec<u8>{
        let mut bytes = self.e.to_be_bytes().to_vec();
        bytes.append(&mut self.n.to_be_bytes().to_vec());
        
        bytes
    }

    pub fn deserialize(serialized: Vec<u8>) -> Result<Self, PubKeyDeserializeError>{
        if serialized.len() != 16 {
            return Err(PubKeyDeserializeError::new("Input Vec must be of size 16".to_string()));
        }

        let e_be_bytes: [u8; 8] = serialized[..8].try_into().unwrap();
        let e = u64::from_be_bytes(e_be_bytes);
    
        let n_be_bytes: [u8; 8] = serialized[8..16].try_into().unwrap();
        let n = u64::from_be_bytes(n_be_bytes);

        Ok(Self::new(e, n))
    }
}
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyLength: {}", self.n)
    }
}