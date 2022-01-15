use crate::game_of_life::{Cell, Universe};
use crate::grid_view::{Controller, Entity, GridViewModel};

const ON_COLOR: [f32; 4] = [0.0, 0.0, 255.0, 1.0]; // BLUE
const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; //BLACK

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
