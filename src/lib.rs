use anyhow::Context;
use serde::Deserialize;
use std::sync::Arc;

mod dns_data;
pub mod dns_server;
pub mod dropshot_server;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log: dropshot::ConfigLogging,
    pub dropshot: dropshot::ConfigDropshot,
    pub data: dns_data::Config,
    pub dns: dns_server::Config,
}

pub async fn start_server(
    config: Config,
    log: slog::Logger,
    db: Arc::<sled::Db>,
) -> Result<dropshot::HttpServer<Arc<dropshot_server::Context>>, anyhow::Error>
{
    let data_client = dns_data::Client::new(
        log.new(slog::o!("component" => "DataClient")),
        &config.data,
        db,
    );

    let api = dropshot_server::api();
    let api_context = Arc::new(dropshot_server::Context::new(data_client));

    Ok(dropshot::HttpServerStarter::new(
        &config.dropshot,
        api,
        api_context,
        &log,
    )
    .context("init dropshot")?
    .start())
}
