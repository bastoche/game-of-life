#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
  Dead,
  Alive
}

pub fn next_state(state: State, living_neighbours_count: u8) -> State {
  match (state, living_neighbours_count) {
    (State::Dead, 3) => State::Alive,
    (State::Alive, c) if 2 <= c && c <= 3 => State::Alive,
    _ => State::Dead
  }
}

#[cfg(test)]
mod tests {
  use super::*;

   #[test]
  fn a_live_cell_with_less_than_2_living_neighbours_will_die() {
    assert_eq!(State::Dead, next_state(State::Alive, 0));
    assert_eq!(State::Dead, next_state(State::Alive, 1));
  }

  #[test]
  fn a_live_cell_with_2_or_3_living_neighbours_will_stay_alive() {
    assert_eq!(State::Alive, next_state(State::Alive, 2));
    assert_eq!(State::Alive, next_state(State::Alive, 3));
  }

  #[test]
  fn a_live_cell_with_more_than_3_living_neighbours_will_die() {
    assert_eq!(State::Dead, next_state(State::Alive, 4));
    assert_eq!(State::Dead, next_state(State::Alive, 5));
    assert_eq!(State::Dead, next_state(State::Alive, 6));
    assert_eq!(State::Dead, next_state(State::Alive, 7));
    assert_eq!(State::Dead, next_state(State::Alive, 8));
  }

  #[test]
  fn a_dead_cell_with_3_living_neighbours_will_live() {
    assert_eq!(State::Alive, next_state(State::Dead, 3));
  }
}