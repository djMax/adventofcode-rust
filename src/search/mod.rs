use std::collections::BinaryHeap;
use std::collections::HashSet;

pub trait State where Self: Ord + Sized + Clone {
  type World;

  fn key(&self) -> String;
  fn desc(&self) -> String;
  fn is_complete(&self, world: &Self::World) -> bool;
  fn should_prune(&self, world: &Self::World, best: &Self) -> bool;
  fn next_states(&self, world: &Self::World, visited: &HashSet<String>) -> Vec<Self> where Self: Sized;

  fn search(&self, world: &Self::World) -> Option<Self> {
    let mut q: BinaryHeap<Self> = BinaryHeap::new();
    let mut best: Option<Self> = None;
    let mut visited: HashSet<String> = HashSet::new();
    q.push(self.clone());

    while let Some(state) = q.pop() {
      if state.is_complete(world) {
        if best.is_none() || state.cmp(best.as_ref().unwrap()) == std::cmp::Ordering::Less {
          best = Some(state);
        }
        continue;
      }

      let key = state.key();
      if visited.contains(&key) {
        continue;
      }

      visited.insert(key);

      if best.is_some() && state.should_prune(world, &best.as_ref().unwrap()) {
        continue;
      }

      let next_states = state.next_states(world, &visited);
      for next_state in next_states {
        if !visited.contains(&next_state.key()) {
          q.push(next_state);
        }
      }
    }

    best
  }
}
