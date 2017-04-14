fn main() {
  println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq)]
enum State {
  Dead,
  Alive
}

#[derive(Debug, PartialEq)]
struct Grid {
  cells: Vec<Vec<State>>
}

impl Grid {
  fn from_string(representation: &str) -> Grid {
    let vec: Vec<&str> = representation.split("\n").collect();
    let cells : Vec<Vec<State>> = vec.iter().map(|s| {
      s.chars().map(|c| {
        if c == '*' {
          State::Alive
        } else {
          State::Dead
        }
      }).collect()
    }).collect();
    Grid { cells: cells }
  }
}


fn next_state(state: State, living_neighbours_count: u8) -> State {
  match (state, living_neighbours_count) {
    (State::Dead, 3) => State::Alive,
    (State::Alive, c) if 2 <= c && c <= 3 => State::Alive,
    _ => State::Dead
  }
}

fn next_generation(grid: &Grid) -> Grid {
  Grid { cells: grid.cells.clone() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_string() {
    let representation = "...";
    let grid = Grid::from_string(representation);
    assert_eq!(vec!(vec![State::Dead, State::Dead, State::Dead]), grid.cells);
  }

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

  #[test]
  fn next_generation_when_all_cells_are_dead() {
    let grid = Grid::from_string("...");
    assert_eq!(grid, next_generation(&grid));
  }

  #[test]
  #[ignore]
  fn next_generation_for_basic_blinker() {
    let grid = Grid::from_string("...\n***\n...");
    let next_generation_grid = Grid::from_string(".*.\n.*.\n.*.");
    assert_eq!(next_generation_grid, next_generation(&grid));
  }
}