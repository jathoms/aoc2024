use cgmath::Vector2;
use clap::Parser;
use itertools::Itertools;
use std::{collections::HashMap, fs, time};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}
#[derive(Debug, Clone)]
struct Warehouse {
    tiles: HashMap<Vector2<i32>, WarehouseTile>,
    robot_position: Vector2<i32>,
}
impl Warehouse {
    fn from_string(s: &String) -> Self {
        let mut map = HashMap::<Vector2<i32>, WarehouseTile>::new();
        let mut robot_position = None;
        for (y, line) in s.lines().enumerate() {
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = WarehouseTile::from_char(c) {
                    let pos = Vector2::new(x as i32, y as i32);
                    if tile == WarehouseTile::Robot {
                        robot_position = Some(pos);
                    }
                    map.insert(pos, tile);
                }
            }
        }
        if let Some(pos) = robot_position {
            Self {
                tiles: map,
                robot_position: pos,
            }
        } else {
            panic!("no robot found");
        }
    }

    fn to_string(&self) -> String {
        let max_x = self.tiles.keys().max_by_key(|v| v.x).unwrap().x;
        let max_y = self.tiles.keys().max_by_key(|v| v.y).unwrap().y;

        let mut s = String::new();
        for i in 0..=max_y {
            for j in 0..=max_x {
                s.push(
                    self.tiles
                        .get(&Vector2::new(j, i))
                        .expect(format!("couldn't file tile at x={}, y={}", j, i).as_str())
                        .to_char(),
                );
            }
            s.push('\n');
        }
        s
    }

    fn do_move(&mut self, dir: &Direction) {
        let new_robot_pos = self.robot_position + dir.to_vec2();
        let new_robot_tile = self.tiles.get(&new_robot_pos);

        // println!(
        //     "movement vector: {:?}, takes our robot ({:?}) to {:?}",
        //     dir.to_vec2(),
        //     self.robot_position,
        //     self.robot_position + dir.to_vec2()
        // );
        // println!(
        //     "found next to robot: {:?} at {:?}",
        //     new_robot_tile, new_robot_pos
        // );

        match new_robot_tile {
            Some(WarehouseTile::Free) => {
                self.tiles.insert(self.robot_position, WarehouseTile::Free);
                self.tiles.insert(new_robot_pos, WarehouseTile::Robot);
                self.robot_position = new_robot_pos;
            }
            Some(WarehouseTile::Box) => {
                if self.move_box(new_robot_pos, dir) {
                    self.tiles.insert(self.robot_position, WarehouseTile::Free);
                    self.tiles.insert(new_robot_pos, WarehouseTile::Robot);
                    self.robot_position = new_robot_pos;
                };
            }
            _ => (),
        };
    }

    fn move_box(&mut self, from: Vector2<i32>, dir: &Direction) -> bool {
        let next_pos = from + dir.to_vec2();
        let next_tile = self.tiles.get(&next_pos);

        match next_tile {
            Some(WarehouseTile::Free) => {
                self.tiles.insert(next_pos, WarehouseTile::Box);
                true
            }
            Some(WarehouseTile::Box) => self.move_box(next_pos, dir),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseTile {
    Wall,
    Box,
    Robot,
    Free,
}

impl WarehouseTile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Wall),
            'O' => Some(Self::Box),
            '@' => Some(Self::Robot),
            '.' => Some(Self::Free),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
            Self::Free => '.',
        }
    }
}

#[derive(Debug, Clone)]
struct WarehousePart2 {
    tiles: HashMap<Vector2<i32>, WarehouseTilePart2>,
    robot_position: Vector2<i32>,
}

impl WarehousePart2 {
    fn from_warehouse(warehouse: &Warehouse) -> Self {
        let s = warehouse.to_string();

        let pt2_string = s
            .chars()
            .flat_map(|c| match c {
                '#' => "##".chars(),
                '.' => "..".chars(),
                '@' => "@.".chars(),
                'O' => "[]".chars(),
                '\n' => "\n".chars(),
                _ => "".chars(),
            })
            .collect::<String>();

        WarehousePart2::from_string(pt2_string)
    }
    fn from_string(s: String) -> Self {
        let mut map = HashMap::<Vector2<i32>, WarehouseTilePart2>::new();
        let mut robot_position = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = WarehouseTilePart2::from_char(c) {
                    let pos = Vector2::new(x as i32, y as i32);
                    if tile == WarehouseTilePart2::Robot {
                        robot_position = Some(pos);
                    }
                    map.insert(pos, tile);
                }
            }
        }
        if let Some(pos) = robot_position {
            Self {
                tiles: map,
                robot_position: pos,
            }
        } else {
            panic!("no robot found");
        }
    }
    fn to_string(&self) -> String {
        let max_x = self.tiles.keys().max_by_key(|v| v.x).unwrap().x;
        let max_y = self.tiles.keys().max_by_key(|v| v.y).unwrap().y;

        let mut s = String::new();
        for i in 0..=max_y {
            for j in 0..=max_x {
                s.push(
                    self.tiles
                        .get(&Vector2::new(j, i))
                        .expect(format!("couldn't file tile at x={}, y={}", j, i).as_str())
                        .to_char(),
                );
            }
            s.push('\n');
        }
        s
    }
    fn do_move(&mut self, dir: &Direction) {
        let new_robot_pos = self.robot_position + dir.to_vec2();
        let new_robot_tile = self.tiles.get(&new_robot_pos);

        match new_robot_tile {
            Some(WarehouseTilePart2::Free) => {
                self.tiles
                    .insert(self.robot_position, WarehouseTilePart2::Free);
                self.tiles.insert(new_robot_pos, WarehouseTilePart2::Robot);
                self.robot_position = new_robot_pos;
            }
            Some(WarehouseTilePart2::LeftBox) if dir == &Direction::Right => {
                if self.can_move(new_robot_pos, dir) {
                    self.move_box(new_robot_pos, dir);
                    self.tiles
                        .insert(self.robot_position, WarehouseTilePart2::Free);
                    self.tiles.insert(new_robot_pos, WarehouseTilePart2::Robot);
                    self.robot_position = new_robot_pos;
                };
            }
            Some(WarehouseTilePart2::RightBox) if dir == &Direction::Left => {
                if self.move_box(new_robot_pos, dir) {
                    self.tiles
                        .insert(self.robot_position, WarehouseTilePart2::Free);
                    self.tiles.insert(new_robot_pos, WarehouseTilePart2::Robot);
                    self.robot_position = new_robot_pos;
                };
            }
            Some(WarehouseTilePart2::LeftBox) | Some(WarehouseTilePart2::RightBox) => {
                let other_side_offset = if new_robot_tile == Some(&WarehouseTilePart2::LeftBox) {
                    Direction::Right.to_vec2()
                } else {
                    Direction::Left.to_vec2()
                };
                let other_side_of_box_pos = new_robot_pos + other_side_offset;
                if self.can_move(new_robot_pos, dir) && self.can_move(other_side_of_box_pos, dir) {
                    self.naive_move(new_robot_pos, dir);
                    self.naive_move(other_side_of_box_pos, dir);
                    self.tiles
                        .insert(self.robot_position, WarehouseTilePart2::Free);
                    self.tiles.insert(new_robot_pos, WarehouseTilePart2::Robot);

                    self.robot_position = new_robot_pos;
                };
            }
            _ => (),
        };
    }

    fn move_box(&mut self, from: Vector2<i32>, dir: &Direction) -> bool {
        let next_pos = from + dir.to_vec2();
        let next_tile = self.tiles.get(&next_pos);
        let this_tile = self.tiles.get(&from).unwrap().clone();

        let this_box_other_side_offset = if this_tile == WarehouseTilePart2::LeftBox {
            Direction::Right.to_vec2()
        } else {
            Direction::Left.to_vec2()
        };
        let this_box_other_tile = if this_tile == WarehouseTilePart2::LeftBox {
            WarehouseTilePart2::RightBox
        } else {
            WarehouseTilePart2::LeftBox
        };

        let is_horizontal_move = dir == &Direction::Right || dir == &Direction::Left;

        let other_box_other_side_offset = if next_tile == Some(&WarehouseTilePart2::LeftBox) {
            Direction::Right.to_vec2()
        } else {
            Direction::Left.to_vec2()
        };

        let other_side_of_other_box_pos = next_pos + other_box_other_side_offset;

        let other_box_tile = if next_tile == Some(&WarehouseTilePart2::LeftBox) {
            WarehouseTilePart2::RightBox
        } else {
            WarehouseTilePart2::LeftBox
        };

        let beside_from = from + this_box_other_side_offset;

        assert!(
            this_tile == WarehouseTilePart2::LeftBox || this_tile == WarehouseTilePart2::RightBox
        );

        match next_tile {
            Some(WarehouseTilePart2::Free) => {
                self.tiles.insert(from, WarehouseTilePart2::Free);
                self.tiles.insert(beside_from, WarehouseTilePart2::Free);
                self.tiles.insert(next_pos, this_tile);
                self.tiles
                    .insert(next_pos + this_box_other_side_offset, this_box_other_tile);
                true
            }
            Some(WarehouseTilePart2::LeftBox) | Some(WarehouseTilePart2::RightBox) => {
                if self.can_move(next_pos, dir)
                    && (!is_horizontal_move || self.can_move(other_side_of_other_box_pos, dir))
                {
                    self.move_box(next_pos, dir);

                    if !is_horizontal_move
                        && other_box_other_side_offset != this_box_other_side_offset
                    {
                        self.move_box(next_pos + other_box_other_side_offset, dir);
                    }
                    self.tiles.insert(from, WarehouseTilePart2::Free);
                    self.tiles.insert(next_pos, this_tile);
                    if !is_horizontal_move {
                        self.tiles.insert(beside_from, WarehouseTilePart2::Free);
                        self.tiles
                            .insert(next_pos + this_box_other_side_offset, other_box_tile);
                    };
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }

    fn naive_move(&mut self, from: Vector2<i32>, dir: &Direction) -> bool {
        let next_pos = from + dir.to_vec2();
        let next_tile = self.tiles.get(&next_pos);
        let this_tile = self.tiles.get(&from).unwrap().clone();

        match next_tile {
            Some(WarehouseTilePart2::Free) => {
                self.tiles.insert(from, WarehouseTilePart2::Free);
                self.tiles.insert(next_pos, this_tile);
                true
            }
            Some(WarehouseTilePart2::Wall) => false,
            _ => {
                let other_box_other_side_offset = if next_tile == Some(&WarehouseTilePart2::LeftBox)
                {
                    Direction::Right.to_vec2()
                } else {
                    Direction::Left.to_vec2()
                };

                let other_side_of_other_box_pos = next_pos + other_box_other_side_offset;
                if self.can_move(next_pos, dir)
                    && self.can_move(other_side_of_other_box_pos, dir)
                    && self.naive_move(next_pos, dir)
                    && self.naive_move(other_side_of_other_box_pos, dir)
                {
                    self.tiles.insert(from, WarehouseTilePart2::Free);
                    self.tiles.insert(next_pos, this_tile);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn can_move(&self, from: Vector2<i32>, dir: &Direction) -> bool {
        let next_pos = from + dir.to_vec2();
        let next_tile = self.tiles.get(&next_pos);
        let this_tile = self.tiles.get(&from).unwrap().clone();

        assert!(
            this_tile == WarehouseTilePart2::LeftBox || this_tile == WarehouseTilePart2::RightBox
        );

        match next_tile {
            Some(WarehouseTilePart2::Free) => true,
            Some(WarehouseTilePart2::LeftBox) | Some(WarehouseTilePart2::RightBox)
                if dir == &Direction::Up || dir == &Direction::Down =>
            {
                let other_side_offset = if next_tile == Some(&WarehouseTilePart2::LeftBox) {
                    Direction::Right.to_vec2()
                } else {
                    Direction::Left.to_vec2()
                };
                let other_side_of_box_pos = next_pos + other_side_offset;
                self.can_move(next_pos, dir) && self.can_move(other_side_of_box_pos, dir)
            }
            Some(WarehouseTilePart2::LeftBox) | Some(WarehouseTilePart2::RightBox) => {
                self.can_move(next_pos, dir)
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseTilePart2 {
    Wall,
    LeftBox,
    RightBox,
    Robot,
    Free,
}

impl WarehouseTilePart2 {
    fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::LeftBox => '[',
            Self::RightBox => ']',
            Self::Robot => '@',
            Self::Free => '.',
        }
    }
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Wall),
            '[' => Some(Self::LeftBox),
            ']' => Some(Self::RightBox),
            '@' => Some(Self::Robot),
            '.' => Some(Self::Free),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::Up),
            '>' => Some(Self::Right),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            _ => None,
        }
    }

    fn to_vec2(&self) -> Vector2<i32> {
        match self {
            Self::Up => Vector2::new(0, -1),
            Self::Right => Vector2::new(1, 0),
            Self::Down => Vector2::new(0, 1),
            Self::Left => Vector2::new(-1, 0),
        }
    }
}

fn parse_file(filename: &str) -> Result<(Warehouse, Vec<Direction>), String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let warehouse = Warehouse::from_string(&content);

    let moves = content
        .chars()
        .filter_map(|c| Direction::from_char(c))
        .collect_vec();

    Ok((warehouse, moves))
}

fn main() {
    let args = FilenameArg::parse();
    let mut start = time::Instant::now();
    let (warehouse, moves) = parse_file(&args.input).expect("Failed to parse file.");
    println!("Parsed file in {:?}", start.elapsed());

    start = time::Instant::now();
    let part_1_result = part1(warehouse.clone(), &moves);
    println!("Part 1: {:?}", start.elapsed());

    start = time::Instant::now();
    let part_2_result = part2(WarehousePart2::from_warehouse(&warehouse), &moves);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(mut warehouse: Warehouse, moves: &Vec<Direction>) -> usize {
    // println!("{}", warehouse.to_string());
    for robot_move in moves {
        // println!("move {:?}:", robot_move);
        warehouse.do_move(robot_move);
        // println!("{}", warehouse.to_string());
    }
    warehouse
        .tiles
        .iter()
        .filter(|(_, tile)| **tile == WarehouseTile::Box)
        .map(|(&pos, _)| get_gps_coord(pos))
        .sum()
}

fn get_gps_coord(v: Vector2<i32>) -> usize {
    (v.y as usize * 100) + v.x as usize
}

fn part2(mut warehouse: WarehousePart2, moves: &Vec<Direction>) -> usize {
    println!("{}", warehouse.to_string());
    for robot_move in moves {
        // println!("move {:?}:", robot_move);
        warehouse.do_move(robot_move);
        // println!("{}", warehouse.to_string());
    }
    println!("{}", warehouse.to_string());

    let max_x = warehouse.tiles.keys().max_by_key(|v| v.x).unwrap().x;
    let max_y = warehouse.tiles.keys().max_by_key(|v| v.y).unwrap().y;
    println!("max x: {:?}, max y: {:?}", max_x, max_y);
    warehouse
        .tiles
        .iter()
        .filter(|(_, tile)| **tile == WarehouseTilePart2::LeftBox)
        .map(|(&pos, _)| {
            let n = get_gps_coord(pos);
            println!("{:?}: {}", pos, n);
            n
        })
        .sum()
}
