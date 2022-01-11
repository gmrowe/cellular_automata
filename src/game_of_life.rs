use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn next_cell_state(&self, living_neighbors: u8) -> Self {
        match self {
            Cell::Alive => {
                if living_neighbors < 2 || living_neighbors > 3 {
                    Cell::Dead
                } else {
                    Cell::Alive
                }
            }

            Cell::Dead => {
                if living_neighbors == 3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
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

    fn display_grid(&self) -> String {
        let symbols: Vec<char> = self
            .cells
            .iter()
            .map(|n| match n {
                Cell::Dead => '.',
                Cell::Alive => '*',
            })
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

    pub fn set_cell_at(&mut self, row: usize, col: usize, new_cell: Cell) {
        let index = row * self.width + col;
        self.cells[index] = new_cell;
    }

    pub fn toggle_cell_at(&mut self, row: usize, col: usize) {
        let curr_cell = self.cell_at(row, col);
        let toggled = curr_cell.toggle();
        self.set_cell_at(row, col, toggled);
    }

    pub fn clear(&mut self) {
        let all_dead = vec![Cell::Dead; self.cells.len()];
        self.cells = all_dead;
        self.generation = 0;
    }

    pub fn in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.height as isize && col >= 0 && col < self.width as isize
    }

    pub fn next_gen(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::with_capacity(self.cells.len());
        let mut living_neighbors: Vec<u8> = vec![0; self.cells.len()];
        for r in 0..self.height {
            for c in 0..self.width {
                if self.cell_at(r, c).is_alive() {
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr != 0 || dc != 0 {
                                let nrow = dr + r as isize;
                                let ncol = dc + c as isize;
                                if self.in_bounds(nrow, ncol) {
                                    let index = nrow as usize * self.width + ncol as usize;
                                    living_neighbors[index] += 1
                                }
                            }
                        }
                    }
                }
            }
        }

        for (cell, count) in self.cells.iter().zip(living_neighbors.iter()) {
            let new_cell = cell.next_cell_state(*count);
            new_cells.push(new_cell);
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
            "Generation 1:", "4 8", "........", "....*...", "...**...", "........"
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
        let universe_string = format!("{}\n{}\n{}\n{}", "Generation 1:", "2 2", ".*", "**");
        let mut universe = Universe::from_string(&universe_string);
        universe.next_gen();
        let expected_next_gen = format!("{}\n{}\n{}\n{}", "Generation 2:", "2 2", "**", "**");
        assert_eq!(expected_next_gen, universe.to_string());
    }
}
