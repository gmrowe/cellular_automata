extern crate piston_window;

use piston_window::*;

pub trait Controller {
    fn update(&mut self, e: &Event) -> GridViewModel;
}

pub struct Entity {
    color: [f32; 4],
    row: usize,
    col: usize,
}

impl Entity {
    pub fn new(color: [f32; 4], row: usize, col: usize) -> Self {
        Self {
            color,
            row,
            col,
        }
    }
}

pub struct GridViewModel {
    rows: usize,
    cols: usize,
    entities: Vec<Entity>,
    background_color: [f32; 4],
}

impl GridViewModel {
    pub fn new(
        rows: usize,
        cols: usize,
        entities: Vec<Entity>,
        background_color: [f32; 4]
    ) -> Self {
        Self {
            rows,
            cols,
            entities,
            background_color,
        }
    }
}

pub struct GridView<C>
where
    C: Controller
{
    window: PistonWindow,
    events: Events,
    controller: C,
    view_model: Option<GridViewModel>,
}

impl<C> GridView<C>
where
    C: Controller
{
    pub fn new(window: PistonWindow, events: Events, controller: C) -> Self {
        Self {
            window,
            events,
            controller,
            view_model: None,
        }
    }

    fn render(&mut self, e: &Event, _args: &RenderArgs) {
        let size = self.window.size();
        if let Some(model) = &self.view_model {
            let height = size.height / model.rows as f64;
            let width = size.width / model.cols as f64;
            self.window.draw_2d(e, |cxt, g, _device| {
                clear(model.background_color, g);

                for entity in model.entities.iter() {
                    let r = entity.row as f64;
                    let c = entity.col as f64;
                    let dims = [c * width, r * height, width, height];
                    rectangle(entity.color, dims, cxt.transform, g);                   
                }

            });
        }
    }

    fn update(&mut self, e: &Event, _args: &UpdateArgs) {
        self.view_model = Some(self.controller.update(&e));
        
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
