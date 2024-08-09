use std::error::Error;

use crate::client::utility;

/// DNS resource record
pub struct ResourceRecord {
    /// A domain name to which this resource record pertains
    pub an_name: Vec<u8>,
    /// RR type codes specifying the meaning in rdata
    pub an_type: u16,
    /// Class of the data in rdata
    pub an_class: u16,
    /// Time interval in seconds
    pub an_ttl: u32,
    /// Length of rdata field
    pub an_rdlength: u16,
    /// A variable length string describing the resource. The format
    /// varies according to the type and class of the resource record
    pub an_rdata: Vec<u8>,
}

impl ResourceRecord {
    /// Transform a resource record to a vector of bytes
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

    /// Parse a vector of bytes into a resource record
    pub fn parse(
        message: &Vec<u8>,
        start: usize,
    ) -> Result<(usize, ResourceRecord), Box<dyn Error>> {
        let is_pointer = message[start] & 0xC0;
        if is_pointer == 0xC0 {
            let offset = (utility::to_u16(&message[start..start + 2]) & 0x3F) as usize;
            let null_pos = utility::find_first_null(&message[offset as usize..])?;
            let an_name = message[offset as usize.. offset + null_pos + 1].to_vec();
            let an_type = utility::to_u16(&message[start + 2..start + 4]);
            let an_class = utility::to_u16(&message[start + 4..start + 6]);
            let an_ttl = utility::to_u32(&message[start + 6..start + 10]);
            let an_rdlength = utility::to_u16(&message[start + 10..start + 12]);
            let an_rdata = message[start + 12..start + 12 + an_rdlength as usize].to_vec();

            let rr = ResourceRecord {
                an_name,
                an_type,
                an_class,
                an_ttl,
                an_rdlength,
                an_rdata,
            };
            Ok((start + 12 + an_rdlength as usize, rr))
        } else {
            let null_pos = utility::find_first_null(&message[start..])?;
            let offset = start + null_pos;
            let an_name = message[start..offset + 1].to_vec();
            let an_type = utility::to_u16(&message[offset + 1..offset + 3]);
            let an_class = utility::to_u16(&message[offset + 3..offset + 5]);
            let an_ttl = utility::to_u32(&message[offset + 5..offset + 9]);
            let an_rdlength = utility::to_u16(&message[offset + 9..offset + 11]);
            let an_rdata = message[offset + 11..offset + 11 + an_rdlength as usize].to_vec();

            let rr = ResourceRecord {
                an_name,
                an_type,
                an_class,
                an_ttl,
                an_rdlength,
                an_rdata,
            };
            Ok((offset + 11 + an_rdlength as usize, rr))
        }
    }
}
