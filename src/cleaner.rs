
pub fn clean_program(lines: Vec<&str>) -> Vec<&str> {
    let mut instructions: Vec<&str> = vec![];
    let delimiter = "//";
    for l in lines {
        match l.split_once(delimiter) {
            Some((line, _)) => {
                let cleaned_line = line.trim();
                if !cleaned_line.is_empty() {
                    instructions.push(cleaned_line);
                }
            },
            None => {
                let cleaned_line = l.trim();
                if !cleaned_line.is_empty() {
                    instructions.push(cleaned_line);
                }
            }
        }
    }
    instructions
}


// Use this when lifetimes are clear
//fn push_if_not_empty(instructions: &mut Vec<&str>, line: &str)  {
//    let cleaned_line = line.trim();
//    if !cleaned_line.is_empty() {
//        instructions.push(cleaned_line);
//    }
//}
