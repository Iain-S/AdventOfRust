use std::fs::read_to_string;

#[allow(dead_code)]
pub(crate) fn example_input(day: &str) -> String {
    read_to_string("example/".to_string() + day + ".txt").unwrap()
}

#[allow(dead_code)]
pub(crate) fn problem_input(day: &str) -> String {
    read_to_string("input/".to_string() + day + ".txt").unwrap()
}

#[allow(dead_code)]
fn main() {}
