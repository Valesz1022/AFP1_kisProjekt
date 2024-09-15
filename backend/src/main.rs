use vicc_explorer::{configuration::Settings, telemetry, Application};

#[tokio::main]
async fn main() {
    telemetry::init();

    let config = Settings::parse("base.toml").unwrap();

    let app = Application::build(config).await.unwrap();

    app.run_until_stopped().await.unwrap();
}
