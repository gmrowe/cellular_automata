extern crate piston_window;

use piston_window::draw_state::DrawState;
use piston_window::*;

pub trait Controller {
    fn update(&mut self) -> GridViewModel;
    fn mouse_click(&mut self, row: usize, col: usize) -> GridViewModel;
    fn clear(&mut self) -> GridViewModel;
}

pub struct Entity {
    color: [f32; 4],
    row: usize,
    col: usize,
}

impl Entity {
    pub fn new(color: [f32; 4], row: usize, col: usize) -> Self {
        Self { color, row, col }
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
        background_color: [f32; 4],
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
    C: Controller,
{
    window: PistonWindow,
    events: Events,
    controller: C,
    view_model: Option<GridViewModel>,
    play_state: PlayState,
}

impl<C> GridView<C>
where
    C: Controller,
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

    fn cell_height(&self) -> f64 {
        let size = self.window.size();
        self.view_model
            .as_ref()
            .map(|model| size.height / model.rows as f64)
            .unwrap_or(size.height)
    }

    fn cell_width(&self) -> f64 {
        let size = self.window.size();
        self.view_model
            .as_ref()
            .map(|model| size.width / model.cols as f64)
            .unwrap_or(size.width)
    }

    fn render_view_model(&mut self, e: &Event) {
        if let Some(model) = &self.view_model {
            let width = self.cell_width();
            let height = self.cell_height();
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

    fn render_pause(&mut self, e: &Event) {
        const PAUSE_BAR_W: f64 = 30.0;
        const PAUSE_BAR_H: f64 = 100.0;
        const PAUSE_GAP: f64 = 20.0;
        const PAUSE_CORNER_RAD: f64 = 10.5;
        const PAUSE_BAR_ALPHA: f32 = 0.15;
        const PAUSE_BAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, PAUSE_BAR_ALPHA]; // 0xCCCCC
        let win_size = self.window.size();
        
        let left_bar_dims = [
            (win_size.width - PAUSE_GAP) / 2.0 - PAUSE_BAR_W,
            (win_size.height - PAUSE_BAR_H) / 2.0,
            PAUSE_BAR_W,
            PAUSE_BAR_H,
        ];
        let right_bar_dims = [
            (win_size.width + PAUSE_GAP) / 2.0,
            (win_size.height - PAUSE_BAR_H) / 2.0,
            PAUSE_BAR_W,
            PAUSE_BAR_H,
        ];
        let rect = Rectangle::new_round(PAUSE_BAR_COLOR, PAUSE_CORNER_RAD);
        
        self.window.draw_2d(e, |cxt, g, _device| {
            rect.draw(left_bar_dims, &DrawState::default(), cxt.transform, g);
            rect.draw(right_bar_dims, &DrawState::default(), cxt.transform, g);
        });
    }

    fn render(&mut self, e: &Event) {
        match self.play_state {
            PlayState::Running => self.render_view_model(e),
            PlayState::Paused => {
                self.render_view_model(e);
                self.render_pause(e);
            }
        }
    }

    fn update(&mut self, _e: &Event) {
        self.view_model = Some(self.controller.update());
    }

    fn handle_button_event(&mut self, _e: &Event, args: &ButtonArgs, pos: Option<[f64; 2]>) {
        if let Button::Keyboard(Key::Space) = args.button {
            if let ButtonState::Press = args.state {
                self.play_state = self.play_state.toggle();
            }
        }

        if let Button::Keyboard(Key::X) = args.button {
            if let ButtonState::Press = args.state {
                self.view_model = Some(self.controller.clear());
            }
        }

        if let Button::Mouse(MouseButton::Left) = args.button {
            if let ButtonState::Press = args.state {
                if let Some([x, y]) = pos {
                    let row = (y / self.cell_height()).floor() as usize;
                    let col = (x / self.cell_width()).floor() as usize;
                    self.view_model = Some(self.controller.mouse_click(row, col));
                }
            }
        }
    }

    pub fn game_loop(&mut self) {
        let mut last_cursor_pos = None;
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(pos) = e.mouse_cursor_args() {
                last_cursor_pos = Some(pos);
            }

            if let Some(_) = e.render_args() {
                self.render(&e);
            }

            if let PlayState::Running = self.play_state {
                if let Some(_) = e.update_args() {
                    self.update(&e);
                }
            }

            if let Some(args) = e.button_args() {
                self.handle_button_event(&e, &args, last_cursor_pos);
            }
        }
    }
}
