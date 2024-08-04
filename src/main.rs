pub mod client;

fn main() {
    let dns_client = client::DnsClient::new();
    dns_client.ask("dns.google.com", "8.8.8.8");
}
