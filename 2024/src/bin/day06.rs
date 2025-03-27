mod utils;
use core::panic;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    FellOffMap,
    HitWall,
    Looped,
}

struct Laboratory {
    rows: Vec<Vec<char>>,
    position: (usize, usize),
    obstructions: HashSet<(usize, usize)>,
    fork: bool,
    moves: u32,
    print: bool,
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
            obstructions: HashSet::new(),
            fork: true,
            moves: 0,
            print: false,
        }
    }

    fn copy(&self) -> Self {
        // make a deep copy of the laboratory
        let mut rows = Vec::new();
        for row in &self.rows {
            rows.push(row.clone());
        }
        Laboratory {
            rows: rows,
            position: self.position,
            obstructions: HashSet::new(),
            fork: false,
            moves: 0,
            print: false,
        }
    }

    fn move_up(&mut self) -> Reason {
        loop {
            if self.print {
                print!("{}", self.to_image());
            }
            if self.fork {
                println!("Move {:?}", self.moves);
                self.moves += 1;
            }
            if self.position.0 == 0 {
                return Reason::FellOffMap;
            }
            let next_position = (self.position.0 - 1, self.position.1);
            if self.rows[next_position.0][next_position.1] == '#' {
                // Turn 90 degrees to the right.
                self.rows[self.position.0][self.position.1] = '>';
                return Reason::HitWall;
            }

            if self.fork && self.rows[next_position.0][next_position.1] == '.' {
                // Look to the right.
                let mut fork = self.copy();
                fork.rows[next_position.0][next_position.1] = '#';
                fork.rows[self.position.0][self.position.1] = '>';
                if fork.run() == Reason::Looped {
                    self.obstructions.insert(next_position);
                }
            }
            if next_position.0 > 0 {
                // Check if the next position is a wall.
                if self.rows[next_position.0][next_position.1] == '>'
                    && self.rows[next_position.0 - 1][next_position.1] == '#'
                {
                    if self.fork {
                        panic!("We are in the fork");
                    }
                    return Reason::Looped;
                }
            }

            // Move to the next position.
            self.position = next_position;
            self.rows[self.position.0][self.position.1] = '^';
        }
    }

    fn move_right(&mut self) -> Reason {
        loop {
            if self.print {
                print!("{}", self.to_image());
            }
            if self.fork {
                println!("Move {:?}", self.moves);
                self.moves += 1;
            }
            if self.position.1 == self.rows[0].len() - 1 {
                return Reason::FellOffMap;
            }
            let next_position = (self.position.0, self.position.1 + 1);
            if self.rows[next_position.0][next_position.1] == '#' {
                // Turn 90 degrees to the right.
                self.rows[self.position.0][self.position.1] = 'v';
                return Reason::HitWall;
            }

            if self.fork && self.rows[next_position.0][next_position.1] == '.' {
                // Look down.
                let mut fork = self.copy();
                if self.moves == 87 {
                    fork.print = true;
                }
                fork.rows[next_position.0][next_position.1] = '#';
                fork.rows[self.position.0][self.position.1] = 'v';
                if fork.run() == Reason::Looped {
                    self.obstructions.insert(next_position);
                }
            }
            if next_position.1 + 1 < self.rows[0].len() {
                // Check if the next position is a wall.
                if self.rows[next_position.0][next_position.1] == 'v'
                    && self.rows[next_position.0][next_position.1 + 1] == '#'
                {
                    if self.fork {
                        panic!("We are in the fork");
                    }
                    return Reason::Looped;
                }
            }

            // Move to the next position.
            self.position = next_position;
            self.rows[self.position.0][self.position.1] = '>';
        }
    }

    fn move_left(&mut self) -> Reason {
        loop {
            if self.print {
                print!("{}", self.to_image());
            }
            if self.fork {
                println!("Move {:?}", self.moves);
                self.moves += 1;
            }
            if self.position.1 == 0 {
                return Reason::FellOffMap;
            }
            let next_position = (self.position.0, self.position.1 - 1);
            if self.rows[next_position.0][next_position.1] == '#' {
                // Turn 90 degrees to the right.
                self.rows[self.position.0][self.position.1] = '^';
                return Reason::HitWall;
            }

            if self.fork && self.rows[next_position.0][next_position.1] == '.' {
                // Look up.
                let mut fork = self.copy();
                fork.rows[next_position.0][next_position.1] = '#';
                fork.rows[self.position.0][self.position.1] = '^';
                if fork.run() == Reason::Looped {
                    self.obstructions.insert(next_position);
                }
            }
            if next_position.1 > 0 {
                if self.rows[next_position.0][next_position.1] == '^'
                    && self.rows[next_position.0][next_position.1 - 1] == '#'
                {
                    if self.fork {
                        panic!("We are in the fork");
                    }
                    return Reason::Looped;
                }
            }

            // Move to the next position.
            self.position = next_position;
            self.rows[self.position.0][self.position.1] = '<';
        }
    }

    fn move_down(&mut self) -> Reason {
        loop {
            if self.print {
                print!("{}", self.to_image());
            }
            if self.fork {
                println!("Move {:?}", self.moves);
                self.moves += 1;
            }
            if self.position.0 == self.rows.len() - 1 {
                return Reason::FellOffMap;
            }
            let next_position = (self.position.0 + 1, self.position.1);
            if self.rows[next_position.0][next_position.1] == '#' {
                // Turn 90 degrees to the right.
                self.rows[self.position.0][self.position.1] = '<';
                return Reason::HitWall;
            }

            if self.fork && self.rows[next_position.0][next_position.1] == '.' {
                // Look left.
                let mut fork = self.copy();
                fork.rows[next_position.0][next_position.1] = '#';
                fork.rows[self.position.0][self.position.1] = '<';
                if fork.run() == Reason::Looped {
                    self.obstructions.insert(next_position);
                }
            }
            if next_position.0 + 1 < self.rows.len() {
                if self.rows[next_position.0][next_position.1] == '<'
                    && self.rows[next_position.0 + 1][next_position.1] == '#'
                {
                    if self.fork {
                        panic!("We are in the fork");
                    }
                    return Reason::Looped;
                }
            }

            // Move to the next position.
            self.position = next_position;
            self.rows[self.position.0][self.position.1] = 'v';
        }
    }

    fn move_once(&mut self) -> Reason {
        // Move until we encounter a '#'.
        let char = self.rows[self.position.0][self.position.1];
        match char {
            '^' => self.move_up(),
            'v' => self.move_down(),
            '<' => self.move_left(),
            '>' => self.move_right(),
            _ => panic!("Invalid character"),
        }
    }

    fn visited(&self) -> u32 {
        // Count the number of visited positions.
        let mut count = 0;
        for row in &self.rows {
            for c in row {
                // if my tuple contains c
                if ['^', '>', '<', 'v'].contains(&c) {
                    count += 1;
                }
            }
        }
        count
    }

    fn run(&mut self) -> Reason {
        loop {
            match self.move_once() {
                Reason::FellOffMap => return Reason::FellOffMap,
                Reason::HitWall => continue,
                Reason::Looped => return Reason::Looped,
            }
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
    let mut lab = Laboratory::new(s);
    lab.fork = false;
    lab.run();
    lab.visited()
}

fn part_two(s: &str) -> u32 {
    let mut lab = Laboratory::new(s);
    lab.run();
    lab.obstructions.len() as u32
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
        let mut lab = Laboratory::new(&example_input("06"));

        let expected = example_input("06_i");
        assert_eq!(Reason::HitWall, lab.move_once());
        assert_eq!(lab.visited(), 6);

        let actual = lab.to_image();
        assert_eq!(expected, actual);

        let expected = example_input("06_ii");
        assert_eq!(Reason::HitWall, lab.move_once());

        let actual = lab.to_image();
        assert_eq!(expected, actual);

        let expected = example_input("06_iii");
        assert_eq!(Reason::HitWall, lab.move_once());

        let actual = lab.to_image();
        assert_eq!(expected, actual);

        let expected = example_input("06_iv");
        assert_eq!(Reason::HitWall, lab.move_once());

        let actual = lab.to_image();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("06"));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_tutu() {
        let result = part_two(&example_input("06_v"));
        assert_eq!(result, 1);
    }
}
