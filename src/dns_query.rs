use trust_dns_resolver::error::ResolveErrorKind;
use trust_dns_resolver::proto::error::ProtoErrorKind;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn perform_dns_query(record_type: String, hostname: String) {
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );

    // Perform the DNS query based on the record type
    match record_type.as_str() {
        "A" => {
            match resolver.lookup_ip(hostname).await {
                Ok(ips) => ips.iter().for_each(|ip| println!("A: {}", ip)),
                Err(err) => handle_resolve_error(&err),
            }
        }
        "AAAA" => {
            match resolver.ipv6_lookup(hostname).await {
                Ok(ips) => ips.iter().for_each(|ip| println!("AAAA: {}", ip)),
                Err(err) => handle_resolve_error(&err),
            }
        }
        "MX" => {
            match resolver.mx_lookup(hostname).await {
                Ok(records) => records.iter().for_each(|record| println!("MX: {:?}", record)),
                Err(err) => handle_resolve_error(&err),
            }
        }
        "NS" => {
            match resolver.ns_lookup(hostname).await {
                Ok(records) => records.iter().for_each(|record| println!("NS: {:?}", record)),
                Err(err) => handle_resolve_error(&err),
            }
        }
        // Add here any additional record types
        _ => {
            eprintln!("Unsupported record type: {}", record_type);
        }
    }
}


fn handle_resolve_error(err: &trust_dns_resolver::error::ResolveError) {
    // Match on the error kind
    match err.kind() {
        ResolveErrorKind::NoRecordsFound { query, .. } => {
            eprintln!("No records found for {:?}", query.name());
        }
        ResolveErrorKind::Io(io_err) => {
            eprintln!("IO error occurred: {:?}", io_err);
        }
        ResolveErrorKind::Proto(proto_err) => match proto_err.kind() {
            ProtoErrorKind::Timeout => {
                eprintln!("DNS query timed out");
            }
            _ => {
                eprintln!("Protocol error occurred: {:?}", proto_err);
            }
        },
        _ => {
            eprintln!("An unexpected error occurred: {:?}", err);
        }
    }
}