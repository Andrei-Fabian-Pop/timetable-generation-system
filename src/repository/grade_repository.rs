use std::collections::HashMap;
use crate::task2::domain::grade::Grade;

pub struct GradeRepository {
    grades: HashMap<u32, Grade>,
    grade_free_ids: Vec<u32>,
}

impl GradeRepository {
    pub fn new() -> Self {
        GradeRepository {
            grades: HashMap::new(),
            grade_free_ids: Vec::new(),
        }
    }

    pub fn insert(&mut self, teacher_uid: u32, student_uid: u32, grade: u8) {
        let id_option: Option<u32> = self.grade_free_ids.pop();
        let id: u32 = match id_option {
            None => { (self.grades.len() + 1) as u32 }
            Some(id) => { id }
        };

        let grade: Grade = Grade::new(id, teacher_uid, student_uid, grade);
        self.grades.insert(id, grade);
    }

    pub fn remove(&mut self, id: u32) -> Option<Grade> {
        // we can push the id regardless if it is valid or not because the only thing that it will
        // influence is the next id that will be assigned (not an issue here)
        self.grade_free_ids.push(id);
        self.grades.remove(&id)
    }

    pub fn update(&mut self, id: u32, teacher_uid: u32, student_uid: u32, grade: u8) -> Option<Grade> {
        let grade: Grade = Grade::new(id, teacher_uid, student_uid, grade);
        self.grades.insert(id, grade)
    }

    pub fn get(&mut self, grade: Grade) -> Option<Grade> {
        // this function is just for simplicity
        let id: u32 = grade.uid();
        self.get_by_id(id)
    }

    pub fn get_by_id(&mut self, id: u32) -> Option<Grade> {
        self.grades.get(&id).cloned()
    }

    pub fn size(&mut self) -> usize {
        self.grades.len()
    }
}
