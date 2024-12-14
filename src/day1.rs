use std::collections::HashMap;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = (usize, usize);

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Input> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once("   ").ok_or(anyhow!("Incorrect line format"))?;
            Ok((left.parse()?, right.parse()?))
        })
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &InputRef) -> usize {
    let mut lefts: Vec<_> = input.iter().map(|a| a.0).collect();
    let mut rights: Vec<_> = input.iter().map(|a| a.1).collect();

    lefts.sort();
    rights.sort();

    lefts.iter().zip(rights.iter())
        .map(|(left, right)| {
            left.abs_diff(*right)
        })
        .sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &InputRef) -> usize {
    let lefts = {
        let mut lefts: HashMap<_, _> = HashMap::with_capacity(input.len());

        for (left, _) in input {
            let entry = lefts.entry(left).or_insert(0);
            *entry += 1;
        }

        lefts
    };
    let rights = {
        let mut rights: HashMap<_, _> = HashMap::with_capacity(input.len());

        for (_, right) in input {
            let entry = rights.entry(right).or_insert(0);
            *entry += 1;
        }

        rights
    };

    lefts.iter()
        .map(|(val, count)| {
            *val * count * rights.get(val).unwrap_or(&0)
        })
        .sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT1: &'static str =
r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 31);
    }
}


