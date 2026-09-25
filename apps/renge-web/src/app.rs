use crate::{
    handlers::{event_handlers, participant_handlers},
    state::AppState,
};
use axum::{
    Router,
    extract::{ConnectInfo, Request},
    http::{
        HeaderMap, HeaderValue,
        header::{HOST, USER_AGENT},
    },
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
};
use std::net::{IpAddr, SocketAddr};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, Span};
use uuid::Uuid;

#[derive(Clone, Copy)]
struct RequestId(Uuid);

pub(crate) fn router(state: AppState, trust_proxy_headers: bool) -> Router {
    Router::new()
        .route("/", get(event_handlers::index))
        .route("/events", post(event_handlers::create_event))
        .route(
            "/events/{id}",
            get(event_handlers::show_event).delete(event_handlers::delete_event),
        )
        .route(
            "/events/{id}/participants",
            post(participant_handlers::create_participant),
        )
        .route(
            "/events/{id}/participants/{participant_id}/attendance",
            post(participant_handlers::update_attendance),
        )
        .route(
            "/events/{id}/participants/{participant_id}",
            delete(participant_handlers::delete_participant),
        )
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(move |request: &axum::http::Request<_>| {
                    request_span(request, trust_proxy_headers)
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(middleware::from_fn(add_request_id))
}

async fn add_request_id(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4();
    request.extensions_mut().insert(RequestId(request_id));

    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id.to_string()).expect("a UUID is a valid header value"),
    );
    response
}

fn request_span<B>(request: &axum::http::Request<B>, trust_proxy_headers: bool) -> Span {
    let peer_ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|address| address.ip());
    let client_ip = client_ip(request.headers(), peer_ip, trust_proxy_headers);
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .map(|request_id| request_id.0.to_string())
        .unwrap_or_else(|| "unknown".to_owned());
    let peer_ip = peer_ip
        .map(|address| address.to_string())
        .unwrap_or_else(|| "unknown".to_owned());
    let client_ip = client_ip
        .map(|address| address.to_string())
        .unwrap_or_else(|| "unknown".to_owned());
    let host = header_value(request.headers(), HOST);
    let user_agent = header_value(request.headers(), USER_AGENT);

    tracing::info_span!(
        "request",
        request_id,
        client_ip,
        peer_ip,
        method = %request.method(),
        path = request.uri().path(),
        http_version = ?request.version(),
        host,
        user_agent,
    )
}

fn client_ip(
    headers: &HeaderMap,
    peer_ip: Option<IpAddr>,
    trust_proxy_headers: bool,
) -> Option<IpAddr> {
    if !trust_proxy_headers {
        return peer_ip;
    }

    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .and_then(|value| value.parse().ok())
        .or(peer_ip)
}

fn header_value(headers: &HeaderMap, name: axum::http::header::HeaderName) -> &str {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
}

#[cfg(test)]
mod tests {
    use super::{add_request_id, client_ip};
    use axum::{
        Router,
        body::Body,
        http::{HeaderMap, HeaderValue, Request},
        middleware,
        routing::get,
    };
    use std::net::IpAddr;
    use tower::ServiceExt;
    use uuid::Uuid;

    fn ip(value: &str) -> IpAddr {
        value.parse().unwrap()
    }

    #[test]
    fn uses_tcp_peer_address_when_proxy_headers_are_untrusted() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("198.51.100.1"));

        assert_eq!(
            client_ip(&headers, Some(ip("192.0.2.1")), false),
            Some(ip("192.0.2.1"))
        );
    }

    #[test]
    fn uses_first_forwarded_address_when_proxy_headers_are_trusted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.1, 192.0.2.1"),
        );

        assert_eq!(
            client_ip(&headers, Some(ip("192.0.2.1")), true),
            Some(ip("198.51.100.1"))
        );
    }

    #[test]
    fn falls_back_to_tcp_peer_address_for_invalid_forwarded_address() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("invalid"));

        assert_eq!(
            client_ip(&headers, Some(ip("192.0.2.1")), true),
            Some(ip("192.0.2.1"))
        );
    }

    #[tokio::test]
    async fn returns_a_generated_request_id_header() {
        let app = Router::new()
            .route("/", get(|| async {}))
            .layer(middleware::from_fn(add_request_id));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let request_id = response.headers().get("x-request-id").unwrap();
        let request_id = request_id.to_str().unwrap();
        assert!(Uuid::parse_str(request_id).is_ok());
    }
}
