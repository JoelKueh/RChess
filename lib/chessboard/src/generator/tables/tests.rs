
use crate::board::*;
use crate::generator::tables::*;

#[test]
fn test_pawn_attacks() {
    let tables: &MoveTables = MoveTables::get_instance();
    assert_eq!(MoveTables::read_pawn_attacks(tables,  1, WHITE as u8), 0x0000000000000000u64);
    assert_eq!(MoveTables::read_pawn_attacks(tables,  9, BLACK as u8), 0x0000000000050000u64);
    assert_eq!(MoveTables::read_pawn_attacks(tables, 24, BLACK as u8), 0x0000000200000000u64);
    assert_eq!(MoveTables::read_pawn_attacks(tables, 31, BLACK as u8), 0x0000004000000000u64);
    assert_eq!(MoveTables::read_pawn_attacks(tables, 47, WHITE as u8), 0x0000004000000000u64);
}

#[test]
fn test_knight_attacks() {
    let tables: &MoveTables = MoveTables::get_instance();
    assert_eq!(MoveTables::read_knight_attacks(tables,  1), 0x0000000000050800u64);
    assert_eq!(MoveTables::read_knight_attacks(tables, 28), 0x0000284400442800u64);
    assert_eq!(MoveTables::read_knight_attacks(tables, 62), 0x0010A00000000000u64);
}

#[test]
fn test_rook_attacks() {
    struct Pattern {
        sq: u8,
        occ: u64,
        res: u64
    }

    let tables: &MoveTables = MoveTables::get_instance();
    let tests: [Pattern; 7] = [
        Pattern { sq:  2, occ: 0x0000004000000000u64, res: 0x04040404040404FBu64 },
        Pattern { sq:  0, occ: 0x0100000000000080u64, res: 0x01010101010101FEu64 },
        Pattern { sq:  0, occ: 0x0000000000000000u64, res: 0x01010101010101FEu64 },
        Pattern { sq:  0, occ: 0x0000000001000020u64, res: 0x000000000101013Eu64 },
        Pattern { sq:  0, occ: 0x0000000001000040u64, res: 0x000000000101017Eu64 },
        Pattern { sq: 28, occ: 0x1000000022001000u64, res: 0x101010102E101000u64 },
        Pattern { sq: 63, occ: 0x4080000000000000u64, res: 0x4080000000000000u64 },
    ];

    for test in tests {
        asrt_bb(
            MoveTables::read_rook_attacks(tables, test.sq, test.occ),
            test.res,
            test.occ,
            "lookup!"
        );
    }
}

#[test]
fn test_bishop_attacks() {
    struct Pattern {
        sq: u8,
        occ: u64,
        res: u64
    }

    let tables: &MoveTables = MoveTables::get_instance();
    let tests: [Pattern; 4] = [
        Pattern { sq:  0, occ: 0x0000000000000000u64, res: 0x8040201008040200u64 },
        Pattern { sq:  2, occ: 0x0000000020010000u64, res: 0x0000000020110A00u64 },
        Pattern { sq:  2, occ: 0x0000000000000080u64, res: 0x0000804020110A00u64 },
        Pattern { sq: 63, occ: 0x0000000008000000u64, res: 0x0040201008000000u64 },
    ];

    for test in tests {
        asrt_bb(
            MoveTables::read_bishop_attacks(tables, test.sq, test.occ),
            test.res,
            test.occ,
            "lookup!"
        );
    }
}

fn asrt_bb(bb: u64, actual: u64, occ: u64, msg: &str) {
    if bb != actual {
        println!("\nFAILURE: {}", msg);
        println!("Result: {}", bb);
        println!("Actual: {}\n", actual);
        bitboard::print(occ);
        println!("");
        bitboard::print_debug(bb, actual);
        println!("");
    }
    assert_eq!(bb, actual);
}
