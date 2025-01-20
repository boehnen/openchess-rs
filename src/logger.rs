use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use std::time::Instant;
use tracing::{error, info};

pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status().as_u16();

    if status >= 500 {
        error!(
            "Request: {} {} -> Status: {}, Duration: {:?}",
            method, uri, status, duration
        );
    } else {
        info!(
            "Request: {} {} -> Status: {}, Duration: {:?}",
            method, uri, status, duration
        );
    }

    Ok(response)
}
