
use crate::moves::{Move, MoveList};
use std::num::Wrapping;

pub mod bitboard;
pub mod mailbox;
pub mod state;

pub const BLACK: u8 = 0;
pub const WHITE: u8 = 1;

pub const PIECE_TYPE_PAWN: u8   = 0;
pub const PIECE_TYPE_KNIGHT: u8 = 1;
pub const PIECE_TYPE_BISHOP: u8 = 2;
pub const PIECE_TYPE_ROOK: u8   = 3;
pub const PIECE_TYPE_QUEEN: u8  = 4;
pub const PIECE_TYPE_KING: u8   = 5;
pub const PIECE_TYPE_EMPTY: u8  = 6;

pub const PID_EMPTY: u8         = 0b0000;

pub const PID_WHITE_PAWN: u8    = 0b0001;
pub const PID_WHITE_KNIGHT: u8  = 0b0010;
pub const PID_WHITE_BISHOP: u8  = 0b0011;
pub const PID_WHITE_ROOK: u8    = 0b0100;
pub const PID_WHITE_QUEEN: u8   = 0b0101;
pub const PID_WHITE_KING: u8    = 0b0110;

pub const PID_BLACK_PAWN: u8    = 0b1001;
pub const PID_BLACK_KNIGHT: u8  = 0b1010;
pub const PID_BLACK_BISHOP: u8  = 0b1011;
pub const PID_BLACK_ROOK: u8    = 0b1100;
pub const PID_BLACK_QUEEN: u8   = 0b1101;
pub const PID_BLACK_KING: u8    = 0b1110;

pub struct HistoryElement {
    pub last_move: Move,
    pub new_state: Move
}

pub struct Board {
    pub bitboard: bitboard::BitBoard,
    pub mailbox: mailbox::Mailbox,
    pub turn: u8
}

pub struct BoardHistory {
    pub data: Vec<HistoryElement>
}

impl Board {
    pub fn type_at(&self, row: u8, col: u8) -> u8 {
        return (Wrapping(self.mailbox.at(row, col) & 0b111) - Wrapping(1u8)).0;
    }

    pub fn type_at_sq(&self, sq: u8) -> u8 {
        return (Wrapping(self.mailbox.at_sq(sq) & 0b111) - Wrapping(1u8)).0;
    }

    pub fn pid_at(&self, row: u8, col: u8) -> u8 {
        return self.mailbox.at(row, col);
    }

    pub fn pid_at_sq(&self, sq: u8) -> u8 {
        return self.mailbox.at_sq(sq);
    }
}

