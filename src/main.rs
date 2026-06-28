use std::env;
use std::process;
use std::fs;
mod cleaner;
mod codes;
mod parser;
mod symbol_table;

use symbol_table::SymbolTable;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.input_file)?;

    // clean the lines vec from empty lines and comments
    let instructions = cleaner::clean_program(input.lines().collect());
     
    // first pass
    let mut symbol_table: SymbolTable = symbol_table::init_table();
    symbol_table.add_labels(&instructions);

    // second pass
    let output: String = parser::parse_program(symbol_table, instructions)?;
    println!("{output}");
    Ok(())
}

struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let output_file = args[2].clone();

        Ok(Config { input_file, output_file })
    }
}

