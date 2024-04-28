use std::fmt;

#[derive(Clone)]
pub struct Teacher {
    uid: u32,
    name: String,
    is_doctor: bool
}

impl Teacher {
    pub fn new(uid: u32, name: String, is_doctor: bool) -> Self {
        Self { uid, name, is_doctor }
    }
    pub fn uid(&self) -> u32 {
        self.uid
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_doctor(&self) -> bool {
        self.is_doctor
    }
    pub fn set_uid(&mut self, uid: u32) {
        self.uid = uid;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_is_doctor(&mut self, is_doctor: bool) {
        self.is_doctor = is_doctor;
    }
}

impl fmt::Display for Teacher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UID: {}, Name: {}, isDoctor: {}", self.uid, self.name, self.is_doctor)
    }
}
