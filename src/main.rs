mod grid;
mod state;

use grid::*;
use std::io::*;

fn main() {
  let mut g = Grid::from_string(".....\n.....\n.***.\n.....\n.....");
  loop {
    println!("{}", g);
    g = next_generation(&g);
    println!("Press enter to see the next generation...");
    let _ = stdin().read(&mut [0u8]).unwrap();
  }
}