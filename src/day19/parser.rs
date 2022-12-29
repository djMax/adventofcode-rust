use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Resource {
  Ore,
  Clay,
  Obsidian,
  Geode,
}

impl Resource {
  pub fn iter() -> impl Iterator<Item = Resource> {
    [Resource::Ore, Resource::Clay, Resource::Obsidian, Resource::Geode].iter().copied()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceMap<T: Clone> {
  pub ore: T,
  pub clay: T,
  pub obsidian: T,
  pub geode: T,
}

impl<T: Clone> ResourceMap<T> {
  pub fn new(default: T) -> ResourceMap<T> {
    ResourceMap {
      ore: default.clone(),
      clay: default.clone(),
      obsidian: default.clone(),
      geode: default.clone(),
    }
  }

  pub fn set(&mut self, ore: T, clay: T, obsidian: T, geode: T) {
    self.ore = ore;
    self.clay = clay;
    self.obsidian = obsidian;
    self.geode = geode;
  }

  pub fn change_resource(&self, r: &Resource, v: T) -> ResourceMap<T> {
    ResourceMap {
      ore: if r == &Resource::Ore { v.clone() } else { self.ore.clone() },
      clay: if r == &Resource::Clay { v.clone() } else { self.clay.clone() },
      obsidian: if r == &Resource::Obsidian { v.clone() } else { self.obsidian.clone() },
      geode: if r == &Resource::Geode { v.clone() } else { self.geode.clone() },
    }
  }

  pub fn has_at_least(&self, ore: T, clay: T, obsidian: T) -> bool
  where
    T: PartialOrd,
  {
    self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian
  }

  pub fn get(&self, resource: &Resource) -> T
  where
    T: Copy,
  {
    match resource {
      Resource::Ore => self.ore,
      Resource::Clay => self.clay,
      Resource::Obsidian => self.obsidian,
      Resource::Geode => self.geode,
    }
  }

  pub fn consume(&self, ore: T, clay: T, obsidian: T) -> ResourceMap<T>
  where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy
  {
    assert!(self.ore >= ore && self.clay >= clay && self.obsidian >= obsidian, "Not enough resources");
    ResourceMap {
      ore: self.ore - ore,
      clay: self.clay - clay,
      obsidian: self.obsidian - obsidian,
      geode: self.geode,
    }
  }

  pub fn add(&self, ore: T, clay: T, obsidian: T, geode: T) -> ResourceMap<T>
  where
    T: std::ops::Add<Output = T> + Copy,
  {
    ResourceMap {
      ore: self.ore + ore,
      clay: self.clay + clay,
      obsidian: self.obsidian + obsidian,
      geode: self.geode + geode,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
  pub name: String,
  pub costs: ResourceMap<ResourceMap<i16>>,
  pub max_robots: ResourceMap<i16>,
  pub minutes: i8,
}

pub fn parse(input: &str) -> Vec<Blueprint> {
  lazy_static! {
    static ref BP: Regex = Regex::new(r"\s(?P<index>\d+)").unwrap();
    static ref ORE: Regex = Regex::new(r"Each ore robot costs (?P<ore>\d+) ore").unwrap();
    static ref CLAY: Regex = Regex::new(r"Each clay robot costs (?P<ore>\d+) ore").unwrap();
    static ref OBSIDIAN: Regex = Regex::new(r"Each obsidian robot costs (?P<ore>\d+) ore and (?P<clay>\d+)").unwrap();
    static ref GEODE: Regex = Regex::new(r"Each geode robot costs (?P<ore>\d+) ore and (?P<obsidian>\d+) obsidian").unwrap();
  }
  return input
    .trim()
    .split("\n")
    .map(|bp| {
      let lead: Vec<&str> = bp.split(":").collect();
      let lines: Vec<&str> = lead[1].trim().split(".").collect();
      let mut costs = ResourceMap {
        ore: ResourceMap::new(0),
        clay: ResourceMap::new(0),
        obsidian: ResourceMap::new(0),
        geode: ResourceMap::new(0),
      };
      let mut max_robots = ResourceMap::new(0);

      let ore_ore = ORE.captures(lines[0]).unwrap()["ore"].parse::<i16>().unwrap();
      max_robots.ore = ore_ore;
      costs.ore.ore = ore_ore;

      let clay_ore = CLAY.captures(lines[1]).unwrap()["ore"].parse::<i16>().unwrap();
      max_robots.ore = i16::max(max_robots.ore, clay_ore);
      costs.clay.ore = clay_ore;

      let caps = OBSIDIAN.captures(lines[2]).unwrap();
      let obs_ore = caps["ore"].parse::<i16>().unwrap();
      let obs_clay = caps["clay"].parse::<i16>().unwrap();
      costs.obsidian.set(obs_ore, obs_clay, 0, 0);
      max_robots.ore = i16::max(max_robots.ore, obs_ore);
      max_robots.clay = i16::max(max_robots.clay, obs_clay);

      let g_caps = GEODE.captures(lines[3]).unwrap();
      let g_ore = g_caps["ore"].parse::<i16>().unwrap();
      let g_obs = g_caps["obsidian"].parse::<i16>().unwrap();
      costs.geode.set(g_ore, 0, g_obs, 0);
      max_robots.ore = i16::max(max_robots.ore, g_ore);
      max_robots.obsidian = i16::max(max_robots.clay, g_obs);

      return Blueprint {
        name: BP.captures(lead[0]).unwrap()["index"].to_string(),
        costs,
        max_robots,
        minutes: 0,
      };
    })
    .collect();
}
