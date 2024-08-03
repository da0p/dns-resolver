pub mod dns;

use std::{
    net::{Ipv4Addr, UdpSocket},
    thread, time,
};

fn init_host() -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

    println!(
        "Initialize host at address: {:#?}",
        socket.local_addr().unwrap()
    );

    Ok(socket)
}

fn send(socket: &UdpSocket, remote_addr: &str, port: u16, msg: &[u8]) -> usize {
    let result: usize = 0;
    let addr = format!("{}:{}", remote_addr, port);
    match socket.send_to(msg, addr) {
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

fn connect(socket: &UdpSocket, remote_addr: &str, port: u16) -> std::io::Result<()> {
    println!("Connecting to {}:{}", remote_addr, port);
    let addr = format!("{}:{}", remote_addr, port);
    socket.connect(addr)
}

fn listen(socket: &UdpSocket) -> Option<Vec<u8>> {
    let mut buffer = [0; 1024];
    match socket.recv_from(&mut buffer) {
        Ok((number_of_bytes, _)) => {
            println!("Received: {} bytes", number_of_bytes);
            let result = Vec::from(&buffer[0..number_of_bytes]);
            Some(result)
        }
        Err(_) => None,
    }
}

fn main() {
    let retry_time = time::Duration::from_secs(5);
    let socket = init_host().expect("Can not initialize host");

    loop {
        let result = connect(&socket, "8.8.8.8", 53);
        if result.is_ok() {
            let dns_msg = dns::DnsMessage::new("dns.google.com");
            send(&socket, "8.8.8.8", 53, &dns_msg.into_bytes());
            println!("Listening...");
            let result = listen(&socket);
        }

        thread::sleep(retry_time);
    }
}
