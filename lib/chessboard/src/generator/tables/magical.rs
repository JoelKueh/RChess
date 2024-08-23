
use std::num::Wrapping;
use crate::board::bitboard;

const ROOK_MAGICS: [u64; 64] = [
    0xa8002c000108020u64,
    0x6c00049b0002001u64,
    0x100200010090040u64,
    0x2480041000800801u64,
    0x280028004000800u64,
    0x900410008040022u64,
    0x280020001001080u64,
    0x2880002041000080u64,
    0xa000800080400034u64,
    0x4808020004000u64,
    0x2290802004801000u64,
    0x411000d00100020u64,
    0x402800800040080u64,
    0xb000401004208u64,
    0x2409000100040200u64,
    0x1002100004082u64,
    0x22878001e24000u64,
    0x1090810021004010u64,
    0x801030040200012u64,
    0x500808008001000u64,
    0xa08018014000880u64,
    0x8000808004000200u64,
    0x201008080010200u64,
    0x801020000441091u64,
    0x800080204005u64,
    0x1040200040100048u64,
    0x120200402082u64,
    0xd14880480100080u64,
    0x12040280080080u64,
    0x100040080020080u64,
    0x9020010080800200u64,
    0x813241200148449u64,
    0x491604001800080u64,
    0x100401000402001u64,
    0x4820010021001040u64,
    0x400402202000812u64,
    0x209009005000802u64,
    0x810800601800400u64,
    0x4301083214000150u64,
    0x204026458e001401u64,
    0x40204000808000u64,
    0x8001008040010020u64,
    0x8410820820420010u64,
    0x1003001000090020u64,
    0x804040008008080u64,
    0x12000810020004u64,
    0x1000100200040208u64,
    0x430000a044020001u64,
    0x280009023410300u64,
    0xe0100040002240u64,
    0x200100401700u64,
    0x2244100408008080u64,
    0x8000400801980u64,
    0x2000810040200u64,
    0x8010100228810400u64,
    0x2000009044210200u64,
    0x4080008040102101u64,
    0x40002080411d01u64,
    0x2005524060000901u64,
    0x502001008400422u64,
    0x489a000810200402u64,
    0x1004400080a13u64,
    0x4000011008020084u64,
    0x26002114058042u64,
    ];

const BISHOP_MAGICS: [u64; 64] = [
    0x89a1121896040240u64,
    0x2004844802002010u64,
    0x2068080051921000u64,
    0x62880a0220200808u64,
    0x4042004000000u64,
    0x100822020200011u64,
    0xc00444222012000au64,
    0x28808801216001u64,
    0x400492088408100u64,
    0x201c401040c0084u64,
    0x840800910a0010u64,
    0x82080240060u64,
    0x2000840504006000u64,
    0x30010c4108405004u64,
    0x1008005410080802u64,
    0x8144042209100900u64,
    0x208081020014400u64,
    0x4800201208ca00u64,
    0xf18140408012008u64,
    0x1004002802102001u64,
    0x841000820080811u64,
    0x40200200a42008u64,
    0x800054042000u64,
    0x88010400410c9000u64,
    0x520040470104290u64,
    0x1004040051500081u64,
    0x2002081833080021u64,
    0x400c00c010142u64,
    0x941408200c002000u64,
    0x658810000806011u64,
    0x188071040440a00u64,
    0x4800404002011c00u64,
    0x104442040404200u64,
    0x511080202091021u64,
    0x4022401120400u64,
    0x80c0040400080120u64,
    0x8040010040820802u64,
    0x480810700020090u64,
    0x102008e00040242u64,
    0x809005202050100u64,
    0x8002024220104080u64,
    0x431008804142000u64,
    0x19001802081400u64,
    0x200014208040080u64,
    0x3308082008200100u64,
    0x41010500040c020u64,
    0x4012020c04210308u64,
    0x208220a202004080u64,
    0x111040120082000u64,
    0x6803040141280a00u64,
    0x2101004202410000u64,
    0x8200000041108022u64,
    0x21082088000u64,
    0x2410204010040u64,
    0x40100400809000u64,
    0x822088220820214u64,
    0x40808090012004u64,
    0x910224040218c9u64,
    0x402814422015008u64,
    0x90014004842410u64,
    0x1000042304105u64,
    0x10008830412a00u64,
    0x2520081090008908u64,
    0x40102000a0a60140u64,
    ];

const NUM_BISHOP_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

const NUM_ROOK_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

const MAX_BITS_IN_TABLE: u8 = 12;
const MAX_TABLE_SIZE: usize = 4096;

pub struct MagicalTable {
    pub num_bits: u8,
    pub data: Vec<u64>,
    pub occupancy_mask: u64,
    pub magic: u64
}

pub fn gen_bishop_attack_table() -> [MagicalTable; 64] {
    gen_table(true)
}

pub fn gen_rook_attack_table() -> [MagicalTable; 64] {
    gen_table(false)
}

pub fn read_magical_table(table: &MagicalTable, occupancy: u64) -> u64 {
    let key: u16 = get_key(&table, occupancy);
    return table.data[key as usize];
}

fn get_key(table: &MagicalTable, mut occupancy: u64) -> u16 {
    occupancy &= table.occupancy_mask;
    occupancy = (Wrapping(occupancy) * Wrapping(table.magic)).0;
    occupancy >>= 64 - table.num_bits;
    return occupancy as u16;
}

fn get_rook_occ_mask(sq: u8) -> u64 {
    let mut result: u64 = 0u64;
    let source_rank: i8 = (sq / 8).try_into().unwrap();
    let source_file: i8 = (sq % 8).try_into().unwrap();

    // Down
    let mut rank = source_rank + 1;
    while rank <= 6 {
        result |= 1u64 << source_file + rank * 8;
        rank += 1;
    }

    // Up
    let mut rank = source_rank - 1;
    while rank >= 1 {
        result |= 1u64 << source_file + rank * 8;
        rank -= 1;
    }

    // Right
    let mut file = source_file + 1;
    while file <= 6 {
        result |= 1u64 << file + source_rank * 8;
        file += 1;
    }

    // Left
    let mut file = source_file - 1;
    while file >= 1 {
        result |= 1u64 << file + source_rank * 8;
        file -= 1;
    }

    return result;
}

fn get_bishop_occ_mask(sq: u8) -> u64 {
    let mut result: u64 = 0u64;
    let source_rank: i8 = (sq / 8).try_into().unwrap();
    let source_file: i8 = (sq % 8).try_into().unwrap();

    // Down-Right
    let mut rank = source_rank + 1;
    let mut file = source_file + 1;
    while rank <= 6 && file <= 6 {
        result |= 1u64 << file + rank * 8;
        rank += 1;
        file += 1;
    }

    // Down-Left
    let mut rank = source_rank + 1;
    let mut file = source_file - 1;
    while rank <= 6 && file >= 1 {
        result |= 1u64 << file + rank * 8;
        rank += 1;
        file -= 1;
    }

    // Up-Right
    let mut rank = source_rank - 1;
    let mut file = source_file + 1;
    while rank >= 1 && file <= 6 {
        result |= 1u64 << file + rank * 8;
        rank -= 1;
        file += 1;
    }

    // Up-Left
    let mut rank = source_rank - 1;
    let mut file = source_file - 1;
    while rank >= 1 && file >= 1 {
        result |= 1u64 << file + rank * 8;
        rank -= 1;
        file -= 1;
    }

    return result;
}

pub fn get_bishop_attack_mask(sq: u8, occupied: u64) -> u64 {
    let mut result: u64 = 0u64;
    let source_rank: i8 = (sq / 8).try_into().unwrap();
    let source_file: i8 = (sq % 8).try_into().unwrap();

    // Down-Right
    let mut rank = source_rank + 1;
    let mut file = source_file + 1;
    while rank <= 7 && file <= 7 {
        result |= 1u64 << (file + rank * 8);
        if occupied & (1u64 << (file + rank * 8)) != 0 { break; }
        rank += 1;
        file += 1;
    }

    // Down-Left
    let mut rank = source_rank + 1;
    let mut file = source_file - 1;
    while rank <= 7 && file >= 0 {
        result |= 1u64 << (file + rank * 8);
        if occupied & (1u64 << (file + rank * 8)) != 0 { break; }
        rank += 1;
        file -= 1;
    }

    // Up-Right
    let mut rank = source_rank - 1;
    let mut file = source_file + 1;
    while rank >= 0 && file <= 7 {
        result |= 1u64 << (file + rank * 8);
        if occupied & (1u64 << (file + rank * 8)) != 0 { break; }
        rank -= 1;
        file += 1;
    }

    // Up-Left
    let mut rank = source_rank - 1;
    let mut file = source_file - 1;
    while rank >= 0 && file >= 0 {
        result |= 1u64 << (file + rank * 8);
        if occupied & (1u64 << (file + rank * 8)) != 0 { break; }
        rank -= 1;
        file -= 1;
    }

    return result;
}

pub fn get_rook_attack_mask(sq: u8, occupied: u64) -> u64 {
    let mut result: u64 = 0u64;
    let source_rank: i8 = (sq / 8).try_into().unwrap();
    let source_file: i8 = (sq % 8).try_into().unwrap();

    // Down
    let mut rank = source_rank + 1;
    while rank <= 7 {
        result |= 1u64 << (source_file + rank * 8);
        if occupied & (1u64 << (source_file + rank * 8)) != 0 { break; }
        rank += 1;
    }

    // Up
    let mut rank = source_rank - 1;
    while rank >= 0 {
        result |= 1u64 << (source_file + rank * 8);
        if occupied & (1u64 << (source_file + rank * 8)) != 0 { break; }
        rank -= 1;
    }

    // Right
    let mut file = source_file + 1;
    while file <= 7 {
        result |= 1u64 << (file + source_rank * 8);
        if occupied & (1u64 << (file + source_rank * 8)) != 0 { break; }
        file += 1;
    }

    // Left
    let mut file = source_file - 1;
    while file >= 0 {
        result |= 1u64 << (file + source_rank * 8);
        if occupied & (1u64 << (file + source_rank * 8)) != 0 { break; }
        file -= 1;
    }

    return result;
}

fn gen_table(is_bishop: bool) -> [MagicalTable; 64] {

    let mut legal_moves: [u64; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];
    let mut occupied_squares: [u64; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];

    let num_bits: &[u8; 64] = if is_bishop { &NUM_BISHOP_BITS } else { &NUM_ROOK_BITS };
    let mut tables: [MagicalTable; 64] = init_table_from_num_bits(num_bits);

    // For each square on the board...
    for sq in 0..64 {
        // Set up the different utility masks and magics for the table.
        let table: &mut MagicalTable = &mut tables[sq as usize];
        table.num_bits = if is_bishop {
            NUM_BISHOP_BITS[sq]
        } else {
            NUM_ROOK_BITS[sq]
        };

        table.occupancy_mask = if is_bishop {
            get_bishop_occ_mask(sq as u8)
        } else {
            get_rook_occ_mask(sq as u8)
        };

        table.magic = if is_bishop {
            BISHOP_MAGICS[sq]
        } else {
            ROOK_MAGICS[sq]
        };

        // Build the sets of all possible occupied squares and the corresponding set of all
        // possible legal moves.
        for j in 0..(1 << table.num_bits) {
            occupied_squares[j] = map_index_to_occupancy_mask(
                j as u16,
                table.num_bits,
                table.occupancy_mask
            );

            legal_moves[j] = if is_bishop {
                get_bishop_attack_mask(sq as u8, occupied_squares[j])
            } else {
                get_rook_attack_mask(sq as u8, occupied_squares[j])
            };
        }

        // Remap those sets into the hash table.
        for j in 0..(1 << table.num_bits) {
            let key: u16 = get_key(table, occupied_squares[j]);

            let legal_recalc: u64 = if is_bishop {
                get_bishop_attack_mask(sq as u8, occupied_squares[j])
            } else {
                get_rook_attack_mask(sq as u8, occupied_squares[j])
            };
            
            if table.data[key as usize] != 0 &&
                table.data[key as usize] != legal_recalc {
                println!("COLLISION ERROR!");
                println!("PIECE: {}", if is_bishop { "BISHOP" } else { "ROOK" });
                println!("SQ: {}", sq);
                println!("IDX: {}", j);
                println!("KEY: {}", key);
                bitboard::print(legal_recalc);
                println!("");
                bitboard::print(table.data[key as usize]);
                println!("");
                bitboard::print(occupied_squares[j]);
                panic!();
            }

            table.data[key as usize] = legal_moves[j as usize];
        }
    }

    return tables;
}

fn map_index_to_occupancy_mask(index: u16, num_bits: u8, mut occupancy_mask: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 0..num_bits {
        let j: u8 = bitboard::pop_rbit(&mut occupancy_mask);
        if index & (1 << i) != 0 {
            result |= 1u64 << j;
        }
    }
    return result;
}

fn init_table_from_num_bits(num_bits: &[u8; 64]) -> [MagicalTable; 64] {
    let mut tables = core::array::from_fn(|_| { MagicalTable {
        num_bits: 0u8,
        data: Vec::new(),
        occupancy_mask: 0u64,
        magic: 0u64
    }});
    for i in 0..64 {
        tables[i].data = vec![0; (1u16 << num_bits[i]) as usize];
    }
    return tables;
}
