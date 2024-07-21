
use crate::board;

pub const QUIET: u16                 =  0 << 12;
pub const DOUBLE_PAWN_PUSH: u16      =  1 << 12;
pub const KING_SIDE_CASTLE: u16      =  2 << 12;
pub const QUEEN_SIDE_CASTLE: u16     =  3 << 12;
pub const CAPTURE: u16               =  4 << 12;
pub const ENPASSANT: u16             =  5 << 12;
pub const KNIGHT_PROMO: u16          =  8 << 12;
pub const BISHOP_PROMO: u16          =  9 << 12;
pub const ROOK_PROMO: u16            = 10 << 12;
pub const QUEEN_PROMO: u16           = 11 << 12;
pub const KNIGHT_PROMO_CAPTURE: u16  = 12 << 12;
pub const BISHOP_PROMO_CAPTURE: u16  = 13 << 12;
pub const ROOK_PROMO_CAPTURE: u16    = 14 << 12;
pub const QUEEN_PROMO_CAPTURE: u16   = 15 << 12;


const TO_MASK: u16      = 0x3F;
const FROM_MASK: u16    = 0x3F << 6;
const FLAG_MASK: u16    = 0xF << 12;

pub const MAX_NUM_MOVES: usize = 218;
pub const INVALID_MOVE: u16 = 0b0110111111111111;

#[derive(Clone, Copy)]
pub struct Move {
    data: u16
}

impl Move {
    /// Returns the FIDE algebraic notation string for the move.
    pub fn to_short_algbr(&self, board: &board::Board, moves: &MoveList) -> String {
        let algbr = String::new();

        let to: u8 = self.get_to();
        let start_piece: i8 = board.at_sq(self.get_from());

        let from_file: char = (self.get_from() % 8  + 'a' as u8) as char;
        let from_rank: char = ('8' as u8 - self.get_from() / 8) as char;
        let to_file: char = (self.get_to() % 8 + 'a' as u8) as char;
        let to_rank: char = ('8' as u8 - self.get_to() / 8) as char;

        let from_file_conflict: bool = false;
        let from_rank_conflict: bool = false;

        for alternative in &moves.moves[..] {
            let alt_to: u8 = alternative.get_to();
            let alt_start_piece: i8 = board.at_sq(alternative.get_from());

            if to == alt_to && start_piece == alt_start_piece {
                let alt_from_file: char = (alternative.get_from() % 8  + 'a' as u8) as char;
                let alt_from_rank: char = ('8' as u8 - alternative.get_from() / 8) as char;
            }
        }

        return algbr;
    }

    /// Returns the UCI algebraic notatition string for the move. 
    pub fn to_long_algbr(&self) -> String {
        let mut algbr: String = String::new();
        algbr.push((self.get_from() % 8 + 'a' as u8) as char);
        algbr.push(('8' as u8 - self.get_from() / 8) as char);
        algbr.push((self.get_to() % 8 + 'a' as u8) as char);
        algbr.push(('8' as u8 - self.get_to() / 8) as char);

        match self.get_flags() {
            KNIGHT_PROMO | KNIGHT_PROMO_CAPTURE => algbr.push('k'),
            BISHOP_PROMO | BISHOP_PROMO_CAPTURE => algbr.push('b'),
            ROOK_PROMO   | ROOK_PROMO_CAPTURE   => algbr.push('r'),
            QUEEN_PROMO  | QUEEN_PROMO_CAPTURE  => algbr.push('q'),
            _ => ()
        };

        return algbr;
    }

    pub fn get_to(&self) -> u8 {
        return (self.data & TO_MASK) as u8;
    }

    pub fn get_from(&self) -> u8 {
        return ((self.data & FROM_MASK) >> 6) as u8;
    }

    pub fn get_flags(&self) -> u16 {
        return self.data & FLAG_MASK;
    }

    /// Builds the move the raw underlying data.
    pub fn new(from: u16, to: u16, flags: u16) -> Move {
        return Move {
            data: flags | (from << 6) | to
        }
    }

    /// Builds the move from a FIDE algebraic string representation of the move.
    pub fn from_short_algbr(algbr: &str, board: &board::Board, moves: &MoveList) {
        todo!();
    }

    /// Builds the move from a UCI algebraic notation string representation of the move.
    pub fn from_uci_algbr(algbr: &str, board: &board::Board) {
        todo!();
    }
}

/// Struct that represents a list of moves for a particular position.
/// Has room for 218 moves because this is the theoretical maximum.
///
/// Due to this functions unchecked array access, the board representation that uses this must
/// guarantee that every possible position must be a valid position playable from the
/// root posiiton.
pub struct MoveList {
    moves: [Move; MAX_NUM_MOVES],
    head: u8
}

impl MoveList {
    pub fn new() -> MoveList {
        return MoveList {
            moves: [Move { data: INVALID_MOVE }; MAX_NUM_MOVES],
            head: 0
        }
    }

    /// Gets the number of elements in the MoveList.
    pub fn size(&self) -> usize {
        return self.head as usize;
    }

    /// Clears the MoveList.
    pub fn clear(&mut self) {
        self.head = 0;
    }

    /// Pushes an element into the MoveList.
    ///
    /// This function is unsafe. Caller must guarantee that no more than MAX_NUM_MOVES moves will
    /// ever be pushed here. We know that this is the case because the maximum number of moves in
    /// any chess position is MAX_NUM_MOVES.
    ///
    /// The board representation that uses this must guarantee that every possible position must
    /// be a valid position playable from the root posiiton.
    pub unsafe fn push(&mut self, new_move: Move) {
        *self.moves[..].get_unchecked_mut(self.head as usize) = new_move;
        self.head += 1;
    }

    /// Pops the top element off of the MoveList.
    ///
    /// This function is unsafe. Caller must guarantee that no more than MAX_NUM_MOVES moves will
    /// ever be pushed here. We know that this is the case because the maximum number of moves in
    /// any chess position is MAX_NUM_MOVES.
    ///
    /// The board representation that uses this must guarantee that every possible position must
    /// be a valid position playable from the root posiiton.
    pub unsafe fn pop(&mut self) -> &Move {
        self.head -= 1;
        self.moves[..].get_unchecked(self.head as usize)
    }

    /// Pops the top element off of the MoveList.
    ///
    /// This function is unsafe. Caller must guarantee that no more than MAX_NUM_MOVES moves will
    /// ever be pushed here. We know that this is the case because the maximum number of moves in
    /// any chess position is MAX_NUM_MOVES.
    ///
    /// The board representation that uses this must guarantee that every possible position must
    /// be a valid position playable from the root posiiton.
    pub unsafe fn at(&self, idx: u8) -> &Move {
        self.moves[..].get_unchecked(idx as usize)
    }

}
