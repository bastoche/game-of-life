mod grid;
mod state;

use grid::*;

fn main() {
  let mut g = Grid::from_string(".....\n.....\n.***.\n.....\n.....");
  loop {
    println!("{}", g);
    g = next_generation(&g);
  }
}