use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use serde_json::from_reader;

#[derive(serde::Deserialize)]
struct Network {
    chain_id: u64,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open("data/networks.json")?;
    let contents: HashMap<String, Network> = from_reader(file).unwrap_or_else(|err| {
        eprintln!("Error reading file, should not happen: {}", err);
        std::process::exit(1);
    });

    let network_data = contents.get(&config.chain).unwrap_or_else(|| {
        eprintln!("Chain not found");
        std::process::exit(1);
    });
    println!("{}", network_data.chain_id);

    Ok(())
}
pub struct Config {
    pub chain: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let chain = args[1].clone();

        Ok(Config {
            chain: chain.to_lowercase(),
        })
    }
}
