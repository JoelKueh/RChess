
use crate::board::bitboard;

pub const WHITE_KING_START: u32                 = 60;
pub const WHITE_KING_SIDE_ROOK_START: u32       = 63;
pub const WHITE_QUEEN_SIDE_ROOK_START: u32      = 56;
pub const WHITE_KING_SIDE_CASTLE_TARGET: u32    = 62;
pub const WHITE_QUEEN_SIDE_CASTLE_TARGET: u32   = 58;

pub const BLACK_KING_START: u32                 = 4;
pub const BLACK_KING_SIDE_ROOK_START: u32       = 7;
pub const BLACK_QUEEN_SIDE_ROOK_START: u32      = 0;
pub const BLACK_KING_SIDE_CASTLE_TARGET: u32    = 6;
pub const BLACK_QUEEN_SIDE_CASTLE_TARGET: u32   = 2;

pub struct Mailbox {
    data: [u8; 64]
}

impl Mailbox {
    /// Gets the PID given a row and a column.
    /// The index of the square is relative to the top left corner seen from white's perspective.
    pub fn at(&self, row: u8, col: u8) -> u8 {
        return self.at_sq(row * 8 + col);
    }

    /// Gets the PID given the number of the square.
    /// The index of the square is relative to the top left corner seen from white's perspective.
    pub fn at_sq(&self, sq: u8) -> u8 {
        return self.data[sq as usize].clone();
    }
}
