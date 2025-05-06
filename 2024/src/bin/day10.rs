use std::{collections::HashSet, hash::Hash};

mod utils;
use std::fmt;

fn main() {
    let input = utils::problem_input("10");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("10");
    println!("Part 2: {}", part_two(&input));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position<'a> {
    // Add fields as needed
    map: &'a String,
    row_col: (isize, isize),
}

impl<'a> fmt::Debug for Position<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print only the row and column.
        write!(f, "({}, {})", self.row_col.0, self.row_col.1)
    }
}

fn find_trailheads(input: &String) -> HashSet<Position> {
    let mut trailheads = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '0' {
                trailheads.insert(Position {
                    map: input,
                    row_col: (i as isize, j as isize),
                });
            }
        }
    }
    trailheads
}

fn in_bounds(pos: &Position) -> bool {
    pos.row_col.0 < pos.map.lines().count() as isize
        && pos.row_col.0 >= 0
        && pos.row_col.1 < pos.map.lines().next().unwrap().len() as isize
        && pos.row_col.1 >= 0
}

// Add all 9s that can be reached from pos to set.
fn score_from<'a>(pos: Position<'a>, set: &mut HashSet<Position<'a>>) {
    let (row, col) = pos.row_col;
    let c = pos
        .map
        .lines()
        .nth(row as usize)
        .unwrap()
        .chars()
        .nth(col as usize)
        .unwrap();

    if c == '9' {
        set.insert(pos);
        return;
    }

    // up, down, left and right
    for _move in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_pos: Position<'a> = Position {
            map: pos.map,
            row_col: (row + _move.0, col + _move.1),
        };
        if in_bounds(&new_pos) {
            let new_c = new_pos
                .map
                .lines()
                .nth(new_pos.row_col.0 as usize)
                .unwrap()
                .chars()
                .nth(new_pos.row_col.1 as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
            let old_c = c.to_digit(10).unwrap();
            if new_c == (old_c + 1) {
                score_from(new_pos, set);
            }
        }
    }
}

fn part_one(s: &String) -> u32 {
    find_trailheads(s)
        .iter()
        .map(|x| {
            let mut set = HashSet::new();
            score_from(x.clone(), &mut set);
            set
        })
        .map(|x| x.len() as u32)
        .sum()
}

fn part_two(s: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_score_from() {
        let input = example_input("10");
        let trailhead = Position {
            map: &input,
            row_col: (6, 6),
        };
        let mut result = HashSet::new();
        score_from(trailhead, &mut result);

        // Todo We expect the result to be a HashSet of positions
        assert_eq!(
            result,
            HashSet::from([
                Position {
                    map: &input,
                    row_col: (2, 5)
                },
                Position {
                    map: &input,
                    row_col: (3, 4)
                },
                Position {
                    map: &input,
                    row_col: (4, 5)
                },
            ])
        );
    }

    #[test]
    fn test_find_trailheads() {
        let input = example_input("10");
        let result = find_trailheads(&input);
        let expected = HashSet::from([
            Position {
                map: &input,
                row_col: (0, 2),
            },
            Position {
                map: &input,
                row_col: (0, 4),
            },
            Position {
                map: &input,
                row_col: (2, 4),
            },
            Position {
                map: &input,
                row_col: (4, 6),
            },
            Position {
                map: &input,
                row_col: (5, 2),
            },
            Position {
                map: &input,
                row_col: (5, 5),
            },
            Position {
                map: &input,
                row_col: (6, 0),
            },
            Position {
                map: &input,
                row_col: (6, 6),
            },
            Position {
                map: &input,
                row_col: (7, 1),
            },
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("10"));
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_two() {
        return;
        let result = part_two(&example_input("10"));
        assert_eq!(result, 0);
    }
}
