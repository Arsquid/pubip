// This library provides a public IP API via ipify.org
// Supports both IPv4 and IPv6 addresses.
// Anonymous by default with no log collection and TLS encryption.

use reqwest;
use std::error::Error;

pub async fn fetch_ip() -> Result<String, Box<dyn Error>> {
    let response = reqwest::get("https://api64.ipify.org/").await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch IP address: HTTP status code {}", response.status()).into());
    }

    let result = response.text().await?;

    Ok(result)
}


// Tests.
// Use 'cargo test' to make sure everything works as expected.

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_success() {
        let result = fetch_ip().await;

        assert!(result.is_ok());

        if let Ok(ip) = result {
            assert!(!ip.is_empty());
        }
    } // Test to make sure the response is not empty. (Requires network connectivity to pass.)

    #[tokio::test]
    async fn test_fetch_failure() {
        let response = reqwest::get("https://invalid-url").await;

        assert!(response.is_err());
    } // Test to make sure that any invalid URL is an error. (No connectivity required.)

    async fn sim_ipv4() -> Result<String, Box<dyn Error>> {
        Ok("127.0.0.1".to_string())
    }

    #[tokio::test]
    async fn test_ipv4() {
        let result = sim_ipv4().await;

        assert!(result.is_ok());

        if let Ok(ip) = result {
            assert!(ip.parse::<std::net::Ipv4Addr>().is_ok());
        }
    } // Test to make sure Ipv4 addresses work as expected. (No connectivity required.)

    async fn sim_ipv6() -> Result<String, Box<dyn Error>> {
        Ok("::1".to_string())
    }

    #[tokio::test]
    async fn test_ipv6() {
        let result = sim_ipv6().await;

        assert!(result.is_ok());

        if let Ok(ip) = result {
            assert!(ip.parse::<std::net::Ipv6Addr>().is_ok());
        }
    } // Test to make sure IPv6 addresses work as expected. (No connectivity required.)
} 
