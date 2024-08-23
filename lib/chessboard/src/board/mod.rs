
#![allow(unused_variables, dead_code)]

use crate::moves::*;
use std::num::Wrapping;
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
pub enum FenError {
    FenMalformedError(String)
}

#[derive(Clone, Debug)]
pub enum UciError<'a> {
    UciMalformedError(&'a str),
    UciInvalidMoveError(&'a str),
    UciIllegalMoveError(Move),
}

#[derive(Clone, Debug)]
pub enum PgnError<'a> {
    PgnMalformedError(&'a str),
    PgnInvalidMoveError(&'a str),
    PgnIllegalMoveError(Move),
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FenError::FenMalformedError(msg) => {
                write!(f, "invalid fen string provided ({})", msg)
            }
        }
    }
}

impl fmt::Display for UciError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UciError::UciMalformedError(msg) => {
                write!(f, "invalid uci string provided ({})", msg)
            },
            UciError::UciInvalidMoveError(mv) => {
                write!(f, "invalid move: {} when parsing uci string", mv)
            },
            UciError::UciIllegalMoveError(mv) => {
                write!(f, "illegal move: {} when parsing uci string", mv.to_long_algbr())
            }
        }
    }
}

impl fmt::Display for PgnError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PgnError::PgnMalformedError(msg) => {
                write!(f, "invalid pgn string provided ({})", msg)
            },
            PgnError::PgnInvalidMoveError(msg) => {
                write!(f, "invalid move: {} when parsing pgn string", msg)
            }
            PgnError::PgnIllegalMoveError(mv) => {
                write!(f, "illegal move: {} when parsing pgn string", mv.to_long_algbr())
            }
        }
    }
}

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

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let mut fen_split = fen.split(' ');

        // Verify that our string has all of the necessary components.
        const MSG_TOO_SHORT: &str = "fen too short";
        let malformed_error = FenError::FenMalformedError(MSG_TOO_SHORT.into());
        let main: &str = fen_split.next().ok_or(malformed_error.clone())?;
        let turn: &str = fen_split.next().ok_or(malformed_error.clone())?;
        let rights: &str = fen_split.next().ok_or(malformed_error.clone())?;
        let enp_col: &str = fen_split.next().ok_or(malformed_error.clone())?;
        let hlfmv: &str = fen_split.next().ok_or(malformed_error.clone())?;
        let fullmv: &str = fen_split.next().ok_or(malformed_error.clone())?;

        if turn.len() != 1 { return Err(FenError::FenMalformedError("malformed fen turn".into())); }
        if rights.len() < 1 || rights.len() > 4 {
            return Err(FenError::FenMalformedError("malformed fen rights".into()));
        }
        if enp_col.len() != 1 {
            return Err(FenError::FenMalformedError("malformed fen enp_col".into()));
        }

        // Parse the main part of the fen string.
        let new_board: Self = Self::from_fen_main(main)?;
        new_board.set_turn_from_fen(turn);
        new_board.set_rights_from_fen(rights);
        
        return Ok(new_board);
    }

    fn set_turn_from_fen(&mut self, turn: &str) -> Result<(), FenError> {
        match turn.chars().nth(0) {
            Some('w') => self.turn = WHITE as u8,
            Some('b') => self.turn = BLACK as u8,
            None => return Err(FenError::FenMalformedError("empty string in fen turn".into())),
            _ => return Err(FenError::FenMalformedError("invalid character in fen turn".into())),
        }
        return Ok(());
    }
    
    fn set_rights_from_fen(&mut self, rights: &str) -> Result<(), FenError> {
        return 
    }

    fn from_fen_main(fen_main: &str) -> Result<Self, FenError> {
        let mut new_board: Self = Self::new_empty();
        let mut sq: u8 = 0;

        let mut current_row: u8 = 0;
        for c in fen_main.chars() {
            match c {
                '1'..='8' => {
                    sq += c.to_digit(10).unwrap() as u8;
                    if sq < current_row * 8 || sq > current_row * 8 + 8 {
                        return Err(FenError::FenMalformedError(
                                "expected '/' before wrap around".to_string()
                        ));
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
                        return Err(FenError::FenMalformedError("unexpected '/'".to_string()))
                    }
                    current_row += 1;
                },
                a => {
                    let msg: String = format!("unexpected character {}", a);
                    return Err(FenError::FenMalformedError(msg))
                }
            }
        }

        if sq < 64 {
            return Err(FenError::FenMalformedError(
                    "incomplete main fen string".to_string()
            ));
        } else if sq > 64 {
            return Err(FenError::FenMalformedError(
                    "mian fen contained too many elements".to_string()
            ));
        }
        
        return Ok(new_board);
    }

    pub fn from_pgn(pgn: &str) -> Result<Self, PgnError> {
        todo!()
    }

    pub fn from_uci(uci: &str) -> Result<Self, UciError> {
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
                    KNIGHT => 'k',
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
            },
            CAPTURE => {
                let ptype: u8 = self.type_at_sq(from);
                let pcolor: u8 = self.turn;
                let cap_ptype: u8 = self.type_at_sq(to);
                let cap_pcolor: u8 = self.enemy_color();
                new_state.set_captured_piece(EMPTY as u8);
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

                let rook_to: u8 = if self.turn as usize== WHITE {
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

        match flags {
            QUIET | DOUBLE_PAWN_PUSH => {
                let ptype: u8 = self.type_at_sq(to);
                let pcolor: u8 = self.turn;
                self.write_piece(from, ptype, pcolor); self.delete_piece(to, ptype, pcolor);
            },
            CAPTURE => {
                let ptype: u8 = self.type_at_sq(from);
                let pcolor: u8 = self.turn;
                let cap_ptype: u8 = state.get_captured_piece();
                let cap_pcolor: u8 = self.enemy_color();
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

