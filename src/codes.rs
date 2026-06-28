use std::error::Error;

pub fn comp(comp: &str) -> Result<&str, Box<dyn Error>> {
    match comp {
        "0" => Ok("0101010"),
        "1" => Ok("0111111"),
        "-1" => Ok("0111010"),
        "D" => Ok("0001100"),
        "A" => Ok("0110000"),
        "M" => Ok("1110000"),
        "!D" => Ok("0001101"),
        "!A" => Ok("0110001"),
        "!M" => Ok("1110001"),
        "-D" => Ok("0001111"),
        "-A" => Ok("0110011"),
        "-M" => Ok("1110011"),
        "D+1" => Ok("0011111"),
        "A+1" => Ok("0110111"),
        "M+1" => Ok("1110111"),
        "D-1" => Ok("0001110"),
        "A-1" => Ok("0110010"),
        "M-1" => Ok("1110010"),
        "D+A" => Ok("0000010"),
        "D+M" => Ok("1000010"),
        "D-A" => Ok("0010011"),
        "D-M" => Ok("1010011"),
        "A-D" => Ok("0000111"),
        "M-D" => Ok("1000111"),
        "D&A" => Ok("0000000"),
        "D&M" => Ok("1000000"),
        "D|A" => Ok("0010101"),
        "D|M" => Ok("1010101"),
        c => Err(format!("{c} is not a valid comp value").into()),
    }
}

pub fn jump(jump: Option<&str>) -> Result<&'static str, Box<dyn Error>> {
    match jump {
        None => Ok("000"),
        Some("JGT") => Ok("001"),
        Some("JEQ") => Ok("010"),
        Some("JGE") => Ok("011"),
        Some("JLT") => Ok("100"),
        Some("JNE") => Ok("101"),
        Some("JLE") => Ok("110"),
        Some("JMP") => Ok("111"),
        Some(j) => Err(format!("{j} is not a valid jump value").into()),
    }
}

pub fn dest(dest: Option<&str>) -> Result<&'static str, Box<dyn Error>> {
    match dest {
        None => Ok("000"),
        Some("M") => Ok("001"),
        Some("D") => Ok("010"),
        Some("MD") => Ok("011"),
        Some("A") => Ok("100"),
        Some("AM") => Ok("101"),
        Some("AD") => Ok("110"),
        Some("AMD") => Ok("111"),
        Some(d) => Err(format!("{d} is not a valid dest value").into()),
    }
}

