
use crate::board::*;
use bitintr::{Popcnt, Tzcnt};
use std::fmt::Debug;
use colored::{Colorize, ColoredString};

pub const RIGHT_COL: u64        = 0x8080808080808080;
pub const LEFT_COL: u64         = 0x0101010101010101;
pub const RIGHT_TWO_COLS: u64   = 0xC0C0C0C0C0C0C0C0;
pub const LEFT_TWO_COLS: u64    = 0x0303030303030303;
pub const TOP_ROW: u64          = 0x00000000000000FF;
pub const BOTTOM_ROW: u64       = 0xFF00000000000000; 
pub const FULL: u64             = 0xFFFFFFFFFFFFFFFF;
pub const EMPTY: u64            = 0x0000000000000000;
pub const BLACK_PAWN_HOME: u64  = 0x000000000000FF00; // Second-to-top row
pub const WHITE_PAWN_HOME: u64  = 0x00FF000000000000; // Second-to-bottom row

pub const WHITE_KING_SIDE_CASTLE_TARGET: u64   = 1u64 << mailbox::WHITE_KING_SIDE_CASTLE_TARGET;
pub const WHITE_QUEEN_SIDE_CASTLE_TARGET: u64  = 1u64 << mailbox::WHITE_QUEEN_SIDE_CASTLE_TARGET;

pub const WHITE_KING_SIDE_CASTLE_OCCUPANCY: u64     = 0x6000000000000000;
pub const WHITE_QUEEN_SIDE_CASTLE_OCCUPANCY: u64    = 0x0E00000000000000;
pub const WHITE_KING_SIDE_CASTLE_CHECK: u64         = 0x7000000000000000;
pub const WHITE_QUEEN_SIDE_CASTLE_CHECK: u64        = 0x1C00000000000000;

pub const BLACK_KING_SIDE_CASTLE_TARGET: u64   = 1u64 << mailbox::BLACK_KING_SIDE_CASTLE_TARGET;
pub const BLACK_QUEEN_SIDE_CASTLE_TARGET: u64  = 1u64 << mailbox::BLACK_QUEEN_SIDE_CASTLE_TARGET;

pub const BLACK_KING_SIDE_CASTLE_OCCUPANCY: u64     = 0x0000000000000060;
pub const BLACK_QUEEN_SIDE_CASTLE_OCCUPANCY: u64    = 0x000000000000000E;
pub const BLACK_KING_SIDE_CASTLE_CHECK: u64         = 0x0000000000000070;
pub const BLACK_QUEEN_SIDE_CASTLE_CHECK: u64        = 0x000000000000001C;

pub struct BitBoard {
    pub color: [u64; 2],
    pub piece: [[u64; 6]; 2],
    pub occupancy: u64
}

impl BitBoard {
    pub fn new_empty() -> Self {
        Self {
            color: [0u64; 2],
            piece: [[0u64; 6]; 2],
            occupancy: 0
        }
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let headers: [&str; 7] = [ "WHITE", "PAWN", "KNIGHT", "BISHOP", "ROOK", "QUEEN", "KING" ];

        let mut wpieces: [[String; 8]; 6] = Default::default();
        let mut bpieces: [[String; 8]; 6] = Default::default();
        let mut color: [[String; 8]; 2] = Default::default();
        let occupancy: [String; 8] = u64_to_bb(self.occupancy);

        // Prepare the piece arrays.
        for i in [ WHITE, BLACK ] {
            color[i] = u64_to_bb(self.color[i]);
        }
        for i in 0..6 {
            wpieces[i] = u64_to_bb(self.piece[WHITE][i]);
            bpieces[i] = u64_to_bb(self.piece[BLACK][i]);
        }

        // Print white pieces.
        for i in 0..7 as usize {
            write!(f, "{: <17}", headers[i])?;
        }
        writeln!(f, "")?;
        for i in 0..7 as usize { write!(f, "---------------  ")?; }
        writeln!(f, "")?;
        for j in 0..8 as usize {
            write!(f, "{} ", color[WHITE][j])?;
            for i in 0..6 as usize {
                write!(f, "{} ", wpieces[i][j])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "\n")?;

        // Print black pieces.
        for i in 0..7 as usize {
            write!(f, "{: <17}", headers[i])?;
        }
        writeln!(f, "")?;
        for i in 0..7 as usize { write!(f, "---------------  ")?; }
        writeln!(f, "")?;
        for j in 0..8 as usize {
            write!(f, "{} ", color[BLACK][j])?;
            for i in 0..6 as usize {
                write!(f, "{} ", bpieces[i][j])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "\n")?;

        // Print occupancy
        writeln!(f, "OCC")?;
        writeln!(f, "---------------")?;
        for j in 0..8 as usize {
            writeln!(f, "{}", occupancy[j])?;
        }
        write!(f, "\n")?;

        return Ok(());
    }
}

pub fn u64_to_bb(bb: u64) -> [String; 8] {
    let mut bytes: [String; 8] = Default::default();
    for i in 0..8 {
        let byte_str = format!("{:08b}", (bb >> (i * 8)) & 0xFF).chars().rev().collect::<String>();
        bytes[i] = byte_str.chars().map(|x| {
            if x == '1' {
                format!("{}", format!("{} ", x).bold().green())
            } else {
                format!("{}", format!("{} ", x).bold().red())
            }
        }).collect::<Vec<String>>().join("");
    }
    return bytes;
}

/// Generates all attacked squares for pawns by shifting the bitboard in a particular direction.
/// Shifts the bitboard either up or down the board depending on the direction specified in is_up.
pub fn pawn_smear(pawns: u64, is_up: bool) -> u64 {
    if is_up {
        (pawns >> 9 & !RIGHT_COL) | (pawns >> 7 & !LEFT_COL)
    } else {
        (pawns << 7 & !RIGHT_COL) | (pawns << 9 & !LEFT_COL)
    }
}

pub fn peek_rbit(bb: &u64) -> u8 {
    return bb.tzcnt() as u8;
}

pub fn pop_rbit(bb: &mut u64) -> u8 {
    let idx: u64 = bb.tzcnt();
    *bb ^= 1u64 << idx;
    return idx as u8;
}

pub fn popcnt(bb: &u64) -> u8 {
    return bb.popcnt() as u8;
}

pub fn print(mut bb: u64) {
    for _i in 0..8 {
        for _j in 0..8 {
            print!("{} ", if bb & 1 == 1 {
                format!("{}", bb & 1).bold().red()
            } else {
                format!("{}", bb & 1).bold()
            });
            bb = bb >> 1;
        }
        println!("");
    }
}

pub fn print_debug(mut bb: u64, mut actual: u64) {
    for _i in 0..8 {
        for _j in 0..8 {
            let is_correct: bool = bb & 1 == actual & 1;
            print!("{} ", if is_correct {
                format!("{}", bb & 1).bold().green()
            } else {
                format!("{}", bb & 1).bold().red()
            });
            bb = bb >> 1;
            actual = actual >> 1;
        }
        println!("");
    }
}
