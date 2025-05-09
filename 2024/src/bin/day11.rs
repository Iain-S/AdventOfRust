use core::ops::Mul;
use core::str::FromStr;

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

fn blink<T>(stones: &mut Vec<T>)
where
    T: From<u16>
        + ToString
        + PartialEq
        + FromStr<Err = std::num::ParseIntError>
        + Copy
        + Mul<Output = T>
        + Into<u64>,
{
    let mut i = 0;
    while i < stones.len() {
        if stones[i] == 0.into() {
            stones[i] = 1.into();
        } else if even_digits(stones[i]) {
            let digits = stones[i].to_string();
            stones[i] = digits[..digits.len() / 2].parse().unwrap();
            stones.insert(i + 1, digits[digits.len() / 2..].parse().unwrap());
            i += 1;
        } else {
            stones[i] = stones[i] * 2024.into();
        }
        i += 1;
    }
}

fn part_one(s: &String) -> usize {
    let mut stones: Vec<u64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for _ in 0..25 {
        blink::<u64>(&mut stones);
    }
    return stones.len();
}

fn part_two(s: &String) -> usize {
    let mut stones: Vec<u16> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for _ in 0..25 {
        blink::<u16>(&mut stones);
    }
    return stones.len();
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_blink() {
        let mut stones: Vec<u32> = vec![0, 1, 10, 99, 999];
        blink(&mut stones);
        assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
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
