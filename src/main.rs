extern crate piston_window;
mod game_of_life;
mod grid_view;
mod rng;

use crate::game_of_life::{Cell, Universe};
use crate::grid_view::{Controller, Entity, GridView, GridViewModel};
use crate::rng::Rng;

use piston_window::*;
use std::io;

const FPS: u64 = 60;
const UPS: u64 = 50;
const ROWS: usize = 50;
const COLS: usize = 50;

const ON_COLOR: [f32; 4] = color::BLUE;
const BACKGROUND_COLOR: [f32; 4] = color::BLACK;

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

struct GameOfLifeController {
    model: Universe,
}

impl GameOfLifeController {
    pub fn new(model: Universe) -> Self {
        Self { model }
    }

    fn build_view_model(&self) -> GridViewModel {
        let mut entities = Vec::new();
        for row in 0..self.model.height() {
            for col in 0..self.model.width() {
                if let Cell::Alive = self.model.cell_at(row, col) {
                    let entity = Entity::new(ON_COLOR, row, col);
                    entities.push(entity);
                }
            }
        }
        GridViewModel::new(
            self.model.height(),
            self.model.width(),
            entities,
            BACKGROUND_COLOR,
        )
    }
}

impl Controller for GameOfLifeController {
    fn update(&mut self) -> GridViewModel {
        self.model.next_gen();
        self.build_view_model()
    }

    fn mouse_click(&mut self, row: usize, col: usize) -> GridViewModel {
        self.model.toggle_cell_at(row, col);
        self.build_view_model()
    }
}

fn main() -> io::Result<()> {
    let window: PistonWindow = WindowSettings::new("piston: hello_world", [600, 600])
        .exit_on_esc(true)
        .build()
        .expect("Couldn't build window");

    let events = Events::new(EventSettings::new()).max_fps(FPS).ups(UPS);

    let cells = random_cells(ROWS * COLS);
    let universe = Universe::new(&cells, COLS);

    let mut view = GridView::new(window, events, GameOfLifeController::new(universe));
    view.game_loop();

    Ok(())
}
