//! Browse command - returns popular products

use crate::api::KuantoKustaClient;
use crate::format::{format_products, OutputFormat};
use anyhow::Result;

/// Execute browse command (popular products)
/// Note: `KuantoKusta` search is SSR-only, this returns popular products instead
pub async fn browse(client: &KuantoKustaClient, max: u32, format: OutputFormat) -> Result<String> {
    let response = client.products(max).await?;

    let header = format!("Popular products ({} total):\n\n", response.total);
    let products = format_products(&response.data, format);

    Ok(format!("{header}{products}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_products_response() -> serde_json::Value {
        serde_json::json!({
            "data": [{
                "id": 12345,
                "name": "Test Product",
                "priceMin": 99.99,
                "totalOffers": 5,
                "badges": {},
                "tags": {}
            }],
            "page": 1,
            "rows": 20,
            "total": 1
        })
    }

    #[tokio::test]
    async fn test_browse_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_products_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = browse(&client, 20, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Popular products"));
        assert!(output.contains("Test Product"));
    }
}
