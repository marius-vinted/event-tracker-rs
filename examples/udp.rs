use serde::Serialize;
use tracing::Level;
use vinted_event_tracker::*;

#[tokio::main(worker_threads = 1)]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    let addr = "0.0.0.0:5005".parse().expect("valid addr");

    let udp_relay = Udp::new(addr);

    if let Err(ref error) = set_relay(udp_relay) {
        tracing::error!(%error, "Couldn't set UDP relay");
    }

    track_events(1_000)
}

fn track_events(iterations: i32) {
    #[derive(Debug, Serialize)]
    struct SearchEvent {
        iteration: i32,
    }

    for iteration in 1..iterations {
        let event = Event::new("event", "portal", SearchEvent { iteration });

        if let Err(ref error) = track(event) {
            tracing::error!(%error, "Couldn't track an event");
        }
    }
}