use crate::codes;

pub enum Instruction {
    A(String),
    C(C),
}

impl Instruction {
    pub fn transform(&self) -> String {
        match self {
            Instruction::A(s) => {
                let num: u16 = s[1..].trim().parse().unwrap();
                format!("0{:015b}", num)
            }
            Instruction::C(c) => {
                let comp: &str = codes::comp(&c.comp);
                let dest: &str = codes::dest(&c.dest);
                let jump: &str = codes::jump(&c.jump);
                format!("111{}{}{}", comp, dest, jump)
            }
        }
    }
}

pub struct C {
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}

pub fn parse(input: &str) -> Instruction {
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
    Instruction::C(C { dest, comp, jump })
}
