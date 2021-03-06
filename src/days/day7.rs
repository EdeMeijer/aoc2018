//! Solutions for https://adventofcode.com/2018/day/7
use std::collections::HashSet;

use regex::Regex;

use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input(), 5, 60));
}

#[derive(Eq, PartialEq, Debug)]
struct Dependency {
    id: char,
    depends_on: char,
}

fn solve_part1(dependencies: Vec<Dependency>) -> String {
    let ids = get_sorted_ids(&dependencies);

    let mut order = Vec::with_capacity(ids.len());

    for _ in 0..ids.len() {
        let next = get_ready_ids(&ids, &dependencies, &order)[0];
        order.push(next);
    }

    order.into_iter().collect()
}

fn solve_part2(dependencies: Vec<Dependency>, num_workers: usize, base_duration: u32) -> u32 {
    let mut pending_ids = get_sorted_ids(&dependencies);
    let num_jobs = pending_ids.len();

    // Track workers working on tasks and when that will be done
    let mut jobs = vec![];

    let mut resolved = Vec::with_capacity(num_jobs);
    let mut time = 0;

    loop {
        // Handle finished jobs
        let (finished, remaining) = jobs.iter().partition(|(_, t)| *t == time);
        for (task, _) in finished {
            resolved.push(task);
        }
        if resolved.len() == num_jobs {
            break;
        }
        jobs = remaining;

        // Get current availability of tasks and workers
        let ready = get_ready_ids(&pending_ids, &dependencies, &resolved);
        let available_workers = num_workers - jobs.len();

        if ready.is_empty() || available_workers == 0 {
            // No tasks or no workers, wait until a job finishes and try again
            let wait_until = jobs.iter()
                .map(|j| j.1)
                .min().unwrap();

            time = wait_until;
            continue;
        }

        // Assign as many workers to tasks as possible
        for task in ready.iter().take(available_workers) {
            let task_duration = base_duration + 1 + (*task as u8 - 'A' as u8) as u32;
            jobs.push((*task, time + task_duration));
            pending_ids = pending_ids.into_iter().filter(|id| id != task).collect();
        }
    }

    time
}

fn get_sorted_ids(dependencies: &Vec<Dependency>) -> Vec<char> {
    let mut ids: Vec<_> = dependencies.iter()
        .flat_map(|d| vec![d.id, d.depends_on].into_iter())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    ids.sort();

    ids
}

fn get_ready_ids(
    ids: &Vec<char>,
    dependencies: &Vec<Dependency>,
    resolved: &Vec<char>,
) -> Vec<char> {
    // Get all ids that are not yet resolved and that have no unresolved dependencies
    ids.into_iter()
        .filter(|id| !resolved.contains(id))
        .filter(|id| {
            dependencies.iter()
                .filter(|d| d.id == **id)
                .all(|d| resolved.contains(&d.depends_on))
        })
        .map(|c| *c)
        .collect()
}

fn get_puzzle_input() -> Vec<Dependency> {
    non_empty_lines(load_data("day7"))
        .into_iter()
        .map(parse_input_line)
        .collect()
}

fn parse_input_line(line: String) -> Dependency {
    let re = Regex::new(
        r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$"
    ).unwrap();

    let cap = re.captures(&line).unwrap();
    Dependency {
        id: cap[2].chars().next().unwrap(),
        depends_on: cap[1].chars().next().unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input_line(String::from("Step B must be finished before step E can begin.")),
            Dependency { id: 'E', depends_on: 'B' }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(get_test_input()),
            String::from("CABDFE")
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(get_test_input(), 2, 0),
            15
        )
    }

    fn get_test_input() -> Vec<Dependency> {
        vec![
            Dependency { id: 'A', depends_on: 'C' },
            Dependency { id: 'F', depends_on: 'C' },
            Dependency { id: 'B', depends_on: 'A' },
            Dependency { id: 'D', depends_on: 'A' },
            Dependency { id: 'E', depends_on: 'D' },
            Dependency { id: 'E', depends_on: 'F' },
        ]
    }
}
