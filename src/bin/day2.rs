use clap::Parser;
use std::time;
use itertools::Itertools;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

fn parse_file(filename: &str) -> Result<Vec<Vec<usize>>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let mut parsed_content = Vec::<Vec<usize>>::new();
    for line in content.lines() {
        let parts = line.split_whitespace();
        let parsed_parts = parts
            .map(|x| x.parse::<usize>().expect("input broke"))
            .collect();
        parsed_content.push(parsed_parts);
    }
    // println!("{:?}",parsed_content);
    Ok(parsed_content)
}

fn main() {
    let args = FilenameArg::parse();
    let val_args = parse_file(args.input.as_str()).expect("error parsing file");

    let mut start = time::Instant::now();
    let part_1_result = part1(val_args.clone());
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(val_args.clone());
    println!("Part 2: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result_2_electric_boogaloo = part2_2(val_args);
    println!("Part 2_2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
    println!("Part 2_2 result: {}", part_2_result_2_electric_boogaloo);
}

fn part1(args: Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for v in args {
        let mut it = v.into_iter().tuple_windows::<(usize, usize)>();
        let safe = it.clone().all(|(x1, x2)| x1 > x2 && x1.abs_diff(x2) <= 3)
            || it.all(|(x1, x2)| x1 < x2 && x1.abs_diff(x2) <= 3);
        if safe {
            result += 1;
        };
    }
    result
}

//not a good approach!!!
fn part2_2(args: Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for v in args {
        let orig_size = v.len();
        if is_safe(v.clone(), usize::gt, orig_size) || is_safe(v.clone(), usize::lt, orig_size) {
            result += 1;
        } 
    }
    result
}

fn is_safe(v: Vec<usize>, operator: fn(&usize, &usize) -> bool, orig_size: usize) -> bool {
    let mut pkbl = v.iter().peekable();
    let mut i = 0;
    while let Some(x1) = pkbl.next() {
        if let Some(x2) = pkbl.peek() {
            if !(operator(x1, x2) && x1.abs_diff(**x2) <= 3) {
                if v.len() != orig_size {
                    return false;
                };
                if is_safe(
                    v.iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, e)| e.clone())
                        .collect(),
                    operator,
                    orig_size,
                ) {
                    return true;
                } else {
                    continue;
                }
            }
        } else if v.len() == orig_size {
            return is_safe(v[..v.len() - 1].to_vec(), operator, orig_size);
        }
        i += 1;
    }
    return true;
}

fn part2(args: Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for v in args.clone() {
        for (i, _) in v.iter().enumerate() {
            let v_without_element: Vec<usize> = v
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, e)| e.clone())
                .collect();
            let mut it = v_without_element.iter().tuple_windows::<(&usize, &usize)>();
            let safe = it.clone().all(|(x1, x2)| x1 > x2 && x1.abs_diff(*x2) <= 3)
                || it.all(|(x1, x2)| x1 < x2 && x1.abs_diff(*x2) <= 3);
            if safe {
                result += 1;
                break;
            };
        }
    }
    result
}
