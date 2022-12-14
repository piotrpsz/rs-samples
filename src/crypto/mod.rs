pub mod blowfish;
pub mod gost;

/// The number of bytes to encrypt must be a multiple of the block size.
/// If not, add padding.
fn align_to_block(input: &[u8], block_size: usize) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(input);
    let n = buffer.len() % block_size;
    if n != 0 {
        buffer.extend(padding(block_size - n));
    }
    buffer
}

fn zeroed_buffer(nbytes: usize) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.resize(nbytes, 0);
    buffer
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(n);
    buffer.resize(n, 0);

    use rand::Rng;
    rand::thread_rng().fill(&mut buffer[..]);
    buffer
}

/// Creates padding vector.
fn padding(nbytes: usize) -> Vec<u8> {
    let mut pad: Vec<u8> = Vec::with_capacity(nbytes);
    pad.resize(nbytes, 0);
    pad[0] = 128;
    pad
}

/// Searches padding begginning in passed bytes.
fn padding_index(data: &[u8]) -> Option<usize> {
    let mut i = data.len();

    while i > 0 {
        i -= 1;
        if data[i] != 0 {
            if data[i] == 128 {
                return Some(i);
            }
            break;
        }
    }
    None
}

/// Converts block (tuple of 2xu32) to bytes.
/// Result is saved in passed data buffer.
fn block2bytes(x: (u32, u32), data: &mut [u8]) {
    words2bytes(x.0, x.1, data);
}

/// Converts two words (u32) to bytes.
/// Result is saved in passed data buffer.
fn words2bytes(xl: u32, xr: u32, data: &mut [u8]) {
    data[3] = xl.wrapping_shr(24) as u8;
    data[2] = xl.wrapping_shr(16) as u8;
    data[1] = xl.wrapping_shr(8) as u8;
    data[0] = xl as u8;

    data[7] = xr.wrapping_shr(24) as u8;
    data[6] = xr.wrapping_shr(16) as u8;
    data[5] = xr.wrapping_shr(8) as u8;
    data[4] = xr as u8;
}

/// Converts bytes from passed data buffer to block (tuple of 2xu32).
fn bytes2block(data: &[u8]) -> (u32, u32) {
    let xl = (data[3] as u32).wrapping_shl(24) |
        (data[2] as u32).wrapping_shl(16) |
        (data[1] as u32).wrapping_shl(8) |
        (data[0] as u32);
    let xr = (data[7] as u32).wrapping_shl(24) |
        (data[6] as u32).wrapping_shl(16) |
        (data[5] as u32).wrapping_shl(8) |
        (data[4] as u32);
    (xl, xr)
}
