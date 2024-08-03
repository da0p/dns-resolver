
pub mod header;
pub mod question;
pub mod rr;
pub mod utility;

use header::{Flag, Header};
use question::Question;
use rr::ResourceRecord;

pub struct DnsMessage {
    pub header: Header,
    pub question: Question,
    pub answer: ResourceRecord,
    pub authority: ResourceRecord,
    pub additional: ResourceRecord,
}

impl Default for DnsMessage {
    fn default() -> DnsMessage {
        let dns_flags = Flag {
            qr: 0,
            op_code: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            r_code: 0,
        };

        let dns_header = Header {
            id: 0,
            flags: dns_flags,
            qd_cnt: 0,
            an_cnt: 0,
            ns_cnt: 0,
            ar_cnt: 0,
        };

        let dns_question = Question {
            q_name: vec![],
            q_type: 0,
            q_class: 0,
        };

        let dns_msg = DnsMessage {
            header: dns_header,
            question: dns_question,
            answer: ResourceRecord::default(),
            authority: ResourceRecord::default(),
            additional: ResourceRecord::default(),
        };

        dns_msg
    }
}

impl DnsMessage {
    pub fn new(address: &str) -> DnsMessage {
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
            id: rand::random::<u16>(),
            flags: dns_flags,
            qd_cnt: 1,
            an_cnt: 0,
            ns_cnt: 0,
            ar_cnt: 0,
        };

        let dns_question = Question {
            q_name: utility::encode_address(address),
            q_type: 1,
            q_class: 1,
        };

        let dns_msg = DnsMessage {
            header: dns_header,
            question: dns_question,
            answer: ResourceRecord::default(),
            authority: ResourceRecord::default(),
            additional: ResourceRecord::default(),
        };

        dns_msg
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut msg = vec![];

        let mut bytes = self.header.to_be_bytes();
        msg.append(&mut bytes);

        bytes = self.question.to_be_bytes();
        msg.append(&mut bytes);

        bytes = self.answer.to_be_bytes();
        msg.append(&mut bytes);

        bytes = self.authority.to_be_bytes();
        msg.append(&mut bytes);

        bytes = self.additional.to_be_bytes();
        msg.append(&mut bytes);

        msg
    }

    pub fn into_bytes(&self) -> [u8; 128] {
        let bytes = self.to_be_bytes();
        let mut buf = [0; 128];
        for i in 0..bytes.len() {
            buf[i] = bytes[i];
        }

        buf
    }
}