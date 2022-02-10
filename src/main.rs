use std::env;           // for the env variables
use std::process;       // for the process

use minigrep::Config;   // for the Config struct

fn main()
{
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|
    {
        eprintln!("Problem occured in parsing: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.m_query);
    println!("In file {}", config.m_filename);

    if let Err(e) = minigrep::run(config)
    {
        eprintln!("Application Error {}", e);
        process::exit(1);
    }
}


