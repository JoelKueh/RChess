
use crate::board::bitboard;

pub fn gen_knight_attack_table() -> [u64; 64] {
    // Offsets of knight moves from a source square.
    const SQ_OFFSET: [i8; 8] = [ -17, -15, -10, -6, 6, 10, 15, 17 ];
    let mut table: [u64; 64] = [0; 64];

    for i in 0..64  {
        // Build all of the knight moves.
        for j in 0..8 {
            let sq: i8 = i + SQ_OFFSET[j];
            
            if sq < 0 || sq >= 64 {
                continue;
            }

            table[i as usize] |= 1u64 << sq;
        }

        // Remove the invalid ones.
        // If we're in the right two columns, we can't jump to the left two columns.
        if (1u64 << i) & bitboard::RIGHT_TWO_COLS != 0 {
            table[i as usize] &= !bitboard::LEFT_TWO_COLS
        }

        // If we're in the left two columns, we can't jump to the right two columns.
        if (1u64 << i) & bitboard::LEFT_TWO_COLS != 0 {
            table[i as usize] &= !bitboard::RIGHT_TWO_COLS
        }
    }

    return table;
}

pub fn gen_king_attack_table() -> [u64; 64] {
    // Offsets of king moves from a source square
    const SQ_OFFSET: [i8; 8] = [ 9, 8, 7, 1, -1, -7, -8, -9 ];
    let mut table: [u64; 64] = [0; 64];

    for i in 0..64  {
        // Build all of the king moves.
        for j in 0..8 {
            let sq: i8 = i + SQ_OFFSET[j];
            
            if sq < 0 || sq >= 64 {
                continue;
            }

            table[i as usize] |= 1u64 << sq;
        }

        // Remove the invalid ones.
        // If we're in the right two columns, we can't jump to the left two columns.
        if (1u64 << i) & bitboard::RIGHT_TWO_COLS != 0 {
            table[i as usize] &= !bitboard::LEFT_TWO_COLS
        }

        // If we're in the left two columns, we can't jump to the right two columns.
        if (1u64 << i) & bitboard::LEFT_TWO_COLS != 0 {
            table[i as usize] &= !bitboard::RIGHT_TWO_COLS
        }
    }

    return table;
}
