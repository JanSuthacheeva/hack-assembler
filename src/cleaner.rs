
pub fn clean_program(lines: Vec<&str>) -> Vec<&str> {
    let mut instructions: Vec<&str> = vec![];
    let delimiter = "//";
    for l in lines {
        match l.split_once(delimiter) {
            Some((line, _)) => {
                push_if_not_empty(&mut instructions, line);
            },
            None => {
                push_if_not_empty(&mut instructions, l);
            }
        }
    }
    instructions
}


// Use this when lifetimes are clear
fn push_if_not_empty<'a>(instructions: &mut Vec<&'a str>, line: &'a str)  {
    let cleaned_line = line.trim();
    if !cleaned_line.is_empty() {
        instructions.push(cleaned_line);
    }
}
