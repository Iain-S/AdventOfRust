use std::vec;

mod utils;

fn main() {
    let input = utils::problem_input("01");
    println!("Part 1: {}", part_one(input));

    let input = utils::problem_input("01");
    println!("Part 2: {}", part_two(input));
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = vec![];
    let mut x = levels.iter();
    x.next();
    for y in levels.iter() {
        let x_next = x.next();
        if x_next.is_none() {
            break;
        }
        let diff: i32 = y - x_next.unwrap();
        diffs.push(diff);
    }
    for diff in diffs.iter() {
        println!("{}", diff);
    }
    diffs.iter().map(|&x| x.abs()).all(|x| x > 0 && x < 3)
}

/// Rewrite is_safe to use more idiomatic Rust.
fn is_safe_attempt_deux(levels: Vec<i32>) -> bool {
    fn pos(x: &[i32]) -> bool {
        x[1] - x[0] > 0 && x[1] - x[0] < 3
    }
    fn neg(x: &[i32]) -> bool {
        x[1] - x[0] < 0 && x[1] - x[0] > 3
    }
    levels.windows(2).all(pos) || levels.windows(2).all(neg)
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
    fn test_is_() {
        println!("{}", vec![1, 2].iter().all(|x| *x < 1));
    }

    #[test]
    fn test_is_safe() {
        assert_eq!(true, is_safe(vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe(vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_is_safe_deux() {
        assert_eq!(true, is_safe(vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe(vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("01"));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(example_input("01"));
        assert_eq!(result, 0);
    }
}
