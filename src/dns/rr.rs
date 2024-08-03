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

    pub fn parse(resource_record: &Vec<u8>) -> ResourceRecord {
        let null_pos = resource_record.iter().position(|&x| x == 0x00);
        if null_pos.is_none() {
            return ResourceRecord::default();
        }

        let term_pos = null_pos.unwrap();
        let an_name = utility::decode_address(&resource_record[0..term_pos + 1].to_vec());
        let an_type = utility::to_u16(&resource_record[term_pos + 2..term_pos + 4]);
        let an_class = utility::to_u16(&resource_record[term_pos + 4..term_pos + 6]);
        let an_ttl = utility::to_u32(&resource_record[term_pos + 6..term_pos + 10]);
        let an_rdlength = utility::to_u16(&resource_record[term_pos + 10..term_pos + 12]);
        let an_rdata = String::from_utf8(resource_record[term_pos + 12..].to_vec()).unwrap();

        ResourceRecord{
            an_name,
            an_type,
            an_class,
            an_ttl,
            an_rdlength,
            an_rdata
        }
    }
}

impl Default for ResourceRecord {
    fn default() -> ResourceRecord {
        ResourceRecord {
            an_name: String::from(""),
            an_type: 0,
            an_class: 0,
            an_ttl: 0,
            an_rdlength: 0,
            an_rdata: String::from(""),
        }
    }
}