use crate::Solution;

pub mod day01;

pub fn get_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        _ => None,
    }
}
