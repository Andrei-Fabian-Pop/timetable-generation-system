use rand::prelude::*;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::generator::configuration::*;
use crate::generator::group_timetable::GroupTimetable;

const FREE_TIME: i32 = -1;

pub(crate) struct Generator {
    m_subject_indexes: Vec<usize>,
    m_time_tables: Vec<GroupTimetable>,
    m_offspring_mutex: Arc<Mutex<()>>,
    m_thread_count: usize,
}

impl Generator {
    pub(crate) fn new() -> Self {
        let mut m_subject_indexes = Vec::new();
        for (i, subject) in SUBJECTS.iter().enumerate() {
            for _ in 0..subject.classes_per_week {
                m_subject_indexes.push(i);
            }
        }

        let mut generator: Generator = Self {
            m_subject_indexes,
            m_time_tables: Vec::new(),
            m_offspring_mutex: Arc::new(Mutex::new(())),
            m_thread_count: GENETIC_THREAD_COUNT,
        };

        generator.genetic_algorithm(CLASSES_PER_DAY * DAYS, GROUPS, GENETIC_POPULATION_SIZE, GENETIC_GENERATIONS);
        generator
    }

    pub(crate) fn get_timetables(&self) -> Vec<GroupTimetable> {
        self.m_time_tables.clone()
    }

    pub(crate) fn print_timetables(&self) {
        for table in &self.m_time_tables {
            table.print_group_table();
            println!();
        }
    }

    fn genetic_algorithm(&mut self, total_periods: usize, student_groups: usize, population_size: usize, generations: usize) {
        let mut population: Vec<Matrix> = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            population.push(self.initialize_timetable(total_periods, student_groups));
        }

        for generation in 0..generations {
            population.sort_by_key(|a| self.fitness(a));

            if self.fitness(&population[0]) == 0 {
                println!("Solution found in generation {}", generation);
                for (i, group) in population[0].iter().enumerate() {
                    self.m_time_tables.push(GroupTimetable::new(&format!("Group {}", i), group.clone()));
                }
                return;
            }

            let parents: Vec<Matrix> = population.iter().take(population_size / 2).cloned().collect();
            let mut offspring: Vec<Matrix> = Vec::new();
            let offspring_count: usize = population_size - parents.len();
            let offspring_per_thread: usize = offspring_count / self.m_thread_count;
            let mut threads = Vec::new();

            for _ in 0..self.m_thread_count {
                let parents: Vec<Matrix> = parents.clone();
                let offspring_mutex: Arc<Mutex<()>> = Arc::clone(&self.m_offspring_mutex);
                threads.push(thread::spawn(move || {
                    let mut local_offspring = Vec::new();
                    while local_offspring.len() < offspring_per_thread {
                        let index1 = thread_rng().gen_range(0..parents.len());
                        let index2 = thread_rng().gen_range(0..parents.len());
                        let mut child = Generator::crossover(&parents[index1], &parents[index2]);

                        if thread_rng().gen_range(0..100) < 20 {
                            Generator::mutate(&mut child);
                        }

                        let _lock = offspring_mutex.lock().unwrap();
                        local_offspring.push(child);
                    }
                    local_offspring
                }));
            }

            for thread in threads {
                offspring.append(&mut thread.join().unwrap());
            }

            population = parents;
            population.extend(offspring);
        }

        println!("No solution found.");
    }

    fn get_random_item(vec: &mut Vec<usize>) -> i32 {
        if vec.is_empty() {
            return FREE_TIME;
        }

        let index: usize = thread_rng().gen_range(0..vec.len());
        vec.remove(index) as i32
    }

    fn initialize_timetable(&self, total_periods: usize, student_groups: usize) -> Matrix {
        let mut timetable: Matrix = Vec::new();

        for _ in 0..student_groups {
            let mut group: Vec<i32> = Vec::with_capacity(total_periods);
            let mut subject_list = self.m_subject_indexes.clone();
            let mut free_time = FREE_TIME;

            for _ in 0..total_periods {
                let item: i32 = Self::get_random_item(&mut subject_list);

                if item < 0 {
                    group.push(free_time);
                    free_time -= 1;
                } else {
                    group.push(item);
                }
            }

            group.shuffle(&mut thread_rng());
            timetable.push(group);
        }
        timetable
    }

    fn fitness(&self, timetable: &Matrix) -> usize {
        let height: usize = timetable.len();
        let width: usize = timetable[0].len();
        let mut clashes: usize = 0;

        for i in 0..width {
            for j in 0..height {
                for k in 0..height {
                    if k != j && timetable[j][i] >= 0 && timetable[k][i] >= 0 && timetable[j][i] == timetable[k][i] {
                        clashes += 1;
                    }
                }
            }
        }
        clashes
    }

    fn crossover(parent1: &Matrix, parent2: &Matrix) -> Matrix {
        let size: usize = parent1.len();
        let crossover_point: usize = thread_rng().gen_range(1..size);
        let mut result: Matrix = Vec::new();

        for i in 0..parent1.len() {
            let mut child: Vec<i32> = parent1[i][..crossover_point].to_vec();

            for &parent2_child in &parent2[i] {
                if !child.contains(&parent2_child) {
                    child.push(parent2_child);
                }
            }

            result.push(child);
        }
        result
    }

    fn mutate(board: &mut Matrix) {
        let mutation_point_x: usize = thread_rng().gen_range(0..board.len());
        let mutation_point_y1: usize = thread_rng().gen_range(0..board[0].len());
        let mutation_point_y2: usize = thread_rng().gen_range(0..board[0].len());

        board[mutation_point_x].swap(mutation_point_y1, mutation_point_y2);
    }
}
