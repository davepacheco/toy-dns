//! Manages DNS data (configured zone(s), records, etc.)

use anyhow::Context;
use serde::Deserialize;
use slog::{info, o, trace};

/// Configuration related to data model
#[derive(Deserialize, Debug)]
pub struct Config {
    /// maximum number of channel messages to buffer
    nmax_messages: usize,
}

/// default maximum number of messages to buffer
const NMAX_MESSAGES_DEFAULT: usize = 16;

impl Default for Config {
    fn default() -> Self {
        Config { nmax_messages: NMAX_MESSAGES_DEFAULT }
    }
}

// XXX
#[derive(Debug)]
pub struct DnsRecord;
#[derive(Debug)]
pub struct DnsRecordKey;
#[derive(Debug)]
pub struct DnsResponse<T> {
    tx: tokio::sync::oneshot::Sender<T>,
}

// XXX some refactors to help
// - each variant should have its own struct containing the data.  This way we
//   can pass it to functions as a bundle without them having to consume the
//   whole enum (which might in principle be a different variant)
// - each variant's data should include some generic responder<T> so that we can
//   have common functions for logging and sending the T
#[derive(Debug)]
pub enum DnsCmd {
    // XXX
    // MakeExist(DnsRecord, DnsResponse<()>),
    // MakeGone(DnsRecordKey, DnsResponse<()>),
    GetRecords(DnsRecordKey, DnsResponse<Vec<DnsRecord>>),
}

/// Data model client
///
/// The Dropshot server has one of these to send commands to modify and update
/// the data model.
pub struct Client {
    log: slog::Logger,
    sender: tokio::sync::mpsc::Sender<DnsCmd>,
}

impl Client {
    pub fn new(log: slog::Logger, config: &Config) -> Client {
        let (sender, receiver) =
            tokio::sync::mpsc::channel(config.nmax_messages);
        let server =
            Server { log: log.new(o!("component" => "DataServer")), receiver };
        tokio::spawn(async move { data_server(server).await });
        Client { log, sender }
    }

    // XXX error type needs to be rich enough for appropriate HTTP response
    pub async fn get_records(
        &self,
        key: DnsRecordKey,
    ) -> Result<Vec<DnsRecord>, anyhow::Error> {
        slog::trace!(&self.log, "get_records"; "key" => ?key);
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .try_send(DnsCmd::GetRecords(key, DnsResponse { tx }))
            .context("send message")?;
        rx.await.context("recv response")
    }
}

/// Runs the body of the data model server event loop
async fn data_server(mut server: Server) {
    let log = &server.log;
    loop {
        trace!(log, "waiting for message");
        let msg = match server.receiver.recv().await {
            None => {
                info!(log, "exiting due to channel close");
                break;
            }
            Some(m) => m,
        };

        trace!(log, "rx message"; "message" => ?msg);
        match msg {
            DnsCmd::GetRecords(key, response) => {
                server.cmd_get_records(key, response).await;
            }
        }
    }
}

/// Data model server
pub struct Server {
    log: slog::Logger,
    receiver: tokio::sync::mpsc::Receiver<DnsCmd>,
}

impl Server {
    async fn cmd_get_records(
        &self,
        key: DnsRecordKey,
        response: DnsResponse<Vec<DnsRecord>>,
    ) {
        // XXX
        // response.send(Vec::new());
    }
}
