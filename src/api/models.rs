//! Data models for `KuantoKusta` API responses

use serde::{Deserialize, Serialize};

/// Product from search/popular/related endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub brand: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub price_min: f64,
    #[serde(default)]
    pub total_offers: u32,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub badges: Badges,
    #[serde(default)]
    pub rating: Option<Rating>,
    #[serde(default)]
    pub tags: Tags,
}

/// Product badges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badges {
    #[serde(default)]
    pub is_best_seller: bool,
    #[serde(default)]
    pub is_best_price: bool,
    #[serde(default)]
    pub is_customers_favorite: bool,
    #[serde(default)]
    pub discount_percentage: Option<u8>,
}

/// Product rating
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    #[serde(default)]
    pub rating_count: f32,
    #[serde(default)]
    pub reviews_count: u32,
}

/// Product tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(default)]
    pub is_marketplace: bool,
    #[serde(default)]
    pub adult_only: bool,
    #[serde(default)]
    pub has_split_payment: bool,
    #[serde(default)]
    pub discount_percentage: Option<u8>,
}

/// Search/deals response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsResponse {
    pub data: Vec<Product>,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub rows: u32,
    #[serde(default)]
    pub total: u64,
}

/// Related products response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedResponse {
    pub data: Vec<Product>,
    #[serde(default)]
    pub count: u64,
}

/// Price history response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistory {
    pub min_axis: f64,
    pub max_axis: f64,
    pub data: Vec<PricePoint>,
}

/// Single price history data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub date: String,
    pub avg: f64,
    pub min: f64,
}

/// Category from /categories endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: u64,
    #[serde(default)]
    pub parent_id: Option<u64>,
    pub label: String,
    pub slug: String,
    #[serde(default)]
    pub has_child: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub image_url: Option<String>,
}

/// Deal product (slightly different structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deal {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub price_min: f64,
    #[serde(default)]
    pub total_offers: u32,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub brand: String,
    #[serde(default)]
    pub badges: Badges,
    #[serde(default)]
    pub rating: Option<Rating>,
    #[serde(default)]
    pub tags: Tags,
}

/// Deals response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealsResponse {
    pub data: Vec<Deal>,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub rows: u32,
    #[serde(default)]
    pub total: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_product() {
        let json = r#"{
            "id": 11406755,
            "name": "Apple iPhone 16 6.1\" 128GB Pink",
            "brand": "Apple",
            "category": "Smartphones",
            "priceMin": 708.00,
            "totalOffers": 38,
            "url": "/p/apple-iphone-16-61-128gb-pink",
            "images": ["https://cdn.kuantokusta.pt/img.jpg"],
            "badges": {
                "isBestSeller": true,
                "isBestPrice": true,
                "discountPercentage": 10
            },
            "rating": {
                "ratingCount": 5.0,
                "reviewsCount": 12
            },
            "tags": {
                "isMarketplace": false,
                "adultOnly": false
            }
        }"#;

        let product: Product = serde_json::from_str(json).unwrap();
        assert_eq!(product.id, 11_406_755);
        assert_eq!(product.name, "Apple iPhone 16 6.1\" 128GB Pink");
        assert_eq!(product.brand, "Apple");
        assert!((product.price_min - 708.0).abs() < f64::EPSILON);
        assert_eq!(product.total_offers, 38);
        assert!(product.badges.is_best_seller);
        assert!(product.badges.is_best_price);
        assert_eq!(product.badges.discount_percentage, Some(10));
    }

    #[test]
    fn parse_product_minimal() {
        let json = r#"{"id": 123, "name": "Test Product"}"#;
        let product: Product = serde_json::from_str(json).unwrap();
        assert_eq!(product.id, 123);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.brand, "");
        assert!((product.price_min - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_category() {
        let json = r#"{
            "id": 155,
            "parentId": 150,
            "label": "Smartphones",
            "slug": "smartphones",
            "hasChild": true,
            "url": "/c/smartphones"
        }"#;

        let category: Category = serde_json::from_str(json).unwrap();
        assert_eq!(category.id, 155);
        assert_eq!(category.parent_id, Some(150));
        assert_eq!(category.label, "Smartphones");
        assert!(category.has_child);
    }

    #[test]
    fn parse_price_history() {
        let json = r#"{
            "minAxis": 500.0,
            "maxAxis": 800.0,
            "data": [
                {"date": "2024-01-01", "avg": 650.0, "min": 600.0},
                {"date": "2024-01-02", "avg": 640.0, "min": 590.0}
            ]
        }"#;

        let history: PriceHistory = serde_json::from_str(json).unwrap();
        assert!((history.min_axis - 500.0).abs() < f64::EPSILON);
        assert_eq!(history.data.len(), 2);
        assert_eq!(history.data[0].date, "2024-01-01");
    }
}
