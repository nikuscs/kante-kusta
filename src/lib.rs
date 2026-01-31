//! kuantokusta - Fast CLI for KuantoKusta.pt price comparison
//!
//! Query Portugal's largest price comparison site from the command line.

pub mod api;
pub mod commands;
pub mod format;

pub use api::{
    Badges, Category, Deal, DealsResponse, KuantoKustaClient, PriceHistory, PricePoint, Product,
    ProductsResponse, Rating, RelatedResponse, SearchResult, Tags,
};
pub use format::OutputFormat;
