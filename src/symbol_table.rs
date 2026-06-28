use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u32>,
    counter: u32
}

impl SymbolTable {

    pub fn new() -> SymbolTable {
        let mut table: HashMap<String, u32> = HashMap::new();
    
        table.insert(String::from("R0"), 0);
        table.insert(String::from("R1"), 1);
        table.insert(String::from("R2"), 2);
        table.insert(String::from("R3"), 3);
        table.insert(String::from("R4"), 4);
        table.insert(String::from("R5"), 5);
        table.insert(String::from("R6"), 6);
        table.insert(String::from("R7"), 7);
        table.insert(String::from("R8"), 8);
        table.insert(String::from("R9"), 9);
        table.insert(String::from("R10"), 10);
        table.insert(String::from("R11"), 11);
        table.insert(String::from("R12"), 12);
        table.insert(String::from("R13"), 13);
        table.insert(String::from("R14"), 14);
        table.insert(String::from("R15"), 15);
        table.insert(String::from("SCREEN"), 16384);
        table.insert(String::from("KBD"), 24576);
        table.insert(String::from("SP"), 0);
        table.insert(String::from("LCL"), 1);
        table.insert(String::from("ARG"), 2);
        table.insert(String::from("THIS"), 3);
        table.insert(String::from("THAT"), 4);

        SymbolTable {
            table,
            counter: 16
        }
    }

    pub fn add_labels(&mut self, instructions: &Vec<String>) {
        let mut line: u32 = 0;
        for instruction in instructions {
            if let Some(label) = instruction.strip_prefix("(")
                .and_then(|s| s.strip_suffix(")")) {
                self.table.entry(String::from(label)).or_insert(line);
            } else {
                line += 1;
            }
        }
    }

    pub fn get_value(&mut self, var: &String) -> String {
        let value = self.table.entry(String::from(var)).or_insert(self.counter);
        if *value == self.counter {
            self.counter += 1;
        }
        format!("{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_table() {
        let mut st = SymbolTable::new();
        assert_eq!("5", st.get_value(&"R5".into()));
        assert_eq!("16384", st.get_value(&"SCREEN".into()));
    }

    #[test]
    fn returns_right_existing_value() {
        let mut st = SymbolTable::new();
        assert_eq!(String::from("3"), st.get_value(&String::from("THIS")));
    }

    #[test]
    fn adds_label() {
        let mut st = SymbolTable::new();
        let instructions = vec![String::from("M=M+D"), String::from("(LOOP)"), String::from("@20")];
        st.add_labels(&instructions);
        assert!(st.table.contains_key(&String::from("LOOP")));
        assert_eq!("1", st.get_value(&"LOOP".into()));
    }

    #[test]
    fn returns_right_new_value() {
        let mut st = SymbolTable::new();
        assert_eq!("16", st.get_value(&String::from("TEST")));
        assert_eq!("17", st.get_value(&String::from("TESTTWO")));
    }
}
