use structopt::{
    StructOpt,
    clap::AppSettings::*,
};
use slog::{Logger, Drain};
use anyhow::Result;
use std::net::Ipv6Addr;
use toy_dns::client::{
    types::{DnsKv, DnsRecordKey, DnsRecord, Srv},
    Client
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "toyadm",
    about = "Administer toys and their names",
    global_setting(ColorAuto),
    global_setting(ColoredHelp),
)]
struct Opt {
    #[structopt(short, long)]
    address: Option<String>,

    #[structopt(short, long)]
    port: Option<usize>,

    #[structopt(subcommand)]
    subcommand: SubCommand
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    ListRecords,
    AddAAAA(AddAAAACommand),
    AddSRV(AddSRVCommand),
    DeleteRecord(DeleteRecordCommand),
}

#[derive(Debug, StructOpt)]
struct AddAAAACommand {
    name: String,
    addr: Ipv6Addr,
}

#[derive(Debug, StructOpt)]
struct AddSRVCommand {
    name: String,
    prio: u16,
    weight: u16,
    port: u16,
    target: String,
}

#[derive(Debug, StructOpt)]
struct DeleteRecordCommand {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    let opt = Opt::from_args();
    let log = init_logger();

    let addr = match opt.address {
        Some(a) => a,
        None => "localhost".into(),
    };
    let port = match opt.port {
        Some(p) => p,
        None => 5353,
    };

    let endpoint = format!("http://{}:{}", addr, port);
    let client = Client::new(&endpoint, log.clone());

    let opt = Opt::from_args();
    match opt.subcommand {
        SubCommand::ListRecords => {
            let records = client.dns_records_get().await?;
            println!("{:#?}", records);
        }
        SubCommand::AddAAAA(cmd) => {
            client.dns_records_set(&vec![DnsKv{
                key: DnsRecordKey{ name: cmd.name },
                record: DnsRecord::Aaaa(cmd.addr),
            }]).await?;
        }
        SubCommand::AddSRV(cmd) => {
            client.dns_records_set(&vec![DnsKv{
                key: DnsRecordKey{ name: cmd.name },
                record: DnsRecord::Srv(Srv{
                    prio: cmd.prio,
                    weight: cmd.weight,
                    port: cmd.port,
                    target: cmd.target,
                }),
            }]).await?;
        }
        SubCommand::DeleteRecord(cmd) => {
            client.dns_records_delete(&vec![DnsRecordKey{name: cmd.name}]).await?;
        }
    }

    Ok(())

}

fn init_logger() -> Logger {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(drain).fuse();
    let drain = slog_async::Async::new(drain).chan_size(0x2000).build().fuse();
    slog::Logger::root(drain, slog::o!())

}
