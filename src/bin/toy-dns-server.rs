// See RFD 248
// See https://github.com/oxidecomputer/omicron/issues/718
//
// Milestones:
// - Dropshot server
// - Sqlite task
// - DNS task

use anyhow::anyhow;
use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    config_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let config_file = &args.config_file;
    let config_file_contents = std::fs::read_to_string(config_file)
        .with_context(|| format!("read config file {:?}", config_file))?;
    let config: toy_dns::Config = toml::from_str(&config_file_contents)
        .with_context(|| format!("parse config file {:?}", config_file))?;
    eprintln!("{:?}", config);

    let log = config.log
        .to_logger("toy-dns")
        .context("failed to create logger")?;

    let db = Arc::new(sled::open(&config.data.storage_path)?);

    {
        let db = db.clone();
        let log = log.clone();
        let config = config.dns.clone();

        tokio::spawn(async move {
            toy_dns::dns_server::run(log, db, config).await 
        });
    }

    let server = toy_dns::start_server(config, log, db).await?;
    server
        .await
        .map_err(|error_message| anyhow!("server exiting: {}", error_message))
}
