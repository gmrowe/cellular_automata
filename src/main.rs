use std::fmt;

fn main() {
   let input = "\
Generation 1:
7 16
............*..*
*...*..***....*.
*..**......**...
.....*..*..****.
*..**......**...
.....*..*..****.
............*..*";

    let num_generations = 20;
    let mut universe = Universe::from_string(input);

    for _ in 0..num_generations {
        println!("{}\r", universe.to_string());
        universe = universe.next_gen();
    }
    
    
}

#[derive(Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {

    pub fn next_cell_state(&self, living_neighbors: u8) -> Cell {
        match self {
            Cell::Alive => if living_neighbors < 2 || living_neighbors > 3  {
                Cell::Dead
            } else {
                Cell::Alive
            },
            
            Cell::Dead => if living_neighbors == 3 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead  => false,
        }
    }
}    
    

struct Universe {
    generation: u32,
    height: usize,
    width: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn parse_generation_number_from_header(header: &str) -> u32 {
        let generation_number_string: String =
            header.chars().filter(|&c| c.is_ascii_digit()).collect();

        generation_number_string.parse().unwrap()
    }

    fn parse_height_width_from_header(header: &str) -> (usize, usize) {
        let height_width: Vec<&str> = header.split_whitespace().collect();
        let height: usize = height_width[0].parse().unwrap();
        let width: usize = height_width[1].parse().unwrap();
        (height, width)
    }

    fn parse_cells_from_grid(grid: &[&str], height: usize, width: usize) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::with_capacity(height * width);

        for row in grid {
            for c in row.chars() {
                if c == '.' {
                    cells.push(Cell::Dead);
                } else {
                    cells.push(Cell::Alive);
                }
            }
        }
        cells
    }

    fn display_grid(&self) -> String {
        let symbols: Vec<char> = self
            .cells
            .iter()
            .map(|n| match n { Cell::Dead =>  '.' , Cell::Alive => '*' })
            .collect();

        symbols
            .chunks(self.width())
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn from_string(world_string: &str) -> Self {
        let lines: Vec<&str> = world_string.lines().collect();
        let generation_header = lines[0];
        let height_width_header = lines[1];
        let cell_grid = &lines[2..];

        let generation = Universe::parse_generation_number_from_header(generation_header);
        let (height, width) = Universe::parse_height_width_from_header(height_width_header);
        let cells: Vec<Cell> = Universe::parse_cells_from_grid(cell_grid, height, width);

        Self {
            generation,
            height,
            width,
            cells,
        }
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn cell_at(&self, row: usize, col: usize) -> &Cell {
        let index = row * self.width + col;
        &self.cells[index]
    }

    pub fn in_bounds(&self, row: isize, col: isize)  -> bool {
        row >= 0
            && row < self.height as isize
            && col >= 0
            && col < self.width as isize
    }

    pub fn count_living_neighbors(&self, row: usize, col: usize) -> u8 {
        let deltas: Vec<(isize, isize)> =
            vec![(-1, -1), (-1, 0), (-1, 1),
                 (0, -1),  /*cell*/ (0, 1),
                 (1, -1),  (1, 0),  (1, 1)];
        
        let irow = row as isize;
        let icol = col as isize;
        
        deltas.into_iter()
            .filter(|(dr, dc)| self.in_bounds(irow + dr, icol + dc))
            .filter(|(dr, dc)| {
                self.cell_at((irow + dr) as usize, (icol + dc) as usize)
                    .is_alive()
            })
            .count() as u8
    }

    pub fn next_gen(&self) -> Self {
        let mut new_cells: Vec<Cell> = Vec::new();
        for r in 0..self.height {
            for c in 0..self.width {
                let living_neighbors = self.count_living_neighbors(r, c);
                let old_cell = self.cell_at(r, c);
                let new_cell = old_cell.next_cell_state(living_neighbors);
                new_cells.push(new_cell);
            }
        }
        
        Self {
            generation: self.generation + 1,
            cells: new_cells,
            ..*self
        }
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generation {}:\n{} {}\n{}",
            self.generation(),
            self.height(),
            self.width(),
            self.display_grid()
        )
    }
}

#[cfg(test)]
mod game_of_life_tests {
    use super::*;

    fn simple_universe_string() -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            "Generation 1:",
            "4 8",
            "........",
            "....*...",
            "...**...",
            "........"
        )
    }

    fn simple_universe() -> Universe {
        Universe::from_string(&simple_universe_string())
    }

    #[test]
    fn a_universe_stores_its_generation_number() {
        assert_eq!(1, simple_universe().generation());
    }

    #[test]
    fn a_universe_stores_its_height() {
        assert_eq!(4, simple_universe().height());
    }

    #[test]
    fn a_universe_storese_its_width() {
        assert_eq!(8, simple_universe().width());
    }

    #[test]
    fn a_universe_stores_its_cells() {
        assert_eq!(32, simple_universe().cell_count());
    }

    #[test]
    fn a_universe_can_query_dead_cell_state_by_row_col() {
        assert_eq!(&Cell::Dead, simple_universe().cell_at(0, 0));
    }

    #[test]
    fn a_universe_can_query_living_cell_state_by_row_col() {
        assert_eq!(&Cell::Alive, simple_universe().cell_at(1, 4));
    }

    #[test]
    fn a_universe_can_ouput_a_string_representation_of_itself() {
        assert_eq!(simple_universe_string(), simple_universe().to_string());
    }

    #[test]
    fn a_live_cell_with_fewer_than_two_neighbors_dies() {
        let living_neighbors = 1;
        let cell = Cell::Alive;
        let next_cell_state = cell.next_cell_state(living_neighbors);
        assert_eq!(Cell::Dead, next_cell_state);
    }

    #[test]
    fn a_live_cell_with_more_than_three_neighbors_dies() {
        let living_neighbors = 4;
        let cell = Cell::Alive;
        let next_cell_state = cell.next_cell_state(living_neighbors);
        assert_eq!(Cell::Dead, next_cell_state);
    }

    #[test]
    fn a_live_cell_with_three_neighbors_lives() {
        let living_neighbors = 3;
        let cell = Cell::Alive;
        let next_cell_state = cell.next_cell_state(living_neighbors);
        assert_eq!(Cell::Alive, next_cell_state);
    }

    #[test]
    fn a_dead_cell_with_exactly_three_neighbors_comes_to_life() {
        let living_neighbors = 3;
        let cell = Cell::Dead;
        let next_cell_state = cell.next_cell_state(living_neighbors);
        assert_eq!(Cell::Alive, next_cell_state);
    }

    #[test]
    fn a_universe_can_advance_a_generation() {
        let universe_string =
            format!("{}\n{}\n{}\n{}",
                    "Generation 1:",
                    "2 2",
                    ".*",
                    "**");
        let universe = Universe::from_string(&universe_string);
        let next_gen = universe.next_gen();
        let expected_next_gen =
            format!("{}\n{}\n{}\n{}",
                    "Generation 2:",
                    "2 2",
                    "**",
                    "**");
        assert_eq!(expected_next_gen, next_gen.to_string());
    }
    
}
