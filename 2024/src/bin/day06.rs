mod utils;

struct Laboratory {}

impl Laboratory {
    fn new(s: &str) -> Self {
        let mut lab = Laboratory {};
        lab.parse_input(s);
        lab
    }

    fn parse_input(&mut self, s: &str) {
        // Parse the input string and initialize the laboratory state.
    }

    fn move_once(&mut self) {
        // Move the particles once.
    }

    fn to_image(&self) -> String {
        // Convert the laboratory state to an image representation.
        String::new()
    }
}

fn main() {
    let input = utils::problem_input("06");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("06");
    println!("Part 2: {}", part_two(&input));
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
        let result = part_one(&example_input("06"));
        assert_eq!(result, 41);
    }

    #[test]
    fn test_move_once() {
        let expected = example_input("06_i");
        let mut lab = Laboratory::new(&example_input("06"));
        lab.move_once();
        let actual = lab.to_image();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("06"));
        assert_eq!(result, 0);
    }
}
