use std::collections::{HashMap, VecDeque};

mod utils;

fn main() {
    let input = utils::problem_input("07");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("07");
    println!("Part 2: {}", part_two(&input));
}

fn part_one(s: &str) -> u64 {
    // 4843335552916 too high
    // 4122618555677 too low

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

    let mut total = 0;
    for (test_val, nums) in mymap.iter() {
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
                can_form(*test_val, nums.clone());
                // panic!("Subtotal {} does not equal test value {}", subtotal, test_val);
                // let a = 1;
            }

            total += test_val;
            println!("{}: {}", test_val, canform.1);
        }
    }
    total
}

fn can_form(test_val: u64, mut nums: VecDeque<u64>) -> (bool, String) {
    let first = nums.pop_front().unwrap();
    // Our stopping condition.
    if nums.len() == 0 {
        if first == test_val {
            return (true, "".to_owned());
        } else {
            return (false, "".to_owned());
        }
    }

    if first > test_val {
        // Subtracting or dividing won't work.
        return (false, "".to_owned());
    }
    let add = can_form(test_val - first, nums.clone());
    if add.0 {
        return (true, "+".to_owned() + &add.1);
    } else {
        if test_val % first != 0 {
            return (false, "".to_owned());
        }
        let mul = can_form(test_val / first, nums.clone());
        if mul.0 {
            return (true, "*".to_owned() + &mul.1);
        } else {
            return (false, "".to_owned());
        }
    }
}

fn part_two(s: &str) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("07"));
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("00"));
        assert_eq!(result, 0);
    }
}
