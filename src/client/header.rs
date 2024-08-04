use std::error::Error;

use crate::client::utility;

pub struct Flag {
    pub qr: u16,
    pub op_code: u16,
    pub aa: u16,
    pub tc: u16,
    pub rd: u16,
    pub ra: u16,
    pub z: u16,
    pub r_code: u16,
}

impl Flag {
    pub fn to_be_bytes(&self) -> u16 {
        self.qr << 15
            | self.op_code << 11
            | self.aa << 10
            | self.tc << 9
            | self.rd << 8
            | self.ra << 7
            | self.z << 4
            | self.r_code
    }

    pub fn parse(flags: &[u8]) -> Flag {
        let flag = utility::to_u16(flags);
        let r_code = utility::get_bits_range(flag, 0, 4);
        let z = utility::get_bits_range(flag, 4, 7);
        let ra = utility::get_bits_range(flag, 7, 8);
        let rd = utility::get_bits_range(flag, 8, 9);
        let tc = utility::get_bits_range(flag, 9, 10);
        let aa = utility::get_bits_range(flag, 10, 11);
        let op_code = utility::get_bits_range(flag, 11, 15);
        let qr = utility::get_bits_range(flag, 15, 16);

        Flag {
            qr,
            op_code,
            aa,
            tc,
            rd,
            ra,
            z,
            r_code,
        }
    }
}

pub struct Header {
    pub id: u16,
    pub flags: Flag,
    pub qd_cnt: u16,
    pub an_cnt: u16,
    pub ns_cnt: u16,
    pub ar_cnt: u16,
}

impl Header {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut header = vec![];

        let mut bytes = self.id.to_be_bytes().to_vec();
        header.append(&mut bytes);

        bytes = self.flags.to_be_bytes().to_be_bytes().to_vec();
        header.append(&mut bytes);

        bytes = self.qd_cnt.to_be_bytes().to_vec();
        header.append(&mut bytes);

        bytes = self.an_cnt.to_be_bytes().to_vec();
        header.append(&mut bytes);

        bytes = self.ns_cnt.to_be_bytes().to_vec();
        header.append(&mut bytes);

        bytes = self.ar_cnt.to_be_bytes().to_vec();
        header.append(&mut bytes);

        header
    }

    pub fn parse(message: &Vec<u8>, start: usize) -> Result<(usize, Header), Box<dyn Error>> {
        let id = utility::to_u16(&message[start..start + 2]);
        let flags = Flag::parse(&message[start + 2..start + 4]);
        let qd_cnt = utility::to_u16(&message[start + 4..start + 6]);
        let an_cnt = utility::to_u16(&message[start + 6..start + 8]);
        let ns_cnt = utility::to_u16(&message[start + 8..start + 10]);
        let ar_cnt = utility::to_u16(&message[start + 10..start + 12]);

        let h = Header {
            id,
            flags,
            qd_cnt,
            an_cnt,
            ns_cnt,
            ar_cnt,
        };

        Ok((start + 12, h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_header() {
        let dns_flags = Flag {
            qr: 0,
            op_code: 0,
            aa: 0,
            tc: 0,
            rd: 1,
            ra: 0,
            z: 0,
            r_code: 0,
        };

        let dns_header = Header {
            id: 1,
            flags: dns_flags,
            qd_cnt: 1,
            an_cnt: 0,
            ns_cnt: 0,
            ar_cnt: 0,
        };

        assert_eq!(
            dns_header.to_be_bytes(),
            vec![0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }
}
