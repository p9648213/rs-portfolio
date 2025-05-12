use std::time::Duration;

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, header},
    middleware::{from_fn, from_fn_with_state},
    response::{IntoResponse, Response},
    routing::get,
};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_redispool::SessionRedisPool;
use deadpool_postgres::Pool;
use redis_pool::SingleRedisPool;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};
use tracing::Span;

use crate::real_estate::router::create_real_estate_router;
use crate::{
    common::{
        middlewares::{auth_mw::auth_middleware, csrf_mw::csrf_middleware, log_mw::request_log},
        models::state::AppState,
        router::create_auth_router,
        utilities::config::EnvConfig,
    },
    portfolio::router::create_portfolio_router,
};

async fn ping() -> StatusCode {
    StatusCode::OK
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

fn response_log(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!("<- Response: status {} in {:?}", response.status(), latency)
}

pub async fn create_router(
    pg_pool: Pool,
    redis_pool: SingleRedisPool,
    config: EnvConfig,
) -> Router {
    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let session_config = SessionConfig::default();

    let session_store =
        SessionStore::<SessionRedisPool>::new(Some(redis_pool.clone().into()), session_config)
            .await
            .unwrap();

    let app_state = AppState { pg_pool, config };

    let portfolio_route = create_portfolio_router();
    let auth_route = create_auth_router();
    let real_estate_route = create_real_estate_router();

    Router::new()
        .merge(portfolio_route)
        .merge(real_estate_route)
        .merge(auth_route)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(SessionLayer::new(session_store))
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .fallback(fallback)
        .layer(TraceLayer::new_for_http().on_response(response_log))
        .layer(from_fn(request_log))
}
