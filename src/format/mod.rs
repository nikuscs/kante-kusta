//! Output formatting for CLI

use crate::api::{Category, Deal, PriceHistory, Product};
use serde::Serialize;

/// Output format
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}

/// Format products for display
pub fn format_products(products: &[Product], format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => to_json(products),
        OutputFormat::Compact => format_products_compact(products),
        OutputFormat::Table => format_products_table(products),
    }
}

/// Format deals for display
pub fn format_deals(deals: &[Deal], format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => to_json(deals),
        OutputFormat::Compact => format_deals_compact(deals),
        OutputFormat::Table => format_deals_table(deals),
    }
}

/// Format price history for display
pub fn format_history(history: &PriceHistory, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => to_json(history),
        _ => format_history_table(history),
    }
}

/// Format categories for display
pub fn format_categories(categories: &[Category], format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => to_json(categories),
        OutputFormat::Compact => format_categories_compact(categories),
        OutputFormat::Table => format_categories_table(categories),
    }
}

fn to_json<T: Serialize + ?Sized>(data: &T) -> String {
    serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
}

fn format_products_table(products: &[Product]) -> String {
    if products.is_empty() {
        return "No products found.".to_string();
    }

    let mut out = String::new();
    out.push_str(&format!(
        "{:<10} {:<50} {:>10} {:>6} {:>5}\n",
        "ID", "Name", "Price", "Stores", "★"
    ));
    out.push_str(&format!("{:-<10} {:-<50} {:->10} {:->6} {:->5}\n", "", "", "", "", ""));

    for p in products {
        let name = truncate(&p.name, 48);
        let rating =
            p.rating.as_ref().map(|r| format!("{:.1}", r.rating_count)).unwrap_or_default();
        let badge = if p.badges.is_best_seller {
            " 🔥"
        } else if p.badges.is_best_price {
            " 💰"
        } else {
            ""
        };

        out.push_str(&format!(
            "{:<10} {:<50} {:>9.2}€ {:>6} {:>5}{}\n",
            p.id, name, p.price_min, p.total_offers, rating, badge
        ));
    }

    out
}

fn format_products_compact(products: &[Product]) -> String {
    products
        .iter()
        .map(|p| format!("{}\t{:.2}€\t{}\t{}", p.id, p.price_min, p.total_offers, p.name))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_deals_table(deals: &[Deal]) -> String {
    if deals.is_empty() {
        return "No deals found.".to_string();
    }

    let mut out = String::new();
    out.push_str(&format!(
        "{:<10} {:<45} {:>10} {:>5} {:>6}\n",
        "ID", "Name", "Price", "Off", "Stores"
    ));
    out.push_str(&format!("{:-<10} {:-<45} {:->10} {:->5} {:->6}\n", "", "", "", "", ""));

    for d in deals {
        let name = truncate(&d.name, 43);
        let discount = d
            .badges
            .discount_percentage
            .or(d.tags.discount_percentage)
            .map(|d| format!("-{d}%"))
            .unwrap_or_default();

        out.push_str(&format!(
            "{:<10} {:<45} {:>9.2}€ {:>5} {:>6}\n",
            d.id, name, d.price_min, discount, d.total_offers
        ));
    }

    out
}

fn format_deals_compact(deals: &[Deal]) -> String {
    deals
        .iter()
        .map(|d| {
            let discount = d.badges.discount_percentage.or(d.tags.discount_percentage).unwrap_or(0);
            format!("{}\t{:.2}€\t-{}%\t{}", d.id, d.price_min, discount, d.name)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_history_table(history: &PriceHistory) -> String {
    if history.data.is_empty() {
        return "No price history available.".to_string();
    }

    let mut out = String::new();
    out.push_str(&format!("Price range: {:.2}€ - {:.2}€\n\n", history.min_axis, history.max_axis));
    out.push_str(&format!("{:<12} {:>10} {:>10}\n", "Date", "Min", "Avg"));
    out.push_str(&format!("{:-<12} {:->10} {:->10}\n", "", "", ""));

    for point in &history.data {
        out.push_str(&format!("{:<12} {:>9.2}€ {:>9.2}€\n", point.date, point.min, point.avg));
    }

    out
}

fn format_categories_table(categories: &[Category]) -> String {
    if categories.is_empty() {
        return "No categories found.".to_string();
    }

    let mut out = String::new();
    out.push_str(&format!("{:<8} {:<8} {:<40} {:<30}\n", "ID", "Parent", "Name", "Slug"));
    out.push_str(&format!("{:-<8} {:-<8} {:-<40} {:-<30}\n", "", "", "", ""));

    // Sort by parent_id for tree-like display
    let mut sorted: Vec<_> = categories.iter().collect();
    sorted.sort_by_key(|c| (c.parent_id, c.id));

    for c in sorted.iter().take(50) {
        let parent = c.parent_id.map_or_else(|| "-".to_string(), |p| p.to_string());
        let label = truncate(&c.label, 38);
        out.push_str(&format!("{:<8} {:<8} {:<40} {:<30}\n", c.id, parent, label, c.slug));
    }

    if categories.len() > 50 {
        out.push_str(&format!("\n... and {} more categories\n", categories.len() - 50));
    }

    out
}

fn format_categories_compact(categories: &[Category]) -> String {
    categories
        .iter()
        .map(|c| {
            let parent = c.parent_id.map(|p| p.to_string()).unwrap_or_default();
            format!("{}\t{}\t{}", c.id, parent, c.label)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Badges, PricePoint, Rating, Tags};

    fn sample_product() -> Product {
        Product {
            id: 12345,
            name: "Test Product".to_string(),
            brand: "TestBrand".to_string(),
            category: "Electronics".to_string(),
            price_min: 99.99,
            total_offers: 5,
            url: "/p/test-product".to_string(),
            images: vec!["https://example.com/img.jpg".to_string()],
            badges: Badges {
                is_best_seller: true,
                is_best_price: false,
                is_customers_favorite: false,
                discount_percentage: Some(10),
            },
            rating: Some(Rating { rating_count: 4.5, reviews_count: 100 }),
            tags: Tags::default(),
        }
    }

    fn sample_deal() -> Deal {
        Deal {
            id: 67890,
            name: "Deal Product".to_string(),
            brand: "DealBrand".to_string(),
            images: vec![],
            price_min: 49.99,
            total_offers: 3,
            url: "/p/deal-product".to_string(),
            badges: Badges {
                is_best_seller: false,
                is_best_price: true,
                is_customers_favorite: false,
                discount_percentage: Some(25),
            },
            rating: None,
            tags: Tags::default(),
        }
    }

    fn sample_category() -> Category {
        Category {
            id: 155,
            parent_id: Some(100),
            label: "Smartphones".to_string(),
            slug: "smartphones".to_string(),
            has_child: true,
            url: "/c/smartphones".to_string(),
            image_url: None,
        }
    }

    fn sample_history() -> PriceHistory {
        PriceHistory {
            min_axis: 500.0,
            max_axis: 800.0,
            data: vec![
                PricePoint { date: "2024-01-01".to_string(), avg: 650.0, min: 600.0 },
                PricePoint { date: "2024-01-02".to_string(), avg: 640.0, min: 590.0 },
            ],
        }
    }

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_long_string() {
        let result = truncate("hello world", 8);
        assert!(result.ends_with('…'));
        // 7 ASCII chars + ellipsis (3 bytes in UTF-8) = 10 bytes
        assert_eq!(result.chars().count(), 8);
    }

    #[test]
    fn format_products_table_output() {
        let products = vec![sample_product()];
        let output = format_products(&products, OutputFormat::Table);

        assert!(output.contains("ID"));
        assert!(output.contains("Name"));
        assert!(output.contains("Price"));
        assert!(output.contains("12345"));
        assert!(output.contains("Test Product"));
        assert!(output.contains("99.99"));
        assert!(output.contains("🔥")); // best seller badge
    }

    #[test]
    fn format_products_json_output() {
        let products = vec![sample_product()];
        let output = format_products(&products, OutputFormat::Json);

        assert!(output.starts_with('['));
        assert!(output.contains("\"id\": 12345"));
        assert!(output.contains("\"name\": \"Test Product\""));
    }

    #[test]
    fn format_products_compact_output() {
        let products = vec![sample_product()];
        let output = format_products(&products, OutputFormat::Compact);

        assert!(output.contains("12345"));
        assert!(output.contains("99.99€"));
        assert!(output.contains('\t'));
    }

    #[test]
    fn format_products_empty() {
        let products: Vec<Product> = vec![];
        let output = format_products(&products, OutputFormat::Table);
        assert_eq!(output, "No products found.");
    }

    #[test]
    fn format_deals_table_output() {
        let deals = vec![sample_deal()];
        let output = format_deals(&deals, OutputFormat::Table);

        assert!(output.contains("ID"));
        assert!(output.contains("Off"));
        assert!(output.contains("67890"));
        assert!(output.contains("-25%"));
    }

    #[test]
    fn format_deals_compact_output() {
        let deals = vec![sample_deal()];
        let output = format_deals(&deals, OutputFormat::Compact);

        assert!(output.contains("67890"));
        assert!(output.contains("-25%"));
    }

    #[test]
    fn format_deals_empty() {
        let deals: Vec<Deal> = vec![];
        let output = format_deals(&deals, OutputFormat::Table);
        assert_eq!(output, "No deals found.");
    }

    #[test]
    fn format_categories_table_output() {
        let categories = vec![sample_category()];
        let output = format_categories(&categories, OutputFormat::Table);

        assert!(output.contains("ID"));
        assert!(output.contains("Parent"));
        assert!(output.contains("155"));
        assert!(output.contains("100"));
        assert!(output.contains("Smartphones"));
    }

    #[test]
    fn format_categories_compact_output() {
        let categories = vec![sample_category()];
        let output = format_categories(&categories, OutputFormat::Compact);

        assert!(output.contains("155"));
        assert!(output.contains("100"));
        assert!(output.contains("Smartphones"));
    }

    #[test]
    fn format_categories_empty() {
        let categories: Vec<Category> = vec![];
        let output = format_categories(&categories, OutputFormat::Table);
        assert_eq!(output, "No categories found.");
    }

    #[test]
    fn format_history_table_output() {
        let history = sample_history();
        let output = format_history(&history, OutputFormat::Table);

        assert!(output.contains("Price range:"));
        assert!(output.contains("500.00€"));
        assert!(output.contains("800.00€"));
        assert!(output.contains("2024-01-01"));
        assert!(output.contains("600.00€"));
    }

    #[test]
    fn format_history_json_output() {
        let history = sample_history();
        let output = format_history(&history, OutputFormat::Json);

        assert!(output.starts_with('{'));
        assert!(output.contains("\"minAxis\": 500.0"));
        assert!(output.contains("\"data\""));
    }

    #[test]
    fn format_history_empty() {
        let history = PriceHistory { min_axis: 0.0, max_axis: 0.0, data: vec![] };
        let output = format_history(&history, OutputFormat::Table);
        assert_eq!(output, "No price history available.");
    }

    #[test]
    fn format_categories_over_50_shows_truncation() {
        let categories: Vec<Category> = (0..60)
            .map(|i| Category {
                id: i,
                parent_id: None,
                label: format!("Category {i}"),
                slug: format!("category-{i}"),
                has_child: false,
                url: format!("/c/category-{i}"),
                image_url: None,
            })
            .collect();

        let output = format_categories(&categories, OutputFormat::Table);
        assert!(output.contains("... and 10 more categories"));
    }

    #[test]
    fn product_with_best_price_badge() {
        let mut product = sample_product();
        product.badges.is_best_seller = false;
        product.badges.is_best_price = true;

        let products = vec![product];
        let output = format_products(&products, OutputFormat::Table);
        assert!(output.contains("💰"));
    }

    #[test]
    fn category_without_parent() {
        let category = Category {
            id: 1,
            parent_id: None,
            label: "Root Category".to_string(),
            slug: "root".to_string(),
            has_child: true,
            url: "/c/root".to_string(),
            image_url: None,
        };

        let output = format_categories(&[category], OutputFormat::Table);
        assert!(output.contains('-')); // parent should show as "-"
    }
}
