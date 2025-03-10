
use crate::board::*;
use crate::generator::*;
use crate::moves::*;

pub fn perfting(board: &mut Board, state: &mut BoardTables, gen: &MoveGenerator,
    depth: u32
) -> u64 {
    let mut cnt: u64 = 0;
    let mut mvlst: MoveList = MoveList::new();

    if depth <= 0 {
        return 1;
    }

    // Generate the moves
    *state = gen.gen_board_tables(board);
    gen.gen_moves(&mut mvlst, board, state);

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        cnt += perfting(board, state, gen, depth - 1);
        board.unmake();
    }

    return cnt;
}

pub fn perft_cheating(board: &mut Board, state: &mut BoardTables, gen: &MoveGenerator,
    depth: u32
) -> u64 {
    let mut cnt: u64 = 0;
    let mut mvlst: MoveList = MoveList::new();

    // Generate the moves
    *state = gen.gen_board_tables(board);
    gen.gen_moves(&mut mvlst, board, state);

    if depth <= 1 {
        return mvlst.size() as u64;
    }

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        cnt += perfting(board, state, gen, depth - 1);
        board.unmake();
    }

    return cnt;
}

