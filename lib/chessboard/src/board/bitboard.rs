
use crate::board::mailbox;
use bitintr::{Popcnt, Lzcnt, Tzcnt};

pub const RIGHT_COL: u64        = 0x8080808080808080;
pub const LEFT_COL: u64         = 0x0101010101010101;
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

pub const PIECE_TYPE_PAWN: i8   = 0;
pub const PIECE_TYPE_KNIGHT: i8 = 1;
pub const PIECE_TYPE_BISHOP: i8 = 2;
pub const PIECE_TYPE_ROOK: i8   = 3;
pub const PIECE_TYPE_QUEEN: i8  = 4;
pub const PIECE_TYPE_KING: i8   = 5;
pub const PIECE_TYPE_EMPTY: i8  = 6;

pub struct BitBoard {
    color: [u64; 2],
    piece: [[u64; 6]; 2],
    occupancy: u64
}

impl BitBoard {
    
}

pub fn peek_rbit(bb: &u64) -> u64 {
    return bb.tzcnt();
}

pub fn pop_rbit(bb: &mut u64) -> u64 {
    let idx: u64 = bb.tzcnt();
    *bb ^= 1u64 << idx;
    return idx;
}

pub fn popcnt(bb: &u64) -> u64 {
    return bb.popcnt();
}

