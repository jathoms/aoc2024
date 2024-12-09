use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Diff(isize, isize);

impl Point {
    fn diff(&self, other: &Point) -> Diff {
        Diff(
            other.0 as isize - self.0 as isize,
            other.1 as isize - self.1 as isize,
        )
    }
    fn add_diff(&self, diff: Diff, upper_limit: &Point, rep: isize) -> Option<Self> {
        let i = self.0 as isize + ((2 + rep) * diff.0);
        let j = self.1 as isize + ((2 + rep) * diff.1);

        println!("(i: {:?}, j: {:?}): ", i, j);

        let added = Point::new(i, j)?;

        if added.fully_lt(&upper_limit) {
            println!("valid.");
            Some(added)
        } else {
            println!("outside");
            None
        }
    }
    fn new(
        i: impl std::convert::TryInto<usize>,
        j: impl std::convert::TryInto<usize>,
    ) -> Option<Self> {
        if let (Ok(i), Ok(j)) = (i.try_into(), j.try_into()) {
            Some(Self(i, j))
        } else {
            None
        }
    }
    fn fully_lt(&self, other: &Point) -> bool {
        self.0 < other.0 && self.1 < other.1
    }
    fn get_anodes_for(
        &self,
        p2: &Point,
        upper_limit: &Point,
    ) -> impl Iterator<Item = Option<Point>> {
        let diff = self.diff(p2);
        let rev_diff = p2.diff(&self);
        println!("self: {:?}, p2: {:?}", self, p2);
        [
            self.add_diff(diff, upper_limit, 0),
            p2.add_diff(rev_diff, upper_limit, 0),
        ]
        .into_iter()
    }
    fn get_repeating_anodes_for(&self, p2: &Point, upper_limit: &Point) -> Vec<Point> {
        let diff = self.diff(p2);
        let rev_diff = p2.diff(&self);
        println!("self: {:?}, p2: {:?}", self, p2);

        let mut anodes = Vec::<Point>::new();

        anodes.extend_from_slice(&[*self, *p2]);

        let mut rep = 0;

        while let Some(p) = self.add_diff(diff, upper_limit, rep) {
            anodes.push(p);
            rep += 1;
        }
        rep = 0;
        while let Some(p) = p2.add_diff(rev_diff, upper_limit, rep) {
            anodes.push(p);
            rep += 1;
        }
        anodes
    }
}

#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
enum Alphanum {
    Char(char),
}

impl Alphanum {
    fn new(c: char) -> Option<Alphanum> {
        if c.is_ascii_alphanumeric() {
            Some(Alphanum::Char(c))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum TileType {
    Freq(Alphanum),
    Free,
}

impl TileType {
    fn from_char(c: char) -> TileType {
        if c == '.' {
            return TileType::Free;
        } else {
            return TileType::Freq(Alphanum::new(c).expect("Found invalid character"));
        }
    }
}

struct AntennaGrid {
    tiles: HashMap<TileType, Vec<Point>>,
    upper_limit: Point,
}

impl AntennaGrid {
    fn get_unique_anodes(&self) -> HashSet<Point> {
        let mut anodes = HashSet::<Point>::new();
        for tiles_vec in self.tiles.values() {
            for comb in tiles_vec.iter().combinations(2) {
                println!("comb: {:?}", comb);
                if let [&p1, &p2] = comb.as_slice() {
                    for anode in p1.get_anodes_for(&p2, &self.upper_limit) {
                        if let Some(pt) = anode {
                            anodes.insert(pt);
                        }
                    }
                }
            }
        }
        anodes
    }
    fn get_unique_anodes_pt2(&self) -> HashSet<Point> {
        let mut anodes = HashSet::<Point>::new();
        for tiles_vec in self.tiles.values() {
            for comb in tiles_vec.iter().combinations(2) {
                println!("comb: {:?}", comb);
                if let [&p1, &p2] = comb.as_slice() {
                    for anode in p1
                        .get_repeating_anodes_for(&p2, &self.upper_limit)
                        .into_iter()
                    {
                        anodes.insert(anode);
                    }
                }
            }
        }
        anodes
    }
}

fn parse_file(filename: &str) -> Result<AntennaGrid, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());
    println!("{:?}", content.lines().collect_vec());
    let content_vec = content.lines().collect_vec();
    let upper_limit = Point(
        content_vec.len(),
        content_vec.get(0).expect("empty file").len(),
    );

    let mut tiles = HashMap::<TileType, Vec<Point>>::new();
    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let tt = TileType::from_char(c);
            if tt == TileType::Free {
                continue;
            }
            let current_pts = tiles.get_mut(&tt);
            if let Some(tt_vec) = current_pts {
                tt_vec.push(Point(i, j));
            } else {
                tiles.insert(tt, vec![Point(i, j)]);
            }
        }
    }
    println!("upper lim: {:?}", upper_limit);
    println!("tiles: {:?}", tiles);
    Ok(AntennaGrid {
        tiles: tiles,
        upper_limit: upper_limit,
    })
}

fn main() {
    let args = FilenameArg::parse();
    let grid = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&grid);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&grid);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(grid: &AntennaGrid) -> usize {
    let anodes = grid.get_unique_anodes();
    println!("{:?}", anodes);
    anodes.len()
}

fn part2(grid: &AntennaGrid) -> usize {
    let anodes = grid.get_unique_anodes_pt2();
    println!("pt2 anodes: {:?}", anodes);
    anodes.len()
}
