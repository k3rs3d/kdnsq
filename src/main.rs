mod cli;
mod dns_query;

use crate::cli::parse_args;
use crate::dns_query::perform_dns_query;
use tokio::runtime::Runtime;

fn main() {
    let (record_type, hostname) = parse_args();

    let runtime = Runtime::new().expect("error creating Tokio runtime");

    runtime.block_on(async move {
        perform_dns_query(record_type, hostname).await;
    });
}