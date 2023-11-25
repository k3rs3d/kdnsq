use hickory_resolver::TokioAsyncResolver;
use hickory_resolver::config::*;
use hickory_resolver::error::ResolveErrorKind;
use hickory_proto::error::ProtoErrorKind;

pub async fn perform_dns_query(record_type: String, hostname: String) {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // Perform the DNS query based on the record type
    match record_type.as_str() {
        "A" => match resolver.lookup_ip(hostname).await {
            Ok(ips) => ips.iter().for_each(|ip| println!("A: {}", ip)),
            Err(err) => handle_resolve_error(&err),
        },
        "AAAA" | "IPV6" => match resolver.ipv6_lookup(hostname).await {
            Ok(ips) => ips.iter().for_each(|ip| println!("AAAA: {}", ip)),
            Err(err) => handle_resolve_error(&err),
        },
        "MX" => match resolver.mx_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("MX: {:?}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        "NS" => match resolver.ns_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("NS: {:?}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        "SOA" => match resolver.soa_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("SOA: {}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        "SRV" => match resolver.srv_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("SRV: {}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        "TLSA" => match resolver.tlsa_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("TLSA: {}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        "TXT" => match resolver.txt_lookup(hostname).await {
            Ok(records) => records
                .iter()
                .for_each(|record| println!("TXT: {}", record)),
            Err(err) => handle_resolve_error(&err),
        },
        _ => {
            eprintln!("Unsupported record type: {}", record_type);
        }
    }
}

fn handle_resolve_error(err: &hickory_resolver::error::ResolveError) {
    // Match on the error kind
    match err.kind() {
        ResolveErrorKind::NoRecordsFound { query, .. } => {
            eprintln!("No records found for {:?}", query.name());
        }
        ResolveErrorKind::Io(io_err) => {
            eprintln!("IO error: {:?}", io_err);
        }
        ResolveErrorKind::Proto(proto_err) => match proto_err.kind() {
            ProtoErrorKind::Timeout => {
                eprintln!("DNS query timed out");
            }
            _ => {
                eprintln!("Protocol error: {:?}", proto_err);
            }
        },
        _ => {
            eprintln!("An unexpected error occurred: {:?}", err);
        }
    }
}
