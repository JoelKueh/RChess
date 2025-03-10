
use chessboard::perft::*;
use chessboard::generator::*;
use chessboard::moves::*;
use chessboard::board::*;

use std::time::Instant;

pub fn perft_cheat(board: &mut Board, depth: u32) {
    let mut mvlst: MoveList = MoveList::new();
    let gen: MoveGenerator = MoveGenerator::new();
    let mut state = gen.gen_board_tables(board);

    // Record the starting time.
    let now = Instant::now();
    let mut cnt: u64 = 0;

    // Generate the moves for the first layer.
    gen.gen_moves(&mut mvlst, board, &state);

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        let result: u64 = perft_cheating(board, &mut state, &gen, depth - 1);
        cnt += result;
        board.unmake();

        println!("{}: {}", mv.to_long_algbr(), result);
    }

    let elapsed = now.elapsed();
    println!("");
    println!("Nodes searched: {}", cnt);
    println!("Time: {:.2?}", elapsed);
    println!("");
}

pub fn perft(board: &mut Board, depth:u32) {
    let mut mvlst: MoveList = MoveList::new();
    let gen: MoveGenerator = MoveGenerator::new();
    let mut state = gen.gen_board_tables(board);

    // Record the starting time.
    let now = Instant::now();
    let mut cnt: u64 = 0;

    // Generate the moves for the first layer.
    gen.gen_moves(&mut mvlst, board, &state);

    if depth <= 0 {
        cnt = 1;
        let elapsed = now.elapsed();
        println!("Searched {} nodes in {:.2?} milliseconds.", cnt, elapsed);
        return;
    }

    for i in 0..mvlst.size() as usize {
        let mv: &Move = &mvlst.moves()[i];
        board.make(mv);
        let result: u64 = perfting(board, &mut state, &gen, depth - 1);
        cnt += result;
        board.unmake();

        println!("{}: {}", mv.to_long_algbr(), result);
    }

    let elapsed = now.elapsed();
    println!("");
    println!("Nodes searched: {}", cnt);
    println!("Time: {:.2?}", elapsed);
    println!("");
}

