#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = match coco_server::service::ServerConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("failed to load config: {err}");
            std::process::exit(1);
        }
    };
    if let Err(err) = coco_server::service::run(config).await {
        eprintln!("service error: {err}");
        std::process::exit(1);
    }
}
