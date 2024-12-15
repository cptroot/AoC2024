use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Input = Vec<Data>;
type InputRef = [Data];
type Data = (u32, u32);


#[aoc_generator(day3, part1)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

type IResult<I, T> = nom::IResult<I, T, nom::error::VerboseError<I>>;
use nom::bytes::complete::{take_while1, take};
use nom::multi::fold_many0;
use nom::combinator;
use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::branch::alt;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = parse_operations(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

fn parse_operations(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    fold_many0(
        alt((
            combinator::map(parse_mul_operation, |v| Some(v)),
            combinator::map(take(1usize), |_| None)
        )),
        Vec::new,
        |mut acc: Vec<_>, val| {
            if let Some(pair) = val {
                acc.push(pair)
            }
            acc
        }
    )(input)
}

fn parse_mul_operation(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, left) = parse_u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, right) = parse_u32(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (left, right)))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[aoc_generator(day3, part2)]
fn input_generator_do_dont(input: &str) -> Result<Input> {
    let (input, result) = parse_input_do_dont(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

fn parse_input_do_dont(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, result) = parse_operations_do_dont(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((input, result))
}

enum Ops {
    Do,
    Dont,
    Mul((u32, u32)),
    None,
}

fn parse_operations_do_dont(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let mut add = true;
    let (input, result) = fold_many0(
        alt((
            combinator::map(parse_do, |_| Ops::Do),
            combinator::map(parse_dont, |_| Ops::Dont),
            combinator::map(parse_mul_operation, |v| Ops::Mul(v)),
            combinator::map(take(1usize), |_| Ops::None)
        )),
        Vec::new,
        |mut acc: Vec<_>, val| {
            match val {
                Ops::Do => { add = true; },
                Ops::Dont => { add = false; },
                Ops::Mul(pair) => {
                    if add {
                        acc.push(pair)
                    }
                }
                Ops::None => { },
            }
            acc
        }
    )(input)?;

    Ok((input, result))
}

fn parse_do(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, ()))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, ()))
}

#[aoc(day3, part1)]
fn solve_part1(input: &InputRef) -> usize {
    input.iter()
        .map(|(left, right)| {
            left * right
        })
        .sum::<u32>() as usize
}

#[aoc(day3, part2)]
fn solve_part2(input: &InputRef) -> usize {
    input.iter()
        .map(|(left, right)| {
            left * right
        })
        .sum::<u32>() as usize
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;
    const TEST_INPUT2: &'static str =
r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator_do_dont(TEST_INPUT2).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 48);
    }
}
