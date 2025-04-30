use std::collections::HashMap;
use std::collections::HashSet;

mod utils;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    y: i16,
    x: i16,
}

fn main() {
    let input = utils::problem_input("08");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("08");
    println!("Part 2: {}", part_two(&input));
}

fn make_map(s: &str) -> HashMap<char, HashSet<Point>> {
    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    for (i, line) in s.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                let point = Point {
                    x: j.try_into().unwrap(),
                    y: i.try_into().unwrap(),
                };
                if map.contains_key(&c) {
                    // Todo: why doesn't h need to be mutable?
                    let h = map.get_mut(&c).unwrap();
                    h.insert(point);
                } else {
                    let hashset = [point].into();
                    map.insert(c, hashset);
                }
            }
        }
    }
    map
}

fn get_antinode(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p2.x + (p2.x - p1.x),
        y: p2.y + (p2.y - p1.y),
    }
}

fn get_antinodes(map: &HashMap<char, HashSet<Point>>) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for (c1, points1) in map.iter() {
        for p1 in points1.iter() {
            for p2 in points1.iter() {
                if p1 != p2 {
                    antinodes.insert(get_antinode(p1, p2));
                    antinodes.insert(get_antinode(p2, p1));
                }
            }
        }
    }
    antinodes
}

fn part_one(s: &str) -> u32 {
    let height = s.lines().count();
    let width = s.lines().next().unwrap().len();

    let map = make_map(s);
    let antinodes = get_antinodes(&map);
    let mut count = 0;
    for antinode in antinodes.iter() {
        if antinode.x >= 0
            && antinode.x < width.try_into().unwrap()
            && antinode.y >= 0
            && antinode.y < height.try_into().unwrap()
        {
            count += 1;
        }
    }
    count
}

fn gcd(a: i16, b: i16) -> i16 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_harmonics(p1: &Point, p2: &Point, ylen: usize, xlen: usize) -> HashSet<Point> {
    let xdiff = p2.x - p1.x;
    let ydiff = p2.y - p1.y;
    let greatest_common_divisor = gcd(xdiff.abs(), ydiff.abs());
    let xmin = xdiff / greatest_common_divisor;
    let ymin = ydiff / greatest_common_divisor;

    let mut harmonics = HashSet::new();
    let mut point = Point { x: p1.x, y: p1.y };
    loop {
        point = Point {
            x: point.x + xmin,
            y: point.y + ymin,
        };
        if point.x < 0
            || point.x >= xlen.try_into().unwrap()
            || point.y < 0
            || point.y >= ylen.try_into().unwrap()
        {
            return harmonics;
        } else {
            harmonics.insert(point);
        }
    }
}

fn part_two(s: &str) -> usize {
    let height = s.lines().count();
    let width = s.lines().next().unwrap().len();

    let map = make_map(s);
    let mut harmonics = HashSet::new();
    for (_, points) in map.iter() {
        for p1 in points.iter() {
            for p2 in points.iter() {
                if p1 != p2 {
                    harmonics.extend(get_harmonics(p1, p2, height, width));
                }
            }
        }
    }
    harmonics.len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_make_map() {
        let input = example_input("08");
        let actual = make_map(&input);
        let mut expected = HashMap::new();
        let hashset1: HashSet<Point> = [
            Point { y: 1, x: 8 },
            Point { y: 2, x: 5 },
            Point { y: 3, x: 7 },
            Point { y: 4, x: 4 },
        ]
        .into();
        let hashset2: HashSet<Point> = [
            Point { y: 5, x: 6 },
            Point { y: 8, x: 8 },
            Point { y: 9, x: 9 },
        ]
        .into();
        expected.insert('0', hashset1);
        expected.insert('A', hashset2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_antinode() {
        let p1 = Point { x: 4, y: 3 };
        let p2 = Point { x: 5, y: 5 };

        let actual = get_antinode(&p1, &p2);
        let expected = Point { y: 7, x: 6 };
        assert_eq!(expected, actual);

        let actual = get_antinode(&p2, &p1);
        let expected = Point { y: 1, x: 3 };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("08"));
        assert_eq!(result, 14);
    }

    #[test]
    fn test_get_harmonics() {
        // A . .
        // . # .
        // . . A
        // Expect 3 harmonics (the middle plus the two antenna points)
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 2, y: 2 };

        let actual = get_harmonics(&p1, &p2, 3, 3);
        let mut expected = HashSet::<Point>::new();
        expected.insert(Point { y: 1, x: 1 });
        expected.insert(Point { y: 2, x: 2 });
        assert_eq!(actual, expected);

        let actual = get_harmonics(&p2, &p1, 3, 3);
        let mut expected = HashSet::<Point>::new();
        expected.insert(Point { y: 1, x: 1 });
        expected.insert(Point { y: 0, x: 0 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("08"));
        assert_eq!(result, 34);
    }

    #[test]
    fn test_mutable_ref() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        let m = &mut map;
        m.insert(1, 2);
    }
}
