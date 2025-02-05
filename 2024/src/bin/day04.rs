use std::vec;

mod utils;

fn main() {
    let input = utils::problem_input("04");
    println!("Part 1: {}", part_one(&input));

    println!("Part 2: {}", part_two(&input));
}

// Rotate the matrix 90 degrees clockwise, so that we can check the columns.
fn rotate_90(cw: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ccw = vec![vec![' '; cw.len()]; cw[0].len()];
    for i in 0..cw.len() {
        for j in 0..cw[0].len() {
            ccw[j][cw.len() - i - 1] = cw[i][j];
        }
    }
    ccw
}

// Rotate the matrix 45 degrees clockwise, so that we can check the diagonals.
fn rotate_45(cw: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ccw = Vec::new();

    // starting at the first col, iterate down the rows, going up and right at 45 degrees
    for i in 0..cw.len() {
        let mut x = i; // rows 0 to n
        let mut y = 0; // first column
        let mut diagonal = Vec::new();
        diagonal.push(cw[x][y]);
        while x > 0 && y < cw[0].len() - 1 {
            x -= 1;  // up
            y += 1;  // right
            diagonal.push(cw[x][y]);
        }
        ccw.push(diagonal);
    }

    // starting at the last row, iterate across the columns, from col 1 (not 0) going up and right at 45 degrees
    for j in 1..cw[0].len() {
        let mut x = cw.len() - 1;  // last row
        let mut y = j;  // columns 1 to n
        let mut diagonal = Vec::new();
        diagonal.push(cw[x][y]);
        while x > 0 && y < cw[0].len() - 1 {
            x -= 1;  // up
            y += 1;  // right
            diagonal.push(cw[x][y]);
        }
        ccw.push(diagonal);
    }

    ccw
}

fn part_one(s: &str) -> usize {
    let mut cw = s
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut xmases = 0;
    for _ in 0..4 {
        let ninety = rotate_90(&cw);
        for row in &ninety {
            let s = row.iter().collect::<String>();
            xmases += s.matches("XMAS").count();
        }
        let fortyfive = rotate_45(&cw);
        for row in &fortyfive {
            let s = row.iter().collect::<String>();
            xmases += s.matches("XMAS").count();
        }
        cw = ninety;
    }
    xmases
}

fn part_two(s: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_rotate_90() {
        let cw = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let ccw = vec![vec!['d', 'a'], vec!['e', 'b'], vec!['f', 'c']];
        assert_eq!(rotate_90(&cw), ccw);
    }

    #[test]
    fn test_rotate_45_one() {
        let orig = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];

        let rotated = vec![vec!['a'], vec!['d', 'b'], vec!['e', 'c'], vec!['f']];
        assert_eq!(rotate_45(&orig), rotated);
    }

    #[test]
    fn test_rotate_45_two() {
        let orig = vec![vec!['a'], vec!['d']];
        assert_eq!(rotate_45(&orig), orig);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("04"));
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("00"));
        assert_eq!(result, 0);
    }
}
