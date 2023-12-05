use util::read_files::read_file_as_vector;

fn main() {
    let almanach = parse_almanach("./files/day5.txt");
    println!("Solution part 1: {}", solve1(&almanach));
    // println!("Solution part 2: {}", solve2(&scratch_cards));
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
    let entries = map.entries.iter().filter(|it| {
        source >= it.source_range_start && source < it.source_range_start + it.range_length
    }).collect::<Vec<&CategoryEntry>>();
    if entries.len() == 0 {
        return source;
    }
    if entries.len() > 2 {
        panic!("Malformed input.");
    }
    let entry = entries[0];
    entry.destnt_range_start + (source - entry.source_range_start)
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

struct CategoryMap {
    entries: Vec<CategoryEntry>,
}

impl CategoryMap {
    pub fn new(entries: Vec<CategoryEntry>) -> CategoryMap {
        CategoryMap { entries }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = parse_almanach("./files/day5.txt");
        let result = solve1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = parse_almanach("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 13);
    }
}
