//! Price history command

use crate::api::KuantoKustaClient;
use crate::format::{format_history, OutputFormat};
use anyhow::Result;

/// Execute price history command
pub async fn history(
    client: &KuantoKustaClient,
    product_id: u64,
    days: u32,
    format: OutputFormat,
) -> Result<String> {
    let response = client.price_history(product_id, days).await?;

    let header = format!("Price history for product {product_id} ({days} days):\n\n");
    let history = format_history(&response, format);

    Ok(format!("{header}{history}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_history_response() -> serde_json::Value {
        serde_json::json!({
            "minAxis": 500.0,
            "maxAxis": 800.0,
            "data": [{"date": "2024-01-01", "avg": 650.0, "min": 600.0}]
        })
    }

    #[tokio::test]
    async fn test_history_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/12345/price-history"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_history_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = history(&client, 12345, 30, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Price history for product 12345"));
        assert!(output.contains("30 days"));
        assert!(output.contains("2024-01-01"));
    }

    #[tokio::test]
    async fn test_history_command_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/12345/price-history"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_history_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = history(&client, 12345, 90, OutputFormat::Json).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("\"minAxis\": 500.0"));
    }
}
