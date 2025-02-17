# Event Tracker

An abstraction for event tracking

## Installation

Add event tracker as a dependency to your `Cargo.toml`:

```toml
vinted_event_tracker = { git = "https://github.com/vinted/event-tracker-rs" }
```

## Usage

Configure relay once during the startup.

```rust
use serde::Serialize;
use vinted_event_tracker::*;


#[tokio::main]
async fn main() {
    let udp_relay = UdpRelay::new("0.0.0.0:5005").await.unwrap();

    set_relay(udp_relay).unwrap();
}
```

In your library code, create an event structure and use it for tracking

```rust
use serde::Serialize;
use vinted_event_tracker::*;

fn track_search_event() {
    #[derive(Debug, Serialize)]
    struct SearchEvent<'a> {
        total: i32,
        timed_out: bool,
        query: &'a str,
    }

    let event = "event_name";
    let portal = "fr";
    let debug_pin = Some(1234);
    let search_event = SearchEvent {
        total: 123,
        timed_out: false,
        query: "shoes",
    };

    track(event, portal, debug_pin, search_event);
}
```
