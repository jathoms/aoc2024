use clap::Parser;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Copy, Clone, Debug)]
struct Rules(usize, usize);

struct Day5Args(Vec<Rules>, Vec<Vec<usize>>);

fn parse_file(filename: &str) -> Result<Day5Args, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let rules_match = Regex::new(r"[0-9]+\|[0-9]+").expect("Invalid regex");
    let rule_tuples = rules_match
        .find_iter(&content)
        .map(|line| {
            line.as_str()
                .split("|")
                .map(|n| n.parse::<usize>().expect("Found invalid n in rule"))
                .collect_tuple()
        })
        .collect_vec();

    let updates_match = Regex::new(r"\n([0-9]+(?:,[0-9]+)*)(\r|$)").expect("Invalid updates regex");
    let updates = updates_match
        .captures_iter(&content)
        .map(|line| {
            line.get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|n| n.parse::<usize>().expect("Found a non-parsable string"))
                .collect_vec()
        })
        .collect_vec();

    let rules = rule_tuples
        .into_iter()
        .map(|tup| {
            let (l, r) = tup.expect("Wrong length tuple found");
            Rules(l, r)
        })
        .collect_vec();

    Ok(Day5Args(rules, updates))
}

fn main() {
    let args = FilenameArg::parse();
    println!("{:?}", args.input);
    let Day5Args(rules, updates) = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&rules, &updates);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&rules, &updates);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(rules: &Vec<Rules>, updates: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    println!("Rules: {:?}", rules);
    println!("Updates: {:?}", updates);

    for update in updates {
        let relevant_rules = rules
            .into_iter()
            .filter(|Rules(l, r)| update.contains(l) && update.contains(r))
            .collect::<Vec<&Rules>>();
        for (i, n) in update.iter().enumerate() {
            let mut found_before = Vec::new();
            let mut found_after = Vec::new();
            let before = get_before(&relevant_rules, *n, &mut found_before);
            let after = get_after(&relevant_rules, *n, &mut found_after);

            // println!("Before {:?}: {:?}", n, before);
            // println!("After {:?}: {:?}", n, after);
            let mut b = String::new();
            // std::io::stdin().read_line(&mut b).expect("msg");
            if i != before.len() {
                break;
            }
            if i == update.len() - 1 {
                let middle = update.get(update.len() / 2).unwrap();
                println!("{:?} is middle of {:?}", middle, update);
                result += middle;
            }

            // if before.len() == after.len() {
            //     println!("{:?} in the middle of {:?}", n, update);
            //     result += n;
            //     break;
            // }
        }
    }

    result
}

fn part2(rules: &Vec<Rules>, updates: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for update in updates {
        let relevant_rules = rules
            .into_iter()
            .filter(|Rules(l, r)| update.contains(l) && update.contains(r))
            .collect::<Vec<&Rules>>();

        let mut middle = Option::None;
        let mut not_correct = false;

        for (i, n) in update.iter().enumerate() {
            let mut found_before = Vec::new();
            let mut found_after = Vec::new();

            let before = get_before(&relevant_rules, *n, &mut found_before);
            let after = get_after(&relevant_rules, *n, &mut found_after);

            // println!("Before {:?}: {:?}", n, before);
            // println!("After {:?}: {:?}", n, after);

            if before.len() == after.len() {
                println!("{:?} in the middle of {:?}", n, update);
                middle = Some(n);
            }
            if i != before.len() {
                not_correct = true;
            }
            if let Some(x) = middle {
                if not_correct {
                    result += x;
                    break;
                }
            }
        }
    }

    result
}
fn get_before<'a>(
    rules: &'a Vec<&'a Rules>,
    n: usize,
    found_before: &mut Vec<usize>,
) -> Vec<&'a usize> {
    let mut before = rules
        .into_iter()
        .filter(|Rules(_, r)| *r == n && !found_before.contains(r))
        .map(|Rules(l, _)| l)
        .collect::<Vec<_>>();

    found_before.extend(before.clone());

    if before.is_empty() {
        return Vec::new();
    }

    before.extend(
        before
            .iter()
            .flat_map(|n| get_before(rules, **n, found_before))
            .collect::<Vec<_>>(),
    );
    before
}

fn get_after<'a>(
    rules: &'a Vec<&'a Rules>,
    n: usize,
    found_after: &mut Vec<usize>,
) -> Vec<&'a usize> {
    let mut after = rules
        .into_iter()
        .filter(|Rules(l, _)| *l == n && !found_after.contains(l))
        .map(|Rules(_, r)| r)
        .collect::<Vec<_>>();

    found_after.extend(after.clone());

    if after.is_empty() {
        return Vec::new();
    }
    after.extend(
        after
            .iter()
            .flat_map(|n| get_after(rules, **n, found_after))
            .collect::<Vec<_>>(),
    );
    after
}
