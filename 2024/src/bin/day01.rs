mod utils;

fn main() {
    let input = utils::problem_input("01");
    println!("Total distance: {}", part_one(input));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn split_line(s: String) -> (u32, u32) {
    let vector: Vec<&str> = s.split("   ").collect();
    if vector.len() != 2 {
        panic!("Expected two numbers, got {}", vector.len());
    }
    return (vector[0].parse::<u32>().unwrap(), vector[1].parse::<u32>().unwrap());
}

fn part_one(s: String) -> u32 {
    // Pair up the smallest number in the left list with the smallest number in the
    // right list, then the second-smallest left number with the second-smallest
    // right number, and so on.

    // Within each pair, figure out how far apart the two numbers are; you'll need
    // to add up all of those distances.
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Split the input into two vectors
    for line in s.split("\n") {
        if line == "" {
            continue;
        }
        let (l, r) = split_line(line.to_string());
        left.push(l);
        right.push(r);
    } 

    // Sort the vectors
    left.sort();
    right.sort();

    // Calculate the distance between each pair of numbers
    let result = left.iter().zip(right.iter()).map(|(l, r)| if r > l {r - l} else {l - r});

    // Sum the result
    return result.sum();
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn it_works() {
        let result = split_line(String::from_str("2   3").unwrap());
        assert_eq!(result, (2, 3));

        let result = split_line(String::from_str("200   30").unwrap());
        assert_eq!(result, (200, 30));
    }

    #[test]
    fn test_total_distance() {
        let result = total_distance(utils::example_input("01"));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("01"));
        assert_eq!(result, 11);
    }}
