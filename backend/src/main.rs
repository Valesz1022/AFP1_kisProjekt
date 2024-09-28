use vicc_explorer::{configuration::Settings, telemetry, Application};

#[tokio::main]
async fn main() {
    telemetry::init();

    let config = Settings::parse("base.toml").unwrap();

    Application::serve(config).await.unwrap();
}
