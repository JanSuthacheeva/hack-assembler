mod cleaner;
mod codes;
mod parser;
mod symbol_table;

use std::error::Error;
use symbol_table::SymbolTable;



pub fn assemble(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions = cleaner::clean_program(input.lines().collect());
     
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_labels(&instructions);

    let output: String = parser::parse_program(symbol_table, instructions)?;

    Ok(output)
}

