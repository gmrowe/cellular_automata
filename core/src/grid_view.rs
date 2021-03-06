
extern crate piston_window;
extern crate image as im;

use piston_window::draw_state::DrawState;
use piston_window::*;

pub trait Controller {
    fn model(&self) -> GridViewModel;
    fn update(&mut self);
    fn mouse_click(&mut self, row: usize, col: usize);
    fn clear(&mut self);
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
    play_state: PlayState,
}

impl<C> GridView<C>
where
    C: Controller,
{
    pub fn new(title: &str, fps: u64, ups: u64, controller: C) -> Self {
        let window: PistonWindow = WindowSettings::new(title, [1920, 1080])
            .exit_on_esc(true)
            .build()
            .expect("Couldn't build window");

        let events = Events::new(EventSettings::new())
            .max_fps(fps)
            .ups(ups);
        
        Self {
            window,
            events,
            controller,
            play_state: PlayState::Running,
        }
    }

    fn cell_height(&self) -> f64 {
        let size = self.window.size();
        let model = self.controller.model();
        size.height / model.rows as f64
    }

    fn cell_width(&self) -> f64 {
        let size = self.window.size();
        let model = self.controller.model();
        size.width / model.cols as f64
    }

    fn to_u8_pixel(pix: &[f32; 4]) -> im::Rgba<u8> {
        const MAX_VAL: f32 = 255.0;
        let [r, g, b, a] = pix;
        let normalize = |c|  {
            (c * MAX_VAL) as u8
        };
        im::Rgba([normalize(r), normalize(g), normalize(b), normalize(a)])
    }

    fn render_view_model(&mut self, e: &Event) {
        let width = self.cell_width();
        let height = self.cell_height();
        let model = self.controller.model();

        let mut img = im::ImageBuffer::from_pixel(
            model.cols as u32,
            model.rows as u32,
            Self::to_u8_pixel(&model.background_color)
        );
        
        for entity in model.entities.iter() {
            let r = entity.row as u32;
            let c = entity.col as u32;
            img.put_pixel(c, r, Self::to_u8_pixel(&entity.color));
        }
        let mut texture_context = TextureContext {
            factory: self.window.factory.clone(),
            encoder: self.window.factory.create_command_buffer().into()
        };
        let texture = Texture::from_image(
                &mut texture_context,
                &img,
                &TextureSettings::new().mag(Filter::Nearest)
            ).expect("Couldn't build texture");
        self.window.draw_2d(e, |cxt, g, _device| {
            // clear(model.background_color, g);
            image(&texture, cxt.transform.scale(width, height), g);
        });
        
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
        self.controller.update();
    }

    fn handle_button_event(&mut self, _e: &Event, args: &ButtonArgs, pos: Option<[f64; 2]>) {
        if let Button::Keyboard(Key::Space) = args.button {
            if let ButtonState::Press = args.state {
                self.play_state = self.play_state.toggle();
            }
        }

        if let Button::Keyboard(Key::X) = args.button {
            if let ButtonState::Press = args.state {
                self.controller.clear();
            }
        }

        if let Button::Mouse(MouseButton::Left) = args.button {
            if let ButtonState::Press = args.state {
                if let Some([x, y]) = pos {
                    let row = (y / self.cell_height()).floor() as usize;
                    let col = (x / self.cell_width()).floor() as usize;
                    self.controller.mouse_click(row, col);
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

            if e.render_args().is_some() {
                self.render(&e);
            }

            if let PlayState::Running = self.play_state {
                if e.update_args().is_some() {
                    self.update(&e);
                }
            }

            if let Some(args) = e.button_args() {
                self.handle_button_event(&e, &args, last_cursor_pos);
            }
        }
    }
}
