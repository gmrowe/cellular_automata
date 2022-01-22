#[derive(Debug)]
pub struct ElemAutomata {
    cells: Vec<u8>,
    lookup_table: [u8; 8]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellState {
    Alive,
    Dead,
}

impl ElemAutomata {
    pub fn new(rule_no: u8, init_cells: &[u8]) -> Self {
        assert!(init_cells.len() > 0, "Must init with at lease one cell");
        Self {
            cells: init_cells.to_vec(),
            lookup_table: Self::build_lookup_table(rule_no),
        }
    }

    
    fn build_lookup_table(rule_no: u8) -> [u8; 8] {
        const BITS_PER_BYTE: usize = 8;
        let mut out = [0; BITS_PER_BYTE];
        let mut n = rule_no;
        for i in 0..BITS_PER_BYTE {
            out[i] = n & 1;
            n = n >> 1;
        }
        out
    }


    pub fn next_gen(&mut self) {
        let first = self.cells[0];
        let last = self.cells[self.cells.len() - 1];
        let padded_cells =
            std::iter::once(&last)
            .chain(self.cells.iter())
            .chain(std::iter::once(&first))
            .collect::<Vec<&u8>>();
        let mut new_cells = Vec::new();

        for chunk in padded_cells.windows(3) {
            let mut n = 0;
            for &bit in chunk.into_iter() {
                n = n << 1;
                n = n | *bit as usize;
            }
            new_cells.push(self.lookup_table[n]);
        }
        self.cells = new_cells;
        
    }

    pub fn cells(&self) -> Vec<CellState> {
        let cell_state = [CellState::Dead, CellState::Alive];
        self.cells.iter()
            .map(|&n| cell_state[n as usize])
            .collect()
    }
}

#[cfg(test)]
mod elementary_tests {
    use super::*;

    #[test]
    fn can_generate_next_gen_for_rule_110() {
        let rule_no = 110;
        let init = vec![1, 0, 1, 1, 0, 1];
        let mut ea = ElemAutomata::new(rule_no, &init);
        let expected = vec![
            1, // 110
            1, // 101
            1, // 011
            1, // 110
            1, // 101
            1, // 011
        ];
        ea.next_gen();
        assert_eq!(expected, ea.cells);
    }
}
