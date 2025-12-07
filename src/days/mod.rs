use crate::Solution;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn get_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        2 => Some(Box::new(day02::Day02)),
        3 => Some(Box::new(day03::Day03)),
        4 => Some(Box::new(day04::Day04)),
        5 => Some(Box::new(day05::Day05)),
        6 => Some(Box::new(day06::Day06)),
        7 => Some(Box::new(day07::Day07)),
        _ => None,
    }
}
