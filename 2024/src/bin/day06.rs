mod utils;

struct Laboratory {
    rows: Vec<Vec<char>>,
    position: (usize, usize),
}

impl Laboratory {
    fn new(s: &str) -> Self {
        let mut starting_position = (usize::MAX, usize::MAX);
        let mut rows = Vec::new();
        for (i, line) in s.lines().enumerate() {
            if line.contains("^") {
                // set the starting position
                starting_position.0 = i;
                starting_position.1 = line.find("^").unwrap();
            }
            rows.push(line.chars().collect());
        }
        assert!(starting_position.0 != usize::MAX);

        Laboratory {
            rows,
            position: starting_position,
        }
    }

    fn move_up(&mut self) -> bool {
        // Move up.
        loop {
            if self.position.0 == 0 {
                self.rows[self.position.0][self.position.1] = 'X';
                return true;
            }
            let next_position = (self.position.0 - 1, self.position.1);
            if self.rows[next_position.0][next_position.1] == '#' {
                // Turn 90 degrees to the right.
                self.rows[self.position.0][self.position.1] = '>';
                return false;
            } else {
                // Mark the current position as visited.
                self.rows[self.position.0][self.position.1] = 'X';

                // Move to the next position.
                self.position = next_position;
                self.rows[self.position.0][self.position.1] = '^';
            }
        }
    }

    fn move_once(&mut self) -> bool {
        // Move until we encounter a '#'.
        let char = self.rows[self.position.0][self.position.1];
        match char {
            '^' => self.move_up(),
            // 'v' => self.move_down(),
            // '<' => self.move_left(),
            // '>' => self.move_right(),
            _ => panic!("Invalid character"),
        }
    }

    fn to_image(&self) -> String {
        // Combine vec of strings into a single string.
        let mut result = String::new();
        for row in &self.rows {
            result.push_str(&row.iter().collect::<String>());
            result.push('\n');
        }
        result.pop(); // Remove the last newline.
        result
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
    fn test_move_zero_times() {
        let lab = Laboratory::new(&example_input("06"));
        assert_eq!(lab.position, (6, 4));

        let expected = example_input("06");
        let actual = lab.to_image();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_move_once() {
        let expected = example_input("06_i");
        let mut lab = Laboratory::new(&example_input("06"));
        let fell_off_map = lab.move_once();
        assert_eq!(fell_off_map, false);
        let actual = lab.to_image();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("06"));
        assert_eq!(result, 0);
    }
}
