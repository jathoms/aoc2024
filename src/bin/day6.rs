use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TileType {
    Obst,
    Player,
    Free,
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
}

impl TileType {
    fn from_char(c: char) -> Option<TileType> {
        match c {
            '#' => Some(TileType::Obst),
            '.' => Some(TileType::Free),
            '^' => Some(TileType::Player),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    tiles: HashMap<Point, TileType>,
}

impl Grid {
    fn from_vecs(grid_vec: Vec<Vec<char>>) -> Grid {
        let mut tiles = HashMap::new();
        for (i, line) in grid_vec.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                tiles.insert(
                    Point(j, i),
                    TileType::from_char(*c).expect("Found invalid grid tile"),
                );
            }
        }
        Grid { tiles: tiles }
    }
    fn with_obst_at(&self, pt: Point) -> Grid {
        let mut new = self.clone();
        new.tiles.insert(pt, TileType::Obst);
        new
    }

    fn find_player(&self) -> Point {
        *self
            .tiles
            .iter()
            .find(|(_, tile)| *tile == &TileType::Player)
            .expect("No player found")
            .0
    }

    fn move_from(p: &Point, d: Direction) -> Point {
        match d {
            Direction::Up => Point(p.0, p.1 - 1),
            Direction::Right => Point(p.0 + 1, p.1),
            Direction::Down => Point(p.0, p.1 + 1),
            Direction::Left => Point(p.0 - 1, p.1),
        }
    }
}

fn parse_file(filename: &str) -> Result<Vec<Vec<char>>, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let char_grid = content
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    Ok(char_grid)
}

fn main() {
    let args = FilenameArg::parse();
    let grid_vec = parse_file(&args.input).expect("Failed to parse file.");

    let grid = Grid::from_vecs(grid_vec);
    println!("{:?}", grid);

    let mut start = time::Instant::now();
    let part_1_result = part1(&grid);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&grid);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(grid: &Grid) -> usize {
    let mut result = 1;
    let mut found_already = HashSet::new();
    let mut pos = grid.find_player();
    let mut dir = Direction::Up;
    while let Some((tile, new_pos)) = {
        let new_pos = Grid::move_from(&pos, dir);
        grid.tiles.get(&new_pos).map(|tt| (tt, new_pos))
    } {
        println!("{:?}", pos);
        match tile {
            TileType::Obst => {
                dir = dir.rotate_right_90();
                println!("rotated to {:?}", dir)
            }
            _ => {
                if !found_already.contains(&pos) {
                    result += 1;
                    found_already.insert(pos);
                };
                pos = new_pos;
                println!("moved to {:?}", new_pos)
            }
        };
    }
    result
}

fn part2(grid: &Grid) -> usize {
    let mut result = 0;
    let starting_pos = grid.find_player();
    for (pt, tile_type) in grid.tiles.iter() {
        match tile_type {
            TileType::Free => (),
            _ => continue,
        };

        let new_grid = grid.with_obst_at(*pt);

        let mut found_already = HashSet::<(Point, Direction)>::new();
        let mut pos = starting_pos;
        let mut dir = Direction::Up;

        while let Some((tile, new_pos)) = {
            let new_pos = Grid::move_from(&pos, dir);
            new_grid.tiles.get(&new_pos).map(|tt| (tt, new_pos))
        } {
            // println!("{:?}", pos);
            match tile {
                TileType::Obst => {
                    dir = dir.rotate_right_90();
                    // println!("rotated to {:?}", dir)
                }
                _ => {
                    if found_already.contains(&(pos, dir)) {
                        //we're looping
                        result += 1;
                        break;
                    }else {
                        found_already.insert((pos, dir));          
                    };
                    pos = new_pos;
                    // println!("moved to {:?}", new_pos)
                }
            };
        };
        //escaped
    }
    result
}
