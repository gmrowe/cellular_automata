pub struct Rng {
    seed: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { seed }
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
