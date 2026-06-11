mod codes;
mod parser;

use parser::Instruction;
use std::io;

fn main() {
    loop {
        println!("Instruction:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let instruction: Instruction = parser::parse(&input);
        let output: String = instruction.transform();
        println!("{output}");
    }
}
