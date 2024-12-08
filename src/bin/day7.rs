use clap::Parser;
use itertools::Itertools;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PotentialOp {
    Plus,
    Mul,
    Comb
}


impl PotentialOp {
    fn f(&self, x: &usize, y: &usize) -> usize {
        match &self {
            PotentialOp::Plus => x + y,
            PotentialOp::Mul => x * y,
            PotentialOp::Comb => (x.to_string() + y.to_string().as_str()).parse::<usize>().expect(format!("failed Comb operator on {x}, {y}").as_str())
        }
    }
    fn variants() -> Vec<PotentialOp> {
        vec![PotentialOp::Plus, PotentialOp::Mul]
    }
    fn variants_pt2() -> Vec<PotentialOp> {
        vec![PotentialOp::Plus, PotentialOp::Mul, PotentialOp::Comb]
    }
    fn cproduct(n: usize) -> Vec<Vec<PotentialOp>> {
        let v = vec![PotentialOp::variants(); n];
        let perms = v.into_iter().multi_cartesian_product().collect_vec();
        debug_assert_eq!(perms.len(), 2_usize.pow(n as u32));
        perms
    }
    fn cproduct_pt2(n: usize) -> Vec<Vec<PotentialOp>> {
        let v = vec![PotentialOp::variants_pt2(); n];
        let perms = v.into_iter().multi_cartesian_product().collect_vec();
        debug_assert_eq!(perms.len(), 3_usize.pow(n as u32));
        perms
    }
}

fn parse_file(filename: &str) -> Result<Vec<(usize, Vec<usize>)>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let v: Vec<(usize, Vec<usize>)> = content
        .lines()
        .map(|line| {
            line.split(":")
                .collect_tuple()
                .map(|(l, r)| {
                    (
                        l.parse::<usize>().expect("invalid l found"),
                        r.split_whitespace()
                            .map(|n| n.parse::<usize>().expect("some invalid r found"))
                            .collect(),
                    )
                })
                .expect("couldn't deconstruct into 2 tuple")
        })
        .collect_vec();

    Ok(v)
}

fn main() {
    let args = FilenameArg::parse();
    let equations = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&equations);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&equations);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(eqs: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;
    for (l, r) in eqs.iter() {
        for ops_perm in PotentialOp::cproduct(r.len() - 1) {
            if apply_ops(r, ops_perm) == *l {
                result += l;
                break;
            }
        }
    }
    println!("whole thing: {:?}", eqs.iter().map(|(x,_)| x).sum::<usize>());
    result
}

fn part2(eqs: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;
    for (l, r) in eqs.iter() {
        for ops_perm in PotentialOp::cproduct_pt2(r.len() - 1) {
            if apply_ops(r, ops_perm) == *l {
                result += l;
                break;
            }
        }
    }
    result
}
fn apply_ops(v: &Vec<usize>, ops_list: Vec<PotentialOp>) -> usize {
    assert_eq!(ops_list.len(), v.len() - 1, "incorrect number of ops for v");
    let mut result = v[0];
    for (n, op) in v[1..].iter().zip(ops_list) {
        result = op.f(&result, n);
    }
    result
}
