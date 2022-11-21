use std::io::Write;
use std::iter;

const DEFAULT_CAPACITY: usize = 128;
const SPACE: char = ' ';

/// StringBuilder declaration
pub struct StringBuilder(Vec<u8>);

/// Builder implementation
impl StringBuilder {
    /// Creates string builder with size
    pub fn new(size: usize) -> StringBuilder {
        StringBuilder(Vec::with_capacity(size))
    }

    /// Adds to builder's buffer item satisfying BytesConvertible trait
    pub fn add<T: BytesCovertible>(mut self, data: T) -> Self {
        self.0.write_all(&data.to_bytes()).unwrap();
        self
    }

    /// Adds to buffer the indicated number of spaces
    pub fn add_spaces(mut self, n: usize) -> Self {
        let one_space = SPACE.to_bytes();
        let mut data: Vec<u8> = vec![];
        for _ in 0..n {
            data.append(&mut one_space.clone());
        }
        self.0.write_all(&data).unwrap();
        self
    }

    /// Returns current number of added bytes
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Checks if buffer is empty
    pub fn empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns string representation of bytes
    pub fn string(&self) -> String {
        let dup = self.0.clone();
        String::from_utf8_lossy(&dup).to_string()
    }

    /// Returns trimmed (left & right) od string representation of byte
    pub fn trimmed_string(&self) -> String {
        self.string().trim().to_string()
    }

    /// Removes all bytes from buffer
    pub fn reset(&mut self) {
        self.0.clear();
    }
}

/// Creates bytes vector from bytes slice
fn as_vector(s: &[u8]) -> Vec<u8> {
    let mut buffer = iter::repeat(0).take(s.len()).collect::<Vec<u8>>();
    buffer.copy_from_slice(s);
    buffer
}

/********************************************************************
*                                                                   *
*             T r a i t - B y t e s C o n v e r t i b l e           *
*                                                                   *
********************************************************************/

pub trait BytesCovertible {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Converts 'String' to bytes vector
impl BytesCovertible for String {
    fn to_bytes(&self) -> Vec<u8> {
        as_vector(self.as_bytes())
    }
}

/// Converts '&str' to bytes vector
impl<'a> BytesCovertible for &'a str {
    fn to_bytes(&self) -> Vec<u8> {
        as_vector(self.as_bytes())
    }
}

/// Converts one byte to bytes vector
impl BytesCovertible for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

/// Converts one char (utf8) to bytes vector
impl BytesCovertible for char {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = [0; 4];
        as_vector(self.encode_utf8(&mut buf).as_bytes())
    }
}

/// Converts bytes slice to bytes vector
impl<'a> BytesCovertible for &'a [u8] {
    fn to_bytes(&self) -> Vec<u8> {
        as_vector(self)
    }
}

/********************************************************************
*                                                                   *
*                      T r a i t - D e f a u l t                    *
*                                                                   *
********************************************************************/

impl Default for StringBuilder {
    fn default() -> Self {
        StringBuilder(Vec::with_capacity(DEFAULT_CAPACITY))
    }
}
