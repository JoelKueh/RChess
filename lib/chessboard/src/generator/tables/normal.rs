
use crate::board::bitboard;

pub fn gen_pawn_attack_table() -> [[u64; 64]; 2] {
    // Offsets of pawn moves from a source square.
    const SQ_OFFSET: [[i8; 2]; 2] = [[ 7, 9 ], [ -7, -9 ]];
    return [
        gen_table_from_offsets(SQ_OFFSET[0].to_vec()),
        gen_table_from_offsets(SQ_OFFSET[1].to_vec())
    ]
}

pub fn gen_knight_attack_table() -> [u64; 64] {
    // Offsets of knight moves from a source square.
    const SQ_OFFSET: [i8; 8] = [ -17, -15, -10, -6, 6, 10, 15, 17 ];
    gen_table_from_offsets(SQ_OFFSET.to_vec())
}

pub fn gen_king_attack_table() -> [u64; 64] {
    // Offsets of king moves from a source square
    const SQ_OFFSET: [i8; 8] = [ 9, 8, 7, 1, -1, -7, -8, -9 ];
    gen_table_from_offsets(SQ_OFFSET.to_vec())
}

pub fn gen_to_from_table() -> [[u64; 64]; 64] {
    let mut table: [[u64; 64]; 64] = [[0; 64]; 64];

    for i in 0..64 {
        for j in 0..64 {
            table[i][j] = get_connecting_ray(i as u8, j as u8);
        }
    }

    return table;
}

fn get_connecting_ray(sq1: u8, sq2: u8) -> u64 {
    let sq1_rank: u8 = sq1 % 8;
    let sq1_file: u8 = sq1 / 8;
    let sq2_rank: u8 = sq1 % 8;
    let sq2_file: u8 = sq1 / 8;

    if sq1 == sq2 {
        return 1u64 << sq1;
    }

    let mut sq: i8 = sq1.try_into().unwrap();
    let mut mask: u64 = 0;
    let direction: i8 = if sq1_rank == sq2_rank {
        if sq1 < sq2 { 1 } else { -1 }
    } else if sq1_file == sq2_file {
        if sq1 < sq2 { 8 } else { -8 }
    } else if sq1_file + sq1_rank == sq2_file + sq2_rank {
        if sq1 < sq2 { 7 } else { -7 }
    } else if sq1_file - sq1_rank == sq2_file - sq2_rank {
        if sq1 < sq2 { 9 } else { -9 }
    } else {
        return 0;
    };

    while <i8 as TryInto<u8>>::try_into(sq).unwrap() != sq2 {
        mask |= 1u64 << sq;
        sq += direction;
    }
    mask |= 1u64 << sq1;

    return mask;
}

fn gen_table_from_offsets(offsets: Vec<i8>) -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];

    for i in 0..64  {
        for j in 0..offsets.len() {
            let sq: i8 = i + offsets[j];
            
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

