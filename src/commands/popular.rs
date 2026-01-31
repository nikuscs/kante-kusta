//! Popular products command

use crate::api::KuantoKustaClient;
use crate::format::{format_products, OutputFormat};
use anyhow::Result;

/// Execute popular products command
pub async fn popular(
    client: &KuantoKustaClient,
    category_id: u64,
    max: u32,
    format: OutputFormat,
) -> Result<String> {
    let products = client.popular(category_id, max).await?;

    let header = format!("Popular products in category {category_id}:\n\n");
    let output = format_products(&products, format);

    Ok(format!("{header}{output}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_popular_response() -> serde_json::Value {
        serde_json::json!([{
            "id": 22222,
            "name": "Popular Product",
            "priceMin": 299.99,
            "totalOffers": 15,
            "badges": {},
            "tags": {}
        }])
    }

    #[tokio::test]
    async fn test_popular_command() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/popular"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_popular_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = popular(&client, 155, 10, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Popular products in category 155"));
        assert!(output.contains("Popular Product"));
    }
}
