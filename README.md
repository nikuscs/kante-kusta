# ⚖️ kk - KuantoKusta CLI

![CI](https://github.com/nikuscs/kante-kusta/actions/workflows/ci.yml/badge.svg)
![Release](https://img.shields.io/github/v/release/nikuscs/kante-kusta)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

Fast CLI for [KuantoKusta.pt](https://kuantokusta.pt) price comparison - Portugal's largest product comparator.

> **Disclaimer:** This project is for **educational purposes and AI automation research only**.
> The authors are not responsible for any misuse or for any damages resulting from the use of this tool.
> Users are solely responsible for ensuring compliance with applicable laws and the terms of service
> of any websites accessed. This software is provided "as-is" without warranty of any kind.
>
> If you are a rights holder and wish to have this project removed, please [contact me](https://github.com/nikuscs).

> **Note:** This project was partially developed with AI assistance and may contain bugs or unexpected behavior. Use at your own risk.

## Features

- **Search** products across 1M+ listings
- **Price history** tracking (30/90 days)
- **Deals** with discount filters
- **Categories** browser
- **Related** products discovery
- Multiple output formats (table, JSON, compact)

## Installation

### Pre-built binaries

Download from [GitHub Releases](https://github.com/nikuscs/kuantokusta/releases):

```bash
# macOS (Apple Silicon)
tar -xzf kk-macos-arm64.tar.gz
chmod +x kk
./kk --help

# Linux (x64)
tar -xzf kk-linux-x64.tar.gz
chmod +x kk
./kk --help

# Linux (ARM64 / Raspberry Pi)
tar -xzf kk-linux-arm64.tar.gz
chmod +x kk
./kk --help
```

### From source

```bash
git clone https://github.com/nikuscs/kuantokusta
cd kuantokusta
cargo build --release
# Binary at target/release/kk
```

## Usage

### Search Products

```bash
# Basic search
kk search "iphone 16"

# Limit results
kk search "playstation 5" --max 10

# JSON output for scripting
kk search "tv samsung" --format json
```

### Price History

```bash
# 30-day history (default)
kk history 11406755

# 90-day history
kk history 11406755 --days 90
```

### Deals & Discounts

```bash
# Current deals
kk deals

# Minimum 20% off
kk deals --min-discount 20

# Price range
kk deals --min-price 50 --max-price 500
```

### Categories

```bash
# Top-level categories
kk categories

# Subcategories of Smartphones (id=155)
kk categories --parent 155
```

### Popular & Related

```bash
# Popular in Smartphones category
kk popular 155

# Related products
kk related 11406755
```

## Output Formats

| Format | Flag | Description |
|--------|------|-------------|
| table | `--format table` | Human-readable table (default) |
| json | `--format json` | JSON for scripting |
| compact | `--format compact` | Tab-separated for piping |

## Examples

```bash
# Find cheapest PlayStation 5
kk search "playstation 5" --format json | jq '.[] | select(.priceMin < 500)'

# Track price over time
kk history 11406755 --format json | jq '.data[-1].min'

# Export deals to file
kk deals --max 100 --format json > deals.json
```

## How It Works

- Uses **wreq** for TLS fingerprint emulation (bypasses CDN protection)
- Search scrapes `__NEXT_DATA__` from SSR pages
- API endpoints used for deals, history, categories, etc.

## Related Projects

- [🕵️ olx-tracker](https://github.com/nikuscs/olx-tracker) - Track OLX.pt listings and get alerts on deals
- [🦎 amz-crawler](https://github.com/nikuscs/amz-crawler) - Search Amazon with TLS fingerprinting, compare EU prices

## License

MIT
