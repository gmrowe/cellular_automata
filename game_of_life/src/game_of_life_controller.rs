use crate::game_of_life::{Cell, Universe};
use automata_core::grid_view::{Controller, Entity, GridViewModel};

pub struct GameOfLifeController {
    model: Universe,
    live_color: [f32; 4],
    dead_color: [f32; 4],
}

impl GameOfLifeController {
    pub fn new(model: Universe, live_color: [f32; 4], dead_color: [f32; 4]) -> Self {
        Self {
            model,
            live_color,
            dead_color,
        }
    }

    fn build_view_model(&self) -> GridViewModel {
        let mut entities = Vec::new();
        for row in 0..self.model.height() {
            for col in 0..self.model.width() {
                if let Cell::Alive = self.model.cell_at(row, col) {
                    let entity = Entity::new(self.live_color, row, col);
                    entities.push(entity);
                }
            }
        }
        GridViewModel::new(
            self.model.height(),
            self.model.width(),
            entities,
            self.dead_color,
        )
    }
}

impl Controller for GameOfLifeController {
    fn update(&mut self) {
        self.model.next_gen();
    }

    fn mouse_click(&mut self, row: usize, col: usize) {
        self.model.toggle_cell_at(row, col);
    }

    fn clear(&mut self) {
        self.model.clear();
    }

    fn model(&self) -> GridViewModel {
        self.build_view_model()
    }
}
