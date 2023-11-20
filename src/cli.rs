use std::env;

pub fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: kdnsq <record_type> <hostname>");
        std::process::exit(1);
    }

    (args[1].to_uppercase(), args[2].clone())
}