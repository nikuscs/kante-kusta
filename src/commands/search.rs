//! Search command (via HTML scraping)

use crate::api::{KuantoKustaClient, SearchResult};
use crate::format::{format_products, OutputFormat};
use anyhow::Result;

/// Execute search command
pub async fn search(
    client: &KuantoKustaClient,
    query: &str,
    max: usize,
    format: OutputFormat,
) -> Result<String> {
    let result = client.search(query, max).await?;
    format_search_result(&result, query, format)
}

/// Format search result (exported for testing)
pub fn format_search_result(
    result: &SearchResult,
    query: &str,
    format: OutputFormat,
) -> Result<String> {
    let header = format!("Found {} products for \"{}\":\n\n", result.total, query);
    let products = format_products(&result.products, format);
    Ok(format!("{header}{products}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Badges, Product, Tags};

    fn sample_search_result() -> SearchResult {
        SearchResult {
            products: vec![Product {
                id: 12345,
                name: "Test Product".to_string(),
                brand: "TestBrand".to_string(),
                category: "Electronics".to_string(),
                price_min: 99.99,
                total_offers: 5,
                url: "/p/test".to_string(),
                images: vec![],
                badges: Badges::default(),
                rating: None,
                tags: Tags::default(),
            }],
            total: 100,
        }
    }

    #[test]
    fn test_format_search_result_table() {
        let result = sample_search_result();
        let output = format_search_result(&result, "test", OutputFormat::Table);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.contains("Found 100 products for \"test\""));
        assert!(output.contains("12345"));
        assert!(output.contains("Test Product"));
    }

    #[test]
    fn test_format_search_result_json() {
        let result = sample_search_result();
        let output = format_search_result(&result, "iphone", OutputFormat::Json);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.contains("Found 100 products for \"iphone\""));
        assert!(output.contains("\"id\": 12345"));
    }

    #[test]
    fn test_format_search_result_empty() {
        let result = SearchResult { products: vec![], total: 0 };
        let output = format_search_result(&result, "nothing", OutputFormat::Table);

        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.contains("Found 0 products"));
        assert!(output.contains("No products found"));
    }
}
