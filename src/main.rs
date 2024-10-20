use std::{env::args, process};

use cuttool::{run, Config};

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args[1..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let content = run(config).unwrap();
    println!("{}", content);
}
