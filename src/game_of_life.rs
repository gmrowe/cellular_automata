use std::fmt;

pub struct Rng {
    seed: u64
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
        }
    }
    
    pub fn next_rnd(&mut self) -> u32 {
        const M: u64 = 1 << 48;
        const A: u64 = 25214903917;
        const C: u64 = 11;
        self.seed = A.wrapping_mul(self.seed).wrapping_add(C) % M;
        ((self.seed >> 16) & 0xFFFFFFFF) as u32
    }
}

impl Iterator for Rng {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_rnd())     
    }


}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
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
    
pub struct Universe {
    generation: u32,
    height: usize,
    width: usize,
    cells: Vec<Cell>,
}

impl Universe {
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

    pub fn new(cell_slice: &[Cell], width: usize) -> Self {
        let height = cell_slice.len() / width;
        let mut cells = Vec::new();
        for &cell in cell_slice {
            cells.push(cell);
        }
        Self {
            generation: 0,
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

    pub fn next_gen(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::new();
        for r in 0..self.height {
            for c in 0..self.width {
                let living_neighbors = self.count_living_neighbors(r, c);
                let old_cell = self.cell_at(r, c);
                let new_cell = old_cell.next_cell_state(living_neighbors);
                new_cells.push(new_cell);
            }
        }

        self.generation += 1;
        self.cells = new_cells;
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
