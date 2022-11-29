use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref ROMAN: HashMap<u8,isize> = {
        let mut m = HashMap::new();
        m.reserve(7);
        // m.insert('M'.to_ascii_char().unwrap().as_byte(), 1_000);
        // m.insert('D'.to_ascii_char().unwrap().as_byte(), 500);
        // m.insert('C'.to_ascii_char().unwrap().as_byte(), 100);
        // m.insert('L'.to_ascii_char().unwrap().as_byte(), 50);
        // m.insert('X'.to_ascii_char().unwrap().as_byte(), 10);
        // m.insert('V'.to_ascii_char().unwrap().as_byte(), 5);
        // m.insert('I'.to_ascii_char().unwrap().as_byte(), 1);

        m.insert(b'M', 1_000);
        m.insert(b'D', 500);
        m.insert(b'C', 100);
        m.insert(b'L', 50);
        m.insert(b'X', 10);
        m.insert(b'V', 5);
        m.insert(b'I', 1);

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

    struct Test<'a> {
        roman: &'a str,
        decimal: isize,
    }

    const TESTS: &[Test] = &[
        Test { roman: "I", decimal: 1 },
        Test { roman: "II", decimal: 2 },
        Test { roman: "III", decimal: 3 },
        Test { roman: "IV", decimal: 4 },
        Test { roman: "V", decimal: 5 },
        Test { roman: "VI", decimal: 6 },
        Test { roman: "VII", decimal: 7 },
        Test { roman: "VIII", decimal: 8 },
        Test { roman: "IX", decimal: 9 },
        Test { roman: "X", decimal: 10 },
        Test { roman: "XI", decimal: 11 },
        Test { roman: "XII", decimal: 12 },
        Test { roman: "XIII", decimal: 13 },
        Test { roman: "XIV", decimal: 14 },
        Test { roman: "XV", decimal: 15 },
        Test { roman: "XVI", decimal: 16 },
        Test { roman: "XVII", decimal: 17 },
        Test { roman: "XVIII", decimal: 18 },
        Test { roman: "XIX", decimal: 19 },
        Test { roman: "XX", decimal: 20 },
        Test { roman: "XXX", decimal: 30 },
        Test { roman: "XL", decimal: 40 },
        Test { roman: "XLIV", decimal: 44 },
        Test { roman: "L", decimal: 50 },
        Test { roman: "LV", decimal: 55 },
        Test { roman: "LVI", decimal: 56 },
        Test { roman: "LXXX", decimal: 80 },
        Test { roman: "DCC", decimal: 700 },
        Test { roman: "CMXCIX", decimal: 999 },
        Test { roman: "CMII", decimal: 902 },
        Test { roman: "MCCXXXIV", decimal: 1234 },
        Test { roman: "MCDXLVI", decimal: 1446 },
        Test { roman: "MCMXCIV", decimal: 1994 },
        Test { roman: "MMVI", decimal: 2006 },
    ];

    #[test]
    fn test_to_int() {
        for tt in TESTS {
            assert_eq!(Some(tt.decimal),  to_int(tt.roman));
        }
    }

    #[test]
    fn test_to_roman() {
        for tt in TESTS {
            assert_eq!(Some(tt.roman.to_string()), to_roman(tt.decimal));
        }
    }
}