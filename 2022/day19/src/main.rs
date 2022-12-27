use regex::Regex;
use util::read_files::read_file_as_vector;

struct Blueprint {
    ore_for_ore: i64,
    ore_for_clay: i64,
    ore_for_obsidian: i64,
    clay_for_obsidian: i64,
    ore_for_geode: i64,
    obsidian_for_geode: i64,
}

impl Blueprint {
    fn new(
        ore_for_ore: i64,
        ore_for_clay: i64,
        ore_for_obsidian: i64,
        clay_for_obsidian: i64,
        ore_for_geode: i64,
        obsidian_for_geode: i64,
    ) -> Blueprint {
        Blueprint {
            ore_for_ore,
            ore_for_clay,
            ore_for_obsidian,
            clay_for_obsidian,
            ore_for_geode,
            obsidian_for_geode,
        }
    }
}

#[derive(Debug, PartialEq)]
enum RobotType {
    Skip = 0,
    Ore = 1,
    Clay = 2,
    Obsidian = 3,
    Geode = 4,
}

impl RobotType {
    fn can_be_built(&self, blueprint: &Blueprint, resources: &Resources) -> bool {
        match *self {
            RobotType::Skip => true,
            RobotType::Ore => resources.ore >= blueprint.ore_for_ore,
            RobotType::Clay => resources.ore >= blueprint.ore_for_clay,
            RobotType::Obsidian => {
                resources.ore >= blueprint.ore_for_obsidian
                    && resources.clay >= blueprint.clay_for_obsidian
            }
            RobotType::Geode => {
                resources.ore >= blueprint.ore_for_geode
                    && resources.obsidian >= blueprint.obsidian_for_geode
            }
        }
    }

    fn iter() -> std::slice::Iter<'static, RobotType> {
        static ROBOTTYPES: [RobotType; 5] = [
            RobotType::Skip,
            RobotType::Ore,
            RobotType::Clay,
            RobotType::Obsidian,
            RobotType::Geode,
        ];
        ROBOTTYPES.iter()
    }
}

#[derive(Debug, Clone)]
struct Robots {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl Robots {
    fn new(ore: u64, clay: u64, obsidian: u64, geode: u64) -> Robots {
        Robots {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn add(&mut self, robot_type: &RobotType) {
        match *robot_type {
            RobotType::Ore => self.ore += 1,
            RobotType::Clay => self.clay += 1,
            RobotType::Obsidian => self.obsidian += 1,
            RobotType::Geode => self.geode += 1,
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct Resources {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Resources {
    fn new(ore: i64, clay: i64, obsidian: i64, geode: i64) -> Resources {
        Resources {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn increase_by_robots(&mut self, robots: &Robots) {
        self.ore += robots.ore as i64;
        self.clay += robots.clay as i64;
        self.obsidian += robots.obsidian as i64;
        self.geode += robots.geode as i64;
    }

    fn decrease_by_built(&mut self, blueprint: &Blueprint, robot_type: &RobotType) {
        match *robot_type {
            RobotType::Skip => {}
            RobotType::Ore => self.ore -= blueprint.ore_for_ore,
            RobotType::Clay => self.ore -= blueprint.ore_for_clay,
            RobotType::Obsidian => {
                self.ore -= blueprint.ore_for_obsidian;
                self.clay -= blueprint.clay_for_obsidian;
            }
            RobotType::Geode => {
                self.ore -= blueprint.ore_for_geode;
                self.obsidian -= blueprint.obsidian_for_geode;
            }
        }
    }
}

struct RobotBuilt {
    ore: bool,
    clay: bool,
    obsidian: bool,
    geode: bool,
}

impl RobotBuilt {
    fn new(ore: bool, clay: bool, obsidian: bool, geode: bool) -> RobotBuilt {
        RobotBuilt {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn set_built(&mut self, robot_type: &RobotType) {
        match *robot_type {
            RobotType::Ore => self.ore = true,
            RobotType::Clay => self.clay = true,
            RobotType::Obsidian => self.obsidian = true,
            RobotType::Geode => self.geode = true,
            _ => {}
        }
    }

    fn built(&self, robot_type: &RobotType) -> bool {
        match *robot_type {
            RobotType::Ore => self.ore,
            RobotType::Clay => self.clay,
            RobotType::Obsidian => self.obsidian,
            RobotType::Geode => self.geode,
            _ => panic!("RobotType {:?} cannot be built.", robot_type),
        }
    }

    fn all_true(&self) -> bool {
        return self.ore && self.clay && self.obsidian && self.geode;
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day19.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i64 {
    let blueprints = lines
        .iter()
        .map(|el| parse_blueprint(el))
        .collect::<Vec<Blueprint>>();

    let mut result = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {} / {}", i + 1, blueprints.len());
        let robots = Robots::new(1, 0, 0, 0);
        let mut resources = Resources::new(0, 0, 0, 0);
        let mut was_built = RobotBuilt::new(false, false, false, false);
        let time = 24;
        let mut curr_max = 0;
        let blueprint_res = get_max_geodes(
            blueprint,
            &robots,
            &mut resources,
            &mut was_built,
            time,
            &mut curr_max,
        );
        result += blueprint_res * (i as i64 + 1);
    }

    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let blueprints = lines
        .iter()
        .map(|el| parse_blueprint(el))
        .collect::<Vec<Blueprint>>();

    let mut result = 1;
    for i in 0..std::cmp::min(3, blueprints.len()) {
        println!(
            "Blueprint {} / {}",
            i + 1,
            std::cmp::min(3, blueprints.len())
        );
        let robots = Robots::new(1, 0, 0, 0);
        let mut resources = Resources::new(0, 0, 0, 0);
        let mut was_built = RobotBuilt::new(false, false, false, false);
        let time = 32;
        let mut curr_max = 0;
        let blueprint_res = get_max_geodes(
            &blueprints[i],
            &robots,
            &mut resources,
            &mut was_built,
            time,
            &mut curr_max,
        );
        result *= blueprint_res;
    }

    result
}

fn parse_blueprint(line: &String) -> Blueprint {
    let re =
    Regex::new(r"Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$")
        .unwrap();
    let caps = re.captures(line).unwrap();
    let mut res = [0; 6];
    for i in 1..caps.len() {
        res[i - 1] = caps.get(i).unwrap().as_str().parse().unwrap();
    }
    Blueprint::new(res[0], res[1], res[2], res[3], res[4], res[5])
}

fn get_max_geodes(
    blueprint: &Blueprint,
    robots: &Robots,
    resources: &mut Resources,
    was_built: &mut RobotBuilt,
    time: u64,
    curr_max: &mut i64,
) -> i64 {
    if time == 0 {
        return resources.geode;
    }

    let possible_max = robots.geode * time + (time * (time - 1)) / 2;
    if (resources.geode + possible_max as i64) < *curr_max {
        return 0;
    }

    if robots.clay == 0 {
        was_built.set_built(&RobotType::Obsidian);
        was_built.set_built(&RobotType::Geode);
    } else if robots.obsidian == 0 {
        was_built.set_built(&RobotType::Geode);
    }

    let mut result = resources.geode;
    let can_build = get_buildable_robots(blueprint, resources, was_built, time);

    resources.increase_by_robots(robots);

    for robot_type in can_build {
        let mut new_resources = resources.clone();
        new_resources.decrease_by_built(blueprint, robot_type);
        was_built.set_built(robot_type);
        let mut new_robots = robots.clone();
        new_robots.add(robot_type);
        let mut new_was_built = RobotBuilt::new(false, false, false, false);
        result = std::cmp::max(
            result,
            get_max_geodes(
                blueprint,
                &new_robots,
                &mut new_resources,
                &mut new_was_built,
                time - 1,
                curr_max,
            ),
        );
    }

    if !was_built.all_true() {
        result = std::cmp::max(
            result,
            get_max_geodes(blueprint, robots, resources, was_built, time - 1, curr_max),
        );
    }

    *curr_max = std::cmp::max(*curr_max, result);
    result
}

fn get_buildable_robots(
    blueprint: &Blueprint,
    resources: &Resources,
    was_built: &mut RobotBuilt,
    time: u64,
) -> Vec<&'static RobotType> {
    RobotType::iter()
        .filter(|&rtype| {
            *rtype != RobotType::Skip
                && rtype.can_be_built(blueprint, resources)
                && !was_built.built(&rtype)
                && time > 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 33);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day19.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 1719);
    }

    #[test]
    fn test_solve2_large() {
        let lines = read_file_as_vector("./files/day19.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 19530);
    }
}
