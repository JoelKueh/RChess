
#![allow(unused_variables, dead_code, unused_mut)]

use chessboard::board::*;
use chessboard::generator::*;
use chessboard::moves::*;
use std::io::stdin;
use std::fmt;

mod board_printer;

#[derive(Debug)]
struct CmdParseErr(String);
impl fmt::Display for CmdParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid command: {}", self.0)
    }
}

#[derive(Debug)]
enum CmdFenErr {
    FenParseErr(FenError),
    CmdParseErr(String)
}
impl fmt::Display for CmdFenErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FenError::FenParseErr(msg) => {
                write!(f, "invalid fen string provided ({})", msg)
            }
        }
    }
}

#[derive(Debug)]
enum CmdMoveErr {
    MoveParseErr(MoveDecodeErr),
    CmdParseErr(String)
}
impl fmt::Display for CmdMoveErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid command: {}", self.0)
    }
}

#[derive(Debug)]
enum CmdDebugErr {
    CmdParseErr(String)
}
impl fmt::Display for CmdDebugErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid command: {}", self.0)
    }
}

#[derive(Debug)]
enum InputErr {
    Fen(CmdFenErr),
    Move(CmdMoveErr),
    Debug(CmdDebugErr),
    Invalid(CmdParseErr)
}
impl fmt::Display for InputErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("");
    }
}

fn get_sq_colored(board_str: &[[char; 8]; 8], row: u8, col: u8) {
    
}

fn get_board_buffer(board: &Board) {

}

fn print_mvlst(mvlst: &MoveList) {
    println!("{}", mvlst.size());
    for i in 0..mvlst.size() {
        let mv: &Move = mvlst.at(i);
        println!("{}", mv.to_long_algbr());
    }
}

fn main() {
    const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let generator: MoveGenerator = MoveGenerator::new();
    let mut moves: MoveList = MoveList::new();
    let mut board: Board = Board::from_fen(DEFAULT_FEN).expect("Unexpected fen error");

    //print_board_ascii(&board);

    let state: BoardTables = generator.gen_board_tables(&board);
    //println!("{:?}", state);
    //println!("{:?}", board.bitboard);
    generator.gen_moves(&mut moves, &board, &state);
    //print_mvlst(&moves);

    //perft::perft(DEFAULT_FEN, 1).expect("Unexpected fen error");
    board.make(&Move::from_uci_algbr("a2a3".into(), &moves).expect(""));
    board.make(&Move::from_uci_algbr("a7a6".into(), &moves).expect(""));
    println!("{:?}", board.bitboard);

    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Input error");
        match parse_input(&input, &mut board) {
            Err(err) => println!("{}", err),
            Ok(_) => ()
        }
    }
}

fn handle_fen(cmd: Vec<&str>, board: &mut Board) -> Result<(), CmdFenErr> {
    let Some(slice) = cmd.get(1..fen.len()) else {
        return Err(CmdFenErr::CmdParseErr("Missing argument \"FEN\": usage fen <FEN>".into()));
    };
    let fen: String = slice.join(" ");
    *board = Board::from_fen(&fen)?;
    return Ok(())
}

fn handle_move(cmd: Vec<&str>, board: &mut Board) -> Result<(), CmdMoveErr> {
    let Some(mv_str) = cmd.get(1) else {
        return Err(CmdMoveErr::CmdParseErr("Missing argument \"MOVE\": usage move <MOVE>".into()));
    };
    let generator = MoveGenerator::new();
    let state = generator.gen_board_tables(board);
    let mut mvlst = MoveList::new();
    generator.gen_moves(&mut mvlst, board, &state);
    let mv = Move::from_uci_algbr(mv_str, &mvlst)?;
    board.make(&mv);
    return Ok(());
}

fn handle_debug(cmd: Vec<&str>, board: &mut Board) -> Result<(), CmdDebugErr> {
    match cmd.get(1) {
        Some(&"moves") => {
            let generator = MoveGenerator::new();
            let state = generator.gen_board_tables(board);
            let mut mvlst = MoveList::new();
            generator.gen_moves(&mut mvlst, board, &state);
            println!("{}", mvlst);
            Ok(())
        },
        Some(&"state") => {
            let generator = MoveGenerator::new();
            let state = generator.gen_board_tables(board);
            println!("{:?}", state);
            Ok(())
        },
        Some(&"bitboard") => {
            println!("{:?}", board.bitboard);
            Ok(())
        },
        None => {
            Err(CmdDebugErr::CmdParseErr("Missing argument \"ITEM\": usage debug <ITEM>".into()))
        },
        _ => Err(CmdDebugErr::CmdParseErr("Invlid argument {} for debug command".into()))
    }
}

fn parse_input(input: &str, board: &mut Board) -> Result<(), InputErr> {
    let command: Vec<&str> = input.split(' ').collect();
    match command.get(0) {
        Some(&"fen") => handle_fen(command, board),
        Some(&"move") => handle_move(command, board),
        Some(&"debug") => handle_debug(command, board),
        None => Ok(()),
        _ => Err(InputErr::Invalid(CmdParseErr(format!("invalid command"))))
    }
}
