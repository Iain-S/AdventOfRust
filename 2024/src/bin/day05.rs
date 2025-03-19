mod utils;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

use pathfinding::prelude::topological_sort;
use std::cmp::Ordering;
use utils::problem_input;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Pair<T> {
    a: T,
    b: T,
}

fn parse_input(s: &str) -> (HashSet<Pair<u32>>, HashSet<Vec<u32>>) {
    let mut sections = s.split("\n\n");
    let rules = sections
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let mut parts = x.split("|");
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            Pair { a: a, b: b }
        })
        .collect();
    let updates = sections
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn part_one(s: &str) -> u32 {
    let (rules, updates) = parse_input(s);

    let mut total = 0;
    for update in updates {
        let update_map = update
            .iter()
            .enumerate()
            .map(|x| (x.1, x.0))
            .collect::<HashMap<_, _>>();
        let mut valid = true;
        for rule in &rules {
            if update_map.contains_key(&rule.a) && update_map.contains_key(&rule.b) {
                let rule_b = update_map.get(&rule.b).unwrap_or(&usize::MAX);
                let rule_a = update_map.get(&rule.a).unwrap_or(&0);
                if rule_b < rule_a {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            total += update[update.len() / 2]
        }
    }

    total
}

fn my_sort(update: &mut Vec<u32>, rules: &HashSet<Pair<u32>>) -> Vec<u32> {
    let mut applicable_rules: HashSet<&Pair<u32>> = HashSet::new();
    for rule in rules {
        // Check if the rule is applicable to the current vector
        if update.contains(&rule.a) && update.contains(&rule.b) {
            applicable_rules.insert(rule);
        }
    }

    let mut possible_roots = update.iter().collect::<HashSet<_>>();
    for rule in &applicable_rules {
        possible_roots.remove(&rule.b);
    }
    assert!(possible_roots.len() == 1);
    let root = **(possible_roots.iter().next().unwrap());

    // let more_rules = &applicable_rules;
    let successors = |node: &u32| {
        let mut result = vec![];
        for rule in &applicable_rules {
            if rule.a == *node {
                result.push(rule.b);
            }
        }
        result
    };

    let sorted = topological_sort(&[root], successors);
    sorted.unwrap()
}

fn part_two(s: &str) -> u32 {
    let (rules, updates) = parse_input(s);

    let mut total = 0;
    for mut update in updates {
        // A map should be quicker to search.
        let update_map = update
            .iter()
            .enumerate()
            .map(|x| (x.1, x.0))
            .collect::<HashMap<_, _>>();
        let mut valid = true;
        for rule in &rules {
            if update_map.contains_key(&rule.a) && update_map.contains_key(&rule.b) {
                let rule_b = update_map.get(&rule.b).unwrap_or(&usize::MAX);
                let rule_a = update_map.get(&rule.a).unwrap_or(&0);
                if rule_b < rule_a {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            let my_update = my_sort(&mut update, &rules);
            total += my_update[my_update.len() / 2]
        }
    }
    total
}

fn alt_parse_input(contents: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = contents.split("\n\n").collect();

    // Parse ordering rules into a HashMap
    let mut ordering: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in parts[0].lines() {
        if line.trim().is_empty() {
            continue;
        }
        let nums: Vec<i32> = line
            .split('|')
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();

        // Add the rule: nums[0] must come before nums[1]
        ordering
            .entry(nums[0])
            .or_insert_with(HashSet::new)
            .insert(nums[1]);
    }

    // Parse updates into Vec<Vec<i32>>
    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (ordering, updates)
}

fn check_update(ordering: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    // for each update, check if it is valid, exiting early if not

    let mut valid = true;

    for i in 0..update.len() - 1 {
        let before = update[i];
        let after = update[i + 1];

        if !ordering.contains_key(&before) || !ordering[&before].contains(&after) {
            valid = false;
            break;
        }
    }

    valid
}

fn get_middle_value(update: &Vec<i32>) -> i32 {
    // get the middle value of the update

    // assert length is odd - we're not told how to handle middle otherwise
    assert_eq!(update.len() % 2, 1);

    // return the middle value
    update[update.len() / 2]
}

fn alt_part_one(s: &str) -> i32 {
    let (ordering, updates) = alt_parse_input(s);

    let mut result = 0;

    for update in updates {
        if check_update(&ordering, &update) {
            result += get_middle_value(&update);
        }
    }

    result
}

fn compare_values(a: &i32, b: &i32, ordering: &HashMap<i32, HashSet<i32>>) -> Ordering {
    // If a must come before b according to ordering
    if ordering.get(a).map_or(false, |set| set.contains(b)) {
        return Ordering::Less;
    }
    // If b must come before a according to ordering
    if ordering.get(b).map_or(false, |set| set.contains(a)) {
        return Ordering::Greater;
    }
    // If no explicit ordering, maintain original order
    Ordering::Equal
}

fn alt_part_two(s: &str) -> i32 {
    let (ordering, updates) = alt_parse_input(s);

    let mut result = 0;

    for update in updates {
        if !check_update(&ordering, &update) {
            let mut sorted = update.clone();
            // sort based on ordering hashmap
            sorted.sort_by(|a, b| compare_values(a, b, &ordering));
            result += get_middle_value(&sorted);
        }
    }

    result
}

fn main() {
    let input = problem_input("05");

    let now = Instant::now();
    let my_solution = part_one(&input);
    let elapsed = now.elapsed();
    println!("Part 1: {}", my_solution);
    println!("Time: {}µs", elapsed.as_micros());

    let now = Instant::now();
    let alt_solution = alt_part_one(&input);
    let elapsed = now.elapsed();
    println!("Part 1 (alt): {}", alt_solution);
    println!("Time: {}µs", elapsed.as_micros());

    let now = Instant::now();
    let my_solution = part_two(&input);
    let elapsed = now.elapsed();
    println!("Part 2: {}", my_solution);
    println!("Time: {}µs", elapsed.as_micros());

    let now = Instant::now();
    let alt_solution = alt_part_two(&input);
    let elapsed = now.elapsed();
    println!("Part 2 (alt): {}", alt_solution);
    println!("Time: {}µs", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::example_input;

    #[test]
    fn test_parse_input() {
        let input = "\
        1|2\n\
        3|4\n\
        \n\
        1,2,3\n\
        4,5,6\n\
        ";
        let result = parse_input(input);
        assert_eq!(
            (
                HashSet::from([Pair { a: 1, b: 2 }, Pair { a: 3, b: 4 }]),
                HashSet::from([vec![1, 2, 3], vec![4, 5, 6]]),
            ),
            result
        );
    }

    #[test]
    fn test_part_one() {
        let input = example_input("05");
        let result = part_one(&input);
        assert_eq!(143, result);
    }

    #[test]
    fn test_part_two() {
        let input = example_input("05");
        let result = part_two(&input);
        assert_eq!(123, result);
    }

    #[test]
    fn test_hash_maps() {
        let mut set = HashSet::new();
        set.insert(&&1);
        set.insert(&&1000000);
        assert!(set.contains(&&1000000));
        assert!(set.contains(&&1));

        let mut set = HashSet::new();
        set.insert(1);
        set.insert(1000000);
        assert!(set.contains(&&&&&1000000));
        assert!(set.contains(&&&&&1));

        // Don't even think about adding any more &s!
        let mut myset = HashSet::new();
        myset.insert("s");
        assert!(myset.contains(&"s"));
    }
}
