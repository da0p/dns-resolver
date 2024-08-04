use std::error::Error;

use crate::dns::utility;

pub struct ResourceRecord {
    pub an_name: String,
    pub an_type: u16,
    pub an_class: u16,
    pub an_ttl: u32,
    pub an_rdlength: u16,
    pub an_rdata: String,
}

impl ResourceRecord {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut reply = vec![];

        let mut bytes = self.an_name.clone().into_bytes();
        reply.append(&mut bytes);

        bytes = self.an_type.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_class.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_ttl.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_rdlength.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_rdata.clone().into_bytes();
        reply.append(&mut bytes);

        reply
    }

    pub fn parse(resource_record: &[u8]) -> Result<(usize, ResourceRecord), Box<dyn Error>> {
        let null_pos = resource_record
            .iter()
            .position(|&x| x == 0x00)
            .ok_or("Can't find null character!")?;
        let an_name = utility::decode_address(&resource_record[0..null_pos + 1].to_vec());
        let an_type = utility::to_u16(&resource_record[null_pos + 2..null_pos + 4]);
        let an_class = utility::to_u16(&resource_record[null_pos + 4..null_pos + 6]);
        let an_ttl = utility::to_u32(&resource_record[null_pos + 6..null_pos + 10]);
        let an_rdlength = utility::to_u16(&resource_record[null_pos + 10..null_pos + 12]);
        let an_rdata = String::from_utf8(resource_record[null_pos + 12..].to_vec()).unwrap();

        let rr = ResourceRecord {
            an_name,
            an_type,
            an_class,
            an_ttl,
            an_rdlength,
            an_rdata,
        };

        Ok((null_pos + 1, rr))
    }
}
