//! Solutions for https://adventofcode.com/2018/day/13

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(147061));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(vec![1, 4, 7, 0, 6, 1]));
}

struct Generator {
    scores: Vec<u8>,
    score_i: usize,
    current: Vec<usize>,
}

impl Generator {
    fn new() -> Self {
        Generator {
            scores: vec![3, 7],
            score_i: 1,
            current: vec![0, 1],
        }
    }

    fn next(&mut self) {
        self.score_i += 1;
        if self.scores.len() == self.score_i {
            self.step()
        }
    }

    fn len(&self) -> usize {
        self.score_i + 1
    }

    fn seek_back(&self, offset: usize) -> u8 {
        self.scores[self.score_i - offset]
    }

    fn step(&mut self) {
        let sum: u8 = self.current.iter().map(|i| self.scores[*i]).sum();

        if sum < 10 {
            self.scores.push(sum)
        } else {
            let d = sum / 10;
            self.scores.push(d);
            self.scores.push(sum - 10 * d);
        }

        self.current = self.current.iter()
            .map(|i| (1 + *i + self.scores[*i] as usize) % self.scores.len())
            .collect();
    }
}

fn solve_part1(prior_recipes: usize) -> u64 {
    let mut gen = Generator::new();

    while gen.len() < prior_recipes + 10 {
        gen.next()
    }
    (0..10).map(|offset| gen.seek_back(offset) as u64 * 10u64.pow(offset as u32)).sum()
}

fn solve_part2(target_seq: Vec<u8>) -> usize {
    let mut gen = Generator::new();
    let target_seq_rev: Vec<_> = target_seq.into_iter().rev().collect();

    loop {
        gen.next();
        if gen.len() >= target_seq_rev.len() {
            // See if it matches
            let mut found = true;
            for (offset, digit) in target_seq_rev.iter().enumerate() {
                if gen.seek_back(offset) != *digit {
                    // Does not match, keep going
                    found = false;
                    break;
                }
            }
            if found {
                break gen.len() - target_seq_rev.len();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(5), 0124515891);
        assert_eq!(solve_part1(18), 9251071085);
        assert_eq!(solve_part1(2018), 5941429882);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(solve_part2(vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(solve_part2(vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(solve_part2(vec![5, 9, 4, 1, 4]), 2018);
    }
}
