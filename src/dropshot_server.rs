//! Dropshot server for configuring DNS namespace

use crate::dns_data::{
    self,
    DnsRecordKey,
    DnsRecord,
};
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
    api.register(dns_records_set).unwrap(); // XXX unwrap
    api.register(dns_records_delete).unwrap(); // XXX unwrap
    api
}

#[endpoint(
    method = GET,
    path = "/get-records",
)]
async fn dns_records_get(
    rqctx: Arc<dropshot::RequestContext<Arc<Context>>>,
) -> Result<
        dropshot::HttpResponseOk<Vec<(DnsRecordKey,DnsRecord)>>,
        dropshot::HttpError
    > 
{
    let apictx = rqctx.context();
    // XXX record key
    let records = apictx
        .client
        .get_records(None)
        .await
        .map_err(|e| {
            dropshot::HttpError::for_internal_error(format!("uh oh: {:?}", e))
        })?;
    Ok(dropshot::HttpResponseOk(records))
}

#[endpoint(
    method = PUT,
    path = "/set-records",
)]
async fn dns_records_set(
    rqctx: Arc<dropshot::RequestContext<Arc<Context>>>,
    rq: dropshot::TypedBody<Vec<(DnsRecordKey, DnsRecord)>>,
) -> Result<dropshot::HttpResponseOk<()>, dropshot::HttpError> {
    let apictx = rqctx.context();
    apictx
        .client
        .set_records(rq.into_inner())
        .await
        .map_err(|e| {
            dropshot::HttpError::for_internal_error(format!("uh oh: {:?}", e))
        })?;
    Ok(dropshot::HttpResponseOk(()))
}

#[endpoint(
    method = PUT,
    path = "/delete-records",
)]
async fn dns_records_delete(
    rqctx: Arc<dropshot::RequestContext<Arc<Context>>>,
    rq: dropshot::TypedBody<Vec<DnsRecordKey>>,
) -> Result<dropshot::HttpResponseOk<()>, dropshot::HttpError> {
    let apictx = rqctx.context();
    apictx
        .client
        .delete_records(rq.into_inner())
        .await
        .map_err(|e| {
            dropshot::HttpError::for_internal_error(format!("uh oh: {:?}", e))
        })?;
    Ok(dropshot::HttpResponseOk(()))
}
