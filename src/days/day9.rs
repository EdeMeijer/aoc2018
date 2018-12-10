//! Solutions for https://adventofcode.com/2018/day/89
use utils::circular_list::CircularList;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve(473, 70904));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve(473, 70904 * 100));
}

fn solve(players: usize, max_marble: usize) -> usize {
    let mut scores = vec![0usize; players];

    let mut circle = CircularList::with_capacity(max_marble + 1);
    circle.insert(0usize);

    for marble in 1..=max_marble {
        if marble % 23 == 0 {
            scores[marble % players] += circle.seek(-7).remove().unwrap() + marble;
        } else {
            circle.next().insert(marble);
        }
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(9, 25), 32);
        assert_eq!(solve(10, 1618), 8317);
        assert_eq!(solve(13, 7999), 146373);
        assert_eq!(solve(17, 1104), 2764);
        assert_eq!(solve(21, 6111), 54718);
        assert_eq!(solve(30, 5807), 37305);
    }
}
