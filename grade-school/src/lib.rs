use std::collections::HashMap;

#[derive(Default)]
pub struct School<'a> {
    students: HashMap<u32, Vec<&'a str>>,
}

impl School<'static> {
    pub fn new() -> School<'static> {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &'static str) {
        match self.students.get_mut(&grade) {
            Some(v) => {
                v.push(student);
            }
            None => {
                self.students.insert(grade, vec![student]);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.iter().map(|s| *s.0).collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students.get(&grade) {
            Some(n) => {
                let mut names: Vec<String> = n.iter().map(|name| (*name).to_string()).collect();
                names.sort();
                Some(names)
            }
            None => None,
        }
    }
}
