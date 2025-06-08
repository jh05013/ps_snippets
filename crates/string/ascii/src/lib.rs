//! Ascii strings & more.

use std::{
    ops::{Deref, Index},
    str::FromStr,
};

static ASCII_CHARS: [char; 128] = {
    let mut arr = ['\0'; 128];
    let mut i = 0;
    while i < 128 {
        arr[i] = i as u8 as char;
        i += 1;
    }
    arr
};

pub struct Ascii(pub String);

impl FromStr for Ascii {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ascii(s.to_owned()))
    }
}

impl Deref for Ascii {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<usize> for Ascii {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        let byte = self.0.as_bytes()[index];
        &ASCII_CHARS[byte as usize]
    }
}

pub fn parse_char(c: char) -> u8 {
    c as u8 - b'0'
}
