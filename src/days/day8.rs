use utils::data::load_data;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

struct Node {
    children: Vec<Node>,
    meta: Vec<u32>,
}

fn solve_part1(input: Vec<u32>) -> u32 {
    get_meta_sum(&decode(input))
}

fn decode(input: Vec<u32>) -> Node {
    let mut iter = input.into_iter();
    decode_step(&mut || iter.next().unwrap())
}

fn decode_step(read: &mut FnMut() -> u32) -> Node {
    let num_children = read() as usize;
    let num_meta_entries = read() as usize;

    let children = repeat(&mut || decode_step(read), num_children);
    let meta = repeat(read, num_meta_entries);

    Node { children, meta }
}

fn repeat<I>(func: &mut FnMut() -> I, n: usize) -> Vec<I> {
    (0..n).into_iter().map(|_| func()).collect()
}

fn get_meta_sum(node: &Node) -> u32 {
    node.meta.iter().map(|m| *m).sum::<u32>() +
        node.children.iter().map(get_meta_sum).sum::<u32>()
}

fn get_puzzle_input() -> Vec<u32> {
    parse_puzzle_input(load_data("day8"))
}

fn parse_puzzle_input(input: String) -> Vec<u32> {
    input.trim().split(' ').into_iter()
        .map(|c| c.parse::<u32>().unwrap())
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

    fn get_test_input() -> Vec<u32> {
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    }
}
