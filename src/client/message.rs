use crate::client::header::{Flag, Header};
use crate::client::question::Question;
use crate::client::rr::ResourceRecord;
use std::error::Error;

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
            q_name: DnsMessage::encode_address(address),
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
        let mut start = 0;
        let parsed_value= Header::parse(&message, start)?;
        start = parsed_value.0;
        let header = parsed_value.1;

        let parsed_value = Question::parse(&message, start)?;
        start = parsed_value.0;
        let question = parsed_value.1;

        let mut answers = vec![];
        for _ in 0..header.an_cnt {
            let answer = ResourceRecord::parse(&message, start)?;
            answers.push(answer.1);
            start = answer.0;
        }

        let mut authorities = vec![];
        for _ in 0..header.ns_cnt {
            let authority = ResourceRecord::parse(&message, start)?;
            authorities.push(authority.1);
            start = authority.0;
        }

        let mut additionals = vec![];
        for _ in 0..header.ar_cnt {
            let additional = ResourceRecord::parse(&message, start)?;
            additionals.push(additional.1);
            start = additional.0;
        }

        let dns_message = DnsMessage {
            header,
            question,
            answers,
            authorities,
            additionals,
        };

        Ok(dns_message)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_valid_address() {
        let enc_addr = DnsMessage::encode_address("dns.google.com");
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
        let enc_addr = DnsMessage::encode_address("dns.google.com");
        assert_eq!(DnsMessage::decode_address(&enc_addr), "dns.google.com");
    }

    #[test]
    fn encode_invalid_address() {
        let enc_addr = DnsMessage::encode_address("abc");
        assert_eq!(enc_addr[0..5], [3, 'a' as u8, 'b' as u8, 'c' as u8, 0]);
    }

    #[test]
    fn decode_invalid_address() {
        let enc_addr = DnsMessage::encode_address("abc");
        assert_eq!(DnsMessage::decode_address(&enc_addr), "abc");
    }

    #[test]
    fn encode_another_invalid_address() {
        let enc_addr = DnsMessage::encode_address(".abc");
        assert_eq!(enc_addr[0..5], [3, 'a' as u8, 'b' as u8, 'c' as u8, 0]);
    }

    #[test]
    fn parse_dns_response() {
        let response_bytes = vec![
            0x00, 0x16, 0x80, 0x80, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64,
            0x6e, 0x73, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
            0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x14,
            0x00, 0x04, 0x08, 0x08, 0x08, 0x08, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0x02, 0x14, 0x00, 0x04, 0x08, 0x08, 0x04, 0x04,
        ];

        let dns_response = DnsMessage::parse(&response_bytes).unwrap();
        let q_name = DnsMessage::decode_address(&dns_response.question.q_name);
        println!("address: {}", q_name);
        let answers = dns_response.answers;
        println!("IP Address:");
        for answer in answers {
            let ip_addr = answer.an_rdata.iter().map(|&seg| seg.to_string()).collect::<Vec<String>>().join(".");
            println!("{}", ip_addr);
        }
    }
}
