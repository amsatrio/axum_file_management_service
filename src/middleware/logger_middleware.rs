use axum::{ extract::Request, middleware::Next, response::Response};

pub async fn log_request(req: Request, next: Next) -> Response {
    log::info!("Request: {:?}", req);
    let response = next.run(req).await;
    log::info!("Response: {:?}", response);
    response
}