// Example use of the API

use pubip::fetch_ip;

#[tokio::main]
async fn main() {
    println!("Getting your address...");
    match fetch_ip().await {
        Ok(ip) => println!("Your public IP address is: {}", ip),
        Err(e) => eprintln!("Error fetching IP: {}", e),
    }
}
