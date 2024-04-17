mod structures;
mod constants;
mod reader;
mod parse;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name.");
        return;
    }

    let name = &args[1];

    match fs::read(name) {
        Ok(data) => {
            let mut parser = parse::Parser::new(&data);
            parser.parse();
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
