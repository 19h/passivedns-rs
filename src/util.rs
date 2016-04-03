//! Utilities shared by parsers
//!
//! includes:
//!
//! parse_name_into(&[u8], &mut String)

use std::str::from_utf8;

pub fn parse_name_into(data: &[u8], s: &mut String) -> u32 {
    let mut i: u32 = 0;
    if data.len() == 0 {
        return 0;
    }
    loop {
        let lbl_len = data[i as usize] as u32;
        if lbl_len == 0x0 {
            break;
        }
        for _ in 0..lbl_len {
            i += 1;
            s.push_str(from_utf8(&[data[i as usize]]).unwrap());
        }
        s.push('.');
        i += 1;
    }
    return i;
}