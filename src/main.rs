extern crate piston_window;
use crate::game_of_life::{Universe, Cell, Rng};

use std::io;
use piston_window::*;
mod game_of_life;

const ROWS: usize = 100;
const COLS: usize = 100;
const ON_COLOR: [f32; 4] = color::RED;
const OFF_COLOR: [f32; 4] = color::BLACK;
const FPS: u64 = 60;
const UPS: u64 = 50;


struct View {
    window: PistonWindow,
    events: Events,
    universe: Universe,
}

impl View {   
    fn render(&mut self, e: &Event, _args: &RenderArgs) {
        let size = self.window.size();
        let height = size.height / ROWS as f64;
        let width = size.width / COLS as f64;
        self.window.draw_2d(e, |cxt, g, _device| {
            clear(OFF_COLOR, g);

            let rect = Rectangle::new(ON_COLOR);
            for row in 0..ROWS {
                for col in 0..COLS {
                    let r = row as f64;
                    let c = col as f64;
                    if *self.universe.cell_at(row, col) == Cell::Alive {
                        let dims = [c * width, r * height, width, height];
                        rect.draw(dims, &cxt.draw_state, cxt.transform, g);
                    }
                }
            }

        });
    }

    fn update(&mut self, _e: &Event, _args: &UpdateArgs) {
        self.universe.next_gen();
    }

    fn game_loop(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&e, &args);
            }
            
            if let Some(args) = e.update_args() {
                self.update(&e, &args);
            }
        }
    }
}

fn random_cells(num: usize) -> Vec<Cell> {  
    let seed = 112975;
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
    let window: PistonWindow = WindowSettings::new(
        "piston: hello_world",
        [600, 600])
        .exit_on_esc(true)
        .build()
        .expect("Couldn't build window");

    let events = Events::new(EventSettings::new())
        .max_fps(FPS)
        .ups(UPS);

    let cells = random_cells(ROWS * COLS);
    let universe = Universe::new(&cells, COLS);

    let mut view = View {
        window,
        events,
        universe,
    };
    view.game_loop();

    Ok(())
}
