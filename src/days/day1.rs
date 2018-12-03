use utils::load_data;
use utils::non_empty_lines;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1() {
    let mut frequency = 0;
    for change in get_change_list() {
        frequency += change;
    }
    println!("{}", frequency)
}

#[allow(dead_code)]
pub fn part2() {
    let mut seen = HashSet::new();
    
    let mut frequency = 0;
    seen.insert(frequency);
    
    let mut changes = get_change_list().into_iter().cycle();
    
    let result = loop {
        frequency += changes.next().unwrap();
        let added = seen.insert(frequency);
        if !added {
            // This was seen before, so the current frequency is the answer
            break frequency;
        }
    };
    
    println!("{}", result);
}

fn get_change_list() -> Vec<i32> {
    non_empty_lines(load_data("day1"))
        .into_iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}
