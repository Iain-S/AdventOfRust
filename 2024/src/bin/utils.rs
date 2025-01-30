use std::fs::read_to_string;

pub(crate) fn example_input(day: &str) -> String {
    read_to_string("example/".to_string() + day + ".txt").unwrap()
}

pub(crate) fn problem_input(day: &str) -> String {
    read_to_string("input/".to_string() + day + ".txt").unwrap()
}

fn main() {
    println!("Utils!");
}
