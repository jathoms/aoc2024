use clap::Parser;
use itertools::Itertools;
use regex::Regex;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::{fs, time};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec2d(usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vecf2d(Decimal, Decimal);

impl Vecf2d {
    fn from_vec2d(v: Vec2d) -> Self {
        Vecf2d(
            Decimal::from_usize(v.0).unwrap(),
            Decimal::from_usize(v.1).unwrap(),
        )
    }

    fn s_divu(&self, n: usize) -> Self {
        let n = Decimal::from_usize(n).unwrap();
        self.s_div(n)
    }

    fn s_div(&self, n: Decimal) -> Self {
        Vecf2d(self.0 / n, self.1 / n)
    }

    fn s_mul(&self, n: Decimal) -> Self {
        Vecf2d(self.0 * n, self.1 * n)
    }

    fn add(&self, other: Vecf2d) -> Self {
        Vecf2d(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: Vecf2d) -> Self {
        Vecf2d(self.0 - other.0, self.1 - other.1)
    }
}

impl Vec2d {
    fn s_mul(&self, n: usize) -> Self {
        Vec2d(n * self.0, n * self.1)
    }

    fn add(&self, other: Vec2d) -> Self {
        Vec2d(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mat2d(Vecf2d, Vecf2d);

impl Mat2d {
    fn transposed(&self) -> Self {
        Mat2d(Vecf2d(self.0 .0, self.1 .0), Vecf2d(self.0 .1, self.1 .1))
    }

    fn det(&self) -> Decimal {
        self.0 .0 * self.1 .1 - self.0 .1 * self.1 .0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ClawProblem {
    a: Vec2d,
    b: Vec2d,
    target: Vec2d,
}

impl ClawProblem {
    fn solve(&self) -> Option<usize> {
        let mut sols = Vec::new();
        for i in 0..=100 {
            for j in 0..=100 {
                if self.a.s_mul(i).add(self.b.s_mul(j)) == self.target {
                    sols.push((i, j));
                }
            }
        }
        let (best_i, best_j) = sols.into_iter().min_by_key(|(i, j)| 3 * i + j)?;
        println!(
            "{:?}*{:?} + {:?}*{:?} = {:?}  ",
            self.a, best_i, self.b, best_j, self.target
        );
        Some(3 * best_i + best_j)
    }

    fn solve_p2_math(&self) -> Option<Decimal> {
        let va = Vecf2d::from_vec2d(self.a);
        let vb = Vecf2d::from_vec2d(self.b);
        let mat = Mat2d(va, vb).transposed();
        let target = Vecf2d::from_vec2d(self.target);

        println!("det: {:?}", mat.det());

        if mat.det() != Decimal::from_u8(0).unwrap() {
            return self.solve_p2();
        };

        let v = if vb.s_mul(Decimal::from_u8(3).unwrap()).0 > va.0 {
            va
        } else {
            vb
        };

        let one_vector_only_solution = Vecf2d(target.0 / v.0, target.1 / v.1);

        if one_vector_only_solution.0.is_integer() && one_vector_only_solution.1.is_integer() {
            println!("found 1v solution");
            assert_eq!(v.s_mul(one_vector_only_solution.0), target);
            Some(one_vector_only_solution.0)
        } else {
            None
        }
    }

    fn solve_p2(&self) -> Option<Decimal> {
        let mut mat = Mat2d(Vecf2d::from_vec2d(self.a), Vecf2d::from_vec2d(self.b)).transposed();

        let mut target = Vecf2d::from_vec2d(self.target);

        println!("{:?} | {:?}\n", mat, target);

        let scalar_to_get_0_0_to_1 = mat.0 .0;

        mat.0 = mat.0.s_div(scalar_to_get_0_0_to_1);
        target.0 = target
            .0
            .checked_div(scalar_to_get_0_0_to_1)
            .expect("button has x = 0");

        println!(
            "{:?} | {:?}\n{:?} | {:?}\n",
            mat.0, target.0, mat.1, target.1
        );

        let coeff_to_get_1_0_to_0 = mat.1 .0;

        mat.1 = mat.1.sub(mat.0.s_mul(coeff_to_get_1_0_to_0));
        target.1 = target.1 - (target.0 * coeff_to_get_1_0_to_0);

        println!(
            "{:?} | {:?}\n{:?} | {:?}\n",
            mat.0, target.0, mat.1, target.1
        );

        let coeff_to_get_0_1_to_0 = mat.0 .1 / mat.1 .1;

        mat.0 = mat.0.sub(mat.1.s_mul(coeff_to_get_0_1_to_0));
        target.0 = target.0 + (target.1 * coeff_to_get_0_1_to_0);

        println!(
            "{:?} | {:?}\n{:?} | {:?}\n",
            mat.0, target.0, mat.1, target.1
        );

        let scalar_to_get_1_1_to_1 = mat.1 .1;

        mat.1 = mat.1.s_div(scalar_to_get_1_1_to_1);
        target.1 = target.1 / scalar_to_get_1_1_to_1;

        println!(
            "{:?} | {:?}\n{:?} | {:?}\n",
            mat.0, target.0, mat.1, target.1
        );

        if !(target.0.is_integer() && target.1.is_integer()) {
            None
        } else {
            let tokens = Decimal::from_u8(3).unwrap() * target.0 + target.1;
            println!("{:?}", tokens);
            Some(tokens)
        }
    }
}

fn parse_file(filename: &str) -> Result<Vec<ClawProblem>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let a_re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let problem_strings = content.lines().chunks(4);

    let problems = problem_strings
        .into_iter()
        .map(|chunk| {
            let chunk_string = chunk.collect::<String>();
            ClawProblem {
                a: a_re
                    .captures(&chunk_string)
                    .map(|n| {
                        Vec2d(
                            n.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            n.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        )
                    })
                    .unwrap(),
                b: b_re
                    .captures(&chunk_string)
                    .map(|n| {
                        Vec2d(
                            n.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            n.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        )
                    })
                    .unwrap(),
                target: prize_re
                    .captures(&chunk_string)
                    .map(|n| {
                        Vec2d(
                            n.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            n.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        )
                    })
                    .unwrap(),
            }
        })
        .collect::<Vec<ClawProblem>>();

    Ok(problems)
}

fn main() {
    let args = FilenameArg::parse();
    let problems = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&problems);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&problems);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(problems: &Vec<ClawProblem>) -> usize {
    let mut result = 0;
    for problem in problems.iter() {
        if let Some(solution) = problem.solve() {
            result += solution;
        }
    }
    result
}

fn part2(problems: &Vec<ClawProblem>) -> Decimal {
    let mut result = Decimal::from_u8(0).unwrap();
    for problem in problems
        .iter()
        .map(|&p| {
            let mut p2 = p;
            p2.target = p2.target.add(Vec2d(10_000_000_000_000, 10_000_000_000_000));
            p2
        })
        .collect::<Vec<ClawProblem>>()
    {
        println!("{:?}", problem);
        if let Some(solution) = problem.solve_p2_math() {
            result += solution;
        }
    }
    result
}
