use std::env;

pub fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => (String::from("A"), args[1].clone()),
        3 => (args[1].to_uppercase(), args[2].clone()),
        _ => {
            eprintln!("Usage: kdnsq <record_type> <hostname>");
            std::process::exit(1);
        }
    }
}
