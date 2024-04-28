use std::collections::HashMap;
use crate::task2::domain::teacher::Teacher;

pub struct TeacherRepository {
    teachers: HashMap<u32, Teacher>,
    teacher_free_ids: Vec<u32>,
}

impl TeacherRepository {
    pub fn new() -> Self {
        TeacherRepository {
            teachers: HashMap::new(),
            teacher_free_ids: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: String, is_doctor: bool) {
        let id_option: Option<u32> = self.teacher_free_ids.pop();
        let id: u32 = match id_option {
            None => { (self.teachers.len() + 1) as u32 }
            Some(id) => { id }
        };

        let teacher: Teacher = Teacher::new(id, name, is_doctor);
        self.teachers.insert(id, teacher);
    }

    pub fn remove(&mut self, id: u32) -> Option<Teacher> {
        // we can push the id regardless if it is valid or not because the only thing that it will
        // influence is the next id that will be assigned (not an issue here)
        self.teacher_free_ids.push(id);
        self.teachers.remove(&id)
    }

    pub fn update(&mut self, id: u32, name: String, is_doctor: bool) -> Option<Teacher> {
        let teacher: Teacher = Teacher::new(id, name, is_doctor);
        self.teachers.insert(id, teacher)
    }

    pub fn get(&mut self, teacher: Teacher) -> Option<Teacher> {
        // this function is just for simplicity
        let id: u32 = teacher.uid();
        self.get_by_id(id)
    }

    pub fn get_by_id(&mut self, id: u32) -> Option<Teacher> {
        self.teachers.get(&id).cloned()
    }

    pub fn size(&mut self) -> usize {
        self.teachers.len()
    }
}
