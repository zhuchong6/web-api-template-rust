use std::{net::SocketAddr, time::Duration};

use anyhow::Ok;
use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
    http::StatusCode,
};
use bytesize::ByteSize;
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{app::AppState, config::ServerConfig, latency::LatencyOnResponse};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        // 创建路由
        let router = self.build_router(state, router);
        // 监听端口
        let port = self.config.port();
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        tracing::info!("Listening on http://{}", listener.local_addr()?);

        // 启动服务
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let time_out_layer =
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5));

        let body_size_layer = DefaultBodyLimit::max(ByteSize::mb(10).as_u64() as usize);

        let cors_layer = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));

        let normal_layer = NormalizePathLayer::trim_trailing_slash();

        let tracing_layer = TraceLayer::new_for_http()
            .make_span_with(|req: &Request| {
                let method = req.method();
                let path = req.uri().path();
                let id = xid::new();
                tracing::info_span!("API Request", id=%id,  method=%method, path=%path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);

        Router::new()
            .merge(router)
            .layer(time_out_layer)
            .layer(body_size_layer)
            .layer(tracing_layer)
            .layer(cors_layer)
            .layer(normal_layer)
            .with_state(state)
    }
}
