//! HTML scraper for search (extracts __`NEXT_DATA`__ JSON)
//! Uses wreq for TLS fingerprint emulation to bypass CDN protection.

use super::models::Product;
use anyhow::{Context, Result};
use serde::Deserialize;
use wreq::Client;
use wreq_util::Emulation;

const WEB_BASE: &str = "https://www.kuantokusta.pt";

/// Search response from __`NEXT_DATA`__
#[derive(Debug, Deserialize)]
struct NextData {
    props: Props,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Props {
    page_props: PageProps,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageProps {
    base_page: BasePage,
}

#[derive(Debug, Deserialize)]
struct BasePage {
    data: Vec<Product>,
    total: u64,
}

/// Search result
#[derive(Debug)]
pub struct SearchResult {
    pub products: Vec<Product>,
    pub total: u64,
}

/// Scrape search results from HTML using wreq (TLS fingerprinting)
pub async fn search(query: &str, max: usize) -> Result<SearchResult> {
    search_with_base_url(query, max, WEB_BASE).await
}

/// Scrape search results with a custom base URL (for testing)
pub async fn search_with_base_url(query: &str, max: usize, base_url: &str) -> Result<SearchResult> {
    let url = format!("{base_url}/search?q={}", urlencoding::encode(query));

    // Use wreq for TLS fingerprint emulation
    let client = Client::builder()
        .cookie_store(true)
        .gzip(true)
        .brotli(true)
        .build()
        .context("Failed to create wreq client")?;

    let html = client
        .get(&url)
        .emulation(Emulation::Chrome131)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "pt-PT,pt;q=0.9,en;q=0.8")
        .send()
        .await
        .context("Failed to fetch search page")?
        .text()
        .await
        .context("Failed to read search response")?;

    parse_search_html(&html, max)
}

/// Parse search results from HTML (exported for testing)
pub fn parse_search_html(html: &str, max: usize) -> Result<SearchResult> {
    // Check for access denied
    if html.contains("Access Denied") {
        anyhow::bail!("Access denied by CDN - please try again later");
    }

    // Extract __NEXT_DATA__ JSON
    let start = html
        .find(r#"<script id="__NEXT_DATA__" type="application/json">"#)
        .context("Could not find __NEXT_DATA__ in page")?;

    let json_start = start + r#"<script id="__NEXT_DATA__" type="application/json">"#.len();
    let json_end =
        html[json_start..].find("</script>").context("Could not find end of __NEXT_DATA__")?;

    let json_str = &html[json_start..json_start + json_end];

    let next_data: NextData =
        serde_json::from_str(json_str).context("Failed to parse __NEXT_DATA__ JSON")?;

    let products: Vec<Product> =
        next_data.props.page_props.base_page.data.into_iter().take(max).collect();

    Ok(SearchResult { products, total: next_data.props.page_props.base_page.total })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_next_data_html() -> String {
        let next_data = serde_json::json!({
            "props": {
                "pageProps": {
                    "basePage": {
                        "data": [{
                            "id": 12345,
                            "name": "Test iPhone",
                            "brand": "Apple",
                            "priceMin": 999.99,
                            "totalOffers": 10,
                            "url": "/p/test-iphone",
                            "images": [],
                            "badges": {},
                            "tags": {}
                        }],
                        "total": 100
                    }
                }
            }
        });

        format!(
            r#"<!DOCTYPE html>
<html>
<head><title>Search</title></head>
<body>
<script id="__NEXT_DATA__" type="application/json">{next_data}</script>
</body>
</html>"#
        )
    }

    #[test]
    fn test_parse_search_html_success() {
        let html = mock_next_data_html();
        let result = parse_search_html(&html, 10);

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.total, 100);
        assert_eq!(search_result.products.len(), 1);
        assert_eq!(search_result.products[0].id, 12345);
        assert_eq!(search_result.products[0].name, "Test iPhone");
    }

    #[test]
    fn test_parse_search_html_max_limit() {
        let html = mock_next_data_html();
        let result = parse_search_html(&html, 0);

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.products.len(), 0); // Limited to 0
        assert_eq!(search_result.total, 100); // Total still reported
    }

    #[test]
    fn test_parse_search_html_access_denied() {
        let html = "<html><body>Access Denied</body></html>";
        let result = parse_search_html(html, 10);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Access denied"));
    }

    #[test]
    fn test_parse_search_html_no_next_data() {
        let html = "<html><body>No data here</body></html>";
        let result = parse_search_html(html, 10);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Could not find __NEXT_DATA__"));
    }

    #[test]
    fn test_parse_search_html_invalid_json() {
        let html = r#"<script id="__NEXT_DATA__" type="application/json">invalid json</script>"#;
        let result = parse_search_html(html, 10);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_search_html_missing_script_end() {
        let html = r#"<script id="__NEXT_DATA__" type="application/json">{"test": true}"#;
        let result = parse_search_html(html, 10);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Could not find end"));
    }

    #[tokio::test]
    async fn test_search_with_mock_server() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .respond_with(ResponseTemplate::new(200).set_body_string(mock_next_data_html()))
            .mount(&mock_server)
            .await;

        let result = search_with_base_url("iphone", 10, &mock_server.uri()).await;

        assert!(result.is_ok());
        let search_result = result.unwrap();
        assert_eq!(search_result.total, 100);
        assert_eq!(search_result.products[0].name, "Test iPhone");
    }

    #[tokio::test]
    async fn test_search_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = search_with_base_url("iphone", 10, &mock_server.uri()).await;

        // wreq should handle 500 errors
        assert!(result.is_err());
    }
}
