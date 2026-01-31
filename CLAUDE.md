# CLAUDE.md - KuantoKusta CLI

## Project Overview

Rust CLI for KuantoKusta.pt - Portugal's largest price comparison site.

**Key insight:** KuantoKusta has a public JSON API at `api.kuantokusta.pt` - no HTML scraping needed!

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /products?q=X&rows=N&page=N` | Search products |
| `GET /deals?priceRange=X_Y&discountRange=FROM_N&rows=N` | Deals/discounts |
| `GET /products/{id}/price-history?days=30` | Price history |
| `GET /products/{id}/related` | Related products |
| `GET /products/popular?categoryId=X&rows=N` | Popular by category |
| `GET /categories` | All categories |

## Build & Test

```bash
cargo build
cargo run -- search "iphone"
cargo run -- history 11406755
cargo run -- deals --max 10
cargo run -- categories
```

## Project Structure

```
src/
├── main.rs           # CLI entry (clap)
├── lib.rs            # Library exports
├── api/
│   ├── mod.rs
│   ├── client.rs     # HTTP client (reqwest)
│   └── models.rs     # Product, Category, etc.
├── commands/
│   ├── mod.rs
│   ├── search.rs
│   ├── deals.rs
│   ├── history.rs
│   ├── popular.rs
│   ├── related.rs
│   └── categories.rs
└── format/
    └── mod.rs        # Table/JSON output

```

## Code Style

- rustfmt with max_width=100
- Use anyhow for error handling
- Async/await with tokio

## Known Limitations

- Price history only accepts specific day values (30, 90)
- Deals endpoint needs `discountRange=FROM_X` format
