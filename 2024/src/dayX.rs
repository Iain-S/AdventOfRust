mod utils;

fn main() {
    let input = utils::problem_input("00");
    println!("Part 1: {}", part_one(input));

    let input = utils::problem_input("00");
    println!("Part 2: {}", part_two(input));
}

fn part_one(s: &str) -> u32 {
    1
}

fn part_two(s: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("00"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(example_input("00"));
        assert_eq!(result, 0);
    }
}
