use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Grade {
    uid: u32,
    teacher_uid: u32,
    student_uid: u32,
    grade: u8
}

impl Grade {
    pub fn new(uid: u32, teacher_uid: u32, student_uid: u32, grade: u8) -> Self {
        Self {uid, teacher_uid, student_uid, grade}
    }
    pub fn uid(self) -> u32 {
        self.uid
    }
    pub fn teacher_uid(&self) -> u32 {
        self.teacher_uid
    }
    pub fn student_uid(&self) -> u32 {
        self.student_uid
    }
    pub fn grade(&self) -> u8 {
        self.grade
    }
    pub fn set_uid(&mut self, uid: u32) {
        self.uid = uid;
    }
    pub fn set_teacher_uid(&mut self, teacher_uid: u32) {
        self.teacher_uid = teacher_uid;
    }
    pub fn set_student_uid(&mut self, student_uid: u32) {
        self.student_uid = student_uid;
    }
    pub fn set_grade(&mut self, grade: u8) {
        self.grade = grade;
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "UID: {}, teacherId: {}, studentId: {}, grade: {}", self.uid, self.teacher_uid, self.teacher_uid, self.grade)
    }
}
