//! Type-state pattern — encodes state machine transitions in the type system so
//! that invalid sequences are *compile errors*, not runtime panics.
//!
//! Here an `HttpResponse` must be built in order: status → header → body → send.
//! Calling `.send()` before setting a status is rejected by the compiler.

use std::collections::HashMap;
use std::marker::PhantomData;

// ── State marker types (zero-sized, never instantiated) ───────────────────────

pub struct Start;
pub struct StatusSet;
pub struct HeadersSet;
pub struct BodySet;

// ── Builder ───────────────────────────────────────────────────────────────────

/// An HTTP response being assembled. The `State` type parameter tracks which
/// fields have been set and which transitions are currently available.
pub struct HttpResponse<State> {
    status: Option<u16>,
    headers: HashMap<String, String>,
    body: Option<String>,
    _state: PhantomData<State>,
}

impl HttpResponse<Start> {
    pub fn new() -> Self {
        HttpResponse {
            status: None,
            headers: HashMap::new(),
            body: None,
            _state: PhantomData,
        }
    }

    pub fn with_status(self, status: u16) -> HttpResponse<StatusSet> {
        HttpResponse {
            status: Some(status),
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl Default for HttpResponse<Start> {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponse<StatusSet> {
    pub fn with_header(mut self, key: &str, value: &str) -> HttpResponse<HeadersSet> {
        self.headers.insert(key.to_owned(), value.to_owned());
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl HttpResponse<HeadersSet> {
    pub fn with_body(self, body: &str) -> HttpResponse<BodySet> {
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: Some(body.to_owned()),
            _state: PhantomData,
        }
    }
}

impl HttpResponse<BodySet> {
    /// Finalises and "sends" the response. All fields are guaranteed to be set
    /// by this point — the `unwrap` calls here are infallible.
    pub fn send(self) {
        println!("Status : {}", self.status.unwrap());
        for (k, v) in &self.headers {
            println!("Header : {k}: {v}");
        }
        println!("Body   : {}", self.body.unwrap());
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    HttpResponse::<Start>::new()
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"message": "OK"}"#)
        .send();

    // The following would be a compile error — you cannot call `.send()` before
    // setting a status, header, and body:
    //
    // HttpResponse::<Start>::new().with_status(200).send();
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_produces_start_state() {
        let r = HttpResponse::<Start>::default();
        // Transitions must follow the required order.
        let _ready = r.with_status(404).with_header("X-Test", "1").with_body("not found");
    }

    #[test]
    fn status_is_preserved_through_transitions() {
        let r = HttpResponse::<Start>::new()
            .with_status(201)
            .with_header("Location", "/items/1")
            .with_body("created");
        // `status` is `Option<u16>` — safe to inspect before `.send()` consumes `r`.
        assert_eq!(r.status, Some(201));
    }
}
