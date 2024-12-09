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

struct DiskMap {
    map: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ExpandedDiskMap {
    map: Vec<FileBlock>,
}

#[derive(Debug, Clone, Copy)]
enum FileBlock {
    Free,
    Taken(usize),
}
impl FileBlock {
    fn to_string(&self) -> String{
        match self {
            &FileBlock::Free => ".".to_string(),
            &FileBlock::Taken(i) => i.to_string(),
        }
    }
}

impl DiskMap {
    fn from_string(s: String) -> Self {
        Self {
            map: s
                .chars()
                .map(|c| c.to_digit(10).expect("couldn't parse diskmap string") as usize)
                .collect::<Vec<usize>>(),
        }
    }
}

impl ExpandedDiskMap {
    fn from_diskmap(m: &DiskMap) -> Self {
        let mut v = Vec::<FileBlock>::new();
        for (i, n) in m.map.iter().enumerate() {
            if i % 2 == 1 {
                v.extend(vec![FileBlock::Free; *n as usize].into_iter());
            } else {
                v.extend(vec![FileBlock::Taken(i / 2); *n as usize].into_iter());
            }
        }
        Self { map: v }
    }
    fn find_first_free_space(&self) -> Option<usize> {
        for (i, block) in self.map.iter().enumerate() {
            match block {
                FileBlock::Free => return Some(i),
                _ => (),
            };
        }
        None
    }
    fn find_last_filled_space(&self) -> Option<usize> {
        for (i, block) in self.map.iter().rev().enumerate() {
            match block {
                FileBlock::Taken(_) => return Some(self.map.len() - 1 - i),
                _ => (),
            };
        }
        None
    }
    fn checksum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                FileBlock::Taken(n) => n * i,
                _ => 0,
            })
            .sum()
    }
    fn to_string(&self) -> String {
        self.map.iter().map(|file_block| file_block.to_string()).collect::<String>()
    }
}

#[derive(Debug, Clone)]
struct CompactDiskMap {
    map: Vec<WholeFile>,
}

impl CompactDiskMap {
    fn from_diskmap(m: &DiskMap) -> Self {
        let mut v = Vec::<WholeFile>::new();
        for (i, n) in m.map.iter().enumerate() {
            if i % 2 == 1 {
                v.push(WholeFile::Free(*n));
            } else {
                v.push(WholeFile::Taken(i / 2, *n));
            }
        }
        Self { map: v }
    }
    fn find_first_free_space_big_enough(&self, file: &WholeFile) -> Option<usize> {
        let n = match file {
            &WholeFile::Taken(_, n) => Some(n),
            _ => None,
        }?;

        for (i, block) in self.map.iter().enumerate() {
            match block {
                &WholeFile::Free(size) if size >= n => return Some(i),
                _ => (),
            };
        }
        None
    }
    fn to_expanded_disk_map(&self) -> ExpandedDiskMap {
        ExpandedDiskMap {
            map: self
                .map
                .iter()
                .flat_map(|f| f.to_file_blocks())
                .collect::<Vec<_>>(),
        }
    }
    fn checksum(&self) -> usize {
        self.to_expanded_disk_map().checksum()
    }
    //sillyness...
    fn flatten_frees_in_place(&mut self) {
        let mut i = 0;

        while i < self.map.len() {
            if let WholeFile::Free(mut size) = self.map[i] {
                if size == 0 {
                    self.map.remove(i);
                    continue;
                }
                let j = i + 1;
                while j < self.map.len() {
                    if let WholeFile::Free(next_size) = self.map[j] {
                        size += next_size;
                        self.map.remove(j);
                    } else {
                        break;
                    }
                }
                self.map[i] = WholeFile::Free(size);
            }
            i += 1;
        }
    }
    fn to_string(&self) -> String {
        self.map.iter().map(|f| f.to_string()).collect::<String>()
    }
    fn find_index_of_file_id(&self, n: usize) -> Option<(usize, &WholeFile)> {
        self.map
            .iter()
            .find_position(|f| f.get_file_id() == Some(n))
    }
}

#[derive(Debug, Clone, Copy)]
enum WholeFile {
    Taken(usize, usize),
    Free(usize),
}

impl WholeFile {
    fn get_file_id(&self) -> Option<usize> {
        match self {
            &WholeFile::Taken(n, _) => Some(n),
            _ => None,
        }
    }
    fn to_string(&self) -> String {
        match self {
            &WholeFile::Free(n) => ".".repeat(n),
            &WholeFile::Taken(i, n) => i.to_string().repeat(n),
        }
    }
    fn to_file_blocks(&self) -> Vec<FileBlock> {
        match self {
            &WholeFile::Free(n) => vec![FileBlock::Free; n],
            &WholeFile::Taken(i, n) => vec![FileBlock::Taken(i); n],
        }
    }
}

fn parse_file(filename: &str) -> Result<DiskMap, String> {
    let content =
        fs::read_to_string(filename).expect(format!("Could not read file {}", filename).as_str());
    Ok(DiskMap::from_string(content))
}

fn main() {
    let args = FilenameArg::parse();
    let map = parse_file(&args.input).expect("Failed to parse file.");
    let a = ExpandedDiskMap::from_diskmap(&map);
    let b = CompactDiskMap::from_diskmap(&map);
    assert_eq!(a.to_string(), b.to_string());

    let mut start = time::Instant::now();
    let part_1_result = part1(ExpandedDiskMap::from_diskmap(&map));
    println!("Part 1: {:?}", start.elapsed());
    start = time::Instant::now();
    let part_2_result = part2(CompactDiskMap::from_diskmap(&map));
    println!("Part 2: {:?}", start.elapsed());

    println!("Part 1 result: {}", part_1_result);
    println!("Part 2 result: {}", part_2_result);
}

fn part1(mut map: ExpandedDiskMap) -> usize {
    let mut first_free_index = map
        .find_first_free_space()
        .expect("found no free space in map");
    let mut last_filled_index = map
        .find_last_filled_space()
        .expect("found no filled space in map");

    while first_free_index < last_filled_index {
        map.map.swap(first_free_index, last_filled_index);
        first_free_index = map.find_first_free_space().unwrap();
        last_filled_index = map.find_last_filled_space().unwrap();
    }
    map.checksum()
}

fn part2(mut map: CompactDiskMap) -> usize {
    for (id, file) in map
        .clone()
        .map
        .iter()
        .filter_map(|f| f.get_file_id().map(|id| (id, f)))
        .sorted_by_key(|&(id, _)| id)
        .skip(1)
        .rev()
    {
        if let Some(free_space_index) = map.find_first_free_space_big_enough(file) {
            if free_space_index
                >= map
                    .find_index_of_file_id(id)
                    .expect("didn't find file to get index of")
                    .0
            {
                continue;
            }
            let file_size = match file {
                WholeFile::Taken(_, n) => n,
                _ => panic!("wtf"),
            };
            let free_space = map.map.get(free_space_index).unwrap();
            let free_space_size = match free_space {
                WholeFile::Free(n) => n,
                _ => panic!("wtf"),
            };

            map.map.splice(
                free_space_index..free_space_index + 1,
                [
                    WholeFile::Free(*file_size),
                    WholeFile::Free(*free_space_size - file_size),
                ],
            );
            let (new_block_index, _) = map
                .find_index_of_file_id(id)
                .expect("didn't find file to get index of");
            map.map.swap(new_block_index, free_space_index);
            map.flatten_frees_in_place();
        }
    }
    map.checksum()
}
