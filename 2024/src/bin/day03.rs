mod utils;
use regex::Regex;

fn main() {
    let input = utils::problem_input("03");
    println!("Part 1: {}", part_one(input));

    let input = utils::problem_input("03");
    println!("Part 2: {}", part_two(input));
}

fn part_one(s: String) -> u32 {
    get_muls(s).iter().map(|(x, y)| x * y).sum()
}

fn get_muls(s: String) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut muls = vec![];
    for cap in re.captures_iter(&s) {
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();
        muls.push((x, y));
    }
    muls
}

fn part_two(s: String) -> u32 {
    // call our mutually recursive functions to get the string of "enabled" statements
    let enabled = get_to_dont(s);

    // re-use part 1 solution
    get_muls(enabled).iter().map(|(x, y)| x * y).sum()
}

// We're currently in "don't" mode so discard what we find.
fn get_to_do(s: String) -> String {
    let re = regex::Regex::new(r"do\(\)").unwrap();

    let mat = re.find(&s);

    if mat.is_none() {
        return "".to_string();
    }

    let substr = mat.unwrap();

    // ignore everything up to the end of the match
    let after_do = &s[substr.end()..];

    get_to_dont(after_do.to_string())
}

// We're currently in "do" mode so return what we find.
fn get_to_dont(s: String) -> String {
    let re = Regex::new(r"don't\(\)").unwrap();
    let mat = re.find(&s);

    // If we have no match, return the string as is
    if mat.is_none() {
        return s;
    }
    let substr = mat.unwrap();

    // get string up to but not including the match
    let before_dont = &s[..substr.start()];

    // get string after the match
    let after_dont = &s[substr.end()..];

    return before_dont.to_string() + &get_to_do(after_dont.to_string());
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(example_input("03"));
        assert_eq!(result, 161);
    }

    #[test]
    fn test_get_muls() {
        let result = get_muls(example_input("03"));
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(example_input("03b"));
        assert_eq!(result, 48);
    }

    #[test]
    fn test_get_do_dont() {
        let result = get_to_dont(example_input("03b"));
        assert_eq!(result, "xmul(2,4)&mul[3,7]!^?mul(8,5))\n");
    }
}
