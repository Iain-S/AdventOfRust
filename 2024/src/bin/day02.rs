mod utils;

fn main() {
    let input = utils::problem_input("01");
    println!("Part 1: {}", part_one(input));

    let input = utils::problem_input("01");
    println!("Part 2: {}", part_two(input));
}

fn part_one(s: String) -> u32 {
    1
}

fn part_two(s: String) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("01"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(example_input("01"));
        assert_eq!(result, 0);
    }
}
