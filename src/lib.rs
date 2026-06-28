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
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let mut output_file: Option<String> = None;
        let mut input_file: Option<String> = None;

        if args.len() < 2 {
            return Err("not enough arguments".into());
        }
        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--output" | "-o" => {
                    let value = iter.next().ok_or("missing value for --output")?;
                    output_file = Some(String::from(value));
                }
                _ => { 
                    if arg.starts_with('-') {

                        return Err(format!("unknown argument: {arg}").into());
                    } 
                    if input_file.is_some() {
                        return Err("too many arguments".into());
                    }
                    input_file = Some(arg.clone());
                }
            }
        }
        let input_file = input_file.ok_or("no input file")?;

        let stem = input_file
            .strip_suffix(".asm")
            .ok_or("input must be a .asm file")?;
        let output_file = output_file
            .unwrap_or_else(|| format!("{stem}.hack"));

        Ok(Config {
            input_file: input_file.into(),
            output_file: output_file.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_no_flags() {
        let args = [String::from("test"), String::from("input.asm")]; 
        let config = Config::build(&args).unwrap();
        assert_eq!(PathBuf::from("input.asm"), config.input_file);
        assert_eq!(PathBuf::from("input.hack"), config.output_file);
    }

    #[test]
    fn happy_path_output_flag_after_input() {
        let args = [String::from("test"), String::from("input.asm"), String::from("--output"), String::from("test.hack")]; 
        let config = Config::build(&args).unwrap();
        assert_eq!(PathBuf::from("input.asm"), config.input_file);
        assert_eq!(PathBuf::from("test.hack"), config.output_file);
    }

    #[test]
    fn happy_path_o_flag_after_input() {
        let args = [String::from("test"), String::from("input.asm"), String::from("-o"), String::from("test.hack")]; 
        let config = Config::build(&args).unwrap();
        assert_eq!(PathBuf::from("input.asm"), config.input_file);
        assert_eq!(PathBuf::from("test.hack"), config.output_file);
    }

    #[test]
    fn happy_path_o_flag_before_input() {
        let args = [String::from("test"), String::from("-o"), String::from("test.hack"), String::from("input.asm")]; 
        let config = Config::build(&args).unwrap();
        assert_eq!(PathBuf::from("input.asm"), config.input_file);
        assert_eq!(PathBuf::from("test.hack"), config.output_file);
    }

    #[test]
    fn input_file_not_asm() {
        let args = [String::from("test"), String::from("input.notasm")]; 
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn too_many_args() {
        let args = [String::from("test"), String::from("input.asm"), String::from("input2.asm")]; 
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn unknown_flag() {
        let args = [String::from("test"), String::from("input.asm"), String::from("--unknown")]; 
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn no_output_provided() {
        let args = [String::from("test"), String::from("input.asm"), String::from("--output")]; 
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn no_input_provided() {
        let args = [String::from("test"), String::from("--output"), String::from("input.hack")]; 
        let config = Config::build(&args);
        assert!(config.is_err());
    }
}
