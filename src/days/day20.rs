use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

use day20::Direction::*;
use utils::data::load_data;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

type Trace = Vec<Direction>;

type TracesChoice = Vec<Trace>;

type Graph = HashMap<(i32, i32), HashSet<(i32, i32)>>;

enum Direction {
    Step(char),
    SubTrace(Box<TracesChoice>),
}

fn solve_part1(trace: Trace) -> u32 {
    get_furthest_node(build_graph(trace), 0, 0)
}

fn solve_part2(trace: Trace) -> usize {
    get_node_distances(build_graph(trace), 0, 0).values()
        .filter(|d| **d >= 1000)
        .count()
}

fn get_furthest_node(graph: Graph, y: i32, x: i32) -> u32 {
    *get_node_distances(graph, y, x).values().max().unwrap()
}

fn get_node_distances(graph: Graph, y: i32, x: i32) -> HashMap<(i32, i32), u32> {
    let mut node_distances = HashMap::new();
    node_distances.insert((y, x), 0);

    let mut frontier = vec![(y, x)];
    let mut distance = 0;

    loop {
        distance += 1;
        let mut new_frontier = vec![];

        for (y, x) in frontier {
            for (y2, x2) in graph[&(y, x)].iter() {
                let loc = (*y2, *x2);
                if node_distances.get(&loc).map_or(true, |d| distance < *d) {
                    node_distances.insert(loc, distance);
                    new_frontier.push(loc);
                }
            }
        }
        frontier = new_frontier;
        if frontier.is_empty() {
            break node_distances;
        }
    }
}

fn build_graph(trace: Trace) -> Graph {
    add_trace_to_graph(trace, Graph::new(), 0, 0)
}

fn add_trace_to_graph(trace: Trace, mut graph: Graph, mut y: i32, mut x: i32) -> Graph {
    for dir in trace {
        match dir {
            Step(c) => {
                let (y2, x2) = match c {
                    'N' => (y - 1, x),
                    'E' => (y, x + 1),
                    'W' => (y, x - 1),
                    'S' => (y + 1, x),
                    _ => panic!("{}", c)
                };
                add_edge(&mut graph, y, x, y2, x2);
                add_edge(&mut graph, y2, x2, y, x);
                y = y2;
                x = x2;
            }
            SubTrace(c) => {
                for trace in *c {
                    graph = add_trace_to_graph(trace, graph, y, x);
                }
            }
        }
    }
    graph
}

fn add_edge(graph: &mut Graph, y1: i32, x1: i32, y2: i32, x2: i32) {
    graph.entry((y1, x1)).or_insert(HashSet::new()).insert((y2, x2));
}

fn get_puzzle_input() -> Trace {
    parse_input(load_data("day20"))
}

fn parse_input(input: String) -> Trace {
    let mut chars = input.chars().peekable();
    assert_eq!(chars.next(), Some('^'));
    let trace = parse_trace(&mut chars);
    assert_eq!(chars.next(), Some('$'));
    trace
}

fn parse_trace(chars: &mut Peekable<Chars>) -> Trace {
    let mut result = Trace::new();
    loop {
        let next = if let Some(p) = chars.peek() { *p } else { break; };
        result.push(match next {
            'N' | 'E' | 'W' | 'S' => Step(chars.next().unwrap()),
            '(' => SubTrace(Box::new(parse_traces_choice(chars))),
            _ => break
        });
    }
    result
}

fn parse_traces_choice(chars: &mut Peekable<Chars>) -> TracesChoice {
    let mut result = TracesChoice::new();
    assert_eq!(chars.next(), Some('('));
    loop {
        let next = if let Some(p) = chars.peek() { *p } else { break; };
        if !result.is_empty() {
            if next == ')' {
                break;
            }
            if next == '|' {
                chars.next();
            }
        }
        result.push(parse_trace(chars));
    }
    assert_eq!(chars.next(), Some(')'));
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(parse_input(String::from("^WNE$"))),
            3
        );

        assert_eq!(
            solve_part1(parse_input(String::from("^ENWWW(NEEE|SSE(EE|N))$"))),
            10
        );

        assert_eq!(
            solve_part1(parse_input(String::from(
                "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
            ))),
            18
        );

        assert_eq!(
            solve_part1(parse_input(String::from(
                "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
            ))),
            23
        );

        assert_eq!(
            solve_part1(parse_input(String::from(
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
            ))),
            31
        );
    }
}
