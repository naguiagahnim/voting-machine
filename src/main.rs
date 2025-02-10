use votingmachine::app_builder::run_app;
use votingmachine::configuration::Configuration;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::new();
    run_app(configuration).await
}
