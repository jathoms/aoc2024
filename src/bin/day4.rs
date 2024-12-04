use clap::Parser;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

fn parse_file(filename: &str) -> Result<Vec<Vec<char>>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let parsed_content = content.lines().map(|line| line.chars().collect()).collect();

    Ok(parsed_content)
}

fn main() {
    let args = FilenameArg::parse();
    println!("{:?}", args.input);
    let val_args = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&val_args);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&val_args);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(v: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (i, c_vec) in v.iter().enumerate() {
        for (j, c) in c_vec.iter().enumerate() {
            if *c == 'X' {
                for op in [usize::wrapping_add, usize::wrapping_sub] {
                    if check_horizontal(c_vec, j, op) {
                        result += 1;
                    }
                    if check_vertical(&v, i, j, op) {
                        result += 1;
                    }
                    if check_diagonal(&v, i, j, op) {
                        result += 1;
                    }
                    if check_other_diagonal(&v, i, j, op) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn check_horizontal(v: &Vec<char>, j: usize, op: fn(usize, usize) -> usize) -> bool {
    v.get(op(j, 1)) == Some(&'M') && v.get(op(j, 2)) == Some(&'A') && v.get(op(j, 3)) == Some(&'S')
}

fn check_vertical(v: &Vec<Vec<char>>, i: usize, j: usize, op: fn(usize, usize) -> usize) -> bool {
    v.get(op(i, 1)).and_then(|row| row.get(j)) == Some(&'M')
        && v.get(op(i, 2)).and_then(|row| row.get(j)) == Some(&'A')
        && v.get(op(i, 3)).and_then(|row| row.get(j)) == Some(&'S')
}

fn check_diagonal(v: &Vec<Vec<char>>, i: usize, j: usize, op: fn(usize, usize) -> usize) -> bool {
    v.get(op(i, 1)).and_then(|row| row.get(op(j, 1))) == Some(&'M')
        && v.get(op(i, 2)).and_then(|row| row.get(op(j, 2))) == Some(&'A')
        && v.get(op(i, 3)).and_then(|row| row.get(op(j, 3))) == Some(&'S')
}

fn check_other_diagonal(
    v: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    op: fn(usize, usize) -> usize,
) -> bool {
    let other_op = {
        if op == usize::wrapping_add {
            usize::wrapping_sub as fn(usize, usize) -> usize
        } else {
            usize::wrapping_add as fn(usize, usize) -> usize
        }
    };

    v.get(op(i, 1)).and_then(|row| row.get(other_op(j, 1))) == Some(&'M')
        && v.get(op(i, 2)).and_then(|row| row.get(other_op(j, 2))) == Some(&'A')
        && v.get(op(i, 3)).and_then(|row| row.get(other_op(j, 3))) == Some(&'S')
}

fn part2(v: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (i, c_vec) in v.iter().enumerate() {
        for (j, c) in c_vec.iter().enumerate() {
            if *c == 'A' {
                if check_diagonal_sam(&v, i, j, usize::wrapping_add) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn check_diagonal_sam(
    v: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    op: fn(usize, usize) -> usize,
) -> bool {
    let other_op = {
        if op == usize::wrapping_add {
            usize::wrapping_sub as fn(usize, usize) -> usize
        } else {
            usize::wrapping_add as fn(usize, usize) -> usize
        }
    };

    ((v.get(op(i, 1)).and_then(|row| row.get(op(j, 1))) == Some(&'M')
        && v.get(other_op(i, 1))
            .and_then(|row| row.get(other_op(j, 1)))
            == Some(&'S'))
        || (v.get(op(i, 1)).and_then(|row| row.get(op(j, 1))) == Some(&'S')
            && v.get(other_op(i, 1))
                .and_then(|row| row.get(other_op(j, 1)))
                == Some(&'M')))
        && ((v
            .get(other_op(i, 1))
            .and_then(|row| row.get(op(j, 1)))
            == Some(&'M')
            && v.get(op(i, 1)).and_then(|row| row.get(other_op(j, 1))) == Some(&'S'))
            || (v
                .get(other_op(i, 1))
                .and_then(|row| row.get(op(j, 1)))
                == Some(&'S')
                && v.get(op(i, 1)).and_then(|row| row.get(other_op(j, 1))) == Some(&'M')))
}
