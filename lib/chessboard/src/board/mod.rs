
#![allow(unused_variables, dead_code)]

use std::error::Error;
use crate::moves::*;
use std::fmt;

pub mod bitboard;
pub mod mailbox;
pub mod hist_state;

pub const BLACK: usize = 0;
pub const WHITE: usize = 1;

pub const PAWN: usize   = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize   = 3;
pub const QUEEN: usize  = 4;
pub const KING: usize   = 5;
pub const EMPTY: usize  = 6;

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

#[derive(Default)]
pub struct HistoryElement {
    pub last_move: Move,
    pub new_state: hist_state::HistState
}

pub struct Board {
    pub bitboard: bitboard::BitBoard,
    pub mailbox: mailbox::Mailbox,
    pub history: BoardHistory,
    pub turn: u8,
    pub fullmv_num: u32
}

pub struct BoardHistory {
    pub data: Vec<HistoryElement>
}

#[derive(Clone, Debug)]
pub enum ParseError {
    Malformed(&'static str),
    InvalidMove(&'static str),
    IllegalMove(Move)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malformed(context) => write!(f, "malformed string: {}", context),
            Self::InvalidMove(mv_str) => write!(f, "invalid move: {}", mv_str),
            Self::IllegalMove(mv) => write!(f, "illegal move: {}", mv.to_long_algbr())
        }
    }
}

impl Error for ParseError {}

impl Board {
    fn new_empty() -> Self {
        Board {
            bitboard: bitboard::BitBoard::new_empty(),
            mailbox: mailbox::Mailbox::new_empty(),
            history: BoardHistory {
                data: vec![]
            },
            turn: WHITE as u8,
            fullmv_num: 0
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, ParseError> {
        let mut fen_split = fen.split(' ');

        // Verify that our string has all of the necessary components.
        const TOO_SHORT: ParseError = ParseError::Malformed("fen too short");
        let main: &str = fen_split.next().ok_or(TOO_SHORT)?;
        let turn: &str = fen_split.next().ok_or(TOO_SHORT)?;
        let rights: &str = fen_split.next().ok_or(TOO_SHORT)?;
        let enp_col: &str = fen_split.next().ok_or(TOO_SHORT)?;
        let hlfmv: &str = fen_split.next().ok_or(TOO_SHORT)?;
        let fullmv: &str = fen_split.next().ok_or(TOO_SHORT)?;

        // Verify the length requirements of the fen string pieces.
        if turn.len() != 1 { return Err(ParseError::Malformed("pieces too short")); }
        if rights.len() < 1 || rights.len() > 4 {
            return Err(ParseError::Malformed("rights too short"));
        }
        if enp_col.len() != 1 {
            return Err(ParseError::Malformed("enp too short"));
        }

        // Parse the main part of the fen string.
        let mut new_board: Self = Self::from_fen_main(main)?;
        new_board.set_turn_from_fen(turn)?;
        new_board.set_rights_from_fen(rights)?;
        
        return Ok(new_board);
    }

    fn set_turn_from_fen(&mut self, turn: &str) -> Result<(), ParseError> {
        match turn.chars().nth(0) {
            Some('w') => self.turn = WHITE as u8,
            Some('b') => self.turn = BLACK as u8,
            None => return Err(ParseError::Malformed("empty fen turn")),
            _ => return Err(ParseError::Malformed("invalid character in fen turn")),
        }
        return Ok(());
    }
    
    fn set_rights_from_fen(&mut self, rights: &str) -> Result<(), ParseError> {
        match rights.chars().nth(0) {
            Some('K') => self.history.data.last_mut().unwrap().new_state.add_ksc_right(WHITE as u8),
            Some('Q') => self.history.data.last_mut().unwrap().new_state.add_qsc_right(WHITE as u8),
            Some('k') => self.history.data.last_mut().unwrap().new_state.add_ksc_right(BLACK as u8),
            Some('q') => self.history.data.last_mut().unwrap().new_state.add_qsc_right(BLACK as u8),
            Some('-') => return Ok(()),
            None => return Err(ParseError::Malformed("empty fen rights")),
            _ => return Err(ParseError::Malformed("invalid character in fen rights")),
        }
        return Ok(());
    }

    fn from_fen_main(fen_main: &str) -> Result<Self, ParseError> {
        let mut new_board: Self = Self::new_empty();
        let mut sq: u8 = 0;

        // Initialize the board by looping through the characters in the string.
        let mut current_row: u8 = 0;
        for c in fen_main.chars() {
            match c {
                '1'..='8' => {
                    sq += c.to_digit(10).unwrap() as u8;
                    if sq < current_row * 8 || sq > current_row * 8 + 8 {
                        return Err(ParseError::Malformed("expected '/' before wrap around"));
                    }
                },
                'p' | 'P' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, PAWN as u8, pcolor);
                    sq += 1;
                },
                'n' | 'N' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, KNIGHT as u8, pcolor);
                    sq += 1;
                },
                'b' | 'B' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, BISHOP as u8, pcolor);
                    sq += 1;
                },
                'r' | 'R' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, ROOK as u8, pcolor);
                    sq += 1;
                },
                'q' | 'Q' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, QUEEN as u8, pcolor);
                    sq += 1;
                },
                'k' | 'K' => {
                    let pcolor: u8 = if c.is_uppercase() { WHITE as u8 } else { BLACK as u8 };
                    new_board.write_piece(sq, KING as u8, pcolor);
                    sq += 1;
                },
                '/' => {
                    if sq % 8 != 0 {
                        return Err(ParseError::Malformed("unexpected '/'"))
                    }
                    current_row += 1;
                },
                a => {
                    return Err(ParseError::Malformed("unexpected character"))
                }
            }
        }

        // Throw errors for the write head not being at the end of the board.
        if sq < 64 {
            return Err(ParseError::Malformed("incomplete main fen string"));
        } else if sq > 64 {
            return Err(ParseError::Malformed("main fen contained too many elements"));
        }

        // Initialize the board state.
        new_board.history.data.push(Default::default());

        return Ok(new_board);
    }

    pub fn from_pgn(pgn: &str) -> Result<Self, ParseError> {
        todo!()
    }

    pub fn from_uci(uci: &str) -> Result<Self, ParseError> {
        todo!()
    }

    pub fn str_rep(&self) -> Box<[[char; 8]; 8]> {
        let mut arr: Box<[[char; 8]; 8]> = Box::new([[' '; 8]; 8]);

        for row in 0..8 as usize {
            for col  in 0..8 as usize {
                let ptype = self.type_at(row as u8, col as u8);
                let pcolor = self.color_at(row as u8, col as u8);
                let c = match ptype as usize {
                    PAWN => 'p',
                    KNIGHT => 'n',
                    BISHOP => 'b',
                    ROOK => 'r',
                    QUEEN => 'q',
                    KING => 'k',
                    EMPTY => ' ',
                    _ => panic!("Invalid piece type in string conversion: ({},{}:{})",
                        row, col, ptype),
                };
                arr[row][col] = if pcolor as usize == WHITE { c.to_ascii_uppercase() } else { c };
            }
        }

        return arr
    }

    pub fn str_rep_utf8(&self) -> Box<[[char; 8]; 8]> {
        let mut arr: Box<[[char; 8]; 8]> = Box::new([[' '; 8]; 8]);

        for row in 0..8 as usize {
            for col  in 0..8 as usize {
                let ptype = self.type_at(row as u8, col as u8);
                let pcolor = self.color_at(row as u8, col as u8);
                let c = match ptype as usize {
                    PAWN => if pcolor as usize == WHITE { '\u{2659}' } else { '\u{265F}' }
                    KNIGHT => if pcolor as usize == WHITE { '\u{2658}' } else { '\u{265E}' }
                    BISHOP => if pcolor as usize == WHITE { '\u{2657}' } else { '\u{265D}' }
                    ROOK => if pcolor as usize == WHITE { '\u{2656}' } else { '\u{265B}' }
                    QUEEN => if pcolor as usize == WHITE { '\u{2655}' } else { '\u{265C}' }
                    KING => if pcolor as usize == WHITE { '\u{2654}' } else { '\u{265A}' }
                    EMPTY => ' ',
                    _ => panic!("Invalid piece type in string conversion: ({},{}:{})",
                        row, col, ptype),
                };
                arr[row][col] = c
            }
        }

        return arr
    }

    pub fn type_at(&self, row: u8, col: u8) -> u8 {
        return self.mailbox.at(row, col);
    }

    pub fn type_at_sq(&self, sq: u8) -> u8 {
        return self.mailbox.at_sq(sq);
    }

    pub fn color_at(&self, row: u8, col: u8) -> u8 {
        return self.color_at_sq((row << 3) + col);
    }

    pub fn color_at_sq(&self, sq: u8) -> u8 {
        return if self.bitboard.color[WHITE] & (1u64 << sq) == 0 { 0 } else { 1 }
    }

//    pub fn pid_at(&self, row: u8, col: u8) -> u8 {
//        return self.mailbox.at(row, col);
//    }
//
//    pub fn pid_at_sq(&self, sq: u8) -> u8 {
//        return self.mailbox.at_sq(sq);
//    }

    fn replace_piece(&mut self, sq: u8, ptype: u8, pcolor: u8, old_ptype: u8, old_pcolor: u8) {
        self.mailbox.data[sq as usize] = ptype;
        self.bitboard.piece[pcolor as usize][ptype as usize] |= 1u64 << sq;
        self.bitboard.color[pcolor as usize] |= 1u64 << sq;
        self.bitboard.piece[old_pcolor as usize][old_ptype as usize] &= !(1u64 << sq);
        self.bitboard.color[old_pcolor as usize] &= !(1u64 << sq);
    }

    fn write_piece(&mut self, sq: u8, ptype: u8, pcolor: u8) {
        self.mailbox.data[sq as usize] = ptype;
        self.bitboard.piece[pcolor as usize][ptype as usize] |= 1u64 << sq;
        self.bitboard.color[pcolor as usize] |= 1u64 << sq;
        self.bitboard.occupancy |= 1u64 << sq;
    }

    fn delete_piece(&mut self, sq: u8, old_ptype: u8, old_pcolor: u8) {
        self.mailbox.data[sq as usize] = EMPTY as u8;
        self.bitboard.piece[old_pcolor as usize][old_ptype as usize] &= !(1u64 << sq);
        self.bitboard.color[old_pcolor as usize] &= !(1u64 << sq);
        self.bitboard.occupancy &= !(1u64 << sq);
    }
    
    fn wipe_board(&mut self) {
        for datum in &mut self.mailbox.data { *datum = 0; }
        for bb in &mut self.bitboard.piece[WHITE] { *bb = 0; }
        for bb in &mut self.bitboard.piece[BLACK] {*bb = 0; }
        self.bitboard.color[WHITE] = 0;
        self.bitboard.color[BLACK] = 0;
        self.bitboard.occupancy = 0;
    }

    pub fn make(&mut self, mv: &Move) {
        let extra: &hist_state::HistState = &self.history.data.last().unwrap().new_state;
        let mut new_state: hist_state::HistState = extra.clone();

        let flags: u16 = mv.get_flags();
        let to: u8 = mv.get_to();
        let from: u8 = mv.get_from();

        match flags {
            QUIET => {
                let ptype: u8 = self.type_at_sq(from);
                let pcolor: u8 = self.turn;
                new_state.set_captured_piece(EMPTY as u8);
                new_state.decay_castle_rights(self.turn, to, from);
                self.write_piece(to, ptype, pcolor);
                self.delete_piece(from, ptype, pcolor);
            },
            CAPTURE => {
                let ptype: u8 = self.type_at_sq(from);
                let pcolor: u8 = self.turn;
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(cap_ptype);
                new_state.decay_castle_rights(self.turn, to, from);
                self.replace_piece(to, ptype, pcolor, cap_ptype, cap_pcolor);
                self.delete_piece(from, ptype, pcolor);
            },
            DOUBLE_PAWN_PUSH => {
                let ptype: u8 = PAWN as u8;
                let pcolor: u8 = self.turn;
                new_state.set_enp(to & 0b111);
                self.write_piece(to, ptype, pcolor);
                self.delete_piece(from, ptype, pcolor);
            },
            KING_SIDE_CASTLE => {
                let king_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_START
                } else {
                    mailbox::BLACK_KING_START
                };

                let king_to: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_SIDE_CASTLE_TARGET
                } else {
                    mailbox::BLACK_KING_SIDE_CASTLE_TARGET
                };

                let rook_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_SIDE_ROOK_START
                } else {
                    mailbox::BLACK_KING_SIDE_ROOK_START
                };

                let rook_to: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_SIDE_ROOK_TARGET
                } else {
                    mailbox::BLACK_KING_SIDE_ROOK_TARGET
                };

                new_state.set_captured_piece(EMPTY as u8);
                new_state.remove_castle_rights(self.turn);
                self.delete_piece(king_from, KING as u8, self.turn);
                self.write_piece(king_to, KING as u8, self.turn);
                self.delete_piece(rook_from, ROOK as u8, self.turn);
                self.write_piece(rook_to, ROOK as u8, self.turn);
            },
            QUEEN_SIDE_CASTLE => {
                let king_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_START
                } else {
                    mailbox::BLACK_KING_START
                };

                let king_to: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_QUEEN_SIDE_CASTLE_TARGET
                } else {
                    mailbox::BLACK_QUEEN_SIDE_CASTLE_TARGET
                };

                let rook_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_QUEEN_SIDE_ROOK_START
                } else {
                    mailbox::BLACK_QUEEN_SIDE_ROOK_START
                };

                let rook_to: u8 = if self.turn as usize== WHITE {
                    mailbox::WHITE_QUEEN_SIDE_ROOK_TARGET
                } else {
                    mailbox::BLACK_QUEEN_SIDE_ROOK_TARGET
                };

                new_state.set_captured_piece(EMPTY as u8);
                new_state.remove_castle_rights(self.turn);
                self.delete_piece(king_from, KING as u8, self.turn);
                self.write_piece(king_to, KING as u8, self.turn);
                self.delete_piece(rook_from, ROOK as u8, self.turn);
                self.write_piece(rook_to, ROOK as u8, self.turn);
            },
            ENPASSANT => {
                let direction: i8 = if self.turn as usize == WHITE { 1 } else { -1 };
                new_state.set_captured_piece(PAWN as u8);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.write_piece(to, PAWN as u8, self.turn);
                self.delete_piece((to as i8 + 8 * direction) as u8, PAWN as u8, self.enemy_color());

                // TODO: Remove
                println!("Enpassant");
            },
            KNIGHT_PROMO => {
                new_state.set_captured_piece(EMPTY as u8);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.write_piece(from, KNIGHT as u8, self.turn);
            },
            BISHOP_PROMO => {
                new_state.set_captured_piece(EMPTY as u8);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.write_piece(from, BISHOP as u8, self.turn);
            },
            ROOK_PROMO => {
                new_state.set_captured_piece(EMPTY as u8);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.write_piece(from, ROOK as u8, self.turn);
            },
            QUEEN_PROMO => {
                new_state.set_captured_piece(EMPTY as u8);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.write_piece(from, QUEEN as u8, self.turn);
            },
            KNIGHT_PROMO_CAPTURE => {
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(cap_ptype);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.replace_piece(from, KNIGHT as u8, self.turn, cap_ptype, cap_pcolor);
            },
            BISHOP_PROMO_CAPTURE => {
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(cap_ptype);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.replace_piece(from, BISHOP as u8, self.turn, cap_ptype, cap_pcolor);
            },
            ROOK_PROMO_CAPTURE => {
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(cap_ptype);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.replace_piece(from, ROOK as u8, self.turn, cap_ptype, cap_pcolor);
            },
            QUEEN_PROMO_CAPTURE => {
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(cap_ptype);
                self.delete_piece(from, PAWN as u8, self.turn);
                self.replace_piece(from, QUEEN as u8, self.turn, cap_ptype, cap_pcolor);
            },
            _ => {
                panic!();
            }
        }

        self.turn = if self.turn == WHITE as u8 { BLACK as u8 } else { WHITE as u8 };
        let ele: HistoryElement = HistoryElement {
            last_move: mv.clone(),
            new_state
        };
        self.history.data.push(ele);
    }
    
    pub fn unmake(&mut self) {
        self.turn = if self.turn == WHITE as u8 { BLACK as u8 } else { WHITE as u8 };

        let ele = self.history.data.pop();
        if ele.is_none() {
            return;
        }
        let mv: &Move = &ele.as_ref().unwrap().last_move;
        let state: &hist_state::HistState = &ele.as_ref().unwrap().new_state;

        let flags: u16 = mv.get_flags();
        let to: u8 = mv.get_to();
        let from: u8 = mv.get_from();

        println!("{:?}", flags);
        match flags {
            QUIET | DOUBLE_PAWN_PUSH => {
                let ptype: u8 = self.type_at_sq(to);
                let pcolor: u8 = self.turn;
                self.write_piece(from, ptype, pcolor);
                self.delete_piece(to, ptype, pcolor);
            },
            CAPTURE => {
                let ptype: u8 = self.type_at_sq(to);
                let pcolor: u8 = self.turn;
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
                println!("{:?}", ptype);
                println!("{:?}", pcolor);
                println!("{:?}", cap_ptype);
                println!("{:?}", cap_pcolor);
                self.write_piece(from, ptype, pcolor);
                self.replace_piece(to, ptype, pcolor, cap_ptype, cap_pcolor);
            },
            KING_SIDE_CASTLE => {
                let king_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_START
                } else {
                    mailbox::BLACK_KING_START
                };

                let king_to: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_SIDE_CASTLE_TARGET
                } else {
                    mailbox::BLACK_KING_SIDE_CASTLE_TARGET
                };

                let rook_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_SIDE_ROOK_START
                } else {
                    mailbox::BLACK_KING_SIDE_ROOK_START
                };

                let rook_to: u8 = if self.turn as usize== WHITE {
                    mailbox::WHITE_KING_SIDE_ROOK_TARGET
                } else {
                    mailbox::BLACK_KING_SIDE_ROOK_TARGET
                };

                self.write_piece(king_from, KING as u8, self.turn);
                self.delete_piece(king_to, KING as u8, self.turn);
                self.write_piece(rook_from, ROOK as u8, self.turn);
                self.delete_piece(rook_to, ROOK as u8, self.turn);
            },
            QUEEN_SIDE_CASTLE => {
                let king_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_KING_START
                } else {
                    mailbox::BLACK_KING_START
                };

                let king_to: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_QUEEN_SIDE_CASTLE_TARGET
                } else {
                    mailbox::BLACK_QUEEN_SIDE_CASTLE_TARGET
                };

                let rook_from: u8 = if self.turn as usize == WHITE {
                    mailbox::WHITE_QUEEN_SIDE_ROOK_START
                } else {
                    mailbox::BLACK_QUEEN_SIDE_ROOK_START
                };

                let rook_to: u8 = if self.turn as usize== WHITE {
                    mailbox::WHITE_QUEEN_SIDE_ROOK_TARGET
                } else {
                    mailbox::BLACK_QUEEN_SIDE_ROOK_TARGET
                };

                self.write_piece(king_from, KING as u8, self.turn);
                self.delete_piece(king_to, KING as u8, self.turn);
                self.write_piece(rook_from, ROOK as u8, self.turn);
                self.delete_piece(rook_to, ROOK as u8, self.turn);
            },
            ENPASSANT => {
                let direction: i8 = if self.turn as usize == WHITE { 1 } else { -1 };
                self.delete_piece(to, PAWN as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.enemy_color());
            },
            KNIGHT_PROMO => {
                self.delete_piece(to, KNIGHT as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            BISHOP_PROMO => {
                self.delete_piece(to, BISHOP as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            ROOK_PROMO => {
                self.delete_piece(to, ROOK as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            QUEEN_PROMO => {
                self.delete_piece(to, QUEEN as u8, self.turn);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            KNIGHT_PROMO_CAPTURE => {
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
                self.replace_piece(to, KNIGHT as u8, self.turn, cap_ptype, cap_pcolor);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            BISHOP_PROMO_CAPTURE => {
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
                self.replace_piece(to, BISHOP as u8, self.turn, cap_ptype, cap_pcolor);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            ROOK_PROMO_CAPTURE => {
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
                self.replace_piece(to, ROOK as u8, self.turn, cap_ptype, cap_pcolor);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            QUEEN_PROMO_CAPTURE => {
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
                self.replace_piece(to, QUEEN as u8, self.turn, cap_ptype, cap_pcolor);
                self.write_piece(from, PAWN as u8, self.turn);
            },
            _ => {
                panic!()
            }
        }
    }
    
    pub fn enemy_color(&self) -> u8 {
        return (!(self.turn as usize == WHITE)) as u8;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SEPARATOR_LINE: &str  = "   +---+---+---+---+---+---+---+---+";
        const PIECE_LINE: [&str; 3] = [ "| ", " | ", " |"];
        const FILE_LINE: &str = "     A   B   C   D   E   F   G   H";

        writeln!(f, "{}", FILE_LINE)?;
        writeln!(f, "{}", SEPARATOR_LINE)?;
        let board_str = self.str_rep();
        for row in 0..8 as usize {
            write!(f, " {} ", 8 - row)?;
            write!(f, "{}", PIECE_LINE[0])?;
            for col in 0..7 as usize {
                write!(f, "{}{}", board_str[row][col], PIECE_LINE[1])?;
            }
            write!(f, "{}{}", board_str[row][7], PIECE_LINE[2])?;
            writeln!(f, " {}", 8 - row)?;
            writeln!(f, "{}", SEPARATOR_LINE)?;
        }
        writeln!(f, "{}", FILE_LINE)?;
        Ok(())
    }
}
