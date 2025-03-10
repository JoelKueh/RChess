
use std::sync::OnceLock;

mod normal;
mod magical;

#[cfg(test)]
mod tests;

// TODO: Move later I'm lazy. This is also in "normal".
pub const DIR_R: u8     = 0;
pub const DIR_UR: u8    = 1;
pub const DIR_U: u8     = 2;
pub const DIR_UL: u8    = 3;
pub const DIR_L: u8     = 4;
pub const DIR_DL: u8    = 5;
pub const DIR_D: u8     = 6;
pub const DIR_DR: u8    = 7;

pub struct MoveTables {
    pawn_attacks: [[u64; 64]; 2],
    knight_attacks: [u64; 64],
    bishop_attacks: [magical::MagicalTable; 64],
    rook_attacks: [magical::MagicalTable; 64],
    king_attacks: [u64; 64],
    to_from_table: [[u64; 64]; 64]
}

impl MoveTables {
    pub fn get_instance() -> &'static Self {
        static TABLES: OnceLock<MoveTables> = OnceLock::new();
        TABLES.get_or_init(|| MoveTables::new())
    }

    /// Reads the knight attack table. Must not be passed in a square value greater than 64.
    pub fn read_pawn_attacks(&self, sq: u8, turn: u8) -> u64 {
        return self.pawn_attacks[turn as usize][sq as usize];
    }

    /// Reads the knight attack table. Must not be passed in a square value greater than 64.
    pub fn read_knight_attacks(&self, sq: u8) -> u64 {
        return self.knight_attacks[sq as usize];
    }

    /// Reads the bishop attack table. Must not be passed in a square value greater than 64.
    pub fn read_bishop_attacks(&self, sq: u8, occupancy: u64) -> u64 {
        return magical::read_magical_table(
            &self.bishop_attacks[sq as usize],
            occupancy
        );
    }

    /// Reads the rook attack table. Must not be passed in a square value greater than 64.
    pub fn read_rook_attacks(&self, sq: u8, occupancy: u64) -> u64 {
        return magical::read_magical_table(
            &self.rook_attacks[sq as usize],
            occupancy
        );
    }

    /// Reads the king attack table. Must not be passed in a square value greater than 64.
    pub fn read_king_attacks(&self, sq: u8) -> u64 {
        return self.king_attacks[sq as usize];
    }

    pub fn read_to_from_table(&self, sq1: u8, sq2: u8) -> u64 {
        return self.to_from_table[sq1 as usize][sq2 as usize];
    }

    pub fn read_ray_direction(&self, sq1: u8, sq2: u8) -> u8 {
        return normal::get_ray_direction(sq1, sq2);
    }
}

impl MoveTables {
    fn new() -> Self {
        Self {
            pawn_attacks: normal::gen_pawn_attack_table(),
            knight_attacks: normal::gen_knight_attack_table(),
            bishop_attacks: magical::gen_bishop_attack_table(),
            rook_attacks: magical::gen_rook_attack_table(),
            king_attacks: normal::gen_king_attack_table(),
            to_from_table: normal::gen_to_from_table()
        }
    }
}

