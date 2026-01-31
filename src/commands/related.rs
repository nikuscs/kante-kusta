//! Related products command

use crate::api::KuantoKustaClient;
use crate::format::{format_products, OutputFormat};
use anyhow::Result;

/// Execute related products command
pub async fn related(
    client: &KuantoKustaClient,
    product_id: u64,
    max: u32,
    format: OutputFormat,
) -> Result<String> {
    let response = client.related(product_id).await?;

    let products: Vec<_> = response.data.into_iter().take(max as usize).collect();
    let header = format!("Related products for {} ({} total):\n\n", product_id, response.count);
    let output = format_products(&products, format);

    Ok(format!("{header}{output}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_related_response() -> serde_json::Value {
        serde_json::json!({
            "data": [{
                "id": 11111,
                "name": "Related Product",
                "priceMin": 199.99,
                "totalOffers": 10,
                "badges": {},
                "tags": {}
            }],
            "count": 1
        })
    }

    #[tokio::test]
    async fn test_related_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/12345/related"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_related_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = related(&client, 12345, 10, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Related products for 12345"));
        assert!(output.contains("1 total"));
        assert!(output.contains("Related Product"));
    }
}
