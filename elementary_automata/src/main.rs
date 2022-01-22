mod elementary;
mod elementary_controller;

use elementary::ElemAutomata;
use elementary_controller::ElemAutomataController;
use automata_core::grid_view::GridView;

struct ElemAutomataApp {
    rule_no: u8,
    fps: u64,
    ups: u64,
    rows: usize,
    cols: usize,
    live_color: [f32; 4],
    dead_color: [f32; 4],
}

impl ElemAutomataApp {
    pub fn new(rule_no: u8) -> Self {
        Self {
            rule_no,
            fps: 30,
            ups: 10,
            rows: 50,
            cols: 90,
            live_color: [1.0, 1.0, 1.0, 1.0], // WHITE
            dead_color: [0.0, 0.0, 0.0, 1.0], //BLACK
        }
    }

    pub fn fps(&self, fps: u64) -> Self {
        Self {
            fps,
            ..*self
        }
    }

    pub fn ups(&self, ups: u64) -> Self {
        Self {
            ups,
            ..*self
        }        
    }

    pub fn rows(&self, rows: usize) -> Self {
        Self {
            rows,
            ..*self
        }        
    }

    pub fn cols(&self, cols: usize) -> Self {
        Self {
            cols,
            ..*self
        }        
    }

    pub fn live_color(&self, live_color: [f32; 4]) -> Self {
        Self {
            live_color,
            ..*self
        }        
    }

    pub fn dead_color(&self, dead_color: [f32; 4]) -> Self {
        Self {
            dead_color,
            ..*self
        }        
    }
    
    pub fn start(&self) {
        let mut cells = vec![0; self.cols];
        cells[self.cols / 2] = 1;
        let universe = ElemAutomata::new(self.rule_no, &cells);
        let controller = ElemAutomataController::new(
                universe, self.rows, self.live_color, self.dead_color);

        let mut view = GridView::new(
            &format!("Rule {}", self.rule_no),
            self.fps,
            self.ups,
            controller
        );
        view.game_loop();    
    }
}


fn hex_to_rgba(hex_color: u32) -> [f32; 4] {
    //rrggbb
    let r = (hex_color >> (8 * 2)) & 0xFF;
    let g = (hex_color >> (8 * 1)) & 0xFF;
    let b = (hex_color >> (8 * 0)) & 0xFF;
    const MAX_HEX: f32 = 255.0;
    [r as f32 / MAX_HEX, g as f32 / MAX_HEX, b as f32 / MAX_HEX, 1.0]
}

fn main() {
    const RULE_NO: u8 = 90;
    const ROWS: usize = 10 * 25;
    const COLS: usize = 18 * 25;
    const UPS: u64 = 12;
    const FPS: u64 = 60;
    let bluish: [f32; 4] = hex_to_rgba(0x4ca8bf);
    let golden: [f32; 4] = hex_to_rgba(0xbfa84c);
    
    ElemAutomataApp::new(RULE_NO)
        .rows(ROWS)
        .cols(COLS)
        .ups(UPS)
        .fps(FPS)
        .live_color(bluish)
        .dead_color(golden)
        .start();
    
}
