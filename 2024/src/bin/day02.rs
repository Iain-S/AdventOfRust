use std::vec;

mod utils;

fn main() {
    let input = utils::problem_input("02");
    println!("Part 1: {}", part_one(input));

    let input = utils::problem_input("02");
    println!("Part 2: {}", part_two(input));
}

fn is_safe(levels: Vec<i32>) -> bool {
    fn pos(x: &[i32]) -> bool {
        let result = x[1] - x[0] > 0 && x[1] - x[0] <= 3;
        result
    }
    fn neg(x: &[i32]) -> bool {
        let result = x[1] - x[0] < 0 && x[1] - x[0] >= -3;
        result
    }
    let all_pos = levels.windows(2).all(pos);
    let all_neg = levels.windows(2).all(neg);
    all_neg || all_pos
}

/// Rewrite is_safe to use more idiomatic Rust.
fn is_safe_deux(levels: Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut myvec = levels[..i].to_vec();
        myvec.extend_from_slice(&levels[i + 1..]);
        if is_safe(myvec) {
            return true;
        }
    }
    false
}

fn part_one(s: String) -> u32 {
    // for each line in the input
    // split the line into a vector of integers
    fn inner(ss: &str) -> u32 {
        let levels = ss.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let safe = is_safe(levels);
        if safe {
            return 1;
        }
        0
    }
    let temp: Vec<u32> = s.lines().map(inner).collect();
    temp.iter().sum()
}

fn part_two(s: String) -> u32 {
    fn inner(ss: &str) -> u32 {
        let levels = ss.split_whitespace().map(|x| x.parse().unwrap()).collect();
        match is_safe_deux(levels) {
            true => 1,
            false => 0,
        }
    }
    let temp: Vec<u32> = s.lines().map(inner).collect();
    temp.iter().sum()
}

#[cfg(test)]
mod tests {
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
        assert_eq!(false, is_safe(vec![9, 7, 6, 2, 1]));
        assert_eq!(false, is_safe(vec![1, 3, 2, 4, 5]));
        assert_eq!(false, is_safe(vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe(vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_is_safe_deux() {
        assert_eq!(true, is_safe_deux(vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe_deux(vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_safe_deux(vec![9, 7, 6, 2, 1]));
        assert_eq!(true, is_safe_deux(vec![1, 3, 2, 4, 5]));
        assert_eq!(true, is_safe_deux(vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe_deux(vec![1, 3, 6, 7, 9]));

        //
        assert_eq!(true, is_safe_deux(vec![1, 3, 6, 7, 6]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("02"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(example_input("02"));
        assert_eq!(4, result);
    }
}
