// Update this if you want a subject to have more information, e.g. the type (Course, Seminary, Laboratory)
// In case you modify this, not forget to modify the logic if needed.
#[derive(Debug, Clone)]
pub struct SubjectSpecs {
    pub title: &'static str,
    pub classes_per_week: i32,
}

// Real life information here
pub const DAYS: usize = 5;
pub const GROUPS: usize = 5;
pub const CLASSES_PER_DAY: usize = 6;
pub const WEEKDAYS: &[&str] = &["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]; // Days when the classes can be added

// Generator information here
pub const GENETIC_POPULATION_SIZE: usize = 10;
pub const GENETIC_GENERATIONS: usize = 1000;
pub const GENETIC_THREAD_COUNT: usize = 4;

// This is more of a test/proof of concept
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

// Type aliases for ease of use
pub type DayTimetable = Vec<SubjectSpecs>;
pub type GroupTimetable = Vec<DayTimetable>;
pub type FullTimetable = Vec<GroupTimetable>;
pub type Matrix = Vec<Vec<i32>>;
