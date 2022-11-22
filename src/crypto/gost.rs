use crate::crypto::*;

const BLOCK_SIZE: usize = 8;
// 8 bytes = 2 u32 = 54 bit
const KEY_SIZE: usize = 32;
// 32 bytes = 8 u32 = 256 bit
const K8: [u8; 16] = [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7];
const K7: [u8; 16] = [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10];
const K6: [u8; 16] = [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8];
const K5: [u8; 16] = [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15];
const K4: [u8; 16] = [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9];
const K3: [u8; 16] = [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11];
const K2: [u8; 16] = [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1];
const K1: [u8; 16] = [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7];

pub struct Gost {
    k: [u32; 8],
    k87: [u8; 256],
    k65: [u8; 256],
    k43: [u8; 256],
    k21: [u8; 256],
}

impl Gost {
    /// Creates gost-object for passed string|&str as key.
    pub fn with_key_as_text<T: AsRef<str>>(key: T) -> Result<Gost, &'static str> {
        Gost::new(key.as_ref().as_bytes())
    }

    pub fn new(key: &[u8]) -> Result<Gost, &'static str> {
        if key.len() != KEY_SIZE {
            return Err("(GOST) invalid key size");
        }

        let mut k = [0u32; 8];
        let mut k87 = [0u8; 256];
        let mut k65 = [0u8; 256];
        let mut k43 = [0u8; 256];
        let mut k21 = [0u8; 256];

        let mut i = 0usize;
        while i < 256 {
            let idx1 = i >> 4;
            let idx2 = i & 15;
            k87[i] = (K8[idx1] << 4) | K7[idx2];
            k65[i] = (K6[idx1] << 4) | K5[idx2];
            k43[i] = (K4[idx1] << 4) | K3[idx2];
            k21[i] = (K2[idx1] << 4) | K1[idx2];
            i += 1;
        }

        i = 0usize;
        while i < 8 {
            let mut idx = (i * 4) + 3;
            let mut v = 0u32;
            v = (v << 8) + (key[idx] as u32);
            idx -= 1;
            v = (v << 8) + (key[idx] as u32);
            idx -= 1;
            v = (v << 8) + (key[idx] as u32);
            idx -= 1;
            v = (v << 8) + (key[idx] as u32);
            k[i] = v;
            i += 1;
        }

        Ok(Gost { k, k87, k65, k43, k21 })
    }

    /// Encrypts passed plain text (ECB mode).
    pub fn encrypt_ecb(&self, input: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
        if input.is_empty() {
            return Err("(GOST-ECB) nothing to encrypt");
        }

        let plain = {
            let mut buffer = Vec::new();
            buffer.extend(input);
            let n = buffer.len() % BLOCK_SIZE;
            if n != 0 {
                buffer.extend(padding(BLOCK_SIZE - n));
            }
            buffer
        };
        let nbytes = plain.len();

        let mut cipher = Vec::with_capacity(nbytes);
        cipher.resize(nbytes, 0);

        let mut i = 0usize;
        while i < nbytes {
            let x = bytes2block(&plain[i..]);
            let x = self.encrypt_block(x);
            block2bytes(x, &mut cipher[i..]);
            i += BLOCK_SIZE;
        }

        Ok(cipher)
    }

    /// Decrypts passsed cipher text (ECB mode).
    pub fn decrypt_ecb(&self, cipher: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
        let nbytes = cipher.len();
        if nbytes == 0 {
            return Err("(GOST-ECB) nothing to decrypt");
        }

        let mut plain = Vec::with_capacity(nbytes);
        plain.resize(nbytes, 0);

        let mut i = 0usize;
        while i < nbytes {
            let x = bytes2block(&cipher[i..]);
            let x = self.decrypt_block(x);
            block2bytes(x, &mut plain[i..]);
            i += BLOCK_SIZE;
        }

        match padding_index(&plain) {
            Some(idx) => Ok(plain[..idx].to_vec()),
            _ => Ok(plain)
        }
    }


    /// Encrypts plain tuple (2xu32).
    /// Returns encryptet tuple (2xu32).
    fn encrypt_block(&self, x: (u32, u32)) -> (u32, u32) {
        self.encrypt(x.0, x.1)
    }

    /// Encrypts two u32 words.
    /// Returns encryptet block (tuple of 2xu32).
    fn encrypt(&self, mut xl: u32, mut xr: u32) -> (u32, u32) {
        xr ^= self.f(xl.wrapping_add(self.k[0]));
        xl ^= self.f(xr.wrapping_add(self.k[1]));
        xr ^= self.f(xl.wrapping_add(self.k[2]));
        xl ^= self.f(xr.wrapping_add(self.k[3]));
        xr ^= self.f(xl.wrapping_add(self.k[4]));
        xl ^= self.f(xr.wrapping_add(self.k[5]));
        xr ^= self.f(xl.wrapping_add(self.k[6]));
        xl ^= self.f(xr.wrapping_add(self.k[7]));

        xr ^= self.f(xl.wrapping_add(self.k[0]));
        xl ^= self.f(xr.wrapping_add(self.k[1]));
        xr ^= self.f(xl.wrapping_add(self.k[2]));
        xl ^= self.f(xr.wrapping_add(self.k[3]));
        xr ^= self.f(xl.wrapping_add(self.k[4]));
        xl ^= self.f(xr.wrapping_add(self.k[5]));
        xr ^= self.f(xl.wrapping_add(self.k[6]));
        xl ^= self.f(xr.wrapping_add(self.k[7]));

        xr ^= self.f(xl.wrapping_add(self.k[0]));
        xl ^= self.f(xr.wrapping_add(self.k[1]));
        xr ^= self.f(xl.wrapping_add(self.k[2]));
        xl ^= self.f(xr.wrapping_add(self.k[3]));
        xr ^= self.f(xl.wrapping_add(self.k[4]));
        xl ^= self.f(xr.wrapping_add(self.k[5]));
        xr ^= self.f(xl.wrapping_add(self.k[6]));
        xl ^= self.f(xr.wrapping_add(self.k[7]));

        xr ^= self.f(xl.wrapping_add(self.k[7]));
        xl ^= self.f(xr.wrapping_add(self.k[6]));
        xr ^= self.f(xl.wrapping_add(self.k[5]));
        xl ^= self.f(xr.wrapping_add(self.k[4]));
        xr ^= self.f(xl.wrapping_add(self.k[3]));
        xl ^= self.f(xr.wrapping_add(self.k[2]));
        xr ^= self.f(xl.wrapping_add(self.k[1]));
        xl ^= self.f(xr.wrapping_add(self.k[0]));

        (xr, xl)
    }

    /// Decrypts cipher tuple (2xu32)
    pub fn decrypt_block(&self, x: (u32, u32)) -> (u32, u32) {
        self.decrypt(x.0, x.1)
    }

    /// Decrypts two u32 words.
    /// Returns plain tuple (2xu32).
    pub fn decrypt(&self, mut xl: u32, mut xr: u32) -> (u32, u32) {
        xr ^= self.f(xl.wrapping_add(self.k[0]));
        xl ^= self.f(xr.wrapping_add(self.k[1]));
        xr ^= self.f(xl.wrapping_add(self.k[2]));
        xl ^= self.f(xr.wrapping_add(self.k[3]));
        xr ^= self.f(xl.wrapping_add(self.k[4]));
        xl ^= self.f(xr.wrapping_add(self.k[5]));
        xr ^= self.f(xl.wrapping_add(self.k[6]));
        xl ^= self.f(xr.wrapping_add(self.k[7]));

        xr ^= self.f(xl.wrapping_add(self.k[7]));
        xl ^= self.f(xr.wrapping_add(self.k[6]));
        xr ^= self.f(xl.wrapping_add(self.k[5]));
        xl ^= self.f(xr.wrapping_add(self.k[4]));
        xr ^= self.f(xl.wrapping_add(self.k[3]));
        xl ^= self.f(xr.wrapping_add(self.k[2]));
        xr ^= self.f(xl.wrapping_add(self.k[1]));
        xl ^= self.f(xr.wrapping_add(self.k[0]));

        xr ^= self.f(xl.wrapping_add(self.k[7]));
        xl ^= self.f(xr.wrapping_add(self.k[6]));
        xr ^= self.f(xl.wrapping_add(self.k[5]));
        xl ^= self.f(xr.wrapping_add(self.k[4]));
        xr ^= self.f(xl.wrapping_add(self.k[3]));
        xl ^= self.f(xr.wrapping_add(self.k[2]));
        xr ^= self.f(xl.wrapping_add(self.k[1]));
        xl ^= self.f(xr.wrapping_add(self.k[0]));

        xr ^= self.f(xl.wrapping_add(self.k[7]));
        xl ^= self.f(xr.wrapping_add(self.k[6]));
        xr ^= self.f(xl.wrapping_add(self.k[5]));
        xl ^= self.f(xr.wrapping_add(self.k[4]));
        xr ^= self.f(xl.wrapping_add(self.k[3]));
        xl ^= self.f(xr.wrapping_add(self.k[2]));
        xr ^= self.f(xl.wrapping_add(self.k[1]));
        xl ^= self.f(xr.wrapping_add(self.k[0]));

        (xr, xl)
    }


    /// Heart of the algorithm.
    fn f(&self, x: u32) -> u32 {
        let i0 = x.wrapping_shr(24) & 0xff;
        let i1 = x.wrapping_shr(16) & 0xff;
        let i2 = x.wrapping_shr(8) & 0xff;
        let i3 = x & 0xff;

        let w0 = (self.k87[i0 as usize] as u32).wrapping_shl(24);
        let w1 = (self.k65[i1 as usize] as u32).wrapping_shl(16);
        let w2 = (self.k43[i2 as usize] as u32).wrapping_shl(8);
        let w3 = self.k21[i3 as usize] as u32;


        let x = w0 | w1 | w2 | w3;
        x.wrapping_shl(11) | x.wrapping_shr(32 - 11)
    }
}