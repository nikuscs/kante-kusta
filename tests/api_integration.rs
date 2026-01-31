//! Integration tests for the `KuantoKusta` API client

use kuantokusta::{Category, Deal, PriceHistory, PricePoint, Product};

/// Create a mock product response
fn mock_products_response() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {
                "id": 12345,
                "name": "Test Product",
                "brand": "TestBrand",
                "category": "Electronics",
                "priceMin": 99.99,
                "totalOffers": 5,
                "url": "/p/test-product",
                "images": ["https://example.com/img.jpg"],
                "badges": {
                    "isBestSeller": true,
                    "isBestPrice": false,
                    "isCustomersFavorite": false,
                    "discountPercentage": 10
                },
                "rating": {
                    "ratingCount": 4.5,
                    "reviewsCount": 100
                },
                "tags": {
                    "isMarketplace": false,
                    "adultOnly": false,
                    "hasSplitPayment": false
                }
            }
        ],
        "page": 1,
        "rows": 20,
        "total": 1
    })
}

/// Create a mock deals response
fn mock_deals_response() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {
                "id": 67890,
                "name": "Deal Product",
                "brand": "DealBrand",
                "images": [],
                "priceMin": 49.99,
                "totalOffers": 3,
                "url": "/p/deal-product",
                "badges": {
                    "isBestSeller": false,
                    "isBestPrice": true,
                    "discountPercentage": 25
                },
                "tags": {}
            }
        ],
        "page": 1,
        "rows": 20,
        "total": 1
    })
}

/// Create a mock price history response
fn mock_history_response() -> serde_json::Value {
    serde_json::json!({
        "minAxis": 500.0,
        "maxAxis": 800.0,
        "data": [
            {"date": "2024-01-01", "avg": 650.0, "min": 600.0},
            {"date": "2024-01-02", "avg": 640.0, "min": 590.0}
        ]
    })
}

/// Create a mock categories response
fn mock_categories_response() -> serde_json::Value {
    serde_json::json!([
        {
            "id": 1,
            "parentId": null,
            "label": "Electronics",
            "slug": "electronics",
            "hasChild": true,
            "url": "/c/electronics"
        },
        {
            "id": 155,
            "parentId": 1,
            "label": "Smartphones",
            "slug": "smartphones",
            "hasChild": false,
            "url": "/c/smartphones"
        }
    ])
}

/// Create a mock related products response
fn mock_related_response() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {
                "id": 11111,
                "name": "Related Product",
                "brand": "Brand",
                "priceMin": 199.99,
                "totalOffers": 10,
                "url": "/p/related",
                "images": [],
                "badges": {},
                "tags": {}
            }
        ],
        "count": 1
    })
}

#[test]
fn parse_products_response() {
    let json = mock_products_response();
    let response: kuantokusta::api::ProductsResponse =
        serde_json::from_value(json).expect("Failed to parse products response");

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[0].name, "Test Product");
    assert!((response.data[0].price_min - 99.99).abs() < f64::EPSILON);
    assert_eq!(response.total, 1);
}

#[test]
fn parse_deals_response() {
    let json = mock_deals_response();
    let response: kuantokusta::api::DealsResponse =
        serde_json::from_value(json).expect("Failed to parse deals response");

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 67890);
    assert_eq!(response.data[0].badges.discount_percentage, Some(25));
}

#[test]
fn parse_history_response() {
    let json = mock_history_response();
    let history: PriceHistory =
        serde_json::from_value(json).expect("Failed to parse price history");

    assert!((history.min_axis - 500.0).abs() < f64::EPSILON);
    assert!((history.max_axis - 800.0).abs() < f64::EPSILON);
    assert_eq!(history.data.len(), 2);
}

#[test]
fn parse_categories_response() {
    let json = mock_categories_response();
    let categories: Vec<Category> =
        serde_json::from_value(json).expect("Failed to parse categories");

    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].label, "Electronics");
    assert!(categories[0].parent_id.is_none());
    assert_eq!(categories[1].parent_id, Some(1));
}

#[test]
fn parse_related_response() {
    let json = mock_related_response();
    let response: kuantokusta::api::RelatedResponse =
        serde_json::from_value(json).expect("Failed to parse related response");

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.count, 1);
}

#[test]
fn product_with_missing_optional_fields() {
    let json = serde_json::json!({
        "id": 1,
        "name": "Minimal Product"
    });

    let product: Product = serde_json::from_value(json).expect("Failed to parse minimal product");

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Minimal Product");
    assert_eq!(product.brand, "");
    assert!((product.price_min - 0.0).abs() < f64::EPSILON);
    assert!(product.rating.is_none());
}

#[test]
fn deal_with_discount_in_tags() {
    let json = serde_json::json!({
        "id": 1,
        "name": "Tagged Deal",
        "priceMin": 100.0,
        "totalOffers": 5,
        "badges": {},
        "tags": {
            "discountPercentage": 15
        }
    });

    let deal: Deal = serde_json::from_value(json).expect("Failed to parse deal");
    assert_eq!(deal.tags.discount_percentage, Some(15));
}

#[test]
fn category_as_root() {
    let json = serde_json::json!({
        "id": 1,
        "label": "Root",
        "slug": "root",
        "hasChild": true,
        "url": "/c/root"
    });

    let category: Category = serde_json::from_value(json).expect("Failed to parse category");
    assert!(category.parent_id.is_none());
    assert!(category.has_child);
}

#[test]
fn price_point_parsing() {
    let json = serde_json::json!({
        "date": "2024-06-15",
        "avg": 299.99,
        "min": 279.99
    });

    let point: PricePoint = serde_json::from_value(json).expect("Failed to parse price point");
    assert_eq!(point.date, "2024-06-15");
    assert!((point.avg - 299.99).abs() < f64::EPSILON);
    assert!((point.min - 279.99).abs() < f64::EPSILON);
}
