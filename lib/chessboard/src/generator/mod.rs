
use crate::board;
use crate::moves::MoveList;

mod tables;

pub struct BoardTables {
    pub threats: u64,
    pub checkers: u64,
    pub check_blocks: u64,
    pub pins: [u64; 9]
}

pub struct MoveGenerator {
    tables: &'static tables::MoveTables
}

impl MoveGenerator {
    // Creates a new move generator by getting a reference to the static tables it will use.
    pub fn new() -> Self {
        MoveGenerator {
            tables: tables::MoveTables::get_instance()
        }
    }

    pub fn gen_moves(move_list: &mut MoveList, board: &board::Board, state: BoardTables) {

    }

    fn get_pawn_threat_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn get_pawn_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn get_knight_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()   
    }

    fn get_bishop_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn get_rook_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn get_queen_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn get_king_move_mask(sq: u8, occupancy: u64) -> u64 {
        todo!()
    }

    fn gen_pseudo_move_mask(sq: u8, piece_type: u8, occupancy: u64) -> u64 {
        match piece_type {
            board::PIECE_TYPE_PAWN   => Self::get_pawn_move_mask(sq, occupancy),
            board::PIECE_TYPE_KNIGHT => Self::get_knight_move_mask(sq, occupancy),
            board::PIECE_TYPE_BISHOP => Self::get_bishop_move_mask(sq, occupancy),
            board::PIECE_TYPE_ROOK   => Self::get_rook_move_mask(sq, occupancy),
            board::PIECE_TYPE_QUEEN  => Self::get_queen_move_mask(sq, occupancy),
            board::PIECE_TYPE_KING   => Self::get_king_move_mask(sq, occupancy),
            _ => unreachable!()
        }
    }

    fn gen_threats(state: &mut BoardTables, board: &board::Board) {
        // Smear the pawns to get all of their attacks.
        let pawns: u64 = board.bitboard.piece[board.turn as usize][board::PIECE_TYPE_PAWN as usize];
        let king: u64 = board.bitboard.piece[board.turn as usize][board::PIECE_TYPE_KING as usize];

        state.threats = bitboard::pawn_smear(pawns, board.turn == board::WHITE);

        // For each of the remaining pieces, gen the legal moves after removing the desired king
        // from the occupancy mask. This is because the king cannot block a threat, pieces should "see
        // through" the king).
        let mut pieces: u64 = board.bitboard.color[board.turn as usize] ^ pawns;
        let occupancy: u64 = board.bitboard.occupancy ^ king;
        while pieces != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut pieces);
        }
    }

    fn gen_check_blocks(state: &mut BoardTables, board: &Board) {

    }

    fn gen_pins(state: &mut BoardTables, board: &Board) {

    }
}
