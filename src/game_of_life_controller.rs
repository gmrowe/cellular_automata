 use piston_window::*;
use crate::game_of_life::{Cell, Universe};
use crate::grid_view::{Controller, Entity, GridViewModel};

pub const ROWS: usize = 50;
pub const COLS: usize = 50;

pub const ON_COLOR: [f32; 4] = color::BLUE;
pub const BACKGROUND_COLOR: [f32; 4] = color::BLACK;

pub struct GameOfLifeController {
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

    fn clear(&mut self) -> GridViewModel {
        for row in 0..self.model.height() {
            for col in 0..self.model.width() {
                self.model.set_cell_at(row, col, Cell::Dead);
            }
            
        }
        self.build_view_model()
    }
}
