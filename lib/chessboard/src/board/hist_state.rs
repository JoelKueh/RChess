
use crate::board::*;

pub const INIT_BOARD_STATE: u16 = 0;
pub const ENP_COL: u16          = 0b11100000;
pub const PID_COL: u16          = 0b11100000;
pub const ENP_AVAILABLE: u16    = 0b10000;
pub const ENP_ALL: u16          = 0b11110000;
pub const HALFMOVE_CLOCK: u16   = 0b11111100000000;
pub const HALFMOVE_FIFTY: u16   = 50 << 8;

/// Represents the parts of a state of a board that are not captured when making and unmaking
/// moves. The bit ordering of the raw data is as follows.
///
/// Bits 13 - 8 : HALFMOVE_CLOCK
/// Bits  7 - 5 : ENP_COLUMN | CAPTURED_PIECE_ID
/// Bit       4 : ENP_AVAILABILITY
/// Bits  3 - 0 : CASTLE_RIGHTS
#[derive(Clone)]
pub struct HistState {
    data: u16
}

impl HistState {
    /// Returns true if the player still has the right to king side castle.
    pub fn has_ksc_right(&self, turn: u8) -> bool {
        return self.data & (0b1000 >> (turn as i16 * 2)) != 0;
    }

    /// Returns true if the player still has the right to queen side castle.
    pub fn has_qsc_right(&self, turn: u8) -> bool {
        return self.data & (0b100 >> (turn as i16 * 2)) != 0;
    }


    /// Removes the right for a player to queen side castle.
    pub fn remove_ksc_right(&mut self, turn: u8) {
        self.data &= !0b1000 >> (turn as i16 * 2);
    }

    /// Removes the right for a player to queen side castle.
    pub fn remove_qsc_right(&mut self, turn: u8) {
        self.data &= !0b100 >> (turn as i16 * 2);
    }

    /// Removes all rights for a player to castle.
    pub fn remove_castle_rights(&mut self, turn: u8) {
        self.data &= !0b1100 >> (turn as i16 * 2);
    }


    /// Removes the right for a player to queen side castle.
    pub fn add_ksc_right(&mut self, turn: u8) {
        self.data |= 0b1000 >> (turn as i16 * 2);
    }

    /// Removes the right for a player to queen side castle.
    pub fn add_qsc_right(&mut self, turn: u8) {
        self.data |= 0b100 >> (turn as i16 * 2);
    }

    /// Removes all rights for a player to castle.
    pub fn add_castle_rights(&mut self, turn: u8) {
        self.data |= 0b1100 >> (turn as i16 * 2);
    }


    /// Checks the availiability of an enpassant.
    pub fn enp_avaliable(&self) -> bool {
        return self.data & ENP_AVAILABLE != 0;
    }

    /// Gets the column of an enpassant.
    pub fn get_enp_col(&self) -> u8 {
        return (self.data & ENP_COL >> 5) as u8;
    }

    /// Gets the piece type of the captured piece as defined in bitboard.
    pub fn get_captured_piece(&self) -> u8 {
        return (self.data & ENP_COL >> 5) as u8;
    }


    /// Sets up this move state to hold an enpassant square.
    pub fn set_enp(&mut self, enp_col: u8) {
        self.data = (self.data & !ENP_COL) | ((enp_col << 5) as u16);
        self.data |= ENP_AVAILABLE;
    }

    /// Decay the enpassant stored in this square.
    pub fn decay_enp(&mut self) {
        self.data &= !ENP_ALL;
    }

    /// Sets up this move state to hold a captured piece.
    pub fn set_captured_piece(&mut self, enp_col: u8) {
        self.data = (self.data & !ENP_COL) | ((enp_col << 5) as u16);
        self.data |= ENP_AVAILABLE;
    }


    /// Returns true if the 50-move rule has been met.
    pub fn halfmove_clock_done(&self) -> bool {
        return (self.data & HALFMOVE_CLOCK) == HALFMOVE_FIFTY;
    }

    /// Resets the halfmove clock.
    pub fn reset_halfmove_clock(&mut self) {
        self.data &= !HALFMOVE_CLOCK;
    }

    /// Increments the halfmove clock.
    pub fn increment_halfmove_clock(&mut self) {
        self.data += 1u16 << 8;
    }
    
    /// Decays castle rights after a move.
    pub fn decay_castle_rights(&mut self, turn: u8, to: u8, from: u8) {
        let enemy_color: u8 = (!(turn as usize == WHITE)) as u8;

        // Remove castling rights caused by moving a king or rook.
        if from == mailbox::WHITE_KING_START || from == mailbox::BLACK_KING_START {
            self.remove_castle_rights(turn);
        } else if from == mailbox::WHITE_KING_SIDE_ROOK_START
            || from == mailbox::BLACK_KING_SIDE_ROOK_START {
            self.remove_ksc_right(turn);
        } else if from == mailbox::WHITE_QUEEN_SIDE_ROOK_START
            || from == mailbox::BLACK_QUEEN_SIDE_ROOK_START {
            self.remove_qsc_right(turn);
        }

        // Remove castling rights caused by taking an enemy rook.
        if to == mailbox::WHITE_KING_SIDE_ROOK_START
            || to == mailbox::BLACK_KING_SIDE_ROOK_START {
            self.remove_ksc_right(enemy_color);
        } else if to == mailbox::WHITE_KING_SIDE_ROOK_START
            || to == mailbox::BLACK_KING_SIDE_ROOK_START {
            self.remove_qsc_right(enemy_color);
        }
        
    }

    pub fn from_data(data: u16) -> HistState {
        return HistState {
            data
        }
    }
}
