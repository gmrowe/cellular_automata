extern crate piston_window;

use piston_window::*;

pub trait Controller {
    fn update(&mut self) -> GridViewModel;
    fn mouse_click(&mut self, row: usize, col: usize) -> GridViewModel;
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

#[derive(PartialEq, Debug)]
enum PlayState {
    Paused,
    Running,
}

impl PlayState {
    fn toggle(&self) -> Self {
        match self {
            Self::Paused => Self::Running,
            Self::Running => Self::Paused,
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
    play_state: PlayState,
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
            play_state: PlayState::Running,
            view_model: None,
        }
    }

    fn render(&mut self, e: &Event) {
        if let Some(model) = &self.view_model {
            let size = self.window.size();
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

    fn update(&mut self, _e: &Event) {
        self.view_model = Some(self.controller.update());
        
    }

    fn handle_button_event(
        &mut self,
        e: &Event,
        args: &ButtonArgs,
        pos: Option<[f64; 2]>
    ) {
        if let Button::Keyboard(Key::Space) = args.button {
            if let ButtonState::Press = args.state {
                self.play_state = self.play_state.toggle();  
            }
        }

        if let Button::Mouse(MouseButton::Left) = args.button {
            if let ButtonState::Press = args.state {
                if let Some([x, y]) = pos {
                    let size = self.window.size();
                    if let Some(model) = &self.view_model{
                        let height = size.height / model.rows as f64;
                        let width = size.width / model.cols as f64;
                        let row = (y / height).floor() as usize;
                        let col = (x / width).floor() as usize;
                        self.view_model = Some(self.controller.mouse_click(row, col));
                        self.render(e)
                    }
                }
            }
        }
    }

    pub fn game_loop(&mut self) {
        let mut last_cursor_pos = None;
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(_) = e.render_args() {
                self.render(&e);
            }
            
            if let PlayState::Running = self.play_state {
                if let Some(_) = e.update_args() {
                   self.update(&e);
                }
            }
           
            
            if let Some(pos) = e.mouse_cursor_args() {
               last_cursor_pos = Some(pos);
            }
            
            if let Some(args) = e.button_args() {
                self.handle_button_event(&e, &args, last_cursor_pos);
           }
        }
    }
}

