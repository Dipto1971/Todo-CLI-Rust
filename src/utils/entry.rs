use colored::*;

pub struct Entry {
    pub todo_entry: String,
    pub done: bool
}

impl Entry {
    pub fn new (todo_entry: String, done: bool) -> Self {
        Self{
            todo_entry,
            done
        }
    }
    pub fn file_line(&self) -> String {
        let symbol = if self.done {
            "[* ]"
        } else {
            "[ ]"
        };
        format!("{} {}\n", symbol, self.todo_entry)
    }

    pub fn list_line(&self, number: usize) -> String {
        let todo_entry = if self.done{
            self.todo_entry.strikethrough().to_string()
        } else {
            self.todo_entry.clone()
        };
        format!("{number} {todo_entry}\n")
    }

    pub fn read_line(line: &String) -> Self {
        let done = &line[..4] == "[*] ";
        let todo_entry= (&line[4..]).to_string();
        Self{
            todo_entry,
            done
        }
    }
    pub fn raw_line(&self) -> String {
        format!("{}\n", self.todo_entry)
    }
}