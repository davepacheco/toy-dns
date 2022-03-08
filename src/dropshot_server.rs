//! Dropshot server for configuring DNS namespace

use crate::dns_data;
use dropshot::endpoint;
use std::sync::Arc;

pub struct Context {
    client: dns_data::Client,
}

impl Context {
    pub fn new(client: dns_data::Client) -> Context {
        Context { client }
    }
}

pub fn api() -> dropshot::ApiDescription<Arc<Context>> {
    let mut api = dropshot::ApiDescription::new();

    // XXX
    // api.register(dns_zone_put).unwrap();
    // api.register(dns_record_put).unwrap();
    api.register(dns_records_get).unwrap(); // XXX unwrap
    api
}

#[endpoint(
    method = GET,
    path = "/dummy",
)]
async fn dns_records_get(
    rqctx: Arc<dropshot::RequestContext<Arc<Context>>>,
) -> Result<dropshot::HttpResponseOk<usize>, dropshot::HttpError> {
    let apictx = rqctx.context();
    // XXX record key
    let records = apictx
        .client
        .get_records(dns_data::DnsRecordKey {})
        .await
        .map_err(|e| {
            dropshot::HttpError::for_internal_error(format!("uh oh: {:?}", e))
        })?;
    Ok(dropshot::HttpResponseOk(records.len()))
}
