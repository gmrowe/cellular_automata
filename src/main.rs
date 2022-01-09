extern crate piston_window;
mod game_of_life;
mod grid_view;

use crate::game_of_life::{Cell, Rng, Universe};
use crate::grid_view::GridView;
use piston_window::*;
use std::io;

const FPS: u64 = 60;
const UPS: u64 = 50;
const ROWS: usize = 500;
const COLS: usize = 500;

fn random_cells(num: usize) -> Vec<Cell> {
    let seed = 96155;
    let mut rng = Rng::new(seed);
    let mut cells = Vec::new();
    for _ in 0..num {
        if let Some(n) = rng.next() {
            let cell = match n % 2 {
                0 => Cell::Alive,
                1 => Cell::Dead,
                _ => unreachable!("i % 2 can only be 0 or 1"),
            };
            cells.push(cell);
        }
    }
    cells
}

fn main() -> io::Result<()> {
    let window: PistonWindow = WindowSettings::new("piston: hello_world", [600, 600])
        .exit_on_esc(true)
        .build()
        .expect("Couldn't build window");

    let events = Events::new(EventSettings::new()).max_fps(FPS).ups(UPS);

    let cells = random_cells(ROWS * COLS);
    let universe = Universe::new(&cells, COLS);

    let mut view = GridView::new(window, events, universe, ROWS, COLS);
    view.game_loop();

    Ok(())
}
