use rand;
use std::collections::{HashMap, HashSet, VecDeque};

mod utils;

fn main() {
    let input = utils::problem_input("07");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("07");
    println!("Part 2: {}", part_two(&input));
}

fn find_duplicate_key(myvec: &Vec<(u64, VecDeque<u64>)>) -> Option<u64> {
    let mut seen = HashSet::new();
    for (key, _) in myvec {
        if !seen.insert(key) {
            return Some(*key); // Duplicate key found
        }
    }
    None // No duplicates found
}

fn part_one(s: &str) -> u64 {
    let mymap = s
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let test_val: u64 = parts.next().unwrap().parse().unwrap();
            let nums: VecDeque<u64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (test_val, nums)
        })
        .collect::<HashMap<_, _>>();

    let myvec = s
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let test_val: u64 = parts.next().unwrap().parse().unwrap();
            let nums: VecDeque<u64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (test_val, nums)
        })
        .collect::<Vec<_>>();

    // Beware using HashMaps if there might be duplicate entries.
    if let Some(duplicate_key) = find_duplicate_key(&myvec) {
        println!("Duplicate key found: {}", duplicate_key);
    }

    let mut total = 0;
    for (test_val, nums) in myvec.iter() {
        let test_val_val = *test_val;
        let debug_nums = nums.clone();
        let canform = can_form(*test_val, nums.clone());
        if canform.0 {
            let mut clone = nums.clone();
            let mut subtotal = clone.pop_front().unwrap();
            for c in canform.1.chars() {
                if c == '+' {
                    subtotal = subtotal + clone.pop_front().unwrap();
                } else if c == '*' {
                    subtotal = subtotal * clone.pop_front().unwrap();
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
            if subtotal != *test_val {
                // can_form(*test_val, nums.clone());
                panic!(
                    "Subtotal {} does not equal test value {}",
                    subtotal, test_val
                );
                // let a = 1;
            }

            total += test_val;
            // println!("{}: {}", test_val, canform.1);
        }
    }
    total
}

fn can_form(test_val: u64, mut nums: VecDeque<u64>) -> (bool, String) {
    let first = nums.pop_back().unwrap();
    // Our stopping condition.
    if nums.len() == 0 {
        if first == test_val {
            return (true, "".to_owned());
        } else {
            return (false, "".to_owned());
        }
    }

    if test_val >= first {
        let add = can_form(test_val - first, nums.clone());
        if add.0 {
            return (true, add.1 + "+");
        }
    }

    if test_val % first == 0 {
        let mul = can_form(test_val / first, nums.clone());
        if mul.0 {
            return (true, mul.1 + "*");
        }
    }

    return (false, "".to_owned());
}

fn part_two(s: &str) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("07"));
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_can_form() {
        let result = can_form(11, vec![2, 5, 1].into());
        assert_eq!(result.0, true);
        assert_eq!(result.1, "*+");
        let nums: VecDeque<u64> = vec![1, 2, 3, 4, 5].into();
        let result = can_form(120, nums.clone());
        assert_eq!(result.0, true);
        assert_eq!(result.1, "++**");

        let mut clone = nums.clone();
        let mut subtotal = clone.pop_front().unwrap();
        for c in result.1.chars() {
            if c == '+' {
                subtotal = subtotal + clone.pop_front().unwrap();
            } else if c == '*' {
                subtotal = subtotal * clone.pop_front().unwrap();
            } else {
                panic!("Unexpected character: {}", c);
            }
        }
        assert_eq!(subtotal, 120);

        let result = can_form(10, vec![2, 3].into());
        assert_eq!(result.0, false);
        assert_eq!(result.1, "");
    }

    #[test]
    fn dither() {
        return;
        // Test with random strings.
        let mut i = 0;
        for _ in 0..100 {
            // create a random string of + and * of length 1 to 10
            let len = (rand::random::<u8>() % 5) + 1;
            let mut s = String::new();
            for _ in 0..len {
                let c = if rand::random::<bool>() { '+' } else { '*' };
                s.push(c);
            }

            // a vector of random numbers of length len
            let mut nums = VecDeque::new();
            for _ in 0..len + 1 {
                let n = (rand::random::<u64>() % 999) + 1;
                nums.push_back(n);
            }
            let num_copy = nums.clone();

            // work out the total
            let mut total = nums.pop_front().unwrap();
            for c in s.chars() {
                if c == '+' {
                    total += nums.pop_front().unwrap();
                } else if c == '*' {
                    total *= nums.pop_front().unwrap();
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }

            let can = can_form(total, num_copy.clone());
            if !can.0 {
                panic!(
                    "Can form returned false for {} and {}",
                    total,
                    num_copy
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            i += 1;
            if i % 10 == 0 {
                println!("{} iterations", i);
            }
        }
    }

    #[test]
    fn print_u64_max() {
        let max = u64::MAX;
        println!("u64 max: {}", max);

        // what is u64max log base 999?
        let log = (max as f64).log(999.0);
        println!("u64 max log base 999: {}", log);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("00"));
        assert_eq!(result, 0);
    }
}
