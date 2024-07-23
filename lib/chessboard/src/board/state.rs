
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
pub struct State {
    data: u16
}

impl State {
    /// Returns true if the player still has the right to king side castle.
    fn has_ksc_right(&self, is_white: bool) -> bool {
        return self.data & (0b1000 >> (is_white as i16 * 2)) != 0;
    }

    /// Returns true if the player still has the right to queen side castle.
    fn has_qsc_right(&self, is_white: bool) -> bool {
        return self.data & (0b100 >> (is_white as i16 * 2)) != 0;
    }


    /// Removes the right for a player to queen side castle.
    fn remove_ksc_right(&mut self, is_white: bool) {
        self.data &= !0b1000 >> (is_white as i16 * 2);
    }

    /// Removes the right for a player to queen side castle.
    fn remove_qsc_right(&mut self, is_white: bool) {
        self.data &= !0b100 >> (is_white as i16 * 2);
    }

    /// Removes all rights for a player to castle.
    fn remove_castle_rights(&mut self, is_white: bool) {
        self.data &= !0b1100 >> (is_white as i16 * 2);
    }


    /// Removes the right for a player to queen side castle.
    fn add_ksc_right(&mut self, is_white: bool) {
        self.data |= 0b1000 >> (is_white as i16 * 2);
    }

    /// Removes the right for a player to queen side castle.
    fn add_qsc_right(&mut self, is_white: bool) {
        self.data |= 0b100 >> (is_white as i16 * 2);
    }

    /// Removes all rights for a player to castle.
    fn add_castle_rights(&mut self, is_white: bool) {
        self.data |= 0b1100 >> (is_white as i16 * 2);
    }


    /// Checks the availiability of an enpassant.
    fn enp_avaliable(&self) -> bool {
        return self.data & ENP_AVAILABLE != 0;
    }

    /// Gets the column of an enpassant.
    fn get_enp_col(&self) -> u8 {
        return (self.data & ENP_COL >> 5) as u8;
    }

    /// Gets the piece type of the captured piece as defined in bitboard.
    fn get_captured_piece(&self) -> u8 {
        return (self.data & ENP_COL >> 5) as u8;
    }


    /// Sets up this move state to hold an enpassant square.
    fn set_enp(&mut self, enp_col: u8) {
        self.data = (self.data & !ENP_COL) | ((enp_col << 5) as u16);
        self.data |= ENP_AVAILABLE;
    }

    /// Decay the enpassant stored in this square.
    fn decay_enp(&mut self) {
        self.data &= !ENP_ALL;
    }

    /// Sets up this move state to hold a captured piece.
    fn set_captured_piece(&mut self, enp_col: u8) {
        self.data = (self.data & !ENP_COL) | ((enp_col << 5) as u16);
        self.data |= ENP_AVAILABLE;
    }


    /// Returns true if the 50-move rule has been met.
    fn halfmove_clock_done(&self) -> bool {
        return (self.data & HALFMOVE_CLOCK) == HALFMOVE_FIFTY;
    }

    /// Resets the halfmove clock.
    fn reset_halfmove_clock(&mut self) {
        self.data &= !HALFMOVE_CLOCK;
    }

    /// Increments the halfmove clock.
    fn increment_halfmove_clock(&mut self) {
        self.data += 1u16 << 8;
    }
}
