# OSRS Wiki Prices

A Rust library for fetching and working with Old School RuneScape (OSRS) item price data from
the [OSRS Wiki](https://oldschool.runescape.wiki/). This crate provides async-friendly endpoints for retrieving the
latest prices, historical timeseries, and mapping item IDs to names.

## Features

- Fetch the latest prices for OSRS items
- Retrieve price timeseries (5 minutes, 1 hour, etc.)
- Map item IDs to item names and vice versa
- Async API using [tokio](https://tokio.rs/)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
osrs-wiki-prices = "0.1.0"
```

## Usage

### Create a Client

```rust
use osrs_wiki_prices::{Client, ApiEndpoint};
use std::borrow::Cow;

// Provide a descriptive user agent string here, as required by the OSRS Wiki API.
// This is important for identifying your application and ensuring compliance with their API usage policies.
let client = Client::try_new(Cow::Borrowed("my-user-agent"), ApiEndpoint::OldSchoolRuneScape).unwrap();
```

### Fetch the Latest Price for an Item

```rust
use osrs_wiki_prices::endpoints::latest::LatestEndpoint;
use osrs_wiki_prices::types::ItemId;

#[tokio::main]
async fn main() {
    let client = /* create client as above */;
    let latest = client.latest().await.unwrap();
    if let Some(item) = latest.get(&ItemId::new(4151)) { // 4151 = Abyssal whip
        println!("Abyssal whip high price: {:?}", item.high);
    }
}
```

### Fetch 5-Minute Timeseries Data

```rust
use osrs_wiki_prices::endpoints::timeseries::{TimeseriesEndpoint, Timestep};
use osrs_wiki_prices::types::ItemId;

#[tokio::main]
async fn main() {
    let client = /* create client as above */;
    let timeseries = client.timeseries(ItemId::new(4151), Timestep::FiveMinutes).await.unwrap();
    for entry in timeseries {
        println!("Timestamp: {}, Avg High Price: {:?}", entry.timestamp, entry.avg_high_price);
    }
}
```

### Map Item ID to Name

```rust
use osrs_wiki_prices::endpoints::mapping::MappingEndpoint;

#[tokio::main]
async fn main() {
    let client = /* create client as above */;
    let mapping = client.mapping().await.unwrap();
    for item in mapping.iter().filter(|i| i.id.id() == 4151) {
        println!("Item name: {}", item.name);
    }
}
```

## Endpoints

- `endpoints::latest` — Latest prices for all items
- `endpoints::prices::five_minutes` — 5-minute interval price timeseries
- `endpoints::prices::one_hour` — 1-hour interval price timeseries
- `endpoints::mapping` — Item ID/name mapping
- `endpoints::timeseries` — Historical price timeseries

## Requirements

- Rust 1.75+ (edition 2024)
- Tokio async runtime

## License

MIT

