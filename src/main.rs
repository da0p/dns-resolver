use clap::Parser;

pub mod client;

#[derive(Parser, Debug)]
struct Options {
    /// Host name that is needed to resolve
    host: String,
    /// DNS server
    dns_server: String,
}

fn main() {
    let options = Options::parse();
    let dns_client = client::DnsClient::new();
    dns_client.ask(&options.host, &options.dns_server);
}
