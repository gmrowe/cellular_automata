use std::fmt;
use std::io;
use std::io::Read;

use crate::game_of_life::Universe;


mod game_of_life;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
 
    let num_generations = 20;
    let mut universe = Universe::from_string(&input);

    for _ in 0..num_generations {
        println!("{}\r", universe.to_string());
        universe = universe.next_gen();
    }
    Ok(())
}
