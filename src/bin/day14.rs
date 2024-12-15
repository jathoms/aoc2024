use cgmath::Vector2;
use clap::Parser;
use image::{ImageBuffer, Luma};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashSet, fs, sync::Mutex, time};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Robot {
    pos: Vector2<isize>,
    vel: Vector2<isize>,
}

impl Robot {
    fn move_once(&mut self, max_x: isize, max_y: isize) {
        self.pos.x = (((self.pos.x + self.vel.x) % max_x) + max_x) % max_x;
        self.pos.y = (((self.pos.y + self.vel.y) % max_y) + max_y) % max_y;
    }
}

#[derive(Debug, Clone)]
struct RobotGrid {
    robots: Vec<Robot>,
    max_x: isize,
    max_y: isize,
}

impl RobotGrid {
    fn do_iteration(&mut self) {
        self.robots
            .iter_mut()
            .for_each(|r| r.move_once(self.max_x, self.max_y));
    }

    fn n_robots_in_top_left(&self) -> usize {
        self.robots
            .iter()
            .filter(|r| r.pos.x < self.max_x / 2 && r.pos.y < self.max_y / 2)
            .count()
    }

    fn n_robots_in_top_right(&self) -> usize {
        self.robots
            .iter()
            .filter(|r| r.pos.x > self.max_x / 2 && r.pos.y < self.max_y / 2)
            .count()
    }

    fn n_robots_in_bottom_left(&self) -> usize {
        self.robots
            .iter()
            .filter(|r| r.pos.x < self.max_x / 2 && r.pos.y > self.max_y / 2)
            .count()
    }

    fn n_robots_in_bottom_right(&self) -> usize {
        self.robots
            .iter()
            .filter(|r| r.pos.x > self.max_x / 2 && r.pos.y > self.max_y / 2)
            .count()
    }

    fn get_largest_contiguous_region(&self) -> usize {
        let mut regions = Vec::<HashSet<Robot>>::new();
        let mut largest = 0;
        for r in self.robots.iter() {
            if !regions.par_iter().any(|v| v.contains(&r)) {
                let region = self.find_region_of(*r);
                if region.len() > largest {
                    largest = region.len();
                }
                // println!("found region of size: {:?}", region.len());
                // assert!(regions
                //     .iter()
                //     .all(|existing_region| region.is_disjoint(existing_region)));
                regions.push(region);
            }
        }
        largest
        // regions.into_par_iter().max_by_key(|r| r.len()).unwrap()
    }
    fn check_point(&self, x: usize, y: usize) -> Option<Robot> {
        self.robots
            .par_iter()
            .find_any(|robot| robot.pos.x == x as isize && robot.pos.y == y as isize)
            .cloned()
    }
    fn find_region_of(&self, robot: Robot) -> HashSet<Robot> {
        let mut set = HashSet::<Robot>::new();

        // println!("finding region around {:?}", robot);

        set.insert(robot);

        let mut neighbors = self.neighbors_of(robot);

        while !neighbors.is_empty() {
            set.extend(neighbors.clone());
            neighbors = neighbors
                .into_iter()
                .flat_map(|robot| self.neighbors_of(robot))
                .unique()
                .filter(|robot| !set.contains(robot))
                .collect();
        }
        set
    }
    fn neighbors_of(&self, robot: Robot) -> Vec<Robot> {
        let i = robot.pos.y as usize;
        let j = robot.pos.x as usize;
        [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)]
            .into_iter()
            .filter_map(|(i, j)| self.check_point(i, j))
            .collect_vec()
    }
}

fn parse_file(filename: &str) -> Result<RobotGrid, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());

    let robot_re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let robots = robot_re
        .captures_iter(&content)
        .map(|captures| {
            let pos = Vector2::new(
                captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            );
            let vel = Vector2::new(
                captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            );
            Robot { pos, vel }
        })
        .collect_vec();

    Ok(RobotGrid {
        robots,
        max_x: 101,
        max_y: 103,
    })
}

fn main() {
    let args = FilenameArg::parse();
    let mut start = time::Instant::now();
    let grid = parse_file(&args.input).expect("Failed to parse file.");
    println!("Parsed file in {:?}", start.elapsed());

    start = time::Instant::now();
    let part_1_result = part1(grid.clone());
    println!("Part 1: {:?}", start.elapsed());

    start = time::Instant::now();
    let part_2_result = part2_print(grid.clone());
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(mut grid: RobotGrid) -> usize {
    for _ in 0..100 {
        grid.do_iteration();
    }
    println!(
        "bl: {:?}, br: {:?}, tl: {:?}, tr: {:?}",
        grid.n_robots_in_bottom_left(),
        grid.n_robots_in_bottom_right(),
        grid.n_robots_in_top_left(),
        grid.n_robots_in_top_right()
    );
    grid.n_robots_in_bottom_left()
        * grid.n_robots_in_bottom_right()
        * grid.n_robots_in_top_left()
        * grid.n_robots_in_top_right()
}

fn part2(mut grid: RobotGrid) -> u32 {
    let largest = Mutex::new((0, 0));
    let mut grids = Vec::new();
    for i in 0..100000 {
        grids.push((grid.clone(), i));
        grid.do_iteration();
    }
    let largest_index = grids
        .par_iter()
        .map(|(grid, i)| {
            // println!("still largest {:?} at {:?}", largest, largest_index);
            // print!("{i}");
            let region_size = grid.get_largest_contiguous_region() as u32;
            let mut largest_ = largest.lock().unwrap();
            if largest_.0 < region_size {
                println!(
                    "\n new largest region found at index {:?}: len {:?}",
                    i, region_size
                );
                *largest_ = (region_size, *i);
            };
            // }
            let largest_index = largest.lock().unwrap().1;
            largest_index
        })
        .reduce(|| 0, |a, b| if a > b { a } else { b });
    largest_index
}

fn part2_nopar(mut grid: RobotGrid) -> u32 {
    let mut largest = 0;
    let mut largest_index = 0;

    for i in 0..100000 {
        // println!("still largest {:?} at {:?}", largest, largest_index);
        print!("{i}");
        grid.do_iteration();
        let region_size = grid.get_largest_contiguous_region() as u32;
        if largest < region_size {
            println!(
                "\n new largest region found at index {:?}: len {:?}",
                i, region_size
            );
            largest = region_size;
            largest_index = i;
        };
        // }
    }
    largest_index
}

fn part2_print(mut grid: RobotGrid) -> u32 {
    let mut img = ImageBuffer::new(grid.max_x as u32, grid.max_y as u32);
    for i in 0..10000000 {
        grid.do_iteration();

        let points = grid.robots.iter().map(|r| (r.pos.x as u32, r.pos.y as u32)).unique().collect_vec();

        for (x,y,px) in img.enumerate_pixels_mut() {
            if points.contains(&(x, y)) {
                *px = Luma([255]);
            } else{
                *px = Luma([0])
            }
        }

        println!("{}", i);
        
        image::save_buffer(format!("M:/repos/aoc2024/output/{}.png",i).as_str(), &img, grid.max_x as u32, grid.max_y as u32,image::ExtendedColorType::L8).expect("failed to save image");
        
        
    };
    1
}
