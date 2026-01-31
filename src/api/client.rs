//! HTTP client for `KuantoKusta` API

use super::models::{
    Category, DealsResponse, PriceHistory, Product, ProductsResponse, RelatedResponse,
};
use anyhow::{Context, Result};
use reqwest::Client;

const API_BASE: &str = "https://api.kuantokusta.pt";

/// `KuantoKusta` API client
#[derive(Debug, Clone)]
pub struct KuantoKustaClient {
    client: Client,
    base_url: String,
}

impl KuantoKustaClient {
    /// Create a new client
    pub fn new() -> Result<Self> {
        Self::with_base_url(API_BASE)
    }

    /// Create a client with a custom base URL (for testing)
    pub fn with_base_url(base_url: &str) -> Result<Self> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; kuantokusta-cli/0.1)")
            .gzip(true)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, base_url: base_url.to_string() })
    }

    /// Get products (popular products - note: search is SSR-only)
    pub async fn products(&self, rows: u32) -> Result<ProductsResponse> {
        let url = format!("{}/products", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("rows", &rows.to_string())])
            .send()
            .await
            .context("Failed to fetch products")?;

        resp.json().await.context("Failed to parse products response")
    }

    /// Get deals/discounts
    pub async fn deals(
        &self,
        rows: u32,
        page: u32,
        min_discount: Option<u8>,
        min_price: Option<f64>,
        max_price: Option<f64>,
    ) -> Result<DealsResponse> {
        let url = format!("{}/deals", self.base_url);
        let mut req = self.client.get(&url);

        // Build price range
        let price_range =
            format!("{}_{}", min_price.unwrap_or(0.0) as u32, max_price.unwrap_or(50000.0) as u32);
        req = req.query(&[("priceRange", &price_range)]);

        // Discount filter
        let discount_range = format!("FROM_{}", min_discount.unwrap_or(5));
        req = req.query(&[("discountRange", &discount_range)]);

        req = req.query(&[("rows", &rows.to_string()), ("page", &page.to_string())]);

        let resp = req.send().await.context("Failed to fetch deals")?;

        resp.json().await.context("Failed to parse deals response")
    }

    /// Get price history for a product
    pub async fn price_history(&self, product_id: u64, days: u32) -> Result<PriceHistory> {
        let url = format!("{}/products/{product_id}/price-history", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("days", &days.to_string())])
            .send()
            .await
            .context("Failed to fetch price history")?;

        resp.json().await.context("Failed to parse price history")
    }

    /// Get popular products in a category
    pub async fn popular(&self, category_id: u64, rows: u32) -> Result<Vec<Product>> {
        let url = format!("{}/products/popular", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("categoryId", &category_id.to_string()), ("rows", &rows.to_string())])
            .send()
            .await
            .context("Failed to fetch popular products")?;

        resp.json().await.context("Failed to parse popular products")
    }

    /// Get related products
    pub async fn related(&self, product_id: u64) -> Result<RelatedResponse> {
        let url = format!("{}/products/{product_id}/related", self.base_url);
        let resp =
            self.client.get(&url).send().await.context("Failed to fetch related products")?;

        resp.json().await.context("Failed to parse related products")
    }

    /// Get all categories
    pub async fn categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}/categories", self.base_url);
        let resp = self.client.get(&url).send().await.context("Failed to fetch categories")?;

        resp.json().await.context("Failed to parse categories")
    }

    /// Search products (via HTML scraping with wreq)
    pub async fn search(&self, query: &str, max: usize) -> Result<super::scraper::SearchResult> {
        super::scraper::search(query, max).await
    }
}

impl Default for KuantoKustaClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_products_response() -> serde_json::Value {
        serde_json::json!({
            "data": [{
                "id": 12345,
                "name": "Test Product",
                "brand": "TestBrand",
                "priceMin": 99.99,
                "totalOffers": 5,
                "url": "/p/test",
                "images": [],
                "badges": {},
                "tags": {}
            }],
            "page": 1,
            "rows": 20,
            "total": 1
        })
    }

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

    fn mock_history_response() -> serde_json::Value {
        serde_json::json!({
            "minAxis": 500.0,
            "maxAxis": 800.0,
            "data": [
                {"date": "2024-01-01", "avg": 650.0, "min": 600.0}
            ]
        })
    }

    fn mock_categories_response() -> serde_json::Value {
        serde_json::json!([
            {"id": 1, "label": "Electronics", "slug": "electronics", "hasChild": true, "url": "/c/electronics"},
            {"id": 155, "parentId": 1, "label": "Smartphones", "slug": "smartphones", "hasChild": false, "url": "/c/smartphones"}
        ])
    }

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
    async fn test_products_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products"))
            .and(query_param("rows", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_products_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.products(20).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id, 12345);
        assert_eq!(response.data[0].name, "Test Product");
    }

    #[tokio::test]
    async fn test_deals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/deals"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_deals_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.deals(20, 1, Some(10), None, None).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id, 67890);
        assert_eq!(response.data[0].badges.discount_percentage, Some(25));
    }

    #[tokio::test]
    async fn test_deals_with_price_range() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/deals"))
            .and(query_param("priceRange", "50_500"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_deals_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.deals(20, 1, None, Some(50.0), Some(500.0)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_price_history_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/12345/price-history"))
            .and(query_param("days", "30"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_history_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.price_history(12345, 30).await;

        assert!(result.is_ok());
        let history = result.unwrap();
        assert!((history.min_axis - 500.0).abs() < f64::EPSILON);
        assert_eq!(history.data.len(), 1);
    }

    #[tokio::test]
    async fn test_popular_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/popular"))
            .and(query_param("categoryId", "155"))
            .and(query_param("rows", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_popular_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.popular(155, 10).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, 22222);
    }

    #[tokio::test]
    async fn test_related_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products/12345/related"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_related_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.related(12345).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.count, 1);
    }

    #[tokio::test]
    async fn test_categories_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_categories_response()))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.categories().await;

        assert!(result.is_ok());
        let categories = result.unwrap();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].label, "Electronics");
        assert_eq!(categories[1].parent_id, Some(1));
    }

    #[tokio::test]
    async fn test_products_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/products"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.products(20).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_deals_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/deals"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let client = KuantoKustaClient::with_base_url(&mock_server.uri()).unwrap();
        let result = client.deals(20, 1, None, None, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_client_default() {
        let client = KuantoKustaClient::default();
        assert!(!client.base_url.is_empty());
    }

    #[tokio::test]
    async fn test_with_base_url() {
        let client = KuantoKustaClient::with_base_url("http://localhost:8080").unwrap();
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
