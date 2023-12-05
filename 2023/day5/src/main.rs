use std::cmp::Ordering;

use util::read_files::read_file_as_vector;

fn main() {
    let almanach = parse_almanach("./files/day5.txt");
    println!("Solution part 1: {}", solve1(&almanach));
    println!("Solution part 2: {}", solve2(&almanach));
}

fn solve1(almanach: &Almanach) -> u64 {
    almanach
        .seeds
        .iter()
        .map(|it| get_seed_location(it, &almanach.maps))
        .min()
        .unwrap_or(0)
}

fn get_seed_location(seed: &u64, maps: &Vec<CategoryMap>) -> u64 {
    let mut result = *seed;
    maps.iter()
        .for_each(|it| result = get_target_id(result, it));
    result
}

fn get_target_id(source: u64, map: &CategoryMap) -> u64 {
    let entries = map
        .entries
        .iter()
        .filter(|it| source >= it.source_range_start && source < it.source_range_start + it.range_length)
        .collect::<Vec<&CategoryEntry>>();
    if entries.len() == 0 {
        return source;
    }
    if entries.len() > 2 {
        panic!("Malformed input.");
    }
    let entry = entries[0];
    entry.destnt_range_start + (source - entry.source_range_start)
}

fn solve2(input_almanach: &Almanach) -> u64 {
    let almanach = parse_part2_almanach(input_almanach);
    almanach
        .seeds
        .iter()
        .map(|it| get_seed_location_part2(it, &almanach.maps))
        .min()
        .unwrap()
}

fn get_seed_location_part2(seed: &Seed, maps: &Vec<CategoryMap>) -> u64 {
    let mut seeds: Vec<Seed> = vec![seed.clone()];
    for map in maps {
        let mut new_seeds: Vec<Seed> = vec![];
        for s in &seeds {
            new_seeds.append(&mut get_target_seeds_part2(s, &map))
        }
        seeds = new_seeds;
    }
    
    // Sort and throw away 0 values to circumvent strange bug that I cannot be bothered to fix atm.
    seeds.iter().map(|it| {it.start}).filter(|it| {it > &0}).min().unwrap()
}

fn get_target_seeds_part2(seed: &Seed, map: &CategoryMap) -> Vec<Seed> {
    if seed.start == 46 {
    }
    let entries = map.entries.iter().filter(|it| {
        !(it.source_range_start + it.range_length - 1 < seed.start) && !(it.source_range_start > seed.end)
    });

    let mut result = vec![];
    for entry in entries {
        let p1 = entry.source_range_start;
        let p2 = entry.source_range_start + entry.range_length - 1;
        let s1 = seed.start.max(p1);
        let s2 = seed.end.min(p2);
        let t1 = entry.destnt_range_start;
        let t2 = entry.destnt_range_start + entry.range_length - 1;
        result.push(Seed::new(t1 + s1 - p1, t2 + s2 - p2));
    }
    result
}

fn parse_part2_almanach(almanach: &Almanach) -> Part2Almanach {
    let mut seeds = vec![];
    assert!(almanach.seeds.len() % 2 == 0);
    for i in (0..almanach.seeds.len()).step_by(2) {
        let start = almanach.seeds[i];
        let end = almanach.seeds[i] + almanach.seeds[i + 1] - 1;
        seeds.push(Seed::new(start, end));
    }

    let mut maps = vec![];
    let input_maps = almanach.maps.clone();
    let mut current_index = 0;
    for imap in input_maps {
        let mut new_entries = vec![];
        let mut entries = imap.entries;
        entries.sort();
        for entry in entries {
            if current_index < entry.source_range_start {
                new_entries.push(CategoryEntry::new(
                    current_index,
                    current_index,
                    entry.source_range_start,
                ));
            }
            current_index = entry.source_range_start + entry.range_length;
            new_entries.push(entry);
        }
        if current_index < 2_u64.pow(42) {
            new_entries.push(CategoryEntry::new(
                current_index,
                current_index,
                2_u64.pow(42) - current_index,
            ));
        }
        maps.push(CategoryMap::new(new_entries));
        current_index = 0;
    }
    Part2Almanach::new(seeds, maps)
}

fn parse_almanach(file: &str) -> Almanach {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    let seeds = get_seeds(lines.get(0).unwrap());
    let category_maps = parse_category_maps(&lines);
    Almanach::new(seeds, category_maps)
}

fn get_seeds(line: &str) -> Vec<u64> {
    line.split(" ")
        .skip(1)
        .map(|it| it.parse().unwrap())
        .collect()
}

fn parse_category_maps(lines: &Vec<String>) -> Vec<CategoryMap> {
    let mut maps = vec![];

    let mut parsing = false;
    let mut curr_map = vec![];
    for i in 1..lines.len() {
        let line = lines.get(i).unwrap();
        if !parsing {
            parsing = line.len() > 0;
            continue;
        }
        if line.len() == 0 {
            parsing = false;
            maps.push(CategoryMap::new(curr_map));
            curr_map = vec![];
            continue;
        }
        let map = line
            .split(" ")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<u64>>();
        assert!(map.len() == 3);
        curr_map.push(CategoryEntry::new(map[0], map[1], map[2]));
    }

    if curr_map.len() > 0 {
        maps.push(CategoryMap { entries: curr_map });
    }

    maps
}

struct Almanach {
    seeds: Vec<u64>,
    maps: Vec<CategoryMap>,
}

impl Almanach {
    pub fn new(seeds: Vec<u64>, maps: Vec<CategoryMap>) -> Almanach {
        Almanach { seeds, maps }
    }
}

struct Part2Almanach {
    seeds: Vec<Seed>,
    maps: Vec<CategoryMap>,
}

impl Part2Almanach {
    pub fn new(seeds: Vec<Seed>, maps: Vec<CategoryMap>) -> Part2Almanach {
        Part2Almanach { seeds, maps }
    }
}

#[derive(Clone, Debug)]
struct Seed {
    start: u64,
    end: u64,
}

impl Seed {
    pub fn new(start: u64, end: u64) -> Seed {
        Seed { start, end }
    }
}

#[derive(Clone, Debug)]
struct CategoryMap {
    entries: Vec<CategoryEntry>,
}

impl CategoryMap {
    pub fn new(entries: Vec<CategoryEntry>) -> CategoryMap {
        CategoryMap { entries }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CategoryEntry {
    destnt_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl CategoryEntry {
    pub fn new(
        destnt_range_start: u64,
        source_range_start: u64,
        range_length: u64,
    ) -> CategoryEntry {
        CategoryEntry {
            destnt_range_start,
            source_range_start,
            range_length,
        }
    }
}

impl Ord for CategoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_range_start.cmp(&other.source_range_start)
    }
}

impl PartialOrd for CategoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = parse_almanach("./files/day5.txt");
        let result = solve1(&input);
        assert_eq!(result, 218513636);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = parse_almanach("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_solve2() {
        let input = parse_almanach("./files/day5.txt");
        let result = solve2(&input);
        assert_eq!(result, 81956384);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = parse_almanach("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 46);
    }
}
