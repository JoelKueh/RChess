
#![allow(unused_variables, dead_code, unused_mut)]

use crate::board::*;
use crate::generator::*;
use std::time::Instant;

fn perfting(board: &mut Board,
    mvlst: &mut MoveList,
    state: &mut BoardTables,
    gen: &MoveGenerator,
    depth: u32
) -> u64 {
    let mut cnt: u64 = 0;
    mvlst.clear();
    *state = gen.gen_board_tables(board);
    gen.gen_moves(mvlst, board, state);

    if depth == 0 {
        return 1;
    }

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        println!("{:?}", board.bitboard);
        cnt += perfting(board, mvlst, state, gen, depth - 1);
        board.unmake();
    }

    return cnt;
}

fn perft_cheating(board: &mut Board,
    mvlst: &mut MoveList,
    state: &mut BoardTables,
    gen: &MoveGenerator,
    cnt: &mut u64,
    depth: u32
) {

}

pub fn perft_debug(fen: &str, depth: u32) -> Result<(), FenError> {
    let mut board: Board = Board::from_fen(fen)?;
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let mut gen: MoveGenerator = MoveGenerator::new();
    let mut cnt: u64 = 0;

    return Ok(())
}

pub fn perft_cheat(fen: &str, depth: u32) -> Result<(), FenError> {
    let mut board: Board = Board::from_fen(fen)?;
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let mut gen: MoveGenerator = MoveGenerator::new();
    let mut cnt: u64 = 0;

    return Ok(())
}

pub fn perft(fen: &str, depth:u32) -> Result<(), FenError> {
    let mut board: Board = Board::from_fen(fen)?;
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let gen: MoveGenerator = MoveGenerator::new();

    let now = Instant::now();
    let cnt: u64 = perfting(&mut board, &mut mvlst, &mut state, &gen, depth);
    let elapsed = now.elapsed();

    println!("Searched {} nodes in {:.2?} milliseconds.", cnt, elapsed);

    return Ok(())
}
