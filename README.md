# Pubip - A rust library for fetching public IP addresses

**Pubip** is a simple and lightweight Rust library that provides an easy way to retrieve the public IP address of the host. It supports both IPv4 and IPv6 addresses and ensures anonymity by not collecting logs and using TLS encryption for secure communication.

## Features

- **IPv4 and IPv6 Support**: Automatically detects and returns the host's public IP address.
- **Asynchronous API**: Built with `async/await` using `tokio` and `reqwest`.
- **Secure and Anonymous**: Uses TLS encryption and does not collect logs.
- **Error Handling**: Provides clear error messages if something goes wrong.

## Installation

To use **pubip** in your Rust project, add it as a dependency in your `Cargo.toml`:

```
[dependencies]
tokio = "1.39.2"
pubip = { git = "https://github.com/Arsquid/pubip" }
```

## Usage

Here is an example code to utilize **pubip**:

```
use pubip::fetch_ip;

#[tokio::main]
async fn main() {
    println!("Getting your address...");
    match fetch_ip().await {
        Ok(ip) => println!("Your public IP address is: {}", ip),
        Err(e) => eprintln!("Error fetching IP: {}", e),
    }
}
```

## Testing

Use `cargo test` to utilize the unit tests.

These include:
- Success or failure for fetching the IP address.
- Simulated tests for both IPv4 and IPv6 address formats.


Contributions are welcome!

## Kudos

- [ipify](https://ipify.org/) for API
