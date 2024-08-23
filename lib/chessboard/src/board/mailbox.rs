
use crate::board::EMPTY;

pub const WHITE_KING_START: u8                 = 60;
pub const WHITE_KING_SIDE_ROOK_START: u8       = 63;
pub const WHITE_QUEEN_SIDE_ROOK_START: u8      = 56;
pub const WHITE_KING_SIDE_CASTLE_TARGET: u8    = 62;
pub const WHITE_KING_SIDE_ROOK_TARGET: u8      = 61;
pub const WHITE_QUEEN_SIDE_CASTLE_TARGET: u8   = 58;
pub const WHITE_QUEEN_SIDE_ROOK_TARGET: u8     = 59;

pub const BLACK_KING_START: u8                 = 4;
pub const BLACK_KING_SIDE_ROOK_START: u8       = 7;
pub const BLACK_QUEEN_SIDE_ROOK_START: u8      = 0;
pub const BLACK_KING_SIDE_CASTLE_TARGET: u8    = 6;
pub const BLACK_KING_SIDE_ROOK_TARGET: u8      = 5;
pub const BLACK_QUEEN_SIDE_CASTLE_TARGET: u8   = 2;
pub const BLACK_QUEEN_SIDE_ROOK_TARGET: u8     = 3;

pub const WHITE_MIN_ENPASSANT_TARGET: u8 = 40;
pub const BLACK_MIN_ENPASSANT_TARGET: u8 = 16;

pub struct Mailbox {
    pub data: [u8; 64]
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
        return self.data[sq as usize];
    }

    pub fn new_empty() -> Self {
        Mailbox {
            data: [EMPTY as u8; 64]
        }
    }
}
