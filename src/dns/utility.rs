use std::{error::Error, ptr::null};
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

pub fn to_u16(bytes: &[u8]) -> u16 {
    let mut rdr = Cursor::new(bytes);
    rdr.read_u16::<BigEndian>().unwrap()
}

pub fn to_u32(bytes: &[u8]) -> u32 {
    let mut rdr = Cursor::new(bytes);
    rdr.read_u32::<BigEndian>().unwrap()
}

pub fn find_first_null(bytes: &[u8]) -> Result<usize, Box<dyn Error>> {
    let null_pos = bytes
        .iter()
        .position(|&x| x == 0x00)
        .ok_or("Can't find null character!")?;

    Ok(null_pos)
}
