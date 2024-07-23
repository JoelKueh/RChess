
use std::sync::OnceLock;

mod normal;
mod magical;

pub struct MoveTables {
    knight_attacks: [u64; 64],
    bishop_attacks: [magical::MagicalTable; 64],
    rook_attacks: [magical::MagicalTable; 64],
    king_attacks: [u64; 64],
}

impl MoveTables {
    pub fn get_instance() -> &'static Self {
        static TABLES: OnceLock<MoveTables> = OnceLock::new();
        TABLES.get_or_init(|| MoveTables::new())
    }

    /// Reads the knight attack table. Must not be passed in a square value greater than 64.
    pub unsafe fn read_knight_attacks(&self, sq: u8) -> u64 {
        return *self.knight_attacks.get_unchecked(sq as usize);
    }

    /// Reads the bishop attack table. Must not be passed in a square value greater than 64.
    pub unsafe fn read_bishop_attacks(&self, sq: u8, occupancy: u64) -> u64 {
        return magical::read_magical_table(
            self.bishop_attacks.get_unchecked(sq as usize),
            occupancy
        );
    }

    /// Reads the rook attack table. Must not be passed in a square value greater than 64.
    pub unsafe fn read_rook_attacks(&self, sq: u8, occupancy: u64) -> u64 {
        return magical::read_magical_table(
            self.rook_attacks.get_unchecked(sq as usize),
            occupancy
        );
    }

    /// Reads the king attack table. Must not be passed in a square value greater than 64.
    pub unsafe fn read_king_attacks(&self, sq: u8) -> u64 {
        return *self.king_attacks.get_unchecked(sq as usize);
    }
}

impl MoveTables {
    fn new() -> Self {
        Self {
            knight_attacks: normal::gen_knight_attack_table(),
            bishop_attacks: magical::gen_bishop_attack_table(),
            rook_attacks: magical::gen_rook_attack_table(),
            king_attacks: normal::gen_king_attack_table()
        }
    }
}

