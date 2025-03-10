
#![allow(unused_variables, dead_code, unused_mut)]

use chessboard::board::*;
use chessboard::generator::*;
use chessboard::moves::*;
use std::io::stdin;
use std::fmt;
use std::error::Error;
use std::process::exit;

mod board_printer;
mod perft_cli;

// Information about the version and executable name.
const NAME: &str = "RChess";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
enum CommandError {
    Invalid,
    Malformed(&'static str),
    ParseError(ParseError),
    MoveError(MoveError)
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "invalid command"),
            Self::Malformed(context) => write!(f, "malformed command: {}", context),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::MoveError(err) => write!(f, "{}", err),
        }
    }
}

impl From<ParseError> for CommandError {
    fn from(value: ParseError) -> Self {
        return Self::ParseError(value)
    }
}

impl From<MoveError> for CommandError {
    fn from(value: MoveError) -> Self {
        return Self::MoveError(value)
    }
}

impl Error for CommandError {}

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
    // Print version information.
    println!("{} version {} by Joel Kuehne", NAME, VERSION);

    // Setup the initial state.
    const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let generator: MoveGenerator = MoveGenerator::new();
    let mut moves: MoveList = MoveList::new();
    let mut board: Board = Board::from_fen(DEFAULT_FEN).expect("Unexpected fen error");
    let state: BoardTables = generator.gen_board_tables(&board);

    // Accept user commands.
    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Input error");
        input.pop().unwrap();
        match parse_input(&input, &mut board) {
            Err(err) => println!("{}", err),
            Ok(_) => ()
        }
    }
}

fn handle_move(cmd: Vec<&str>, board: &mut Board) -> Result<(), CommandError> {
    let Some(mv_str) = cmd.get(1) else {
        return Err(CommandError::Malformed("usage \"move <MOVE>\""));
    };
    let generator = MoveGenerator::new();
    let state = generator.gen_board_tables(board);
    let mut mvlst = MoveList::new();
    generator.gen_moves(&mut mvlst, board, &state);
    let mv = Move::from_uci_algbr(mv_str, &mvlst)?;
    board.make(&mv);
    return Ok(());
}

fn handle_debug(cmd: Vec<&str>, board: &mut Board) -> Result<(), CommandError> {
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
            Err(CommandError::Malformed("usage \"debug <ITEM>\""))
        },
        _ => Err(CommandError::Malformed("invalid debug item: usage \"debug <ITEM>\""))
    }
}

fn handle_go(cmd: Vec<&str>, board: &mut Board) -> Result<(), CommandError> {
    match cmd.get(1) {
        Some(&"perft") => {
            const MALFORMED: CommandError
                = CommandError::Malformed("usage \"go perft <DEPTH>\"");

            // Rust code can be so stupid at times.
            let depth: u32 = cmd.get(2).ok_or(MALFORMED)?
                .parse::<u32>().ok().ok_or(MALFORMED)?;
            perft_cli::perft(board, depth);

            Ok(())
        },
        _ => Err(CommandError::Malformed("invalid go command"))
    }
}

fn handle_position(cmd: Vec<&str>, board: &mut Board) -> Result<(), CommandError> {
    match cmd.get(1) {
        Some(&"fen") => {
            const MALFORMED: CommandError
                = CommandError::Malformed("usage \"position uci <FEN>\"");

            // Rust code can be so stupid at times.
            let Some(slice) = cmd.get(2..cmd.len()) else {
                return Err(CommandError::Malformed("usage \"position uci <FEN>\""));
            };
            let uci: String = slice.join(" ");
            *board = Board::from_uci(&uci)?;

            Ok(())
        }
        _ => Err(CommandError::Malformed("invalid go command"))
    }
}

fn parse_input(input: &str, board: &mut Board) -> Result<(), CommandError> {
    let command: Vec<&str> = input.split(' ').collect();
    match command.get(0) {
        Some(&"position") => handle_position(command, board),
        Some(&"move") => handle_move(command, board),
        Some(&"undo") => {
            board.unmake();
            Ok(())
        },
        Some(&"debug") => handle_debug(command, board),
        Some(&"board") => {
            board_printer::print_board_ascii(board);
            Ok(())
        },
        Some(&"go") => handle_go(command, board),
        Some(&"quit") => exit(0),
        None => Ok(()),
        _ => Err(CommandError::Invalid)
    }
}

