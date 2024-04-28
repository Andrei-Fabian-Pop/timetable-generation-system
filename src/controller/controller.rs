use crate::task2::repository::student_repository::StudentRepository;

pub struct Controller {
    repository: StudentRepository,
}

impl Controller {
    pub fn new() -> Self {
        let repository: StudentRepository = StudentRepository::new();
        Self { repository }
    }
}
