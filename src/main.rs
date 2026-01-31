//! kk - `KuantoKusta` CLI
//!
//! Fast product search and price tracking for KuantoKusta.pt

use anyhow::Result;
use clap::{Parser, Subcommand};
use kuantokusta::api::KuantoKustaClient;
use kuantokusta::commands;
use kuantokusta::format::OutputFormat;
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "kk",
    version,
    about = "Fast CLI for KuantoKusta.pt price comparison",
    long_about = "Search products, track prices, and find deals on Portugal's largest price comparison site."
)]
struct Cli {
    /// Output format
    #[arg(short, long, default_value = "table", global = true)]
    format: OutputFormat,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for products
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        max: usize,
    },

    /// Browse popular products
    #[command(alias = "b")]
    Browse {
        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        max: u32,
    },

    /// List current deals and discounts
    #[command(alias = "d")]
    Deals {
        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        max: u32,

        /// Minimum discount percentage
        #[arg(long)]
        min_discount: Option<u8>,

        /// Minimum price filter
        #[arg(long)]
        min_price: Option<f64>,

        /// Maximum price filter
        #[arg(long)]
        max_price: Option<f64>,
    },

    /// Get price history for a product
    #[command(alias = "h")]
    History {
        /// Product ID
        product_id: u64,

        /// Number of days of history
        #[arg(short, long, default_value = "30")]
        days: u32,
    },

    /// Get popular products in a category
    #[command(alias = "p")]
    Popular {
        /// Category ID
        category_id: u64,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        max: u32,
    },

    /// Get related products
    #[command(alias = "r")]
    Related {
        /// Product ID
        product_id: u64,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        max: u32,
    },

    /// List categories
    #[command(alias = "c")]
    Categories {
        /// Parent category ID (show subcategories)
        #[arg(short, long)]
        parent: Option<u64>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.verbose {
        EnvFilter::new(Level::DEBUG.to_string())
    } else {
        EnvFilter::from_default_env().add_directive(Level::WARN.into())
    };
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();

    let client = KuantoKustaClient::new()?;

    let output = match cli.command {
        Commands::Search { query, max } => {
            commands::search(&client, &query, max, cli.format).await?
        }

        Commands::Browse { max } => commands::browse(&client, max, cli.format).await?,

        Commands::Deals { max, min_discount, min_price, max_price } => {
            commands::deals(&client, max, min_discount, min_price, max_price, cli.format).await?
        }

        Commands::History { product_id, days } => {
            commands::history(&client, product_id, days, cli.format).await?
        }

        Commands::Popular { category_id, max } => {
            commands::popular(&client, category_id, max, cli.format).await?
        }

        Commands::Related { product_id, max } => {
            commands::related(&client, product_id, max, cli.format).await?
        }

        Commands::Categories { parent } => {
            commands::categories(&client, parent, cli.format).await?
        }
    };

    println!("{output}");
    Ok(())
}
