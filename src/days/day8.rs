use utils::data::load_data;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

struct Node {
    children: Vec<Node>,
    meta: Vec<usize>,
}

fn solve_part1(input: Vec<usize>) -> usize {
    get_meta_sum(&decode(input))
}

fn solve_part2(input: Vec<usize>) -> usize {
    get_node_value(&decode(input))
}

fn decode(input: Vec<usize>) -> Node {
    fn decode_(read: &mut FnMut() -> usize) -> Node {
        let header = repeat(read, 2);
        Node {
            children: repeat(&mut || decode_(read), header[0]),
            meta: repeat(read, header[1]),
        }
    }
    let mut iter = input.into_iter();
    decode_(&mut || iter.next().unwrap())
}

fn repeat<I>(func: &mut FnMut() -> I, n: usize) -> Vec<I> {
    (0..n).into_iter().map(|_| func()).collect()
}

fn get_meta_sum(node: &Node) -> usize {
    node.meta.iter().map(|m| *m).sum::<usize>() +
        node.children.iter().map(get_meta_sum).sum::<usize>()
}

fn get_node_value(node: &Node) -> usize {
    if node.children.is_empty() {
        get_meta_sum(node)
    } else {
        node.meta.iter()
            .filter(|i| **i > 0 && **i <= node.children.len())
            .map(|i| get_node_value(&node.children[(i - 1)]))
            .sum::<usize>()
    }
}

fn get_puzzle_input() -> Vec<usize> {
    parse_puzzle_input(load_data("day8"))
}

fn parse_puzzle_input(input: String) -> Vec<usize> {
    input.trim().split(' ').into_iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_puzzle_input() {
        assert_eq!(
            parse_puzzle_input(String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 ")),
            get_test_input()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(get_test_input()),
            138
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(get_test_input()),
            66
        );
    }

    fn get_test_input() -> Vec<usize> {
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    }
}
