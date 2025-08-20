# Axum

Axum is the web application framework built on top of Tokio (async runtime) and Tower (middleware + services)

## Core concepts

- Router: holds routes and layers (middleware).
- Handlers: async functions returning types that implement `IntoResponse`.
- Extractors: automatically parse input (e.g. `Json<T>`, `Path<T>`, `State<T`).
- Responses: implement `IntoResponse` (e.g. `String`, `Json`, `StatusCode`).
- State: shared app context (DB, config).
- Middleware (Layers): cross-cutting concerns (logging, CORS, auth).
- Error handling: custom error types implementing IntoResponse.

### Router

In Axum, everything starts with a Router. It’s like Django’s `urls.py` or FastAPI’s `app = FastAPI()` routes. We compose
routes into bigger routers and "nest" them.

```rust
use axum::{routing::get, Router};
async fn hello() -> &'static str { "hello" }

let app = Router::new()
    .route("/", get(hello));
```

### Handlers

A handler is just an async fn that returns something implementing `IntoResponse`. This is like a Django view or FastAPI path operation function.
Return types can be:
- String, &'static str
- axum::Json<T>
- Result<T, E> where E: IntoResponse

```rust
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Health { ok: bool }

async fn health() -> Json<Health> {
    Json(Health { ok: true })
}
```

### Extractors

Extractors are parameters to your handler that Axum knows how to fill from the request. Analogy: FastAPI’s dependency injection / request parsing.

Examples:
- Path<T> → URL path params
- Query<T> → query string params
- Json<T> → JSON body
- State<T> → app state (DB pool, config)
- TypedHeader<T> → request headers


```rust
use axum::{extract::Path, Json};

#[derive(serde::Serialize)]
struct User { id: u32 }

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id })
}
```

### Responses

Responses are the return type of your handler. 	Anything implementing IntoResponse can be returned. 

Common ones:
- String, &'static str
- Json<T>
- (StatusCode, Json<T>)
- Custom Result<T, AppError>

```rust
use axum::{http::StatusCode, Json};

async fn fail() -> (StatusCode, Json<&'static str>) {
    (StatusCode::BAD_REQUEST, Json("oops"))
}
```

### State

Shared application state (DB pool, config, cache) is passed into handlers using State<T>. This is like FastAPI’s Depends() or Django’s request object carrying settings.

```rust
use axum::{extract::State, Json};

#[derive(Clone)]
struct AppState { message: String }

async fn hello(State(state): State<AppState>) -> Json<String> {
    Json(state.message.clone())
}
```

### Middleware (Tower Layers)

Axum uses Tower’s concept of “layers” for middleware. 

Examples:
- Logging/tracing
- CORS
- Authentication

You add them with .layer(...) on the router.

```rust
use tower_http::cors::CorsLayer;

let app = Router::new()
    .route("/hello", get(hello))
    .layer(CorsLayer::permissive());
```

### Error handling

Centralized error handling by creating your own error type implementing IntoResponse. Like Django’s middleware or FastAPI’s exception handlers.

```rust
use axum::{response::IntoResponse, Json};
use axum::http::StatusCode;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("not found")]
    NotFound,
}

#[derive(Serialize)]
struct ErrorResp { error: String }

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(ErrorResp { error: self.to_string() });
        (StatusCode::NOT_FOUND, body).into_response()
    }
}
```

