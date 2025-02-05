use std::fs::read_to_string;

#[allow(dead_code)]
pub(crate) fn example_input(day: &str) -> String {
    let result = read_to_string("example/".to_string() + day + ".txt").unwrap();
    if result.len() == 0 {
        panic!("No example found for day {}", day);
    }
    result
}

#[allow(dead_code)]
pub(crate) fn problem_input(day: &str) -> String {
    let result = read_to_string("input/".to_string() + day + ".txt").unwrap();
    if result.len() == 0 {
        panic!("No input found for day {}", day);
    }
    result
}

#[allow(dead_code)]
fn main() {}
