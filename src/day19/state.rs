use std::collections::HashSet;

use super::search::State;
use super::parser::{Blueprint, Resource, ResourceMap};

#[derive(Debug, Clone, Eq)]
pub struct RobotState {
  pub minute: i8,
  pub resources: ResourceMap<i16>,
  pub robots: ResourceMap<i16>,
}

impl RobotState {
  pub fn go_forward(&self) -> RobotState {
    let next = RobotState {
      robots: self.robots.clone(),
      minute: self.minute + 1,
      resources: self.resources.add(
        self.robots.ore,
        self.robots.clay,
        self.robots.obsidian,
        self.robots.geode,
      ),
    };
    next
  }

  pub fn add_robot(&self, world: &Blueprint, resource: &Resource) -> RobotState {
    let c = world.costs.get(&resource);
    let after_build = self.resources.consume(c.ore, c.clay, c.obsidian);
    let geode = match resource {
      Resource::Geode => i16::from(world.minutes) - i16::from(self.minute) - 1,
      _ => 0,
    };
    let growth = after_build.add(self.robots.ore, self.robots.clay, self.robots.obsidian, geode);

    RobotState {
      minute: self.minute + 1,
      resources: growth,
      robots: match resource {
        Resource::Geode => self.robots.clone(),
        _ => self.robots.change_resource(&resource, self.robots.get(&resource) + 1),
      },
    }
  }

  pub fn can_afford(&self, world: &Blueprint, resource: &Resource) -> bool {
    let cost = world.costs.get(resource);
    self.resources.has_at_least(cost.ore, cost.clay, cost.obsidian)
  }

  pub fn needs(&self, world: &Blueprint, resource: &Resource) -> bool {
    if self.minute >= world.minutes - 1 {
      return false;
    }
    if resource == &Resource::Geode {
      return true;
    }
    let count = self.robots.get(resource);
    let goal = world.max_robots.get(resource);
    count < goal
  }

  pub fn max_score(&self, world: &Blueprint) -> i16 {
    if self.minute == world.minutes {
      return self.resources.geode;
    }

    let remaining = i16::from(world.minutes - self.minute);
    self.resources.geode + ((((remaining - 1)) * remaining) / 2) + 1
  }
}

impl State for RobotState {
  type World = Blueprint;

  fn key(&self) -> String {
    format!("{},{},{},{},{},{},{},{}",
      self.minute,
      self.resources.ore,
      self.resources.clay,
      self.resources.obsidian,
      self.resources.geode,
      self.robots.ore,
      self.robots.clay,
      self.robots.obsidian,
    ).to_string()
  }

  fn desc(&self) -> String {
    format!("{} geodes, {} minutes", self.resources.geode, self.minute)
  }

  fn is_complete(&self, world: &Blueprint) -> bool {
    self.minute == world.minutes
  }

  fn should_prune(&self, world: &Blueprint, best: &RobotState) -> bool {
    best.resources.geode >= self.max_score(world)
  }

  fn next_states(&self, world: &Blueprint, _visited: &HashSet<String>) -> Vec<RobotState> {
    let remaining = world.minutes - self.minute;
    if remaining == 0 {
      return vec![];
    }
    let next_minute = self.go_forward();
    let mut next_states = Vec::from([next_minute]);

    Resource::iter().for_each(|r| {
      if !self.can_afford(world, &r) || !self.needs(world, &r) {
        return;
      }
      let with_robot = self.add_robot(world, &r);
      next_states.push(with_robot);
    });

    next_states
  }
}

impl PartialEq for RobotState {
  fn eq(&self, other: &Self) -> bool {
    self.minute == other.minute
  }
}

impl PartialOrd for RobotState {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.minute.cmp(&other.minute))
  }
}

impl Ord for RobotState {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let mut comp = other.resources.geode.cmp(&self.resources.geode);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    comp = other.minute.cmp(&self.minute);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    comp = other.robots.obsidian.cmp(&self.robots.obsidian);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    comp = other.robots.clay.cmp(&self.robots.clay);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    comp = other.resources.obsidian.cmp(&self.resources.obsidian);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    comp = other.resources.clay.cmp(&self.resources.clay);
    if comp != std::cmp::Ordering::Equal {
      return comp;
    }

    return std::cmp::Ordering::Equal;
  }
}
