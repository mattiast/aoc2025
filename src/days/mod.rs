use crate::Solution;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        2 => Some(Box::new(day02::Day02)),
        3 => Some(Box::new(day03::Day03)),
        _ => None,
    }
}
