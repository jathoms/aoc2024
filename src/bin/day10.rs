use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct FilenameArg {
    #[arg(short, long)]
    input: String,
}

struct HikingMap {
    tiles: Vec<Vec<usize>>,
}

impl HikingMap {
    fn get_trailheads(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (i, line) in self.tiles.iter().enumerate() {
            for (j, &n) in line.iter().enumerate() {
                if n == 0 {
                    v.push((i, j));
                }
            }
        }
        v
    }

    fn get_tile(&self, i: usize, j: usize) -> Option<&usize> {
        self.tiles.get(i)?.get(j)
    }

    fn find_tiles_from(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut paths = Vec::new();

        if let Some(&n) = self.get_tile(i, j) {
            let directions = [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)];

            for (row, col) in directions {
                if let Some(&n2) = self.get_tile(row, col) {
                    if n2 == n + 1 {
                        paths.push((row, col));
                    }
                }
            }
        }

        paths
    }

    fn find_peaks_reachable_from(&self, head_i: usize, head_j: usize) -> HashSet<(usize, usize)> {
        let mut peaks = HashSet::new();
        if self.get_tile(head_i, head_j) == Some(&9) {
            peaks.insert((head_i, head_j));
            return peaks;
        }
        for (i, j) in self.find_tiles_from(head_i, head_j) {
            let peaks_from_here = self.find_peaks_reachable_from(i, j);
            peaks.extend(peaks_from_here);
        }
        peaks
    }
    
    fn find_distinct_trails(&self, head_i:usize, head_j: usize) -> usize{
        
        if self.get_tile(head_i, head_j) == Some(&9) {
            return 1
        }
        let mut peaks = 0;
        for (i, j) in self.find_tiles_from(head_i, head_j) {
            let peaks_from_here = self.find_distinct_trails(i, j);
            peaks += peaks_from_here;
        }
        peaks
    }
}

fn parse_file(filename: &str) -> Result<HikingMap, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());
    Ok(HikingMap {
        tiles: content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    })
}

fn main() {
    let args = FilenameArg::parse();
    let map = parse_file(&args.input).expect("Failed to parse file.");

    let mut start = time::Instant::now();
    let part_1_result = part1(&map);
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(&map);
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(map: &HikingMap) -> usize {
    let mut result = 0;
    for (i, j) in map.get_trailheads() {
        result += map.find_peaks_reachable_from(i, j).len()
    }
    result
}

fn part2(map: &HikingMap)-> usize {
    let mut result = 0;
    for (i, j) in map.get_trailheads() {
        result += map.find_distinct_trails(i, j);
    }
    result

}