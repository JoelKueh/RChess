
use crate::moves;

pub mod bitboard;
pub mod mailbox;
pub mod state;


pub struct HistoryElement {
    last_move: moves::Move,
    new_state: moves::Move
}

pub struct Board {
    bitboard: bitboard::BitBoard,
    mailbox: mailbox::Mailbox,
    history: Vec<HistoryElement>
}

impl Board {
    pub fn at(&self, row: u8, col: u8) -> i8 {
        return self.mailbox.at(row, col);
    }

    pub fn at_sq(&self, sq: u8) -> i8 {
        return self.mailbox.at_sq(sq);
    }
}
