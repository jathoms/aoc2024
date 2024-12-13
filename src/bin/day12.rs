use clap::Parser;
use itertools::{concat, Itertools};
use rayon::iter::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{self, Duration};
use std::{fs, thread};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

fn parse_file(filename: &str) -> Result<Vec<Vec<char>>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    Ok(content.lines().map(|line| line.chars().collect()).collect())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right_90(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn rotate_left_90(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}
struct Fence {
    pieces: Vec<FencePiece>,
}

struct FencePiece {
    pos_0: usize,
    pos_1: usize,
    ori: FenceOrientation,
    type_0: char,
    type_1: char,
}

enum FenceOrientation {
    V,
    H,
}

struct CropGrid {
    tiles: Vec<Vec<char>>,
}

impl CropGrid {
    fn get_2d(&self, i: usize, j: usize) -> Option<&char> {
        self.tiles.get(i)?.get(j)
    }

    fn find_region_of(&self, i: usize, j: usize) -> HashSet<(usize, usize)> {
        let mut set = HashSet::<(usize, usize)>::new();

        let c = self
            .get_2d(i, j)
            .expect("tried to get region around a point outside the grid");
        println!("finding region around ({:?}, {:?}) [{:?}]", i, j, c);

        set.insert((i, j));

        let mut neighbors = self.neighbors_of(i, j, *c);

        while !neighbors.is_empty() {
            set.extend(&neighbors);
            println!(
                "neighbors len: {:?}, set len: {:?}",
                neighbors.len(),
                set.len()
            );
            neighbors = neighbors
                .into_iter()
                .flat_map(|(i, j)| self.neighbors_of(i, j, *c))
                .unique()
                .filter(|(i, j)| !set.contains(&(*i, *j)))
                .collect();
            // println!("{:?}", neighbors.iter().map(|(i, j)| set.contains(&(*i, *j))).collect::<Vec<bool>>());
            // thread::sleep(Duration::from_secs(1));
        }

        set
    }

    fn neighbors_of(&self, i: usize, j: usize, c: char) -> Vec<(usize, usize)> {
        [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
            .into_iter()
            .filter(|(i, j)| self.get_2d(*i, *j) == Some(&c))
            .collect()
    }

    fn get_perimeter_length(&self, region: &HashSet<(usize, usize)>) -> usize {
        let (i, j) = region.iter().next().expect("passed empty region");
        let c = self.get_2d(*i, *j).unwrap();
        let mut result = 0;
        for (i, j) in region.iter() {
            result += 4 - self.neighbors_of(*i, *j, *c).len();
        }
        result
    }

    fn get_perimeter_contribution(&self, i: &usize, j: &usize) -> usize {
        let c = self.get_2d(*i, *j).unwrap();
        4 - self.neighbors_of(*i, *j, *c).len()
    }

    fn get_number_of_sides(&self, region: &HashSet<(usize, usize)>) -> usize {
        let mut sides = 1;
        let region_edges = region
            .iter()
            .filter(|(i, j)| self.get_perimeter_contribution(i, j) > 0);
        let region_edges = region_edges
            .filter(|(i, j)| !coords_almost_inside(i, j, region))
            .collect_vec();

        println!(
            "region edges {:?}",
            region
                .iter()
                .filter(|(i, j)| !coords_almost_inside(i, j, region))
                .collect_vec()
        );
        let (i, j) = get_top_left(region);
        let mut dir = Direction::Down;

        let (mut current_i, mut current_j) = (i, j);

        let mut visited = HashSet::<(usize, usize)>::new();

        while visited.len() != region_edges.len() {
            println!(
                "visited len: {:?}, edges len: {:?}",
                visited.len(),
                region_edges.len()
            );
            visited.insert((current_i, current_j));
            thread::sleep(Duration::from_millis(100));
            println!(
                "moving from ({:?}, {:?}), dir: {:?}",
                current_i, current_j, dir
            );
            let (next_i, next_j) = move_from((current_i, current_j), dir);
            if !region_edges.contains(&&(next_i, next_j)) {
                let going_right = move_from((current_i, current_j), dir.rotate_right_90());
                let going_left = move_from((current_i, current_j), dir.rotate_left_90());
                if region_edges.contains(&&going_left) && region_edges.contains(&&going_right) {
                    if visited.contains(&going_left) {
                        dir = dir.rotate_right_90();
                        sides += 1;
                    } else if visited.contains(&&going_right) {
                        dir = dir.rotate_left_90();
                        sides += 1;
                    } else {
                        panic!(
                            "found both places to go from ({:?}, {:?}), dir: {:?}",
                            next_i, next_j, dir
                        )
                    };
                    continue;
                }
                if region_edges.contains(&&going_left) {
                    dir = dir.rotate_left_90();
                    sides += 1;
                    continue;
                } else if region_edges.contains(&&going_right) {
                    {
                        dir = dir.rotate_left_90();
                        sides += 1;
                        continue;
                    };
                } else {
                    dir = dir.rotate_left_90().rotate_left_90();
                    sides += 2;
                }
            } else {
                (current_i, current_j) = (next_i, next_j);
            }
        }
        println!("sides found: {:?}", sides);
        sides
    }
}

fn move_from(p: (usize, usize), d: Direction) -> (usize, usize) {
    match d {
        Direction::Up => (p.0, p.1 - 1),
        Direction::Right => (p.0 + 1, p.1),
        Direction::Down => (p.0, p.1 + 1),
        Direction::Left => (p.0 - 1, p.1),
    }
}
fn main() {
    let args = FilenameArg::parse();
    let chars = parse_file(&args.input).expect("Failed to parse file.");

    let grid = CropGrid { tiles: chars };

    let mut start = time::Instant::now();
    let part_1_result = part1(&grid);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&grid);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(grid: &CropGrid) -> usize {
    let mut regions = Vec::<HashSet<(usize, usize)>>::new();
    for (i, line) in grid.tiles.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if !regions.iter().any(|v| v.contains(&(i, j))) {
                let region = grid.find_region_of(i, j);
                println!("found region of size: {:?}", region.len());
                assert!(regions
                    .iter()
                    .all(|existing_region| region.is_disjoint(existing_region)));
                regions.push(region);
            }
        }
    }
    regions
        .iter()
        .map(|r| grid.get_perimeter_length(&r) * r.len())
        .sum()
}

fn part1_(grid: &CropGrid) -> usize {
    let mut fence_pieces = Vec::<FencePiece>::new();
    let mut fences = Vec::<Fence>::new();
    for (i, line) in grid.tiles.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            let right_tile = grid.get_2d(i, j + 1);
            let down_tile = grid.get_2d(i + 1, j);
            if Some(tile) != right_tile {
                fence_pieces.push(FencePiece {
                    pos_0: j,
                    pos_1: j + 1,
                    ori: FenceOrientation::V,
                    type_0: *tile,
                    type_1: *right_tile.unwrap(),
                });
            } else if Some(tile) != down_tile {
                fence_pieces.push(FencePiece {
                    pos_0: i,
                    pos_1: i + 1,
                    ori: FenceOrientation::H,
                    type_0: *tile,
                    type_1: *down_tile.unwrap(),
                });
            }
        }
    }
    1
}
fn get_top_left(region: &HashSet<(usize, usize)>) -> (usize, usize) {
    *region
        .iter()
        .min_by(|&&(x1, y1), &&(x2, y2)| (x1, y1).cmp(&(x2, y2)))
        .unwrap()
}

fn part2(grid: &CropGrid) -> usize {
    let mut regions = Vec::<HashSet<(usize, usize)>>::new();
    for (i, line) in grid.tiles.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if !regions.iter().any(|v| v.contains(&(i, j))) {
                let region = grid.find_region_of(i, j);
                println!("found region of size: {:?}", region.len());
                assert!(regions
                    .iter()
                    .all(|existing_region| region.is_disjoint(existing_region)));
                regions.push(region);
            }
        }
    }
    println!("finding sides...");

    let mut sides_numbers = regions
        .iter()
        .map(|r| (r, grid.get_number_of_sides(r)))
        .collect_vec();

    let s2 = sides_numbers.clone();
    for (region, mut n) in sides_numbers.iter_mut() {
        for (inside_region, n2) in s2.iter() {
            if region_is_inside(inside_region, region) {
                n += n2;
            }
        }
    }
    sides_numbers.iter().map(|(r, n)| r.len() * n).sum()
}

fn coords_inside(i: &usize, j: &usize, outside_region: &HashSet<(usize, usize)>) -> bool {
    region_is_inside(&HashSet::from([(*i, *j)]), outside_region)
}

fn coords_almost_inside(i: &usize, j: &usize, outside_region: &HashSet<(usize, usize)>) -> bool {
    region_almost_inside(&HashSet::from([(*i, *j)]), outside_region)
}
fn region_is_inside(
    inside_region: &HashSet<(usize, usize)>,
    outside_region: &HashSet<(usize, usize)>,
) -> bool {
    inside_region.iter().all(|(i, j)| {
        let on_same_horizontal = outside_region.iter().filter(|(i2, _)| i == i2);
        let on_same_vertical = outside_region.iter().filter(|(_, j2)| j == j2);

        on_same_vertical
            .clone()
            .find_or_first(|(i2, _)| i2 > i)
            .is_some()
            && on_same_horizontal
                .clone()
                .find_or_first(|(_, j2)| j2 > j)
                .is_some()
            && on_same_vertical.find_or_first(|(i2, _)| i2 < i).is_some()
            && on_same_horizontal.find_or_first(|(_, j2)| j2 < j).is_some()
    })
}

fn region_almost_inside(
    inside_region: &HashSet<(usize, usize)>,
    outside_region: &HashSet<(usize, usize)>,
) -> bool {
    inside_region.iter().all(|(i, j)| {
        let on_same_horizontal = outside_region.iter().filter(|(i2, _)| i == i2);
        let on_same_vertical = outside_region.iter().filter(|(_, j2)| j == j2);

        on_same_vertical
            .clone()
            .find_or_first(|(i2, _)| i2 >= i)
            .is_some()
            && on_same_horizontal
                .clone()
                .find_or_first(|(_, j2)| j2 >= j)
                .is_some()
            && on_same_vertical.find_or_first(|(i2, _)| i2 <= i).is_some()
            && on_same_horizontal.find_or_first(|(_, j2)| j2 <= j).is_some()
    })
}