use std::net::SocketAddr;

use anyhow::Ok;
use axum::{Router, extract::Request, http::method};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

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
            .layer(tracing_layer)
            .with_state(state)
    }
}
