mod utils;

fn main() {
    let mut input = utils::problem_input("12");
    println!("Part 1: {}", part_one(&mut input));

    let input = utils::problem_input("12");
    println!("Part 2: {}", part_two(&input));
}

fn part_one(s: &mut String) -> u32 {
    let mut total = 0;
    for i in 0..s.lines().count() {
        for j in 0..s.lines().nth(i).unwrap().len() {
            let c = s.lines().nth(i).unwrap().chars().nth(j).unwrap();
            if c == c.to_uppercase().next().unwrap() {
                let (a, p) = area_perimeter(s, (i as isize, j as isize));
                total += a * p;
            }
        }
    }
    total
}

fn part_two(s: &String) -> u32 {
    2
}

fn area_perimeter(s: &mut String, start: (isize, isize)) -> (u32, u32) {
    let mut area = 1;
    let mut perimeter = 0;

    // Track progress by lowercasing the character.
    let c = s
        .lines()
        .nth(start.0 as usize)
        .unwrap()
        .chars()
        .nth(start.1 as usize)
        .unwrap();

    // Remember to account for the new line with +1.
    let range = (start.0 as usize * (s.lines().nth(0).unwrap().len() + 1)) + start.1 as usize;
    s.replace_range(
        range..range + 1,
        &c.to_lowercase().next().unwrap().to_string(),
    );
    for _move in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_pos = (start.0 + _move.0, start.1 + _move.1);
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= s.lines().count() as isize
            || new_pos.1 >= s.lines().nth(0).unwrap().len() as isize
        {
            perimeter += 1;
            continue;
        }
        let new_c = s
            .lines()
            .nth(new_pos.0 as usize)
            .unwrap()
            .chars()
            .nth(new_pos.1 as usize)
            .unwrap();

        if new_c == c.to_uppercase().next().unwrap() {
            // In this region.
            let (a, p) = area_perimeter(s, new_pos);
            area += a;
            perimeter += p;
        } else if new_c == c.to_lowercase().next().unwrap() {
            // Already visited.
            continue;
        } else {
            // Another region.
            perimeter += 1;
        }
    }
    (area, perimeter)
}

#[cfg(test)]
mod tests {

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() -> Result<(), String> {
        let actual = part_one(&mut example_input("12"));
        let expected = 1930;
        compare!(actual, expected)
    }

    #[test]
    fn test_area_perimeter() {
        // print the code point of each char in the string
        // for c in example_input("12").chars() {
        //     println!("{}", c as u32);
        // }
        let actual = area_perimeter(&mut example_input("12"), (0, 0));
        let expected = (12, 18);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_part_two() -> Result<(), String> {
        return Ok(());
        let result = part_two(&example_input("12"));
        compare!(result, 0)
    }
}
