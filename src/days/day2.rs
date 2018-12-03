//! Solutions for https://adventofcode.com/2018/day/2
use std::collections::HashMap;
use std::collections::HashSet;

use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

fn solve_part1(box_ids: Vec<String>) -> u32 {
    // Track how many times different kinds of multiples occur in box IDs
    let mut multiple_counts = HashMap::new();

    for id in box_ids {
        // Collect the unique multiples of letters in a hash set
        let unique_counts = count_letters(id)
            .into_iter()
            .map(|(_, count)| count)
            .collect::<HashSet<u32>>();

        // Use them to update the numbers of multiple occurrences
        for count in unique_counts {
            *multiple_counts.entry(count).or_insert(0u32) += 1;
        }
    }

    // Use pattern matching to multiply the occurrences if they both exist, otherwise return 0
    match (multiple_counts.get(&2), multiple_counts.get(&3)) {
        (Some(a), Some(b)) => a * b,
        _ => 0
    }
}

fn solve_part2(box_ids: Vec<String>) -> String {
    for a in box_ids.iter() {
        for b in box_ids.iter() {
            // Count how many character positions have different letters
            let delta: u32 = a.chars().zip(b.chars())
                .map(|(ca, cb)| (ca != cb) as u32)
                .sum();

            if delta == 1 {
                // Return the characters that are the same at their respective positions
                return a.chars().zip(b.chars())
                    .filter(|(ca, cb)| ca == cb)
                    .map(|(ca, _)| ca)
                    .collect();
            }
        }
    }

    panic!("Did not find any matching IDs")
}

/// Count the occurrences of every distinct character in a string
fn count_letters(id: String) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for letter in id.chars() {
        *counts.entry(letter).or_insert(0) += 1;
    }
    counts
}

fn get_puzzle_input() -> Vec<String> {
    non_empty_lines(load_data("day2"))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_letters() {
        assert_eq!(
            count_letters(String::from("abbccccb")),
            hashmap! {
                'a' => 1,
                'b' => 3,
                'c' => 4,
            }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(vec![
                String::from("abcdef"),
                String::from("bababc"),
                String::from("abbcde"),
                String::from("abcccd"),
                String::from("aabcdd"),
                String::from("abcdee"),
                String::from("ababab"),
            ]),
            12
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(vec![
                String::from("abcde"),
                String::from("fghij"),
                String::from("klmno"),
                String::from("pqrst"),
                String::from("fguij"),
                String::from("axcye"),
                String::from("wvxyz"),
            ]),
            String::from("fgij")
        )
    }
}
