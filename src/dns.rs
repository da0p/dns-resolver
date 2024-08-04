use std::error::Error;

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
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
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
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        };

        dns_msg
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut msg = vec![];

        let mut bytes = self.header.to_be_bytes();
        msg.append(&mut bytes);

        bytes = self.question.to_be_bytes();
        msg.append(&mut bytes);

        for i in 0..self.header.an_cnt {
            bytes = self.answers[i as usize].to_be_bytes();
            msg.append(&mut bytes);
        }

        for i in 0..self.header.ns_cnt {
            bytes = self.authorities[i as usize].to_be_bytes();
            msg.append(&mut bytes);
        }

        for i in 0..self.header.ar_cnt {
            bytes = self.additionals[i as usize].to_be_bytes();
            msg.append(&mut bytes);
        }

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

    pub fn parse(message: &Vec<u8>) -> Result<DnsMessage, Box<dyn Error>> {
        let (start, header) = Header::parse(&message[0..96])?;

        let (mut start, question) = Question::parse(&message[start..])?;

        let mut answers = vec![];
        for _ in 0..header.an_cnt {
            let answer = ResourceRecord::parse(&message[start..])?;
            answers.push(answer.1);
            start = answer.0;
        }

        let mut authorities = vec![];
        for _ in 0..header.ns_cnt {
            let authority = ResourceRecord::parse(&message[start..])?;
            authorities.push(authority.1);
            start = authority.0;
        }

        let mut additionals = vec![];
        for _ in 0..header.ar_cnt {
            let additional = ResourceRecord::parse(&message[start..])?;
            additionals.push(additional.1);
            start = additional.0;
        }

        let dns_message = DnsMessage {
            header,
            question,
            answers,
            authorities,
            additionals
        };

        Ok(dns_message)
    }
}

