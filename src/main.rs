use std::io;
use std::io::Read;
use std::time::Duration;   
use crate::game_of_life::Universe;

mod game_of_life;


fn output_animation_to_console(mut universe: Universe, frames: u32, delay: Duration) {
    let esc = 27 as char;
    let clr_screen = format!("{}[2J", esc);
    let mv_cursor_top_right = format!("{}[1;1H", esc);

    print!("{}", clr_screen);
    for _ in 0..frames {
        print!("{}", mv_cursor_top_right);
        print!("{}", universe.to_string());
        universe = universe.next_gen();
        std::thread::sleep(delay);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let universe = Universe::from_string(&input);
    let frames = 50;
    let delay = Duration::from_millis(200);
    output_animation_to_console(universe, frames, delay);
    
    Ok(())
}
