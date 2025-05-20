use core::ops::Mul;
use core::str::FromStr;
use std::collections::HashMap;

mod utils;

fn main() {
    let input = utils::problem_input("11");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("11");
    println!("Part 2: {}", part_two(&input));
}

fn even_digits<T>(stone: T) -> bool
where
    T: From<u16> + ToString + PartialEq + FromStr<Err = std::num::ParseIntError> + Copy + Into<u64>,
{
    let mut digits = 1;
    let ten: u64 = 10;
    while ten.pow(digits) <= stone.into() {
        digits += 1;
    }
    // An alternative, which may be slightly slower.
    // (digits % 2 == 0)
    !(digits & 1 == 1)
}

fn memoized_blink<T>(stone: T, n: usize, memo: &mut HashMap<(T, usize), usize>) -> usize
where
    T: From<u16>
        + ToString
        + PartialEq
        + FromStr<Err = std::num::ParseIntError>
        + Copy
        + Mul<Output = T>
        + Into<u64>
        + std::hash::Hash
        + std::cmp::Eq,
{
    if memo.contains_key(&(stone, n)) {
        return *memo.get(&(stone, n)).unwrap();
    }

    if n == 0 {
        memo.insert((stone, n), 1);
        return 1;
    }
    if stone == 0.into() {
        let result = memoized_blink(1.into(), n - 1, memo);
        memo.insert((stone, n), result);
        return result;
    } else if even_digits(stone) {
        let digits = stone.to_string();
        let a: T = digits[..digits.len() / 2].parse().unwrap();
        let b: T = digits[digits.len() / 2..].parse().unwrap();
        let result = memoized_blink(a, n - 1, memo) + memoized_blink(b, n - 1, memo);
        memo.insert((stone, n), result);
        return result;
    } else {
        let result = memoized_blink(stone * 2024.into(), n - 1, memo);
        memo.insert((stone, n), result);
        return result;
    }
}

fn blink<T>(stone: T, n: usize) -> usize
where
    T: From<u16>
        + ToString
        + PartialEq
        + FromStr<Err = std::num::ParseIntError>
        + Copy
        + Mul<Output = T>
        + Into<u64>,
{
    if n == 0 {
        return 1;
    }
    if stone == 0.into() {
        return blink(1 as u64, n - 1);
    } else if even_digits(stone) {
        let digits = stone.to_string();
        let a: T = digits[..digits.len() / 2].parse().unwrap();
        let b: T = digits[digits.len() / 2..].parse().unwrap();
        return blink(a, n - 1) + blink(b, n - 1);
    } else {
        return blink(stone * 2024.into(), n - 1);
    }
}

fn part_one(s: &String) -> usize {
    let stones: Vec<u64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut total = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        total += memoized_blink(stone, 25, &mut cache);
    }
    total
}

fn part_two(s: &String) -> usize {
    let stones: Vec<u64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut total = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        total += memoized_blink(stone, 75, &mut cache);
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_blink() {
        let stones: Vec<u32> = vec![0, 1, 10, 99, 999];
        let result: Vec<usize> = stones.iter().map(|x| blink(*x, 1)).collect();
        assert_eq!(result, vec![1, 1, 2, 2, 1]);
    }

    #[test]
    fn test_evn() {
        let stone: u32 = 100;
        assert_eq!(false, even_digits(stone));

        let stone: u32 = 1;
        assert_eq!(false, even_digits(stone));

        let stone: u32 = 99;
        assert_eq!(true, even_digits(stone));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("11"));
        assert_eq!(result, 55_312);
    }

    #[test]
    fn test_part_two() {
        return;
        let result = part_two(&example_input("11"));
        assert_eq!(result, 0);
    }
}
