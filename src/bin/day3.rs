use clap::Parser;
use regex::Regex;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

fn parse_file(filename: &str) -> Result<String, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    // println!("{:?}",parsed_content);
    Ok(content)
}

fn main() {
    let args = FilenameArg::parse();
    let val_args = parse_file(args.input.as_str()).expect("error parsing file");

    let mut start = time::Instant::now();
    let part_1_result = part1(val_args.as_str());
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(val_args.as_str());
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(s: &str) -> usize {
    let mut result = 0;

    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))").unwrap();
    for mul_str in re.captures_iter(s) {
        let a = mul_str.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let b = mul_str.get(3).unwrap().as_str().parse::<usize>().unwrap();
        result += a * b;
    }
    result
}

fn part2(s: &str) -> usize {
    let mut result = 0;
    let mut ignore_mul = false;

    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|((don't)|(do))").unwrap();
    for mul_str in re.captures_iter(s) {
        println!("{:?}",mul_str);
        let maybe_ignore_command = mul_str.get(4);
        if let Some(x) = maybe_ignore_command {
            if x.as_str() == "do" {
                ignore_mul = false;
            } else if x.as_str() == "don't" {
                ignore_mul = true;
            }
            continue;
        }
        if !ignore_mul {
            let a = mul_str.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let b = mul_str.get(3).unwrap().as_str().parse::<usize>().unwrap();
            result += a * b;
        }
    }
    result
}
