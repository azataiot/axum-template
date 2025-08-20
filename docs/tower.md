# Tower

Tower is a library of modular and reusable components for building robust networked services. 

Core idea: everything is a Service that takes a Request and produces a Response. 

Tower provides middleware layers (logging, retries, rate limiting, timeout, load balancing, etc).

Axum builds on Tower.


## Core concepts

### Service trait 

```rust
trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

A service is just a function with some readiness checks. Axum’s handlers are automatically turned into Services under the hood.

### Layer

A Layer wraps a Service with additional behavior. Like Django middleware (process_request, process_response) or FastAPI middlewares.

Examples:
- Logging/tracing
- Timeout
- CORS
- Authentication


```rust
use tower_http::trace::TraceLayer;
use axum::{Router, routing::get};

async fn hello() -> &'static str { "hi" }

let app = Router::new()
    .route("/", get(hello))
    .layer(TraceLayer::new_for_http()); // Tower layer
```

### tower-http 


Tower itself is low-level; tower-http provides HTTP-specific middleware. Ready-to-use layers: CORS, compression, logging, auth, timeout.


