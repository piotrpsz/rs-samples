use std::collections::HashMap;

use ascii::ToAsciiChar;
use lazy_static::lazy_static;

lazy_static! {
    static ref ROMAN: HashMap<u8,isize> = {
        let mut m = HashMap::new();
        m.reserve(7);
        m.insert('M'.to_ascii_char().unwrap().as_byte(), 1_000);
        m.insert('D'.to_ascii_char().unwrap().as_byte(), 500);
        m.insert('C'.to_ascii_char().unwrap().as_byte(), 100);
        m.insert('L'.to_ascii_char().unwrap().as_byte(), 50);
        m.insert('X'.to_ascii_char().unwrap().as_byte(), 10);
        m.insert('V'.to_ascii_char().unwrap().as_byte(), 5);
        m.insert('I'.to_ascii_char().unwrap().as_byte(), 1);
        m
    };
}

/// Converts roman number to decimal integer.
pub fn to_int(text: &str) -> Option<isize> {
    if text.is_empty() {
        return None;
    }
    let buffer = Vec::from(text);

    let mut previous: isize;
    let mut value: isize = 0;

    // first roman digit
    previous = match ROMAN.get(&buffer[0]) {
        Some(x) => *x,
        None => return None
    };
    // the rest
    for i in 1..text.len() {
        let current = match ROMAN.get(&buffer[i]) {
            Some(x) => *x,
            None => return None
        };
        match previous < current {
            true => { value -= previous; }
            false => { value += previous; }
        };
        previous = current;
    }

    // Computed value
    Some(previous + value)
}

/// Converts decimal integer to roman number.
pub fn to_roman(n: isize) -> Option<String> {
    // we accept only positive values
    if n <= 0 {
        return None;
    }

    let mut buffer: Vec<char> = vec![];

    let mut n = n;
    loop {
        if n < 1_000 {
            break;
        }
        n -= 1_000;
        buffer.push('M');
    }

    let mdc: [[char; 3]; 3] = [
        ['X', 'V', 'I'],
        ['C', 'L', 'X'],
        ['M', 'D', 'C']];

    let mut v9 = 900;
    let mut v5 = 500;
    let mut v4 = 400;
    let mut v1 = 100;
    let mut order = 2_isize;

    while (n != 0) && (order >= 0) {
        let row = order as usize;
        let m = mdc[row][0];
        let d = mdc[row][1];
        let c = mdc[row][2];

        if (n / v1) == 9 {
            n -= v9;
            buffer.push(c);
            buffer.push(m);
        }
        if (n / v1) == 4 {
            n -= v4;
            buffer.push(c);
            buffer.push(d);
        }
        if (n / v1 >= 5) && (n / v1 <= 8) {
            n -= v5;
            buffer.push(d);
        }
        while n >= v1 {
            n -= v1;
            buffer.push(c);
        }

        v9 /= 10;
        v5 /= 10;
        v4 /= 10;
        v1 /= 10;
        order -= 1;
    }

    Some(buffer.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {}

    #[test]
    fn test_to_roman() {}
}