use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}
type InputRef = Input;

struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>,
}


#[aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Input> {
    let (input, result) = parse_input(input)
        .map_err(|err| err.map(|err| anyhow!(nom::error::convert_error(input, err))))?;
    if !input.is_empty() {
        return Err(anyhow!("Had unparsed input after parsing: {}", input));
    }
    Ok(result)
}

type IResult<I, T> = nom::IResult<I, T, nom::error::VerboseError<I>>;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::multi::{many1, separated_list1};
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, _) = opt(tag("\n"))(input)?;

    let (input, rules) = parse_rules(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    let (input, updates) = parse_updates(input)?;

    let (input, _) = opt(tag("\n"))(input)?;

    Ok((
        input,
        Input {
            rules,
            updates,
        }
    ))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), parse_rule)(input)
}
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (before, after)) = separated_pair(parse_u32, tag("|"), parse_u32)(input)?;

    Ok((
        input,
        Rule {
            before,
            after,
        }
    ))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list1(tag("\n"), parse_update)(input)
}
fn parse_update(input: &str) -> IResult<&str, Update> {
    let (input, pages) = separated_list1(tag(","), parse_u32)(input)?;

    Ok((
        input,
        Update {
            pages
        }
    ))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number_string) = take_while1(is_number)(input)?;
    let number = number_string.parse().unwrap();

    Ok((input, number))
}
fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[derive(Default)]
struct Relation {
    afters: HashSet<u32>,
    befores: HashSet<u32>,
}

#[aoc(day5, part1)]
fn solve_part1(input: &InputRef) -> u32 {
    let mut relations: HashMap<u32, Relation> = HashMap::new();
    
    for rule in &input.rules {
        let after_entry = relations.entry(rule.after).or_default();
        after_entry.befores.insert(rule.before);

        let before_entry = relations.entry(rule.before).or_default();
        before_entry.afters.insert(rule.after);
    }

    let mut valid_updates_sum = 0;

    'updates:
    for update in &input.updates {
        for i in 0..update.pages.len() {
            let before_page = update.pages[i];
            if !relations.contains_key(&before_page) { continue; }

            let before_relation = &relations[&before_page];

            for j in i + 1..update.pages.len() {
                let after_page = update.pages[j];
                
                if before_relation.befores.contains(&after_page) {
                    //println!("invalid_relation: {i}, {j}");
                    continue 'updates;
                }
            }
        }

        //println!("{update:?}");

        valid_updates_sum += update.pages[(update.pages.len() - 1) / 2];
    }

    valid_updates_sum
}

#[aoc(day5, part2)]
fn solve_part2(input: &InputRef) -> u32 {
    let mut relations: HashMap<u32, Relation> = HashMap::new();
    
    for rule in &input.rules {
        let after_entry = relations.entry(rule.after).or_default();
        after_entry.befores.insert(rule.before);

        let before_entry = relations.entry(rule.before).or_default();
        before_entry.afters.insert(rule.after);
    }

    let mut invalid_updates = Vec::with_capacity(input.updates.len());

    'updates:
    for update in &input.updates {
        for i in 0..update.pages.len() {
            let before_page = update.pages[i];
            if !relations.contains_key(&before_page) { continue; }

            let before_relation = &relations[&before_page];

            for j in i + 1..update.pages.len() {
                let after_page = update.pages[j];
                
                if before_relation.befores.contains(&after_page) {
                    invalid_updates.push(update.clone());
                    continue 'updates;
                }
            }
        }
    }

    let mut sorted_updates_sum = 0;

    for mut update in invalid_updates {
        let mut changed = true;
        while changed {
            changed = false;

            'pages:
            for i in 0..update.pages.len() - 1 {
                let before_page = update.pages[i];
                if !relations.contains_key(&before_page) { continue; }

                let before_relation = &relations[&before_page];

                for j in i + 1..update.pages.len() {
                    let after_page = update.pages[j];
                    
                    if before_relation.befores.contains(&after_page) {
                        update.pages.swap(i, i + 1);

                        changed = true;
                        continue 'pages;
                    }
                }
            }
        }

        sorted_updates_sum += update.pages[(update.pages.len() - 1) / 2];
    }

    sorted_updates_sum
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 123);
    }
}
