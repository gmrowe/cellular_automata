use automata_core::grid_view::{Controller, GridViewModel, Entity};
use std::collections::VecDeque;
use crate::elementary::{CellState, ElemAutomata};

pub struct ElemAutomataController {
    model: ElemAutomata,
    max_height: usize,
    width: usize,
    live_color: [f32; 4],
    dead_color: [f32; 4],
    generations: VecDeque<Vec<CellState>>,
}

impl ElemAutomataController {
    pub fn new(
        model: ElemAutomata,
        max_height: usize,
        live_color: [f32; 4],
        dead_color: [f32; 4]
    ) -> Self {
        let width = model.cells().len();
        Self {
            model,
            max_height,
            live_color,
            dead_color,
            width,
            generations: VecDeque::new(),
        }
    }
    
    fn build_view_model(&self) -> GridViewModel {
        let mut entities = Vec::new();
        for (r, row) in self.generations.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == CellState::Alive {
                    let entity = Entity::new(self.live_color, r, c);
                    entities.push(entity);
                }
            }
        }
        GridViewModel::new(
            self.max_height,
            self.width,
            entities,
            self.dead_color
        )
    }
}


impl Controller for ElemAutomataController {
    fn update(&mut self) {
        self.generations.push_back(self.model.cells());
        if self.generations.len() > self.max_height {
            self.generations.pop_front();
        }
        self.model.next_gen();
    }

    fn mouse_click(&mut self, _row: usize, _col: usize) {
        // no-op
    }

    fn clear(&mut self) {
        // no-op
    }

    fn model(&self) -> GridViewModel {
        self.build_view_model()
    }
}

