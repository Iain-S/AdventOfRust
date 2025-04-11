use std::collections::HashMap;
use std::ops::Index;
use std::ops::Range;

mod utils;

struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = utils::problem_input("08");
    println!("Part 1: {}", part_one(&input));

    // let input = utils::problem_input("00");
    // println!("Part 2: {}", part_two(&input));
}

fn make_map(s: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    for (i, line) in s.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                if map.get(&c).is_none() {
                    map.insert(c, vec![(i, j)]);
                } else {
                    let v = map.get_mut(&c).unwrap();
                    v.push((i, j));
                }
            }
        }
    }
    map
}

fn part_one(s: &str) -> u32 {
    1
}

fn part_two(s: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_make_map() {
        let input = example_input("08");
        let actual = make_map(&input);
        let mut expected = HashMap::new();
        expected.insert('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]);
        expected.insert('A', vec![(5, 6), (8, 8), (9, 9)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        return;
        let result = part_one(&example_input("00"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        return;
        let result = part_two(&example_input("00"));
        assert_eq!(result, 0);
    }
}
