#[derive(Debug, Clone)]
pub struct SubjectSpecs {
    pub title: &'static str,
    pub classes_per_week: i32,
}

pub const DAYS: usize = 5;
pub const GROUPS: usize = 5;
pub const CLASSES_PER_DAY: usize = 6;
pub const GENETIC_POPULATION_SIZE: usize = 10;
pub const GENETIC_GENERATIONS: usize = 1000;

pub const SUBJECTS: &[SubjectSpecs] = &[
    SubjectSpecs { title: "Object Oriented Programming, C", classes_per_week: 1 },
    SubjectSpecs { title: "Object Oriented Programming, L", classes_per_week: 1 },
    SubjectSpecs { title: "Database Management, C", classes_per_week: 1 },
    SubjectSpecs { title: "Database Management, S", classes_per_week: 1 },
    SubjectSpecs { title: "Database Management, L", classes_per_week: 1 },
    SubjectSpecs { title: "Operating Systems", classes_per_week: 1 },
    SubjectSpecs { title: "Computer System Architecture", classes_per_week: 1 },
    SubjectSpecs { title: "Software Engineering", classes_per_week: 1 },
    SubjectSpecs { title: "Graph Algorithms", classes_per_week: 1 },
    SubjectSpecs { title: "Data Structures and Algorithms", classes_per_week: 1 },
    SubjectSpecs { title: "Parallel and Distributed Systems", classes_per_week: 1 },
];

pub const WEEKDAYS: &[&str] = &["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

// Type aliases for ease of use
pub type DayTimetable = Vec<SubjectSpecs>;
pub type GroupTimetable = Vec<DayTimetable>;
pub type FullTimetable = Vec<GroupTimetable>;
pub type Matrix = Vec<Vec<i32>>;
