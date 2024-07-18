#[derive(Clone)]
pub struct GroupTimetable {
    group_name: String,
    timetable: Vec<i32>,
}

impl GroupTimetable {
    pub(crate) fn new(group_name: &str, timetable: Vec<i32>) -> Self {
        Self {
            group_name: group_name.to_string(),
            timetable,
        }
    }

    pub(crate) fn print_group_table(&self) {
        println!("Timetable for {}: {:?}", self.group_name, self.timetable);
    }
}