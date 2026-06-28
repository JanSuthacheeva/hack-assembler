mod cleaner;
mod codes;
mod parser;
mod symbol_table;

use std::fs;
use std::path::{PathBuf};
use std::error::Error;
use symbol_table::SymbolTable;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let input = fs::read_to_string(config.input_file)?;

    let output = assemble(&input)?;

    fs::write(config.output_file, output)?;

    Ok(())
}

fn assemble(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions = cleaner::clean_program(input.lines().collect());
     
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_labels(&instructions);

    let output: String = parser::parse_program(symbol_table, instructions)?;

    Ok(output)
}

pub struct Config {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let output_file = match input_file.strip_suffix(".asm") {
            Some(stem) => format!("{stem}.hack"),
            None => return Err("input must be a .asm file"),
        };

        Ok(Config {
            input_file: input_file.into(),
            output_file: output_file.into(),
        })
    }
}
