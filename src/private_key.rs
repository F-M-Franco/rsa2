#[allow(dead_code)]

use std::fmt;
use rand::{ rngs::OsRng, RngCore, CryptoRng};
use sha256::digest;
use crate::{math::{sam, modinverse2, lcm, gen_p_q}, errors::DecryptionError, public_key::PublicKey};

#[derive(Clone, Debug)]
struct PrivateKey{
    d: u64,
    n: u64,
    public_key: PublicKey
}

impl PrivateKey{
    pub fn new<T: RngCore + CryptoRng>(mut rng: T) -> Self{
        let (p, q) = gen_p_q(OsRng);

        let n = p*q;

        let ctf_n = lcm(p - 1, q - 1);

        let e: u64 = 65537;

        let d = modinverse2(e as i64, ctf_n as i64).unwrap() as u64;

        Self { d: d, n: n, public_key: PublicKey::new(e, n)}
    }

    pub fn public_key(&self) -> PublicKey{
        self.public_key.clone()
    }

    pub fn decrypt(&self, ciphertext: Vec<u8>) -> Result<String, DecryptionError>{
        if ciphertext.len() % 8 != 0{
            return Err(DecryptionError::new("Vec.length() mod 8 != 0".to_string()));
        }
    
        let mut utf8_plaintext: Vec<u8> = vec![];
        let mut i = 0;
    
        while i < ciphertext.len(){
            let be_bytes: [u8; 8] = ciphertext[i..i + 8].try_into().unwrap();
    
            utf8_plaintext.push(sam(u64::from_be_bytes(be_bytes), self.d, self.n) as u8);
            
            i += 8;
        }
    
        match String::from_utf8(utf8_plaintext) {
            Ok(plaintext) => return Ok(plaintext),
            Err(err) => return Err(DecryptionError::new(err.to_string())),
        }
    }

    pub fn sign(self, plaintext: &String) -> Vec<u8>{
        let hashed = digest(plaintext);
        let mut signature: Vec<u8> = vec![];
    
        for byte in hashed.as_bytes() {
            signature.append(&mut sam(*byte as u64, self.d, self.n).to_be_bytes().to_vec());
        }

        signature
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyLength: {}", self.n)
    }
}