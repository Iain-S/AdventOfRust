mod utils;
use core::panic;
use std::collections::{LinkedList, VecDeque};

use rand::seq::index;

#[derive(Debug, PartialEq, Clone)]
struct AFile {
    id: usize,
    length: u32,
    moved: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum FileType {
    File(AFile),
    Space(u32),
}

fn main() {
    let x = FileType::File(AFile {
        id: 0,
        length: 1,
        moved: false,
    });
    let input = utils::problem_input("09");
    println!("Part 1: {}", part_one(&input));

    let input = utils::problem_input("09");
    println!("Part 2: {}", part_two(&input));
}

fn part_one(s: &str) -> usize {
    let (files, mut spaces) = split(s);
    let mut file_deque = make_deque(&files);

    spaces.reverse();

    // The filesystem checksum.
    let mut checksum = 0;

    // Our current position.
    let mut i = 0;

    // The active back-most file.
    let mut back_id = 0;
    let mut back_blocks = 0;

    loop {
        if file_deque.is_empty() {
            break;
        }
        // The front-most file.
        let (id_num, blocks) = file_deque.pop_front().unwrap();
        for _ in 0..blocks {
            checksum += i * id_num;
            i += 1;
        }
        if spaces.is_empty() {
            continue;
        }
        // We've reversed spaces so can pop from the end.
        let space = spaces.pop().unwrap();
        if back_blocks == 0 {
            (back_id, back_blocks) = file_deque.pop_back().unwrap();
        }
        for _ in 0..space {
            checksum += i * back_id;
            i += 1;
            back_blocks = back_blocks - 1;
            if back_blocks == 0 {
                if file_deque.is_empty() {
                    break;
                }
                (back_id, back_blocks) = file_deque.pop_back().unwrap();
            }
        }
    }

    if back_blocks > 0 {
        for _ in 0..back_blocks {
            checksum += i * back_id;
            i += 1;
        }
    }
    checksum
}

fn part_deux(s: &str) -> usize {
    // The file lengths and empty spaces.
    let (files, mut spaces) = split(s);

    // So we can pop from the end to get the first space.
    spaces.reverse();

    // The file Ids and lengths.
    let mut file_deque = make_deque(&files);

    let mut total = 0;
    let mut i = 0;
    loop {
        if file_deque.is_empty() {
            break;
        }
        // The front-most file.
        let (id_num, blocks) = file_deque.pop_front().unwrap();
        for _ in 0..blocks {
            print!("{}", id_num);
            total += id_num * i;
            i += 1;
        }

        // Now that we're at a space, find the back-most file that will fit.
        let mut space = spaces.pop().unwrap();

        for j in 0..file_deque.len() {
            // Iterate from back to front.
            let index = file_deque.len() - 1 - j;
            let (id_num, blocks) = file_deque[index];

            if blocks <= space {
                // We can fit this file.
                // Remove it from the deque.
                file_deque.remove(index);

                // println!("Adding {} to space {}", id_num, space);

                // Add the file length to the total.
                for _ in 0..blocks {
                    print!("{}", id_num);
                    total += id_num * i;
                    i += 1;
                }
                space -= blocks;
                if space == 0 {
                    break;
                } else {
                    continue;
                }
            }
        }
        // Increment i as we move through the spaces.
        i += space as usize;
        for _ in 0..space {
            print!("{}", '.');
        }
    }
    total
}

fn get_index(s: &Vec<FileType>, id: usize) -> usize {
    for (i, file) in s.iter().enumerate() {
        match file {
            FileType::File(f) => {
                if f.id == id {
                    return i;
                }
            }
            _ => {}
        }
    }
    panic!("File not found");
}

fn compress(s: &mut Vec<FileType>) {
    let temp = (s.len() / 2) + 1;
    for i in 0..temp {
        let id = temp - i - 1;

        let index = get_index(s, id);

        // It will be an AFile and not a space.
        let afile_length = match &s[index] {
            FileType::File(file) => file.length,
            _ => panic!("Not a file"),
        };

        for j in 0..index {
            // If we find a space big enough, move the file.
            let space = match &s[j] {
                FileType::Space(space) => space.clone(),
                _ => continue,
            };
            if space >= afile_length {
                // Move the file.
                let file = s.remove(index);
                s.insert(j, file);
                s.insert(index + 1, FileType::Space(afile_length));

                // Since the file has moved to j, the space is now at j+1.
                s.remove(j + 1);
                s.insert(j + 1, FileType::Space(space - afile_length));
                break;
            }
        }
    }
}

fn part_two(s: &str) -> usize {
    let mut disk = file_vec(s);
    compress(&mut disk);
    // Debugging.
    // for file in disk.iter() {
    //     match file {
    //         FileType::File(f) => {
    //             println!("File {}: {}", f.id, f.length);
    //         }
    //         FileType::Space(s) => {
    //             println!("Space: {}", s);
    //         }
    //     }
    // }
    score(&disk)
}

// Even and odd numbered items from the input string.
fn split(s: &str) -> (Vec<u32>, Vec<u32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    let string = s.to_owned();
    let mut even = true;
    for char in string.chars() {
        let c: &mut Vec<u32>;
        if even {
            c = &mut a;
        } else {
            c = &mut b;
        }
        c.push(char.to_digit(10).unwrap());
        even = !even;
    }

    (a, b)
}

// The location of each file and its length.
fn make_deque(s: &Vec<u32>) -> VecDeque<(usize, u32)> {
    let mut vec = VecDeque::new();
    for (i, v) in s.iter().enumerate() {
        vec.push_back((i, *v));
    }
    vec
}

fn file_vec(s: &str) -> Vec<FileType> {
    let mut list = Vec::new();
    let mut i = 0;
    let mut even = true;
    for char in s.chars() {
        let size = char.to_digit(10).unwrap();
        if even {
            list.push(FileType::File(AFile {
                id: i,
                length: size,
                moved: false,
            }));
            i += 1;
        } else {
            list.push(FileType::Space(size));
        }
        even = !even;
    }
    list
}

fn score(s: &Vec<FileType>) -> usize {
    let mut total = 0;
    let mut i = 0;
    for file in s.iter() {
        match file {
            FileType::File(f) => {
                for _ in 0..f.length {
                    total += i * f.id;
                    i += 1;
                }
            }
            FileType::Space(s) => {
                for _ in 0..*s {
                    i += 1;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::utils::example_input;

    use super::*;

    #[test]
    fn test_split() {
        let input = "1234";
        let actual = split(&input);
        let expected = (vec![1, 3], vec![2, 4]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_make_vec() {
        let input = vec![2, 3];
        let actual = make_deque(&input);
        let expected = VecDeque::from(vec![(0, 2), (1, 3)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&example_input("09"));
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_file_vec() {
        let result = file_vec("1234");
        let expected = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::Space(2),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
            FileType::Space(4),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_index() {
        let input = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::Space(2),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
        ];
        let actual = get_index(&input, 1);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_compress() {
        let mut actual = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::Space(3),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
        ];
        let expected = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
            FileType::Space(0),
            FileType::Space(3),
        ];
        compress(&mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_compress_2() {
        let mut actual = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::Space(1),
            FileType::File(AFile {
                id: 2,
                length: 1,
                moved: false,
            }),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
        ];
        let expected = vec![
            FileType::File(AFile {
                id: 0,
                length: 1,
                moved: false,
            }),
            FileType::File(AFile {
                id: 2,
                length: 1,
                moved: false,
            }),
            FileType::Space(0),
            FileType::Space(1),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
        ];
        compress(&mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_score() {
        let input = vec![
            FileType::File(AFile {
                id: 0,
                length: 2,
                moved: false,
            }),
            FileType::Space(2),
            FileType::File(AFile {
                id: 1,
                length: 3,
                moved: false,
            }),
        ];
        assert_eq!(score(&input), 15);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&example_input("09"));
        assert_eq!(result, 2858);
    }
}
