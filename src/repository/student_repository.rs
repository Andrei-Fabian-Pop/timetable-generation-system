use std::collections::HashMap;
use crate::task2::domain::student::Student;

pub struct StudentRepository {
    students: HashMap<u32, Student>,
    student_free_ids: Vec<u32>,

    // teachers: Vec<Teacher>,
    // teacher_free_ids: Vec<u32>,
    //
    // grades: Vec<Grade>,
    // grade_free_ids: Vec<u32>,
}

impl StudentRepository {
    pub fn new() -> Self {
        // TODO: add the option of providing a file from where to read stuff
        StudentRepository {
            students: HashMap::new(),
            student_free_ids: Vec::new(),
            // teachers: Vec::new(),
            // teacher_free_ids: Vec::new(),
            // grades: Vec::new(),
            // grade_free_ids: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: String, age: u16, group: u16, is_employed: bool) -> u32 {
        let id_option: Option<u32> = self.student_free_ids.pop();
        let id: u32 = match id_option {
            None => { (self.students.len() + 1) as u32 }
            Some(id) => { id }
        };

        let student: Student = Student::new(id, name, age, group, is_employed);
        self.students.insert(id, student);
        id
    }

    pub fn remove(&mut self, id: u32) -> Option<Student> {
        // we can push the id regardless if it is valid or not because the only thing that it will
        // influence is the next id that will be assigned (not an issue here)
        self.student_free_ids.push(id);
        self.students.remove(&id)
    }

    pub fn update(&mut self, id: u32, name: String, age: u16, group: u16, is_employed: bool) -> Option<Student> {
        let student: Student = Student::new(id, name, age, group, is_employed);
        self.students.insert(id, student)
    }

    pub fn get(&mut self, student: Student) -> Option<Student> {
        // this function is just for simplicity
        let id: u32 = student.uid();
        self.get_by_id(id)
    }

    pub fn get_by_id(&mut self, id: u32) -> Option<Student> {
        self.students.get(&id).cloned()
    }

    pub fn size(&mut self) -> usize {
        self.students.len()
    }
}
