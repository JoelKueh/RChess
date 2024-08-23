
#![allow(unused_variables, dead_code)]

use crate::board::{self, *};
use crate::moves::{*, MoveList};
use std::fmt;

mod tables;

#[cfg(test)]
mod tests;

pub struct BoardTables {
    pub threats: u64,
    pub checks: u64,
    pub check_blocks: u64,
    pub pins: [u64; 9],
}

pub struct MoveGenerator {
    tables: &'static tables::MoveTables
}

impl fmt::Debug for BoardTables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let headers: [&str; 3] = [ "THREATS", "CHECKS", "CHECK_BLOCKS" ];

        let mut boards: [[String; 8]; 3] = Default::default();
        let mut pins: [[String; 8]; 9] = Default::default();

        boards[0] = bitboard::u64_to_bb(self.threats);
        boards[1] = bitboard::u64_to_bb(self.checks);
        boards[2] = bitboard::u64_to_bb(self.check_blocks);
        for i in 0..9 as usize {
            pins[i] = bitboard::u64_to_bb(self.pins[i]);
        }

        // Print Pins
        writeln!(f, "PINS")?;
        for i in 0..9 as usize { write!(f, "---------------  ")?; }
        writeln!(f, "")?;
        for j in 0..9 as usize {
            for i in 0..8 as usize {
                write!(f, "{} ", pins[i][j])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "\n")?;

        // Print Other Stuff
        writeln!(f, "PINS")?;
        for i in 0..3 as usize { write!(f, "---------------  ")?; }
        writeln!(f, "")?;
        for j in 0..3 as usize {
            for i in 0..8 as usize {
                write!(f, "{} ", boards[i][j])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "\n")?;

        return Ok(());
    }
}

impl MoveGenerator {
    // Creates a new move generator by getting a reference to the static tables it will use.
    pub fn new() -> Self {
        MoveGenerator {
            tables: tables::MoveTables::get_instance()
        }
    }

    pub fn gen_moves(&self, move_list: &mut MoveList, board: &Board, state: &BoardTables) {
        move_list.clear();
        self.append_simple_moves(move_list, board, state);
        self.append_castle_moves(move_list, board, state);
        self.append_enp_moves(move_list, board, state);
        self.append_double_pawn_push(move_list, board, state);
        self.append_promos(move_list, board, state);
    }

    fn gen_legal_mv_mask(&self, sq: u8, board: &Board, state: &BoardTables) -> u64 {
        // Generate the pseudo moves.
        let piece_type: u8 = board.type_at_sq(sq);
        let mut moves: u64 = self.gen_pseudo_move_mask(sq, piece_type,
            board.bitboard.occupancy, board.turn);
        moves &= !board.bitboard.color[WHITE];

        // Adjust the moves for pins and checks.
        moves &= if piece_type == KING as u8 { state.threats } else { !0x0 };
        moves = MoveGenerator::pin_adjust(sq, moves, state);
        moves &= state.check_blocks;
        
        return moves;
    }

    fn append_simple_moves(
        &self,
        move_list: &mut MoveList,
        board: &Board,
        state: &BoardTables
    ) {
        let mut pieces = board.bitboard.color[board.turn as usize];

        // Take out the pawns that are on the opposing sides home row as their next move will be
        // a promotion and should be handled separately.
        pieces ^= board.bitboard.piece[board.turn as usize][PAWN] & if board.turn == WHITE as u8 {
            bitboard::BLACK_PAWN_HOME
        } else {
            bitboard::WHITE_PAWN_HOME
        };

        // Append all of the moves to the list.
        while pieces != 0 {
            let sq = bitboard::pop_rbit(&mut pieces);
            let mut simple = self.gen_legal_mv_mask(sq, board, state);
            while simple != 0 {
                let target: u8 = bitboard::pop_rbit(&mut simple);
                let flag: u16 = if (1u64 << target) & board.bitboard.occupancy == 0 {
                    QUIET
                } else {
                    CAPTURE
                };
                move_list.push(Move::new(sq as u16, target as u16, flag))
            }
        }
    }

    fn append_castle_moves(
        &self,
        move_list: &mut MoveList,
        board: &Board,
        state: &BoardTables
    ) {
        let from: u8 = if board.turn as usize == WHITE {
            mailbox::WHITE_KING_START
        } else {
            mailbox::BLACK_KING_START
        };

        if MoveGenerator::ksc_legal(board, state) {
            let to: u8 = if board.turn as usize == WHITE {
                mailbox::WHITE_KING_SIDE_CASTLE_TARGET
            } else {
                mailbox::BLACK_KING_SIDE_CASTLE_TARGET
            };
            move_list.push(Move::new(from as u16, to as u16, KING_SIDE_CASTLE));
        }

        if MoveGenerator::qsc_legal(board, state) {
            let to: u8 = if board.turn as usize == WHITE {
                mailbox::WHITE_QUEEN_SIDE_CASTLE_TARGET
            } else {
                mailbox::BLACK_QUEEN_SIDE_CASTLE_TARGET
            };
            move_list.push(Move::new(from as u16, to as u16, QUEEN_SIDE_CASTLE));
        }
    }

    fn append_enp_moves(
        &self,
        move_list: &mut MoveList,
        board: &Board,
        state: &BoardTables
    ) {
        // Check if there is an available enpassant.
        let extra: &hist_state::HistState = &board.history.data.last().unwrap().new_state;
        if !extra.enp_avaliable() {
            return;
        }

        // Get the column of that enpassanet.
        let enemy_turn: usize = (!(board.turn as usize == WHITE)) as usize;
        let enp_row_start: u8 = if board.turn as usize == WHITE {
            mailbox::WHITE_MIN_ENPASSANT_TARGET
        } else {
            mailbox::BLACK_MIN_ENPASSANT_TARGET
        };
        let enp_sq: u8 = enp_row_start + extra.get_enp_col();
        let enemy_sq: i8 = enp_sq as i8 + if board.turn as usize == WHITE { 8 } else { -8 };

        // Get all of the pieces that can enpassant.
        let mut enp_sources = self.get_pawn_threat_mask(enemy_turn as u8, enp_sq)
            & board.bitboard.piece[board.turn as usize][PAWN];

        while enp_sources != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut enp_sources);
            let mv: Move = Move::new(sq as u16, enp_sq as u16, ENPASSANT);

            // Update the occupancy mask to what it will be after the move happens.
            let mut new_occupancy: u64 = board.bitboard.occupancy;
            new_occupancy &= !(1u64 << sq);
            new_occupancy &= !(1u64 << enemy_sq);
            new_occupancy |= 1u64 << enp_sq;

            let king_sq: u8 = bitboard::peek_rbit(&board.bitboard.piece[enemy_turn][KING]);

            // Check if any pawns are threatening the king after the move.
            let pawn_threats: u64 = self.get_pawn_threat_mask(board.turn, king_sq)
                & board.bitboard.piece[enemy_turn][PAWN];
            if pawn_threats != 0 { continue; }

            // Check if any knights are threatening the king after the move.
            let bishop_threats: u64 = self.get_knight_move_mask(king_sq)
                & board.bitboard.piece[enemy_turn][KNIGHT];
            if pawn_threats != 0 { continue; }

            // Check if any bishops or queens threaten the king after the move (handles pin).
            let bishop_threats: u64 = self.get_bishop_move_mask(king_sq, new_occupancy)
                & (board.bitboard.piece[enemy_turn][BISHOP]
                    | board.bitboard.piece[enemy_turn][QUEEN]);
            if bishop_threats != 0 { continue; }

            // Check if any rooks or queens threaten the king after the move (handles pin).
            let rook_threats: u64 = self.get_bishop_move_mask(king_sq, new_occupancy)
                & (board.bitboard.piece[enemy_turn][BISHOP]
                    | board.bitboard.piece[enemy_turn][QUEEN]);
            if rook_threats != 0 { continue; }

            // Push the move if it does't cause any problems.
            move_list.push(mv);
        }
    }

    fn append_double_pawn_push(
        &self,
        move_list: &mut MoveList,
        board: &Board,
        state: &BoardTables
    ) {
        let mask: u64 = if board.turn as usize == WHITE {
            bitboard::WHITE_PAWN_HOME
        } else {
            bitboard::BLACK_PAWN_HOME
        };

        let mut pawns: u64 = mask & board.bitboard.piece[board.turn as usize][PAWN];
        while pawns != 0 {
            let from: i8 = bitboard::pop_rbit(&mut pawns) as i8;
            let to: i8 = from + if board.turn as usize == WHITE { -16 } else { 16 };
            let passed: i8 = from + if board.turn as usize == WHITE { -8 } else { 8 };

            let mut target: u64 = 1u64 << to;
            let mask: u64 = target | (1u64 << passed);
            if mask & board.bitboard.occupancy == 0 {
                target = MoveGenerator::pin_adjust(from as u8, target, state);
                target &= state.check_blocks;
                if target != 0 {
                    move_list.push(Move::new(from as u16, to as u16, DOUBLE_PAWN_PUSH));
                }
            }
        }
    }

    fn append_promos(
        &self,
        move_list: &mut MoveList,
        board: &Board,
        state: &BoardTables
    ) {
        let promo_row: u64 = if board.turn as usize == WHITE {
            bitboard::BLACK_PAWN_HOME
        } else {
            bitboard::WHITE_PAWN_HOME
        };
        let mut pawns: u64 = board.bitboard.piece[board.turn as usize][PAWN] & promo_row;
        let direction: i8 = if board.turn as usize == WHITE { 1 } else { -1 };
        let enemy_turn: usize = (!(board.turn as usize == WHITE)) as usize;

        while pawns != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut pawns);
            let mut legal_mask: u64 = MoveGenerator::get_pin_mask(sq, state);
            legal_mask &= state.check_blocks;

            let push_target: i8 = sq as i8 + 8 * direction;
            if board.type_at_sq(sq) as usize == EMPTY && (1u64 << push_target) & legal_mask != 0 {
                move_list.push(Move::new(sq as u16, push_target as u16, KNIGHT_PROMO));
                move_list.push(Move::new(sq as u16, push_target as u16, BISHOP_PROMO));
                move_list.push(Move::new(sq as u16, push_target as u16, ROOK_PROMO));
                move_list.push(Move::new(sq as u16, push_target as u16, QUEEN_PROMO));
            }

            let mut cap_targets: u64 = self.get_pawn_threat_mask(board.turn, sq);
            cap_targets &= board.bitboard.color[enemy_turn];
            cap_targets &= legal_mask;
            while cap_targets != 0 {
                let target: u8 = bitboard::pop_rbit(&mut cap_targets);
                move_list.push(Move::new(sq as u16, target as u16, KNIGHT_PROMO_CAPTURE));
                move_list.push(Move::new(sq as u16, target as u16, KNIGHT_PROMO_CAPTURE));
                move_list.push(Move::new(sq as u16, target as u16, ROOK_PROMO_CAPTURE));
                move_list.push(Move::new(sq as u16, target as u16, QUEEN_PROMO_CAPTURE));
            }
        }
    }

    pub fn gen_board_tables(&self, board: &Board) -> BoardTables {
        let threats: u64 = self.gen_threats(board);
        let checks: u64 = self.gen_checks(board, threats);
        let check_blocks: u64 = self.gen_check_blocks(board, checks);
        let pins: [u64; 9] = self.gen_pins(board);

        BoardTables {
            threats,
            checks,
            check_blocks,
            pins
        }
    }

    fn get_pawn_threat_mask(&self, sq: u8, turn: u8) -> u64 {
        self.tables.read_pawn_attacks(sq, turn)
    }

    fn get_pawn_move_mask(sq: u8, occupancy: u64) -> u64 {
        (1u64 << sq) & !occupancy
    }

    fn get_knight_move_mask(&self, sq: u8) -> u64 {
        self.tables.read_knight_attacks(sq)
    }

    fn get_bishop_move_mask(&self, sq: u8, occupancy: u64) -> u64 {
        self.tables.read_bishop_attacks(sq, occupancy)
    }

    fn get_rook_move_mask(&self, sq: u8, occupancy: u64) -> u64 {
        self.tables.read_rook_attacks(sq, occupancy)
    }

    fn get_queen_move_mask(&self, sq: u8, occupancy: u64) -> u64 {
        self.tables.read_bishop_attacks(sq, occupancy)
            | self.tables.read_rook_attacks(sq, occupancy)
    }

    fn get_king_move_mask(&self, sq: u8) -> u64 {
        self.tables.read_king_attacks(sq)
    }

    fn gen_pseudo_move_mask(&self, sq: u8, piece_type: u8, occupancy: u64, turn: u8) -> u64 {
        match piece_type as usize {
            board::PAWN   => self.get_pawn_threat_mask(sq, turn),
            board::KNIGHT => self.get_knight_move_mask(sq),
            board::BISHOP => self.get_bishop_move_mask(sq, occupancy),
            board::ROOK   => self.get_rook_move_mask(sq, occupancy),
            board::QUEEN  => self.get_queen_move_mask(sq, occupancy),
            board::KING   => self.get_king_move_mask(sq),
            _ => unreachable!()
        }
    }

    fn gen_threats(&self, board: &Board) -> u64 {
        let mut threats: u64;
        let not_turn: u8 = (board.turn != 0) as u8;

        // Smear the pawns to get all of their attacks.
        let pawns: u64 = board.bitboard.piece[board.turn as usize][board::PAWN as usize];
        let king: u64 = board.bitboard.piece[board.turn as usize][board::KING as usize];

        threats = bitboard::pawn_smear(pawns, board.turn == board::WHITE as u8);

        // For each of the remaining pieces, gen the legal moves after removing the desired king
        // from the occupancy mask. This is because the king cannot block a threat, pieces should
        // "see through" the king).
        let mut pieces: u64 = board.bitboard.color[board.turn as usize] ^ pawns;
        let occupancy: u64 = board.bitboard.occupancy ^ king;
        while pieces != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut pieces);
            threats |= self.gen_pseudo_move_mask(
                sq,
                board.type_at_sq(sq),
                occupancy,
                not_turn
            );
        }

        return threats;
    }

    fn gen_checks(&self, board: &Board, threats: u64) -> u64 {
        let not_turn: u8 = (board.turn != 0) as u8;
        let pieces: &[u64; 6] = &board.bitboard.piece[not_turn as usize];
        let occupancy: u64 = board.bitboard.occupancy;

        // Exit early if the king isn't on the threat squares.
        if pieces[board::KING] & threats == 0 {
            return 0;
        }

        // Build the list of pieces that are checking the king.
        let king_sq: u8 = bitboard::peek_rbit(&pieces[board::KING]);
        let mut checks = self.get_pawn_threat_mask(king_sq, not_turn) & pieces[board::PAWN];
        checks |= self.get_knight_move_mask(king_sq) & pieces[board::KNIGHT];
        checks |= self.get_bishop_move_mask(king_sq, occupancy)
            & (pieces[board::BISHOP] | pieces[board::QUEEN]);
        checks |= self.get_rook_move_mask(king_sq, occupancy)
            & (pieces[board::ROOK] | pieces[board::QUEEN]);
        // Helpful reminder that a king can never check another king.
        
        return checks;
    }

    fn gen_check_blocks(&self, board: &Board, checks: u64) -> u64 {
        if checks == 0 { return bitboard::FULL; }
        if bitboard::popcnt(&checks) != 1 { return bitboard::EMPTY; }

        let king = board.bitboard.piece[board.turn as usize][board::KING];
        let king_sq: u8 = bitboard::peek_rbit(&king);
        let check_sq: u8 = bitboard::peek_rbit(&checks);
        let check_blocks: u64 = checks | self.tables.read_to_from_table(king_sq, check_sq);

        return check_blocks;
    }

    fn gen_pins(&self, board: &Board) -> [u64; 9] {
        // TODO: Maybe this will be faster without the union of the pins board.
        let mut pins: [u64; 9] = [0; 9];
        let king: u64 = board.bitboard.piece[board.turn as usize][board::KING];
        let king_sq: u8 = bitboard::peek_rbit(&king);

        let mut i = 0;
        let occupancy: u64 = board.bitboard.occupancy;
        let blockers: u64 = board.bitboard.color[board.turn as usize];

        // TODO: REMOVE ME
        let mut pinner: u64 = self.xray_rook_attacks(occupancy, blockers, king_sq)
            & (board.bitboard.piece[board.enemy_color() as usize][ROOK]
                | board.bitboard.piece[board.enemy_color() as usize][QUEEN]);
        println!("");
        while pinner != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut pinner);
            pins[i] = self.tables.read_to_from_table(king_sq, sq);
            pins[8] ^= pins[i];
            i += 1;
        }

        let mut pinner: u64 = self.xray_bishop_attacks(occupancy, blockers, king_sq)
            & (board.bitboard.piece[board.enemy_color() as usize][BISHOP]
                | board.bitboard.piece[board.enemy_color() as usize][QUEEN]);
        println!("");
        i = 0;
        while pinner != 0 {
            let sq: u8 = bitboard::pop_rbit(&mut pinner);
            pins[i] = self.tables.read_to_from_table(king_sq, sq);
            pins[8] ^= pins[i];
            i += 1;
        }

        pins[8] &= !(1u64 << king_sq);
        return pins;
    }

    fn xray_rook_attacks(&self, occupancy: u64, mut blockers: u64, sq: u8) -> u64 {
        let attacks = self.tables.read_rook_attacks(sq, occupancy);
        blockers &= attacks;
        return attacks ^ self.tables.read_rook_attacks(sq, occupancy ^ blockers);
    }

    fn xray_bishop_attacks(&self, occupancy: u64, mut blockers: u64, sq: u8) -> u64 {
        let attacks = self.tables.read_rook_attacks(sq, occupancy);
        blockers &= attacks;
        return attacks ^ self.tables.read_rook_attacks(sq, occupancy ^ blockers);
    }
    
    fn get_pin_mask(sq: u8, state: &BoardTables) -> u64 {
        for mask in state.pins {
            if (1u64 << sq) & mask != 0 {
                return mask;
            }
        }
        return 0;
    }
    
    fn pin_adjust(sq: u8, moves: u64, state: &BoardTables) -> u64 {
        for mask in state.pins {
            if (1u64 << sq) & mask != 0 {
                return moves & mask;
            }
        }
        return moves;
    }
    
    pub fn ksc_legal(board: &Board, state: &BoardTables) -> bool {
        let extra: &hist_state::HistState = &board.history.data[..].last().unwrap().new_state;
        if !extra.has_ksc_right(board.turn) {
            return false;
        }

        let occupancy_mask: u64 = if board.turn as usize == WHITE {
            bitboard::WHITE_KING_SIDE_CASTLE_OCCUPANCY
        } else {
            bitboard::BLACK_KING_SIDE_CASTLE_OCCUPANCY
        };

        let check_mask: u64 = if board.turn as usize == WHITE {
            bitboard::WHITE_KING_SIDE_CASTLE_CHECK
        } else {
            bitboard::BLACK_KING_SIDE_CASTLE_CHECK
        };

        return (board.bitboard.occupancy & occupancy_mask) | (state.threats & check_mask) == 0;
    }
    
    pub fn qsc_legal(board: &Board, state: &BoardTables) -> bool {
        let extra: &hist_state::HistState = &board.history.data[..].last().unwrap().new_state;
        if !extra.has_qsc_right(board.turn) {
            return false;
        }

        let occupancy_mask: u64 = if board.turn as usize == WHITE {
            bitboard::WHITE_QUEEN_SIDE_CASTLE_OCCUPANCY
        } else {
            bitboard::BLACK_QUEEN_SIDE_CASTLE_OCCUPANCY
        };

        let check_mask: u64 = if board.turn as usize == WHITE {
            bitboard::WHITE_QUEEN_SIDE_CASTLE_CHECK
        } else {
            bitboard::BLACK_QUEEN_SIDE_CASTLE_CHECK
        };

        return (board.bitboard.occupancy & occupancy_mask) | (state.threats & check_mask) == 0;
    }
}

