use tracing_subscriber::fmt;
use vicc_explorer::{configuration::Settings, Application};

#[tokio::main]
async fn main() {
    fmt::init();

    let config = Settings::parse().unwrap();

    let app = Application::build(config).await.unwrap();

    app.run_until_stopped().await.unwrap();
}
