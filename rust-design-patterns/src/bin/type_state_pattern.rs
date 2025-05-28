struct Start;
struct StatusSet;
struct HeadersSet;
struct BodySet;

use std::collections::HashMap;
use std::marker::PhantomData;

struct HttpResponse<State> {
    status: Option<u16>,
    headers: HashMap<String, String>,
    body: Option<String>,
    _state: PhantomData<State>,
}

impl HttpResponse<Start> {
    fn new() -> Self {
        HttpResponse {
            status: None,
            headers: HashMap::new(),
            body: None,
            _state: PhantomData,
        }
    }

    fn with_status(mut self, status: u16) -> HttpResponse<StatusSet> {
        self.status = Some(status);
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl HttpResponse<StatusSet> {
    fn with_header(mut self, key: &str, value: &str) -> HttpResponse<HeadersSet> {
        self.headers.insert(key.to_string(), value.to_string());
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl HttpResponse<HeadersSet> {
    fn with_body(mut self, body: &str) -> HttpResponse<BodySet> {
        self.body = Some(body.to_string());
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl HttpResponse<BodySet> {
    fn send(self) {
        println!("Sending HTTP response:");
        println!("Status: {}", self.status.unwrap());
        for (k, v) in &self.headers {
            println!("Header: {}: {}", k, v);
        }
        println!("Body: {}", self.body.unwrap());
    }
}

fn main() {
    let response = HttpResponse::<Start>::new()
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"message": "OK"}"#);

    response.send();

//     HttpResponse::<Start>::new()
//     .with_status(200)
//     .send();
}
