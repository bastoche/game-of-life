fn main() {
  println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
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

fn next_generation(grid: &Grid) -> Grid {
  Grid { cells: Vec::new() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn next_generation_when_all_cells_are_dead() {
    let grid = Grid::from_string("...");
    assert_eq!(grid, next_generation(&grid));
  }

  #[test]
  fn from_string() {
    let representation = "...";
    let grid = Grid::from_string(representation);
    assert_eq!(vec!(vec![State::Dead, State::Dead, State::Dead]), grid.cells);
  }
}