use std::fmt;

#[derive(Clone)]
pub struct Student {
    uid: u32,
    name: String,
    age: u16,
    group: u16,
    is_employed: bool,
}

impl Student {
    pub fn new(uid: u32, name: String, age: u16, group: u16, is_employed: bool) -> Self {
        Self { uid, name, age, group, is_employed }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u16 {
        self.age
    }

    pub fn group(&self) -> u16 {
        self.group
    }

    pub fn is_employed(&self) -> bool {
        self.is_employed
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_age(&mut self, age: u16) {
        self.age = age;
    }

    pub fn set_group(&mut self, group: u16) {
        self.group = group;
    }

    pub fn set_is_employed(&mut self, is_employed: bool) {
        self.is_employed = is_employed;
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UID: {}, Name: {}, Age: {}, Group: {}, isEmployed: {}", self.uid, self.name, self.age, self.group, self.is_employed)
    }
}