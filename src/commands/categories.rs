//! Categories command

use crate::api::KuantoKustaClient;
use crate::format::{format_categories, OutputFormat};
use anyhow::Result;

/// Execute categories command
pub async fn categories(
    client: &KuantoKustaClient,
    parent: Option<u64>,
    format: OutputFormat,
) -> Result<String> {
    let all_categories = client.categories().await?;

    // Filter by parent if specified
    let filtered: Vec<_> = if let Some(parent_id) = parent {
        all_categories.into_iter().filter(|c| c.parent_id == Some(parent_id)).collect()
    } else {
        // Show top-level categories (no parent)
        all_categories.into_iter().filter(|c| c.parent_id.is_none()).collect()
    };

    let header = parent.map_or_else(
        || "Top-level categories:\n\n".to_string(),
        |parent_id| format!("Subcategories of {parent_id}:\n\n"),
    );

    let output = format_categories(&filtered, format);

    Ok(format!("{header}{output}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_categories_response() -> serde_json::Value {
        serde_json::json!([
            {"id": 1, "label": "Electronics", "slug": "electronics", "hasChild": true, "url": "/c/electronics"},
            {"id": 155, "parentId": 1, "label": "Smartphones", "slug": "smartphones", "hasChild": false, "url": "/c/smartphones"}
        ])
    }

    #[tokio::test]
    async fn test_categories_top_level() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_categories_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = categories(&client, None, OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Top-level categories"));
        assert!(output.contains("Electronics"));
        // Smartphones has a parent, so it shouldn't appear
        assert!(!output.contains("Smartphones"));
    }

    #[tokio::test]
    async fn test_categories_with_parent() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_categories_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = categories(&client, Some(1), OutputFormat::Table).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Subcategories of 1"));
        assert!(output.contains("Smartphones"));
        // Electronics has no parent, so it shouldn't appear
        assert!(!output.contains("Electronics"));
    }

    #[tokio::test]
    async fn test_categories_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_categories_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = categories(&client, None, OutputFormat::Json).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("\"id\": 1"));
    }
}
