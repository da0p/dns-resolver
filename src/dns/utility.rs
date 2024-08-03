pub fn to_u16(bytes: &[u8]) -> u16 {
    let mut num_u16: u16 = 0;
    for i in 0..bytes.len() {
        num_u16 |= (bytes[i] as u16) << i;
    }

    num_u16
}

pub fn to_u32(bytes: &[u8]) -> u32 {
    let mut num_u32: u32 = 0;
    for i in 0..bytes.len() {
        num_u32 |= (bytes[i] as u32) << i;
    }

    num_u32
}

pub fn encode_address(address: &str) -> Vec<u8> {
    let mut encoded_addr = vec![];
    let segs = address
        .split(".")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    for seg in segs {
        encoded_addr.push(seg.len() as u8);
        for j in 0..seg.len() {
            encoded_addr.push(seg.chars().nth(j).unwrap() as u8);
        }
    }
    encoded_addr.push(0);

    encoded_addr
}

pub fn decode_address(bytes: &Vec<u8>) -> String {
    let mut segments = vec![];
    let mut i = 0;
    while bytes[i] != 0 {
        let f_seg_len = bytes[i] as usize;
        if f_seg_len != 0 {
            let seg = String::from_utf8(bytes[i + 1..i + 1 + f_seg_len].to_vec()).unwrap();
            segments.push(seg);
        }
        i += f_seg_len + 1;
    }
    segments.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_valid_address() {
        let enc_addr = encode_address("dns.google.com");
        assert_eq!(enc_addr[0], 3);
        assert_eq!(enc_addr[1..4], ['d' as u8, 'n' as u8, 's' as u8]);
        assert_eq!(enc_addr[4], 6);
        assert_eq!(
            enc_addr[5..11],
            ['g' as u8, 'o' as u8, 'o' as u8, 'g' as u8, 'l' as u8, 'e' as u8]
        );
        assert_eq!(enc_addr[11], 3);
        assert_eq!(enc_addr[12..15], ['c' as u8, 'o' as u8, 'm' as u8]);
    }

    #[test]
    fn decode_valid_address() {
        let enc_addr = encode_address("dns.google.com");
        assert_eq!(decode_address(&enc_addr), "dns.google.com");
    }

    #[test]
    fn encode_invalid_address() {
        let enc_addr = encode_address("abc");
        assert_eq!(enc_addr[0..5], [3, 'a' as u8, 'b' as u8, 'c' as u8, 0]);
    }

    #[test]
    fn decode_invalid_address() {
        let enc_addr = encode_address("abc");
        assert_eq!(decode_address(&enc_addr), "abc");
    }

    #[test]
    fn encode_another_invalid_address() {
        let enc_addr = encode_address(".abc");
        assert_eq!(enc_addr[0..5], [3, 'a' as u8, 'b' as u8, 'c' as u8, 0]);
    }
}

