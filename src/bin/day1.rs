use clap::Parser;
use std::fs;

#[derive(Debug, Clone)]
struct Args {
    first: Vec<usize>,
    second: Vec<usize>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long, value_parser=parse_file)]
    input: (Vec<usize>, Vec<usize>),
}

fn parse_file(filename: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let mut first_v = Vec::new();
    let mut second_v = Vec::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let first = parts
            .next()
            .expect(format!("No first value found on line: {}", line).as_str())
            .parse::<usize>()
            .expect(format!("Invalid u8 for line {}", line).as_str());
        let second = parts
            .next()
            .expect(format!("No second value found on line: {}", line).as_str())
            .parse::<usize>()
            .expect(format!("Invalid u8 for line {}", line).as_str());
        first_v.push(first);
        second_v.push(second);
    }
    Ok((first_v, second_v))
}

fn main() {
    let file_args = FilenameArg::parse();
    let val_args = Args {
        first: file_args.input.0,
        second: file_args.input.1,
    };
    assert_eq!(
        val_args.first.len(),
        val_args.second.len(),
        "The number of arguments for the first and second list must be the same!"
    );

    let part_1_result = part1(val_args.clone());
    let part_2_result = part2(val_args);

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(mut val_args: Args) -> usize {
    let first = {
        val_args.first.sort_unstable();
        val_args.first
    };
    let second = {
        val_args.second.sort_unstable();
        val_args.second
    };
    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

fn part2(val_args: Args) -> usize {
    let mut result = 0;
    for l in val_args.first.into_iter() {
        for &r in &val_args.second {
            if l == r {
                result += l;
            }
        }
    }
    result
}
