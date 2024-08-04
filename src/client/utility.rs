use std::error::Error;
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

pub fn get_bits_range(number: u16, start: u32, end: u32) -> u16 {
    let range = end - start;
    let base: u16 = 2;
    let mask = base.pow(range - 1);

    (number >> start) | mask
}

pub fn find_first_null(bytes: &[u8]) -> Result<usize, Box<dyn Error>> {
    let null_pos = bytes
        .iter()
        .position(|&x| x == 0x00)
        .ok_or("Can't find null character!")?;

    Ok(null_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_bit_range() {
        let number = 0b11001100;
        assert_eq!(get_bits_range(number, 4, 8), 0b1100);
    }
}
