use std::collections::HashSet;

use regex::Regex;
use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[derive(Eq, PartialEq, Debug)]
struct Dependency {
    id: char,
    depends_on: char,
}

fn solve_part1(dependencies: Vec<Dependency>) -> String {
    let ids: HashSet<_> = dependencies.iter()
        .flat_map(|d| vec![d.id, d.depends_on].into_iter())
        .collect();
    
    let mut order = Vec::with_capacity(ids.len());
    
    for _ in 0..ids.len() {
        let next = get_next_ready_id(&ids, &dependencies, &order);
        order.push(next);
    }

    order.into_iter().collect()
}

fn get_next_ready_id(
    ids: &HashSet<char>,
    dependencies: &Vec<Dependency>,
    resolved: &Vec<char>,
) -> char {
    // Get all ids that are not yet resolved but that have no unresolved dependencies
    ids.iter()
        .filter(|id| !resolved.contains(id))
        .filter(|id| {
            dependencies.iter()
                .filter(|d| d.id == **id)
                .all(|d| resolved.contains(&d.depends_on))
        })
        .map(|c| *c)
        .min().unwrap()
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
            solve_part1(vec![
                Dependency { id: 'A', depends_on: 'C' },
                Dependency { id: 'F', depends_on: 'C' },
                Dependency { id: 'B', depends_on: 'A' },
                Dependency { id: 'D', depends_on: 'A' },
                Dependency { id: 'E', depends_on: 'D' },
                Dependency { id: 'E', depends_on: 'F' },
            ]),
            String::from("CABDFE")
        )
    }
}
