extern crate piston_window;

use crate::game_of_life;

use game_of_life::{Cell, Universe};
use piston_window::*;

const ON_COLOR: [f32; 4] = color::BLUE;
const OFF_COLOR: [f32; 4] = color::BLACK;

pub struct GridView {
    window: PistonWindow,
    events: Events,
    universe: Universe,
    rows: usize,
    cols: usize,
}

impl GridView {
    pub fn new(
        window: PistonWindow,
        events: Events,
        universe: Universe,
        rows: usize,
        cols: usize
    ) -> Self {
        Self {
            window,
            events,
            universe,
            rows,
            cols
        }
    }
    
    fn render(&mut self, e: &Event, _args: &RenderArgs) {
        let size = self.window.size();
        let height = size.height / self.rows as f64;
        let width = size.width / self.cols as f64;
        self.window.draw_2d(e, |cxt, g, _device| {
            clear(OFF_COLOR, g);

            let rect = Rectangle::new(ON_COLOR);
            for row in 0..self.rows {
                for col in 0..self.cols {
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

    pub fn game_loop(&mut self) {
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
