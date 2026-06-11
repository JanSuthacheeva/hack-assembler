use std::io;

enum Instruction {
    A(String),
    C(C),
}

impl Instruction {
    fn transform(&self) -> String {
        match self {
            Instruction::A(s) => {
                let num: u16 = s[1..].trim().parse().unwrap();
                format!("0{:015b}", num)
            }
            Instruction::C(c) => {
                let comp: &str = transform_comp(&c.comp);
                let dest: &str = transform_dest(&c.dest);
                let jump: &str = transform_jump(&c.jump);
                format!("111{}{}{}", comp, dest, jump)
            }
        }
    }
}

struct C {
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}

fn main() {
    loop {
        println!("Instruction:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let instruction: Instruction = parse_instruction(&input);
        let output: String = instruction.transform();
        println!("{output}");
    }
}

fn parse_instruction(input: &str) -> Instruction {
    if input.starts_with("@") {
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
    Instruction::C(C {
        dest,
        comp,
        jump,
    })
}

fn transform_comp(comp: &str) -> &str {
    match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        c => panic!("{c} is not a valid comp value"),
    }
}

fn transform_jump(jump: &Option<String>) -> &str {
    match jump.as_deref() {
        None => "000",
        Some("JGT") => "001",
        Some("JEQ") => "010",
        Some("JGE") => "011",
        Some("JLT") => "100",
        Some("JNE") => "101",
        Some("JLE") => "110",
        Some("JMP") => "111",
        Some(j) => panic!("{j} is not a valid jump value"),
    }
}

fn transform_dest(dest: &Option<String>) -> &str {
    match dest.as_deref() {
        None => "000",
        Some("M") => "001",
        Some("D") => "010",
        Some("MD") => "011",
        Some("A") => "100",
        Some("AM") => "101",
        Some("AD") => "110",
        Some("AMD") => "111",
        Some(d) => panic!("{d} is not a valid dest value"),
    }
}
