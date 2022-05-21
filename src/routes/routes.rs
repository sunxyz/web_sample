use axum::{
    routing::{get},
    Router,
};

use super::*;

pub fn router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello::root))
}