//! Solutions for https://adventofcode.com/2018/day/13

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(147061));
}

fn solve_part1(prior_recipes: usize) -> u64 {
    let mut scores = Vec::with_capacity(prior_recipes + 10);
    let mut current = vec![0, 1];

    scores.push(3);
    scores.push(7);

    while scores.len() < prior_recipes + 10 {
        let sum: u8 = current.iter().map(|i| scores[*i]).sum();

        if sum < 10 {
            scores.push(sum)
        } else {
            let d = sum / 10;
            scores.push(d);
            scores.push(sum - 10 * d);
        }

        current = current.into_iter()
            .map(|i| (1 + i + scores[i] as usize) % scores.len())
            .collect();
    }

    (prior_recipes..prior_recipes + 10)
        .enumerate()
        .map(|(pos, si)| scores[si] as u64 * 10u64.pow(9 - pos as u32))
        .sum()
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
}
