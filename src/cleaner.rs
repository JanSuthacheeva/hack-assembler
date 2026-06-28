
pub fn clean_program(lines: Vec<&str>) -> Vec<String> {
    let mut instructions: Vec<String> = vec![];
    let delimiter = "//";
    for l in lines {
        match l.split_once(delimiter) {
            Some((line, _)) => {
                let s: String = line.chars().filter(|c| !c.is_whitespace()).collect();
                push_if_not_empty(&mut instructions, s);
            },
            None => {
                let s: String = l.chars().filter(|c| !c.is_whitespace()).collect();
                push_if_not_empty(&mut instructions, s);
            }
        }
    }
    instructions
}

fn push_if_not_empty(instructions: &mut Vec<String>, line: String)  {
    if !line.is_empty() {
        instructions.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn strips_full_comment_line() {
        let input = vec!["// Hi","D=A"];
        assert_eq!(vec!["D=A"], clean_program(input));
    }

    #[test]
    fn strips_comment_after_line() {
        let input = vec!["D=A // Hi"];
        assert_eq!(vec!["D=A"], clean_program(input));
    }

    #[test]
    fn handles_no_comment() {
        let input = vec!["D=A", "M=D"];
        assert_eq!(vec!["D=A", "M=D"], clean_program(input));
    }

    #[test]
    fn handles_blank_line() {
        let input = vec!["", "M=D"];
        assert_eq!(vec!["M=D"], clean_program(input));
    }

    #[test]
    fn deletes_internal_blank_spaces() {
        let input = vec!["M=D", "M = M + D"];
        assert_eq!(vec!["M=D", "M=M+D"], clean_program(input));
    }
}
