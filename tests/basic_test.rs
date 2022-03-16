use std::sync::Arc;
use anyhow::{Context, Result, anyhow};
use toy_dns::client::{
    types::{DnsKv, DnsRecordKey, DnsRecord, Srv},
    Client
};
use std::net::Ipv6Addr;

#[tokio::test]
pub async fn aaaa_crud() -> Result<(), anyhow::Error> {

    let client = init_client_server().await?;

    // records should initially be empty
    let records = client.dns_records_get().await?;
    assert!(records.is_empty());

    // add an aaaa record
    let name = DnsRecordKey{ name: "devron.system".into() };
    let addr = Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0x1);
    let aaaa = DnsRecord::Aaaa(addr);
    client.dns_records_set(&vec![DnsKv{
        key: name.clone(),
        record: aaaa.clone(),
    }]).await?;

    // read back the aaaa record
    let records = client.dns_records_get().await?;
    assert_eq!(1, records.len());
    assert_eq!(records[0].key.name, name.name);
    match records[0].record {
        DnsRecord::Aaaa(ra) => {
            assert_eq!(ra, addr);
        }
        _ => {
            panic!("expected aaaa record")
        }
    }

    Ok(())

}

#[tokio::test]
pub async fn srv_crud() -> Result<(), anyhow::Error> {

    let client = init_client_server().await?;

    // records should initially be empty
    let records = client.dns_records_get().await?;
    assert!(records.is_empty());

    // add a srv record
    let name = DnsRecordKey{ name: "hromi.cluster".into() };
    let srv = Srv{ prio: 47, weight: 74, port: 99, target: "outpost47".into() };
    let rec = DnsRecord::Srv(srv.clone());
    client.dns_records_set(&vec![DnsKv{
        key: name.clone(),
        record: rec.clone(),
    }]).await?;

    // read back the srv record
    let records = client.dns_records_get().await?;
    assert_eq!(1, records.len());
    assert_eq!(records[0].key.name, name.name);
    match records[0].record {
        DnsRecord::Srv(ref rs) => {
            assert_eq!(rs.prio, srv.prio);
            assert_eq!(rs.weight, srv.weight);
            assert_eq!(rs.port, srv.port);
            assert_eq!(rs.target, srv.target);
        }
        _ => {
            panic!("expected srv record")
        }
    }

    Ok(())

}

async fn init_client_server()
-> Result<Client, anyhow::Error> {

    // initialize dns server config
    let (config, dropshot_port, _dns_port) = test_config()?;
    let log =
        config.log.to_logger("toy-dns").context("failed to create logger")?;

    // initialize dns server db
    let db = Arc::new(sled::open(&config.data.storage_path)?);
    db.clear()?;

    let client = Client::new(&format!("http://127.0.0.1:{}", dropshot_port), log.clone());

    // launch a dns server
    tokio::spawn(async move {
        let server = toy_dns::start_server(config, log, db).await?;
        server
            .await
            .map_err(|error_message| anyhow!("server exiting: {}", error_message))
    });

    // wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

    Ok(client)
}

fn test_config() -> Result<(toy_dns::Config, u16, u16), anyhow::Error> {

    let dropshot_port = portpicker::pick_unused_port().expect("pick port");
    let dns_port = portpicker::pick_unused_port().expect("pick port");
    let tmp_dir = tempdir::TempDir::new("toytest")?;
    let mut storage_path = tmp_dir.path().to_path_buf();
    storage_path.push("test");
    let storage_path = storage_path.to_str().unwrap().into();

    let config = toy_dns::Config{
        log: dropshot::ConfigLogging::StderrTerminal{
            level: dropshot::ConfigLoggingLevel::Info,
        },
        dropshot: dropshot::ConfigDropshot{
            bind_address: format!("127.0.0.1:{}", dropshot_port).parse().unwrap(),
            request_body_max_bytes: 1024,
            .. Default::default()
        },
        data: toy_dns::dns_data::Config{
            nmax_messages: 16,
            storage_path,
        },
        dns: toy_dns::dns_server::Config{
            bind_address: format!("127.0.0.1:{}", dns_port).parse().unwrap(),
        },
    };

    Ok((config, dropshot_port, dns_port))
}
