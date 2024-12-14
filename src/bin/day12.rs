use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;
use std::{time, fs};

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
            // println!(
            //     "neighbors len: {:?}, set len: {:?}",
            //     neighbors.len(),
            //     set.len()
            // );
            neighbors = neighbors
                .into_iter()
                .flat_map(|(i, j)| self.neighbors_of(i, j, *c))
                .unique()
                .filter(|(i, j)| !set.contains(&(*i, *j)))
                .collect();
        }

        set
    }
    fn find_anti_region_of(
        &self,
        i: usize,
        j: usize,
        existing_region: &HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut set = HashSet::<(usize, usize)>::new();
        set.insert((i, j));

        let mut neighbors = self.neighbors_of_anti(i, j, existing_region);

        while !neighbors.is_empty() {
            set.extend(&neighbors);
            // println!(
            //     "neighbors len: {:?}, set len: {:?}",
            //     neighbors.len(),
            //     set.len()
            // );
            neighbors = neighbors
                .into_iter()
                .flat_map(|(i, j)| self.neighbors_of_anti(i, j, existing_region))
                .unique()
                .filter(|(i, j)| !set.contains(&(*i, *j)))
                .collect();
        }

        set
    }

    fn find_diag_anti_region_of(
        &self,
        i: usize,
        j: usize,
        existing_region: &HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut set = HashSet::<(usize, usize)>::new();
        set.insert((i, j));

        let mut neighbors = self.neighbors_including_diag_of(i, j, existing_region);

        while !neighbors.is_empty() {
            set.extend(&neighbors);
            // println!(
            //     "neighbors len: {:?}, set len: {:?}",
            //     neighbors.len(),
            //     set.len()
            // );
            neighbors = neighbors
                .into_iter()
                .flat_map(|(i, j)| self.neighbors_including_diag_of(i, j, existing_region))
                .unique()
                .filter(|(i, j)| !set.contains(&(*i, *j)))
                .collect();
        }

        set
    }
    fn neighbors_of(&self, i: usize, j: usize, c: char) -> Vec<(usize, usize)> {
        [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
            .into_iter()
            .filter(|(i, j)| self.get_2d(*i, *j) == Some(&c))
            .collect()
    }

    fn neighbors_of_anti(
        &self,
        i: usize,
        j: usize,
        existing_region: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
            .into_iter()
            .filter(|(i, j)| self.get_2d(*i, *j).is_some() && !existing_region.contains(&(*i, *j)))
            .collect()
    }

    fn neighbors_including_diag_of(
        &self,
        i: usize,
        j: usize,
        existing_region: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        [
            (i + 1, j),
            (i, j + 1),
            (i - 1, j),
            (i, j - 1),
            (i - 1, j - 1),
            (i + 1, j - 1),
            (i - 1, j + 1),
            (i + 1, j + 1),
        ]
        .into_iter()
        .filter(|(i, j)| self.get_2d(*i, *j).is_some() && !existing_region.contains(&(*i, *j)))
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

    fn get_number_of_sides(&self, region: &HashSet<(usize, usize)>) -> usize {
        let mut sides = 0;
        let region_edges = region;
        // .iter()
        // .filter(|(i, j)| self.get_perimeter_contribution(i, j) > 0);
        // let region_edges = region_edges.collect_vec();
        let (i, j) = get_top_left(region);
        let mut dir = Direction::Down;
        let (mut current_i, mut current_j) = (i, j);
        let mut visited = HashSet::<((usize, usize), Direction)>::new();

        while {
            let t = visited.insert(((current_i, current_j), dir));
            // if !t {
            //     println!("found ({:?},{:?}), {:?} in map", current_i, current_j, dir);
            // };
            t
        } {
            // thread::sleep(Duration::from_millis(100));
            // println!(
            //     "[{:?}] moving from ({:?}, {:?}), dir: {:?}",
            //     c, current_i, current_j, dir
            // );

            let going_forward = move_from((current_i, current_j), dir);
            let going_right = move_from((current_i, current_j), dir.rotate_right_90());
            let going_left = move_from((current_i, current_j), dir.rotate_left_90());
            let going_backward = move_from(
                (current_i, current_j),
                dir.rotate_left_90().rotate_left_90(),
            );

            if region_edges.contains(&&going_right)
                && !visited.contains(&(going_right, dir.rotate_right_90()))
            {
                dir = dir.rotate_right_90();
                (current_i, current_j) = going_right;
                sides += 1;
            } else if region_edges.contains(&&going_forward)
                && !visited.contains(&(going_forward, dir))
            {
                (current_i, current_j) = going_forward;
            } else if region_edges.contains(&&going_left)
                && !visited.contains(&(going_left, dir.rotate_left_90()))
            {
                dir = dir.rotate_left_90();
                (current_i, current_j) = going_left;
                sides += 1;
            } else if region_edges.contains(&&going_backward) {
                if visited.contains(&((current_i, current_j), dir.rotate_left_90())) {
                    sides += 1;
                    break;
                }
                dir = dir.rotate_right_90().rotate_right_90();
                sides += 2;
            } else {
                sides = 4;
                break;
            }
        }
        // println!("sides found for {:?}: {:?}", c, sides);
        sides
    }
}

fn move_from(p: (usize, usize), d: Direction) -> (usize, usize) {
    match d {
        Direction::Up => (p.0 - 1, p.1),
        Direction::Right => (p.0, p.1 + 1),
        Direction::Down => (p.0 + 1, p.1),
        Direction::Left => (p.0, p.1 - 1),
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

fn get_top_left(region: &HashSet<(usize, usize)>) -> (usize, usize) {
    *region
        .iter()
        .min_by(|&&(x1, y1), &&(x2, y2)| (x1, y1).cmp(&(x2, y2)))
        .unwrap()
}

fn region_is_inside(
    inside_region: &HashSet<(usize, usize)>,
    outside_region: &HashSet<(usize, usize)>,
) -> bool {
    inside_region.iter().all(|(i, j)| {
        let on_same_horizontal = outside_region.iter().filter(|(i2, _)| i == i2);
        let on_same_vertical = outside_region.iter().filter(|(_, j2)| j == j2);

        on_same_vertical.clone().find(|(i2, _)| i2 > i).is_some()
            && on_same_horizontal.clone().find(|(_, j2)| j2 > j).is_some()
            && on_same_vertical.clone().find(|(i2, _)| i2 < i).is_some()
            && on_same_horizontal.clone().find(|(_, j2)| j2 < j).is_some()
    })
}

fn is_fully_contained(
    inside_region: &HashSet<(usize, usize)>,
    outside_region: &HashSet<(usize, usize)>,
    grid: &CropGrid,
) -> bool {
    let (i, j) = inside_region.iter().next().unwrap();
    let greedy_region = grid.find_diag_anti_region_of(*i, *j, outside_region);
    println!(
        "greedy region found for this anti region: length {:?}",
        greedy_region.len()
    );
    region_is_inside(&greedy_region, outside_region)
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

    let outside_sides_numbers = regions
        .iter()
        .map(|r| (r, grid.get_number_of_sides(r)))
        .collect_vec();

    let mut total_sides_numbers = Vec::<(&HashSet<(usize, usize)>, usize)>::new();

    for (outside_region, n) in outside_sides_numbers.iter() {
        let (first_i, first_j) = outside_region.iter().next().unwrap();
        let c = grid.get_2d(*first_i, *first_j).unwrap();
        let inside_tiles = regions
            .iter()
            .filter(|inside_region| {
                inside_region != outside_region && region_is_inside(inside_region, outside_region)
            })
            .flat_map(|r| r.iter().collect_vec())
            .collect::<Vec<&(usize, usize)>>();

        if !inside_tiles.is_empty() {
            println!("{:?} tiles found inside {c}", inside_tiles.len());
        }
        let mut anti_regions = Vec::<HashSet<(usize, usize)>>::new();
        for (i, j) in inside_tiles {
            // let diff_c = grid.get_2d(*i, *j).unwrap();
            // println!("checking inside tile ({i}, {j}) [{diff_c}]");
            if !anti_regions.iter().any(|v| v.contains(&(*i, *j))) {
                let anti_region = grid.find_anti_region_of(*i, *j, &outside_region);
                println!(
                    "found anti-region of size: {:?} inside region {c} with {:?} sides",
                    anti_region.len(),
                    grid.get_number_of_sides(&anti_region)
                );
                if !region_is_inside(&anti_region, outside_region) {
                    println!("but anti region extends past boundaries of outside region (maybe the outside region is not closed), so this anti region is invalid");
                    continue;
                }
                if !is_fully_contained(&anti_region, outside_region, &grid) {
                    println!("region was not fully contained, so the inside edges of the outside region are probably already counted.");
                    continue;
                }
                assert!(anti_regions
                    .iter()
                    .all(|existing_region| anti_region.is_disjoint(existing_region)));
                anti_regions.push(anti_region);
            }
        }
        total_sides_numbers.push((
            outside_region,
            n + anti_regions
                .into_iter()
                .map(|r| grid.get_number_of_sides(&r))
                .sum::<usize>(),
        ));
    }

    total_sides_numbers.iter().for_each(|(r, n)| {
        let (i, j) = r.iter().next().unwrap();

        let c = grid.get_2d(*i, *j);
        println!(
            "{:?}: {:?} sides * {:?} len = {:?}",
            c,
            n,
            r.len(),
            r.len() * n
        );
    });
    total_sides_numbers.iter().map(|(r, n)| r.len() * n).sum()
}
