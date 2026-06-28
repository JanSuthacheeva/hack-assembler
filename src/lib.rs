mod cleaner;
mod codes;
mod parser;
mod symbol_table;

use std::fs;
use std::error::Error;
use symbol_table::SymbolTable;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let input = fs::read_to_string(config.input_file)?;

    let output = assemble(&input)?;

    fs::write(config.output_file, output)?;

    Ok(())
}

fn assemble(input: &str) -> Result<String, Box<dyn Error>> {
    // clean the lines vec from empty lines and comments
    let instructions = cleaner::clean_program(input.lines().collect());
     
    // first pass
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_labels(&instructions);

    // second pass
    let output: String = parser::parse_program(symbol_table, instructions)?;

    Ok(output)
}

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let output_file = args[2].clone();

        Ok(Config { input_file, output_file })
    }
}
