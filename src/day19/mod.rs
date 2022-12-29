use rayon::prelude::*;
use crate::search::State;

use self::{
    parser::Blueprint,
    state::RobotState,
};
use super::search;

mod parser;
mod state;

fn part1(blueprints: Vec<Blueprint>) -> i32 {
    let best_scores: Vec<i16> = blueprints
        .par_iter()
        .map(|blueprint| {
            let initial = RobotState {
                resources: parser::ResourceMap::new(0),
                robots: parser::ResourceMap::new(0).add(1, 0, 0, 0),
                minute: 0,
            };
            let world = Blueprint {
                minutes: 24,
                ..blueprint.clone()
            };
            let best = initial.search(&world);
            match best {
                Some(state) => state.resources.geode,
                None => 0,
            }
        })
        .collect();
    best_scores
        .iter()
        .enumerate()
        .fold(0, |sum, (index, v)| sum + (*v as i32 * (index as i32 + 1)))
}

pub fn run() {
    let input = include_str!("data.txt");
    let parsed = parser::parse(input);
    let t = std::time::Instant::now();
    let answer = part1(parsed);
    println!("Part 1: {} in {}s", answer, t.elapsed().as_secs_f32());
}

#[cfg(test)]
mod day19 {
    use super::*;
    use parser::Resource;

    #[test]
    fn state_works() {
      let input = include_str!("sample.txt");
      let world = Blueprint {
        minutes: 24,
        ..parser::parse(input)[0].clone()
      };
      let initial = RobotState {
        robots: parser::ResourceMap::new(0).add(1, 0, 0, 0),
        resources: parser::ResourceMap::new(0),
        minute: 0,
      };
      let phase1 = initial
        .go_forward()
        .go_forward(); // minute 3
      println!("Phase 1: {:?}", phase1);
      assert_eq!(phase1.can_afford(&world, &Resource::Clay), true);
      let phase2 = phase1
        .add_robot(&world, &Resource::Clay)
        .go_forward()
        .add_robot(&world, &Resource::Clay)
        .go_forward()
        .add_robot(&world, &Resource::Clay); // minute 7
      assert_eq!(phase2.minute, 7);
      assert_eq!(phase2.robots.clay, 3);

      let phase3 = phase2
        .go_forward()
        .go_forward()
        .go_forward()
        .add_robot(&world, &Resource::Obsidian)
        .add_robot(&world, &Resource::Clay)
        .go_forward()
        .go_forward()
        .add_robot(&world, &Resource::Obsidian);
      assert_eq!(phase3.minute, 15);
      assert_eq!(phase3.robots.obsidian, 2);

      let final_state = phase3
        .go_forward()
        .go_forward()
        .add_robot(&world, &Resource::Geode)
        .go_forward()
        .go_forward()
        .add_robot(&world, &Resource::Geode)
        .go_forward()
        .go_forward()
        .go_forward();

      assert_eq!(final_state.minute, 24);
      assert!(final_state.is_complete(&world));
      assert_eq!(final_state.resources.geode, 9);
    }

    #[test]
    fn it_works() {
        let input = include_str!("sample.txt");
        let parsed = parser::parse(input);
        assert_eq!(part1(parsed), 33);
        // assert_eq!(part2(&parsed), 45000);
    }
}
