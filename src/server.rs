use std::net::SocketAddr;

use anyhow::Ok;
use axum::Router;
use tokio::net::TcpListener;

use crate::{app::AppState, config::ServerConfig};

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
        Router::new()
            .merge(router)
            .with_state(state)
    }
}
