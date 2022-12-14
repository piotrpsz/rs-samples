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
    // k: [u32; 8],
    k0: u32,
    k1: u32,
    k2: u32,
    k3: u32,
    k4: u32,
    k5: u32,
    k6: u32,
    k7: u32,
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
            return Err("(Gost) invalid key size");
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

        let kptr = &k as *const u32;
        let k0 = unsafe { *kptr.offset(0) };
        let k1 = unsafe { *kptr.offset(1) };
        let k2 = unsafe { *kptr.offset(2) };
        let k3 = unsafe { *kptr.offset(3) };
        let k4 = unsafe { *kptr.offset(4) };
        let k5 = unsafe { *kptr.offset(5) };
        let k6 = unsafe { *kptr.offset(6) };
        let k7 = unsafe { *kptr.offset(7) };

        Ok(Gost { k0, k1, k2, k3, k4, k5, k6, k7, k87, k65, k43, k21 })
    }

    /// Encrypts passed plain text (ECB mode).
    pub fn encrypt_ecb(&self, input: &[u8]) -> Result<Vec<u8>, &'static str> {
        if input.is_empty() {
            return Err("(GOST-ECB) nothing to encrypt");
        }

        let plain = align_to_block(input, BLOCK_SIZE);
        let nbytes = plain.len();
        let mut cipher = zeroed_buffer(nbytes);

        // let mut i = 0usize;
        // while i < nbytes {
        for i in (0..nbytes).step_by(BLOCK_SIZE) {
            let x = self.encrypt_block(bytes2block(&plain[i..]));
            block2bytes(x, &mut cipher[i..]);
        }

        Ok(cipher)
    }

    /// Decrypts passsed cipher text (ECB mode).
    pub fn decrypt_ecb(&self, cipher: &[u8]) -> Result<Vec<u8>, &'static str> {
        let nbytes = cipher.len();
        if nbytes == 0 {
            return Err("(GOST-ECB) nothing to decrypt");
        }

        let mut plain = Vec::with_capacity(nbytes);
        plain.resize(nbytes, 0);

        for i in (0..nbytes).step_by(BLOCK_SIZE) {
            let x = self.decrypt_block(bytes2block(&cipher[i..]));
            block2bytes(x, &mut plain[i..]);
        }

        match padding_index(&plain) {
            Some(idx) => Ok(plain[..idx].to_vec()),
            _ => Ok(plain)
        }
    }

    /// Encrypts passed plain-text.
    /// Before encryption creates IV vector.
    pub fn encrypt_cbc(&self, input: &[u8]) -> Result<Vec<u8>, &'static str> {
        self.encrypt_cbc_iv(input, &random_bytes(BLOCK_SIZE))
    }

    /// Encrypts plain-text with passed IV vector.
    pub fn encrypt_cbc_iv(&self, input: &[u8], iv: &[u8]) -> Result<Vec<u8>, &'static str> {
        if iv.len() != BLOCK_SIZE {
            return Err("(Gost:CBC) invalid size of IV vector");
        }
        if input.is_empty() {
            return Err("(Gost:CBC) nothing to encrypt");
        }

        let plain = align_to_block(input, BLOCK_SIZE);
        let nbytes = plain.len();
        let mut cipher = zeroed_buffer(nbytes + BLOCK_SIZE);
        cipher[0..BLOCK_SIZE].copy_from_slice(iv);

        let mut x = bytes2block(iv);
        for i in (0..nbytes).step_by(BLOCK_SIZE) {
            let t = bytes2block(&plain[i..]);
            x = self.encrypt(t.0 ^ x.0, t.1 ^ x.1);
            block2bytes(x, &mut cipher[(i + BLOCK_SIZE)..]);
        }

        Ok(cipher)
    }

    /// Decrypts passed cipher-text.
    pub fn decrypt_cbc(&self, cipher: &[u8]) -> Result<Vec<u8>, &'static str> {
        let nbytes = cipher.len();
        if nbytes <= BLOCK_SIZE {
            return Err("(Gost::CBC) cipher data size is to short");
        }

        let mut plain = zeroed_buffer(nbytes - BLOCK_SIZE);

        let mut p = bytes2block(&cipher[..]);
        for i in (BLOCK_SIZE..nbytes).step_by(BLOCK_SIZE) {
            let x = bytes2block(&cipher[i..]);
            let t = x;
            let c = self.decrypt_block(x);
            words2bytes(c.0 ^ p.0, c.1 ^ p.1, &mut plain[(i - BLOCK_SIZE)..]);
            p = t;
        }

        match padding_index(&plain) {
            Some(idx) => Ok(plain[..idx].to_vec()),
            _ => Ok(plain)
        }
    }

    /****************************************************************
    *                                                               *
    *                 P R I V A T E   M E T H O D S                 *
    *                                                               *
    ****************************************************************/

    /// Encrypts plain tuple (2xu32).
    /// Returns encrypted tuple (2xu32).
    fn encrypt_block(&self, x: (u32, u32)) -> (u32, u32) {
        self.encrypt(x.0, x.1)
    }

    /// Encrypts two u32 words.
    /// Returns encrypted block (tuple of 2xu32).
    fn encrypt(&self, mut xl: u32, mut xr: u32) -> (u32, u32) {
        xr ^= self.f(xl.wrapping_add(self.k0));
        xl ^= self.f(xr.wrapping_add(self.k1));
        xr ^= self.f(xl.wrapping_add(self.k2));
        xl ^= self.f(xr.wrapping_add(self.k3));
        xr ^= self.f(xl.wrapping_add(self.k4));
        xl ^= self.f(xr.wrapping_add(self.k5));
        xr ^= self.f(xl.wrapping_add(self.k6));
        xl ^= self.f(xr.wrapping_add(self.k7));

        xr ^= self.f(xl.wrapping_add(self.k0));
        xl ^= self.f(xr.wrapping_add(self.k1));
        xr ^= self.f(xl.wrapping_add(self.k2));
        xl ^= self.f(xr.wrapping_add(self.k3));
        xr ^= self.f(xl.wrapping_add(self.k4));
        xl ^= self.f(xr.wrapping_add(self.k5));
        xr ^= self.f(xl.wrapping_add(self.k6));
        xl ^= self.f(xr.wrapping_add(self.k7));

        xr ^= self.f(xl.wrapping_add(self.k0));
        xl ^= self.f(xr.wrapping_add(self.k1));
        xr ^= self.f(xl.wrapping_add(self.k2));
        xl ^= self.f(xr.wrapping_add(self.k3));
        xr ^= self.f(xl.wrapping_add(self.k4));
        xl ^= self.f(xr.wrapping_add(self.k5));
        xr ^= self.f(xl.wrapping_add(self.k6));
        xl ^= self.f(xr.wrapping_add(self.k7));

        xr ^= self.f(xl.wrapping_add(self.k7));
        xl ^= self.f(xr.wrapping_add(self.k6));
        xr ^= self.f(xl.wrapping_add(self.k5));
        xl ^= self.f(xr.wrapping_add(self.k4));
        xr ^= self.f(xl.wrapping_add(self.k3));
        xl ^= self.f(xr.wrapping_add(self.k2));
        xr ^= self.f(xl.wrapping_add(self.k1));
        xl ^= self.f(xr.wrapping_add(self.k0));

        (xr, xl)
    }

    /// Decrypts cipher tuple (2xu32)
    fn decrypt_block(&self, x: (u32, u32)) -> (u32, u32) {
        self.decrypt(x.0, x.1)
    }

    /// Decrypts two u32 words.
    /// Returns plain tuple (2xu32).
    pub fn decrypt(&self, mut xl: u32, mut xr: u32) -> (u32, u32) {
        xr ^= self.f(xl.wrapping_add(self.k0));
        xl ^= self.f(xr.wrapping_add(self.k1));
        xr ^= self.f(xl.wrapping_add(self.k2));
        xl ^= self.f(xr.wrapping_add(self.k3));
        xr ^= self.f(xl.wrapping_add(self.k4));
        xl ^= self.f(xr.wrapping_add(self.k5));
        xr ^= self.f(xl.wrapping_add(self.k6));
        xl ^= self.f(xr.wrapping_add(self.k7));

        xr ^= self.f(xl.wrapping_add(self.k7));
        xl ^= self.f(xr.wrapping_add(self.k6));
        xr ^= self.f(xl.wrapping_add(self.k5));
        xl ^= self.f(xr.wrapping_add(self.k4));
        xr ^= self.f(xl.wrapping_add(self.k3));
        xl ^= self.f(xr.wrapping_add(self.k2));
        xr ^= self.f(xl.wrapping_add(self.k1));
        xl ^= self.f(xr.wrapping_add(self.k0));

        xr ^= self.f(xl.wrapping_add(self.k7));
        xl ^= self.f(xr.wrapping_add(self.k6));
        xr ^= self.f(xl.wrapping_add(self.k5));
        xl ^= self.f(xr.wrapping_add(self.k4));
        xr ^= self.f(xl.wrapping_add(self.k3));
        xl ^= self.f(xr.wrapping_add(self.k2));
        xr ^= self.f(xl.wrapping_add(self.k1));
        xl ^= self.f(xr.wrapping_add(self.k0));

        xr ^= self.f(xl.wrapping_add(self.k7));
        xl ^= self.f(xr.wrapping_add(self.k6));
        xr ^= self.f(xl.wrapping_add(self.k5));
        xl ^= self.f(xr.wrapping_add(self.k4));
        xr ^= self.f(xl.wrapping_add(self.k3));
        xl ^= self.f(xr.wrapping_add(self.k2));
        xr ^= self.f(xl.wrapping_add(self.k1));
        xl ^= self.f(xr.wrapping_add(self.k0));

        (xr, xl)
    }

    /// Heart of the algorithm.
    fn f(&self, x: u32) -> u32 {
        let i0 = (x.wrapping_shr(24) & 0xff) as usize;
        let i1 = (x.wrapping_shr(16) & 0xff) as usize;
        let i2 = (x.wrapping_shr(8) & 0xff) as usize;
        let i3 = (x & 0xff) as usize;

        let w0 = unsafe { *self.k87.get_unchecked(i0) } as u32;
        let w1 = unsafe { *self.k65.get_unchecked(i1) } as u32;
        let w2 = unsafe { *self.k43.get_unchecked(i2) } as u32;
        let w3 = unsafe { *self.k21.get_unchecked(i3) } as u32;

        let x = w0.wrapping_shl(24)
            | w1.wrapping_shl(16)
            | w2.wrapping_shl(8)
            | w3;

        x.wrapping_shl(11) | x.wrapping_shr(32 - 11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        let key = vec![0u8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0];
        let gt = Gost::new(&key);
        assert!(gt.is_ok());
        let gt = gt.unwrap();

        let plain = [
            (0u32, 0u32),
            (1u32, 0u32),
            (0u32, 1u32),
            (0xffffffffu32, 0xffffffffu32)
        ];
        let expected = [
            (0x37ef7123u32, 0x361b7184u32),
            (0x1159d751u32, 0xff9b91d2u32),
            (0xc79c4ef4u32, 0x27ac9149u32),
            (0xf9709623u32, 0x56ad8d77u32)
        ];
        let mut i = 0usize;
        while i < plain.len() {
            let encrypted = gt.encrypt_block(plain[i]);
            assert_eq!(expected[i], encrypted);
            let decrypted = gt.decrypt_block(encrypted);
            assert_eq!(plain[i], decrypted);
            i += 1;
        }
    }

    #[test]
    fn test_ecb() {
        let key = [0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0];
        let gt = Gost::new(&key);
        assert!(gt.is_ok());
        let gt = gt.unwrap();

        let plain = "Artur, B??a??ej, Jolanta i Piotr Pszcz????kowscy".as_bytes();
        let expt = [0x5cu8, 0xc8, 0x5a, 0xb2, 0xab, 0xa8, 0x58, 0x98,
            0x52, 0x33, 0x67, 0x6c, 0x4b, 0x60, 0x25, 0x6e, 0x22, 0x4d, 0x2e,
            0xb7, 0x59, 0xe5, 0x63, 0x27, 0x63, 0x5b, 0x61, 0xfd, 0x9b, 0xa3,
            0x3e, 0x3c, 0xa3, 0xa5, 0xe6, 0xd9, 0x6d, 0x89, 0x14, 0x07, 0x63,
            0x6c, 0x1d, 0x19, 0x6f, 0xc2, 0xde, 0x44];

        let cipher = gt.encrypt_ecb(plain);
        assert!(cipher.is_ok());
        assert_eq!(cipher.unwrap(), expt);
    }

    #[test]
    fn test_cbc() {
        let key = [0u8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0];
        let gt = Gost::new(&key);
        assert!(gt.is_ok());
        let gt = gt.unwrap();

        let plain = "Yamato & Musashi".as_bytes();
        let expt = [0xf8u8, 0xa4, 0x9e, 0x45, 0x40, 0xa5, 0x65, 0xc8,
            0xe3, 0x78, 0x2e, 0x4, 0x30, 0x40, 0x45, 0x7a, 0x5a, 0xbf, 0xe4,
            0xc6, 0x9a, 0x53, 0x4f, 0xce];
        let iv = [0xf8u8, 0xa4, 0x9e, 0x45, 0x40, 0xa5, 0x65, 0xc8];

        let cipher = gt.encrypt_cbc_iv(&plain, &iv);
        assert!(cipher.is_ok());
        assert_eq!(cipher.unwrap(), expt);
    }
}