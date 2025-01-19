
#![allow(unused_variables, dead_code, unused_mut)]

use crate::board::*;
use crate::generator::*;
use std::time::Instant;

fn perfting(board: &mut Board, state: &mut BoardTables, gen: &MoveGenerator,
    depth: u32
) -> u64 {
    let mut cnt: u64 = 0;
    let mut mvlst: MoveList = MoveList::new();

    // Generate the moves
    *state = gen.gen_board_tables(board);
    println!("{:?}", state);
    println!("{:?}", board.turn);
    gen.gen_moves(&mut mvlst, board, state);

    if depth == 0 {
        return 1;
    }
    println!("{}", mvlst);

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        println!("{}", mv);
        println!("{}", board);
        println!("{:?}", board.bitboard);
        cnt += perfting(board, state, gen, depth - 1);
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

pub fn perft_debug(board: &mut Board, depth: u32) -> Result<(), ParseError> {
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let mut gen: MoveGenerator = MoveGenerator::new();
    let mut cnt: u64 = 0;

    return Ok(())
}

pub fn perft_cheat(board: &mut Board, depth: u32) -> Result<(), ParseError> {
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let mut gen: MoveGenerator = MoveGenerator::new();
    let mut cnt: u64 = 0;

    return Ok(())
}

pub fn perft(board: &mut Board, depth:u32) {
    let mut mvlst: MoveList = MoveList::new();
    let mut state: BoardTables = Default::default();
    let gen: MoveGenerator = MoveGenerator::new();

    let now = Instant::now();
    let cnt: u64 = perfting(board, &mut state, &gen, depth);
    let elapsed = now.elapsed();

    println!("Searched {} nodes in {:.2?} milliseconds.", cnt, elapsed);
}
