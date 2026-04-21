use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

// in C: similar to struct
// in Python: similar to a dataclass or simple object shape
#[derive(Serialize)] // Rust should automatically generate code that can convert this struct into JSON.
struct HealthResponse {
    status: String, // An owned, growable heap-allocated string. struct owns its data
}

// This defines the shape of one event.
#[derive(Serialize)]
struct Event {
    id: u32,
    title: String,
    categories: Vec<String>, // Vec<T> means: a growable list of values of type T
    venue_name: String,
    district: String,
    starts_at: String,
    ends_at: String,
    source_name: String,
}

 // route handler
async fn root() -> &'static str {
    "Event Hub Platform for Berlin Children: Backend is running"
}

// JSON handler serializes value as JSON HTTP response
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn get_events() -> Json<Vec<Event>> {
    // Creating the list with vec![]
    let events = vec![
        Event {
            id: 1,
            title: "Osterbasteln für Familien".to_string(),
            categories: vec!["Kinder, Jugendliche".to_string()],
            venue_name: "Stadtteilbibliothek Lankwitz".to_string(),
            district: "Steglitz-Zehlendorf".to_string(),
            starts_at: "2026-03-25T16:30:00+01:00".to_string(),
            ends_at: "2026-03-25T18:00:00+01:00".to_string(),
            source_name: "berlin.de".to_string(),
        },
        Event {
            id: 2,
            title: "Vorlesestunde für Kinder ab 4 Jahren".to_string(),
            categories: vec!["Kinder, Jugendliche".to_string()],
            venue_name: "Ingeborg-Drewitz-Bibliothek".to_string(),
            district: "Steglitz-Zehlendorf".to_string(),
            starts_at: "2026-03-28T15:00:00+01:00".to_string(),
            ends_at: "2026-03-28T16:00:00+01:00".to_string(),
            source_name: "berlin.de".to_string(),
        },
        Event {
            id: 3,
            title: "Vorlesestunde von Lesewelt Berlin e.V. für Kinder in der Ingeborg-Drewitz-Bibliothek".to_string(),
            categories: vec![
                "Kinder, Jugendliche".to_string(),
                "Lesungen, Vorträge".to_string(),
            ],
            venue_name: "Ingeborg-Drewitz-Bibliothek".to_string(),
            district: "Steglitz-Zehlendorf".to_string(),
            starts_at: "2026-03-29T10:00:00+01:00".to_string(),
            ends_at: "2026-03-29T12:00:00+01:00".to_string(),
            source_name: "berlin.de".to_string(),
        },
        // one event is intentionally not child-related to test the filter
        Event {
            id: 3,
            title: "Mietberatung in der Bibliothek".to_string(),
            categories: vec![
                "Politik, Bürgerservice".to_string(),
                "Infoveranstaltungen".to_string(),
            ],
            venue_name: "Ingeborg-Drewitz-Bibliothek".to_string(),
            district: "Steglitz-Zehlendorf".to_string(),
            starts_at: "2026-03-29T10:00:00+01:00".to_string(),
            ends_at: "2026-03-29T12:00:00+01:00".to_string(),
            source_name: "berlin.de".to_string(),
        },
    ];

    Json(events)
}


// this is the program entry point
// Tokio should set up the async runtime
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/events", get(get_events));

    // Later, for Docker/Kubernetes, will be changed to 0.0.0.0:3000.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // This binds the port and starts serving requests.
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener"); // expect(...) means:
                                                // if something fails, crash with this message.

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

