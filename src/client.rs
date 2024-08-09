use spdlog::prelude::*;
use std::net::{Ipv4Addr, UdpSocket};

use message::DnsMessage;
use rr::ResourceRecord;

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

        debug!(
            "Initialize host at address: {:#?}",
            socket.local_addr().unwrap()
        );

        DnsClient {
            binding_socket: socket,
        }
    }

    /// Query a host name from a DNS server
    pub fn ask(&self, host_name: &str, root_dns_server: &str, max_retries: u32) {
        let dns_question = message::DnsMessage::new(host_name);
        let mut dns_server = root_dns_server.to_string();
        let mut retry = 0;

        while retry < max_retries {
            let conn = self.connect(&dns_server, 53);
            if conn.is_ok() {
                info!("Querying {} for {}", dns_server, host_name);
                self.send(&dns_server, 53, &dns_question.into_bytes());
                let bytes = self.listen().unwrap();
                let dns_response = DnsMessage::parse(&bytes).unwrap();
                debug!(
                    "qd_cnt = {}, an_cnt = {}, ns_cnt = {}, ar_cnt = {}",
                    dns_response.header.qd_cnt,
                    dns_response.header.an_cnt,
                    dns_response.header.ns_cnt,
                    dns_response.header.ar_cnt
                );
                if dns_response.header.an_cnt > 0 {
                    info!("IP Address:");
                    println!(
                        "[\t{}\n]",
                        self.parse_rr(0x01, &dns_response.answers).join("\n\t")
                    );
                    break;
                } else if dns_response.header.ar_cnt > 0 {
                    let auth_servers = self.parse_rr(0x01, &dns_response.additionals);
                    dns_server = auth_servers[0].clone();
                }
            }
            retry += 1;
        }
    }

    /// Parse resource records and transform them into ip addresses
    fn parse_rr(&self, rr_type: u16, rrs: &Vec<ResourceRecord>) -> Vec<String> {
        let mut ip_addrs = vec![];
        for i in 0..rrs.len() {
            if rrs[i].an_type == rr_type {
                ip_addrs.push(self.get_ip_addr(&rrs[i]));
            }
        }
        ip_addrs
    }

    /// Show IP address in DNS answer section
    fn get_ip_addr(&self, rr: &ResourceRecord) -> String {
        rr.an_rdata
            .iter()
            .map(|&seg| seg.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    /// Send a udp message to a remote address
    fn send(&self, remote_addr: &str, port: u16, msg: &[u8]) -> usize {
        let result: usize = 0;
        let addr = format!("{}:{}", remote_addr, port);
        match self.binding_socket.send_to(msg, addr) {
            Ok(number_of_bytes) => {
                debug!(
                    "Send a {}-byte message to address: {}:{}",
                    number_of_bytes, remote_addr, port
                );
            }
            Err(_) => error!(
                "Failed sending message: {}",
                std::str::from_utf8(msg).unwrap().to_string()
            ),
        }

        result
    }

    /// Connect to a remote address on a port
    fn connect(&self, remote_addr: &str, port: u16) -> std::io::Result<()> {
        debug!("Connecting to {}:{}", remote_addr, port);
        let addr = format!("{}:{}", remote_addr, port);
        self.binding_socket.connect(addr)
    }

    /// Listen to a response from a remote address
    fn listen(&self) -> Option<Vec<u8>> {
        let mut buffer = [0; 1024];
        match self.binding_socket.recv_from(&mut buffer) {
            Ok((number_of_bytes, _)) => {
                debug!("Received: {} bytes", number_of_bytes);
                let result = Vec::from(&buffer[0..number_of_bytes]);
                Some(result)
            }
            Err(_) => None,
        }
    }
}
