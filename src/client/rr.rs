use std::error::Error;

use crate::client::utility;

pub struct ResourceRecord {
    pub an_name: Vec<u8>,
    pub an_type: u16,
    pub an_class: u16,
    pub an_ttl: u32,
    pub an_rdlength: u16,
    pub an_rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut reply = vec![];

        let mut bytes = self.an_name.clone();
        reply.append(&mut bytes);

        bytes = self.an_type.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_class.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_ttl.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_rdlength.to_be_bytes().to_vec();
        reply.append(&mut bytes);

        bytes = self.an_rdata.clone();
        reply.append(&mut bytes);

        reply
    }

    pub fn parse(resource_record: &[u8]) -> Result<(usize, ResourceRecord), Box<dyn Error>> {
        let null_pos = utility::find_first_null(resource_record)?;
        let an_name = resource_record[0..null_pos + 1].to_vec();
        let an_type = utility::to_u16(&resource_record[null_pos + 1..null_pos + 3]);
        let an_class = utility::to_u16(&resource_record[null_pos + 3..null_pos + 5]);
        let an_ttl = utility::to_u32(&resource_record[null_pos + 5..null_pos + 9]);
        let an_rdlength = utility::to_u16(&resource_record[null_pos + 9..null_pos + 11]);
        let an_rdata = resource_record[null_pos + 11..].to_vec();

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
