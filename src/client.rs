use std::net::{Ipv4Addr, UdpSocket};

use message::DnsMessage;

pub mod header;
pub mod message;
pub mod question;
pub mod rr;
pub mod utility;

/// A DNS client to query for a host name
pub struct DnsClient {
    binding_socket: UdpSocket,
}

impl DnsClient {
    /// Create a new DNS client
    pub fn new() -> DnsClient {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).expect("Can't create socket!");

        println!(
            "Initialize host at address: {:#?}",
            socket.local_addr().unwrap()
        );

        DnsClient {
            binding_socket: socket,
        }
    }

    /// Query a host name from a DNS server
    pub fn ask(&self, name: &str, dns_server: &str) {
        let dns_question = message::DnsMessage::new(name);
        let conn = self.connect(dns_server, 53);
        if conn.is_ok() {
            self.send(dns_server, 53, &dns_question.into_bytes());
            let bytes = self.listen().unwrap();
            let dns_response = DnsMessage::parse(&bytes).unwrap();
            println!(
                "Address: {}",
                DnsMessage::decode_address(&dns_response.question.q_name)
            );
            println!("IP Address:");
            for answer in dns_response.answers {
                let ip_addr = answer
                    .an_rdata
                    .iter()
                    .map(|&seg| seg.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                println!("{}", ip_addr);
            }
        }
    }

    /// Send a udp message to a remote address
    fn send(&self, remote_addr: &str, port: u16, msg: &[u8]) -> usize {
        let result: usize = 0;
        let addr = format!("{}:{}", remote_addr, port);
        match self.binding_socket.send_to(msg, addr) {
            Ok(number_of_bytes) => {
                println!(
                    "Send a {}-byte message to address: {}:{}",
                    number_of_bytes, remote_addr, port
                );
            }
            Err(_) => println!(
                "Failed sending message: {}",
                std::str::from_utf8(msg).unwrap().to_string()
            ),
        }

        result
    }

    /// Connect to a remote address on a port
    fn connect(&self, remote_addr: &str, port: u16) -> std::io::Result<()> {
        println!("Connecting to {}:{}", remote_addr, port);
        let addr = format!("{}:{}", remote_addr, port);
        self.binding_socket.connect(addr)
    }

    /// Listen to a response from a remote address
    fn listen(&self) -> Option<Vec<u8>> {
        let mut buffer = [0; 1024];
        match self.binding_socket.recv_from(&mut buffer) {
            Ok((number_of_bytes, _)) => {
                println!("Received: {} bytes", number_of_bytes);
                let result = Vec::from(&buffer[0..number_of_bytes]);
                Some(result)
            }
            Err(_) => None,
        }
    }
}
