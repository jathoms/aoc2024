use clap::Parser;
use itertools::{concat, Itertools};
use rayon::iter::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::sync::Arc;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, Clone)]
struct StoneLine {
    stones: Vec<Stone>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct StoneNode {
    stone: Stone,
    children: Vec<StoneNode>,
}

impl StoneNode {
    fn populate_until_cycles_or_limit(
        mut self,
        map: Arc<RefCell<HashMap<Stone, StoneNode>>>,
        depth: usize,
        limit: usize,
    ) -> Vec<StoneNode> {
        let children = self.stone.do_iteration();
        if depth >= limit {
            return self.children;
        }
        for child in children.into_iter() {
            if let Some(existing_stone) = map.borrow().get(&child) {
                self.children.push(existing_stone.clone());
            } else {
                let new_node = StoneNode {
                    stone: child,
                    children: Vec::new(),
                };
                self.children
                    .extend(new_node.populate_until_cycles_or_limit(map.clone(), depth + 1, limit));
            }
        }
        map.clone().borrow_mut().insert(self.stone, self.clone());
        self.children
    }
}

impl StoneLine {
    fn do_iteration(&mut self) {
        self.stones = self
            .stones
            .iter()
            .flat_map(|s| s.do_iteration())
            .collect::<Vec<Stone>>();
    }

    fn do_iteration_pt2(&mut self) {
        let mut hm = HashMap::new();
        self.stones = self
            .stones
            .iter()
            .flat_map(|s| s.do_iteration_pt2(&mut hm))
            .collect::<Vec<Stone>>();
    }

    fn to_string(&self) -> String {
        self.stones
            .iter()
            .map(|stone| stone.n.to_string())
            .join(" ")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Stone {
    n: usize,
}

impl Stone {
    fn do_iteration(self) -> Vec<Self> {
        match self.n {
            0 => vec![Stone { n: 1 }],
            _ => match self.n.to_string().len() % 2 {
                0 => {
                    let s = self.n.to_string();
                    let (x, y) = s.split_at(s.len() / 2);
                    vec![
                        Stone {
                            n: x.parse::<usize>().unwrap(),
                        },
                        Stone {
                            n: y.parse::<usize>().unwrap(),
                        },
                    ]
                }
                1 => vec![Stone { n: self.n * 2024 }],
                _ => panic!("mod 2 returned value not 0 or 1????"),
            },
        }
    }

    fn do_iteration_pt2<'a>(self, hashmap: &'a mut HashMap<usize, Vec<Stone>>) -> Vec<Self> {
        hashmap
            .entry(self.n)
            .or_insert_with(|| match self.n {
                0 => vec![Stone { n: 1 }],
                _ => match self.n.to_string().len() % 2 {
                    0 => {
                        let s = self.n.to_string();
                        let (x, y) = s.split_at(s.len() / 2);
                        vec![
                            Stone {
                                n: x.parse::<usize>().unwrap(),
                            },
                            Stone {
                                n: y.parse::<usize>().unwrap(),
                            },
                        ]
                    }
                    1 => vec![Stone { n: self.n * 2024 }],
                    _ => panic!("mod 2 returned value not 0 or 1????"),
                },
            })
            .to_vec()
    }
}

fn parse_file(filename: &str) -> Result<StoneLine, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());
    Ok(StoneLine {
        stones: content
            .split_whitespace()
            .map(|s| {
                s.parse::<usize>()
                    .expect("non-valid usize string found in input")
            })
            .map(|n| Stone { n: n })
            .collect::<Vec<Stone>>(),
    })
}

fn main() {
    let args = FilenameArg::parse();
    let stones = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(stones.clone());
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(stones.clone());
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(mut stones: StoneLine) -> usize {
    println!("{:?}", stones.to_string());
    for _ in 0..25 {
        stones.do_iteration();
        // println!("{:?}", stones.to_string());
    }
    stones.stones.len()
}

fn length_is_even(n: &usize) -> bool {
    ((*n as f64).log10() as usize) % 2 == 1
}

fn split_even_length(n: usize) -> (usize, usize) {
    let n_digits = match n {
        0..9 => 1,
        _ => (n as f64).log10() as usize + 1,
    };
    let right_digits = n_digits / 2;
    let divisor = 10_usize.pow(right_digits as u32);

    (n / divisor, n % divisor)
}

fn blink(n: usize, depth: usize, limit: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth >= limit {
        return 1;
    }

    if let Some(&result) = map.get(&(n, depth)) {
        return result;
    }

    let result = match n {
        0 => blink(1, depth + 1, limit, map),
        _n if length_is_even(&_n) => {
            let (left, right) = split_even_length(_n);
            blink(left, depth + 1, limit, map) + blink(right, depth + 1, limit, map)
        }
        _ => blink(n * 2024, depth + 1, limit, map),
    };

    map.insert((n, depth), result);

    result
}

fn part2(stones: StoneLine) -> usize {
    stones
        .stones
        .into_par_iter()
        .map(|n| {
            let mut map = HashMap::new();
            blink(n.n, 0, 75, &mut map)
        })
        .sum()
}
