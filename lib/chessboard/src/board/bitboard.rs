
use crate::board::mailbox;
use bitintr::{Popcnt, Tzcnt};

pub const RIGHT_COL: u64        = 0x8080808080808080;
pub const LEFT_COL: u64         = 0x0101010101010101;
pub const RIGHT_TWO_COLS: u64   = 0xC0C0C0C0C0C0C0C0;
pub const LEFT_TWO_COLS: u64    = 0x0303030303030303;
pub const TOP_ROW: u64          = 0x00000000000000FF;
pub const BOTTOM_ROW: u64       = 0xFF00000000000000; 
pub const FULL: u64             = 0xFFFFFFFFFFFFFFFF;
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

pub const WHITE_MIN_ENPASSANT_TARGET: u32 = 40;
pub const BLACK_MIN_ENPASSANT_TARGET: u32 = 16;

pub struct BitBoard {
    pub color: [u64; 2],
    pub piece: [[u64; 6]; 2],
    pub occupancy: u64
}

impl BitBoard {
    
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

