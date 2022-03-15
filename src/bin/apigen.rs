use std::fs::File;
use anyhow::Result;
use toy_dns::dropshot_server::api;

fn main() -> Result<()> {
    let api = api();
    let openapi = api.openapi("Toy DNS", "v0.1.0",);
    let mut out = File::create("toy-dns.json")?;
    openapi.write(&mut out)?;
    Ok(())
}
