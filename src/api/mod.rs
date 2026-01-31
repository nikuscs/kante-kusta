//! `KuantoKusta` API module

pub mod client;
pub mod models;
pub mod scraper;

pub use client::KuantoKustaClient;
pub use models::*;
pub use scraper::{parse_search_html, search_with_base_url, SearchResult};
