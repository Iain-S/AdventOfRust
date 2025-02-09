mod utils;

fn main() {
    let input = utils::problem_input("04");
    println!("Part 1: {}", part_one(&input));

    println!("Part 2: {}", part_two(&input));
}

fn part_one(s: &str) -> usize {
    let mut cw = s
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut xmases = 0;
    for _ in 0..4 {
        let ninety = utils::rotate_90(&cw);
        for row in &ninety {
            let s = row.iter().collect::<String>();
            xmases += s.matches("XMAS").count();
        }
        let fortyfive = utils::rotate_45(&cw);
        for row in &fortyfive {
            let s = row.iter().collect::<String>();
            xmases += s.matches("XMAS").count();
        }
        cw = ninety;
    }
    xmases
}

fn part_two(s: &str) -> u32 {
    let array_2d = utils::Array2D::new(s.lines().map(|l| l.chars().collect()).collect());
    let mut count = 0;
    for (x, y) in &array_2d {
        let mas = (&'M', &'A', &'S');
        let sam = (&'S', &'A', &'M');

        if (x == mas || x == sam) && (y == mas || y == sam) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("04"));
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("04"));
        assert_eq!(result, 9);
    }
}
