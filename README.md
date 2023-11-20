# kDNSq - Kersed DNS Query

A simple command-line DNS querying tool built in Rust. It utilizes `trust-dns-resolver` and `tokio` to perform asynchronous DNS queries. 

## Features

- Supported record types:
	- A: IPv4 addresses
	- AAAA: IPv6 addresses
	- MX: Mail exchange records
	- NS: Name server records
	- TXT: Multi-purpose records
- Asynchronous resolution using Tokio.
- Basic error handling.


## Usage

The basic usage pattern is as follows:

`kdnsq <RECORD_TYPE> <HOSTNAME>`

Where:

- `<RECORD_TYPE>` is one of A, AAAA, MX, NS, TXT.
- `<HOSTNAME>` is the domain name to query.

Example:

`kdnsq A www.example.com`

This command will query the A records for www.example.com.

To run the project directly from the source without building, use Cargo's run command:

`cargo run -- A www.example.com`

## Contributions

Contributions are welcome! 

If you have a feature request, bug report, etc, please feel free to submit an issue or open a pull request.