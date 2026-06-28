use crate::codes;
use crate::symbol_table::SymbolTable;

use std::error::Error;

enum Instruction {
    A(String),
    C(C),
}

impl Instruction {
    pub fn transform(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Instruction::A(s) => {
                let num: u16 = s[1..].trim().parse()?;
                Ok(format!("0{num:015b}"))
            }
            Instruction::C(c) => {
                let comp: &str = codes::comp(&c.comp)?;
                let dest: &str = codes::dest(c.dest.as_deref())?;
                let jump: &str = codes::jump(c.jump.as_deref())?;
                Ok(format!("111{comp}{dest}{jump}"))
            }
        }
    }
}

struct C {
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}

fn parse(input: &str) -> Instruction {
    if input.starts_with('@') {
        return Instruction::A(input.to_string());
    }
    let (dest, rest) = if let Some((first, second)) = input.split_once('=') {
        (Some(first.trim().to_string()), second)
    } else {
        (None, input)
    };
    let (comp, jump) = if let Some((first, second)) = rest.split_once(';') {
        (first.trim().to_string(), Some(second.trim().to_string()))
    } else {
        (rest.trim().to_string(), None)
    };
    Instruction::C(C { dest, comp, jump })
}

pub fn parse_program(mut symbol_table: SymbolTable, program: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    for line in program {
        if line.starts_with('(') {
            continue;
        }
        // check and substitude var
        let raw_line = match is_variable_instruction(&line) {
            Some(var) => {
                let value = symbol_table.get_value(&var);
                replace_line(&value)
            },
            None => line.clone()
        };

        let instruction: Instruction = parse(&raw_line);
        let output: String = instruction.transform().map_err(|e| format!("instruction '{line}': {e}"))?;
        if result.is_empty() {
            result = output;
        } else {
            result = format!("{result}\n{output}");
        }
    }
    Ok(result)
}

fn is_variable_instruction(line: &str) -> Option<String> {
    if line.starts_with('@') && line.chars().nth(1).is_some_and(|c| !c.is_numeric())
        && let Some(var) = line.strip_prefix('@') {
            return Some(String::from(var));
    }
    None
}

fn replace_line(value: &str) -> String {
    format!("@{value}")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_works() {
        assert_eq!("1110110000010000", parse("D=A").transform().unwrap());
        assert_eq!("0000000000000101", parse("@5").transform().unwrap());
        assert_eq!("1110011111011001", parse("MD=D+1;JGT").transform().unwrap());
        assert_eq!("1110101010000111", parse("0;JMP").transform().unwrap());
    }

    #[test]
    fn parse_overflow() {
        assert!(parse("@9999999").transform().is_err());
    }


    #[test]
    fn parse_error_on_malformed() {
        assert!(parse("D = XXX").transform().is_err());
    }

    #[test]
    fn parse_program_works() {
        let mut st = SymbolTable::new();
        let program = vec!["(LOOP)".into(), "@LOOP".into(), "D=A".into()];
        st.add_labels(&program);
        assert_eq!("0000000000000000\n1110110000010000", parse_program(st, program).unwrap());
    }
}
