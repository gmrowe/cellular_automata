extern crate piston_window;
mod game_of_life;
mod game_of_life_controller;
mod grid_view;
mod rng;

use crate::game_of_life::{Cell, Universe};
use crate::game_of_life_controller::{GameOfLifeController};
use crate::grid_view::GridView;
use crate::rng::Rng;

use piston_window::*;
use std::io;

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
struct GameOfLifeApp {
    fps: u64,
    ups: u64,
    rows: usize,
    cols: usize,
}

impl GameOfLifeApp {
    pub fn new() -> Self {
        Self {
            fps: 60,
            ups: 10,
            rows: 100,
            cols: 180,
        }
    }

    pub fn start(&self) {
        let window: PistonWindow =
            WindowSettings::new("Conway's game of life", [1920, 1080])
            .exit_on_esc(true)
            .build()
            .expect("Couldn't build window");

        let events = Events::new(EventSettings::new())
            .max_fps(self.fps)
            .ups(self.ups);

        let cells = random_cells(self.rows * self.cols);
        let universe = Universe::new(&cells, self.cols);

        let mut view = GridView::new(window, events, GameOfLifeController::new(universe));
        view.game_loop();
       
    }
}

fn main() -> io::Result<()> {
    GameOfLifeApp::new().start();
    Ok(())
}
