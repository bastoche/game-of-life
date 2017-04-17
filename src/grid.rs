use state::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Grid {
  cells: Vec<Vec<State>>
}

pub fn next_generation(grid: &Grid) -> Grid {
  let cells = grid.cells.iter().enumerate().map(
    |(y, line)| line.iter().enumerate().map(
      |(x, &state)| next_state(state, living_neighbours_count(&grid.cells, x, y))
    ).collect()
  ).collect();
  Grid { cells: cells }
}

impl Grid {
  pub fn from_string(representation: &str) -> Grid {
    let vec: Vec<&str> = representation.split("\n").collect();
    let cells = vec.iter().map(|s| {
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

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = String::new();
    for line in self.cells.iter() {
      for &state in line.iter() {
        let c = if state == State::Alive { '*' } else { '.' };
        result.push(c);
      }
      result.push('\n');
    }
    write!(f, "{}", result)
  }
}

fn living_neighbours_count(cells: &Vec<Vec<State>>, x: usize, y: usize) -> u8 {
  let height = cells.len();
  let width = cells[0].len();

  let cell_at = |x: usize, y: usize| -> State {
    cells[y % height][x % width]
  };

  [
    cell_at(x + width - 1, y + width - 1),
    cell_at(x + width - 1, y),
    cell_at(x + width - 1, y + 1),
    cell_at(x, y + width - 1),
    cell_at(x, y + 1),
    cell_at(x + 1, y + width - 1),
    cell_at(x + 1, y),
    cell_at(x + 1, y + 1),
  ].iter().fold(0, |sum, &state| sum + if state == State::Alive { 1 } else { 0 })
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
  fn next_generation_for_block() {
    let grid = Grid::from_string("....\n.**.\n.**.\n....");
    assert_eq!(grid, next_generation(&grid));
  }

  #[test]
  fn next_generation_for_basic_blinker() {
    let grid = Grid::from_string(".....\n.....\n.***.\n.....\n.....");
    let next_generation_grid = Grid::from_string(".....\n..*..\n..*..\n..*..\n.....");
    assert_eq!(next_generation_grid, next_generation(&grid));
  }
}