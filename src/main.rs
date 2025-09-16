mod calculator;

use calculator::Calculator;
use clap::{Parser, ValueEnum};
use rmcp::transport::{
    sse_server::SseServer,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8001";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use (sse or stream-http)
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
}

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Sse,
    StreamHttp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    let args = Args::parse();

    tracing::info!(
        "Starting Cardea Calculator MCP server on {}",
        args.socket_addr
    );

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || Ok(Calculator::new()),
                LocalSessionManager::default().into(),
                Default::default(),
            );

            let router = axum::Router::new().nest_service("/mcp", service);
            let tcp_listener = tokio::net::TcpListener::bind(args.socket_addr).await?;
            let _ = axum::serve(tcp_listener, router)
                .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
                .await;
        }
        TransportType::Sse => {
            let ct = SseServer::serve(args.socket_addr.parse()?)
                .await?
                .with_service(Calculator::new);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
    }

    Ok(())
}
