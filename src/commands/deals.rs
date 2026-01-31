//! Deals command

use crate::api::KuantoKustaClient;
use crate::format::{format_deals, OutputFormat};
use anyhow::Result;

/// Execute deals command
pub async fn deals(
    client: &KuantoKustaClient,
    max: u32,
    min_discount: Option<u8>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    format: OutputFormat,
) -> Result<String> {
    let response = client.deals(max, 1, min_discount, min_price, max_price).await?;

    let header = format!("Found {} deals:\n\n", response.total);
    let deals = format_deals(&response.data, format);

    Ok(format!("{header}{deals}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_deals_response() -> serde_json::Value {
        serde_json::json!({
            "data": [{
                "id": 67890,
                "name": "Deal Product",
                "priceMin": 49.99,
                "totalOffers": 3,
                "badges": {"discountPercentage": 25},
                "tags": {}
            }],
            "page": 1,
            "rows": 20,
            "total": 1
        })
    }

    #[tokio::test]
    async fn test_deals_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/deals"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_deals_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = deals(&client, 20, None, None, None, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Found 1 deals"));
        assert!(output.contains("67890"));
        assert!(output.contains("Deal Product"));
    }

    #[tokio::test]
    async fn test_deals_command_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/deals"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_deals_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result =
            deals(&client, 20, Some(10), Some(10.0), Some(100.0), OutputFormat::Json).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("\"id\": 67890"));
    }
}
