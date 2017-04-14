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


fn next_state(state: &State, living_neighbours_count: u8) -> State {
  match (state, living_neighbours_count) {
    (&State::Dead, 3) => State::Alive,
    (&State::Alive, c) if 2 <= c && c <= 3 => State::Alive,
    _ => State::Dead
  }
}

fn living_neighbours_count(cells: &Vec<Vec<State>>, x: usize, y: usize) -> u8 {
  let height = cells.len();
  let width = cells[0].len();
  [
    cell_at(&cells, x + width - 1, y + width - 1, width, height),
    cell_at(&cells, x + width - 1, y, width, height),
    cell_at(&cells, x + width - 1, y + 1, width, height),
    cell_at(&cells, x, y + width - 1, width, height),
    cell_at(&cells, x, y + 1, width, height),
    cell_at(&cells, x + 1, y + width - 1, width, height),
    cell_at(&cells, x + 1, y, width, height),
    cell_at(&cells, x + 1, y + 1, width, height),
  ].iter().fold(0, |sum, state| sum + if state == &&State::Alive { 1 } else { 0 })
}

fn cell_at(cells: &Vec<Vec<State>>, x: usize, y: usize, width: usize, height: usize) -> &State {
  &cells[y % height][x % width]
}

fn next_generation(grid: &Grid) -> Grid {
  let cells = grid.cells.iter().enumerate().map(
    |(y, line)| line.iter().enumerate().map(
      |(x, state)| next_state(state, living_neighbours_count(&grid.cells, x, y))
    ).collect()
  ).collect();
  Grid { cells: cells }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_string() {
    let representation = "...";
    let grid = Grid::from_string(representation);
    assert_eq!(vec![vec![State::Dead, State::Dead, State::Dead]], grid.cells);
  }

  #[test]
  fn a_live_cell_with_less_than_2_living_neighbours_will_die() {
    assert_eq!(State::Dead, next_state(&State::Alive, 0));
    assert_eq!(State::Dead, next_state(&State::Alive, 1));
  }

  #[test]
  fn a_live_cell_with_2_or_3_living_neighbours_will_stay_alive() {
    assert_eq!(State::Alive, next_state(&State::Alive, 2));
    assert_eq!(State::Alive, next_state(&State::Alive, 3));
  }

  #[test]
  fn a_live_cell_with_more_than_3_living_neighbours_will_die() {
    assert_eq!(State::Dead, next_state(&State::Alive, 4));
    assert_eq!(State::Dead, next_state(&State::Alive, 5));
    assert_eq!(State::Dead, next_state(&State::Alive, 6));
    assert_eq!(State::Dead, next_state(&State::Alive, 7));
    assert_eq!(State::Dead, next_state(&State::Alive, 8));
  }

  #[test]
  fn a_dead_cell_with_3_living_neighbours_will_live() {
    assert_eq!(State::Alive, next_state(&State::Dead, 3));
  }

  #[test]
  fn living_neighbours_count_should_count_alive_neighbours() {
    let cells = vec![
      vec![State::Alive, State::Dead, State::Alive],
      vec![State::Dead, State::Dead, State::Dead],
      vec![State::Dead, State::Alive, State::Dead]
    ];
    assert_eq!(3, living_neighbours_count(&cells, 1, 1));
  }

  #[test]
  fn next_generation_when_all_cells_are_dead() {
    let grid = Grid::from_string("...\n...\n...");
    assert_eq!(grid, next_generation(&grid));
  }

  #[test]
  fn next_generation_for_basic_blinker() {
    let grid = Grid::from_string(".....\n.....\n.***.\n.....\n.....");
    let next_generation_grid = Grid::from_string(".....\n..*..\n..*..\n..*..\n.....");
    assert_eq!(next_generation_grid, next_generation(&grid));
  }
}