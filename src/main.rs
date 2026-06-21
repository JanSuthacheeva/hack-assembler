mod cleaner;
mod codes;
mod parser;
mod symbol_table;

use symbol_table::SymbolTable;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
     let input = "
     // HI
     @a // this is two
     D=A
     (LOOP)
     @3
     D=D+A // yep makes sense
     // this is null
     @0
     (CAFFEEE)
     M=D";

     let lines: Vec<&str> = input.lines().collect();

     // clean the lines vec from empty lines and comments
     let instructions = cleaner::clean_program(lines);
     
     // first pass
     let mut symbol_table: SymbolTable = symbol_table::init_table();
     symbol_table.add_labels(&instructions);

     // second pass
     let output: String = parser::parse_program(symbol_table, instructions)?;
     println!("{output}");
     Ok(())
}
