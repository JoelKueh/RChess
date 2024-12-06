
use chessboard::board::*;
use const_format::formatcp;
use chessboard::moves::*;
use crossterm::style::{Color, Stylize};
use colored::{Colorize, CustomColor, ColoredString};

const LIGHT: Color = Color::Rgb { r: 154, g: 175, b: 219 };
const DARK: Color = Color::Rgb { r: 40, g: 54, b: 84 };
const BORDER: Color = Color::Rgb { r: 0, g: 0, b: 0 };

const V: [&str; 8] = [ " ","\u{2581}","\u{2582}","\u{2583}","\u{2584}",
    "\u{2585}","\u{2586}","\u{2587}"
];
const H: [&str; 8] = [ " ","\u{258F}","\u{258E}","\u{258D}","\u{258C}",
    "\u{258B}","\u{258A}","\u{2589}"
];

#[derive(Default)]
pub struct BoardDrawBuffer {
    pub data: [String; 26]
}

impl BoardDrawBuffer {
    pub fn new() -> BoardDrawBuffer {
        let mut buff: BoardDrawBuffer = Default::default();
        buff.data[0] = BoardDrawBuffer::make_border(true, false);
        buff.data[1] = BoardDrawBuffer::make_line(false);
        buff.data[2] = BoardDrawBuffer::make_border(false, true);
        buff.data[3] = BoardDrawBuffer::make_line(true);
        buff.data[4] = BoardDrawBuffer::make_border(false, false);
        buff.data[5] = BoardDrawBuffer::make_line(false);
        buff.data[6] = BoardDrawBuffer::make_border(false, true);
        buff.data[7] = BoardDrawBuffer::make_line(true);
        buff.data[8] = BoardDrawBuffer::make_border(false, false);
        buff.data[9] = BoardDrawBuffer::make_line(false);
        buff.data[10] = BoardDrawBuffer::make_border(false, true);
        buff.data[11] = BoardDrawBuffer::make_line(true);
        buff.data[12] = BoardDrawBuffer::make_border(false, false);
        buff.data[13] = BoardDrawBuffer::make_line(false);
        buff.data[14] = BoardDrawBuffer::make_border(false, true);
        buff.data[15] = BoardDrawBuffer::make_line(true);
        buff.data[16] = BoardDrawBuffer::make_border(true, true);
        return buff;
    }

    fn make_border(top_is_edge: bool, invert: bool) -> String {
        let e0 = H[4].with(BORDER);
        let s0 = e0.reverse();
        let mut edge = "\u{2584}\u{2584}\u{2584}".with(DARK).on(LIGHT);
        let mut invedge = edge;

        if invert {
            edge = edge.reverse();
        } else {
            invedge = invedge.reverse();
        }

        if top_is_edge {
            edge = edge.on(BORDER);
            invedge = invedge.with(BORDER);

            if invert {
                let temp = edge;
                edge = invedge;
                invedge = temp;
            }
        }

        let solid = " ".on(BORDER);
        return format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            s0, solid, edge, solid, invedge, solid, edge, solid, invedge, 
            solid, edge, solid, invedge, solid, edge, solid, invedge, solid, e0
        );
    }

    fn make_line(invert: bool) -> String {
        let e0 = H[4].with(BORDER);
        let s0 = e0.reverse();
        let mut inner = "   ".on(DARK);
        let mut invinner = "   ".on(LIGHT);
        let mut edge = H[4].on(LIGHT).with(DARK);
        let mut invedge = edge;
        let mut start = edge.reverse().on(BORDER);
        let mut end = edge.reverse().with(BORDER);

        if invert {
            let temp = inner;
            inner = invinner;
            invinner = temp;
            edge = edge.reverse();
            start = invedge.with(BORDER);
            end = invedge.on(BORDER);
        } else {
            invedge = invedge.reverse();
        }

        return format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            s0, start, inner, edge, invinner, invedge, inner, edge, invinner, 
            invedge, inner, edge, invinner, invedge, inner, edge, invinner, end, e0
        );
    }
}

//pub fn print_board_color() {
//    const BH: &str = "\u{2584}";
//    const TH: &str = "\u{2580}";
//    const LH: &str = "\u{258C}";
//    const RH: &str = "\u{2590}";
//    const BR: &str = "\u{2597}";
//    const UR: &str = "\u{259D}";
//    const BL: &str = "\u{2596}";
//    const UL: &str = "\u{2598}";
//
//    let vdl: ColoredString = TH.custom_color(BOARD_DARK).on_custom_color(BOARD_LIGHT);
//    let vld: ColoredString = TH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);
//    let hld: ColoredString = LH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);
//    let hdl: ColoredString = RH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);
//
//    let bl: ColoredString = BL.custom_color(BOARDER);
//    let br: ColoredString = BR.custom_color(BOARDER);
//    let ul: ColoredString = UL.custom_color(BOARDER);
//    let ur: ColoredString = UR.custom_color(BOARDER);
//    let hse: ColoredString = LH.custom_color(BOARDER).on_custom_color(BOARDER);
//    let hes: ColoredString = RH.custom_color(BOARDER).on_custom_color(BOARDER);
//    let vse: ColoredString = TH.custom_color(BOARDER).on_custom_color(BOARDER);
//    let ves: ColoredString = BH.custom_color(BOARDER).on_custom_color(BOARDER);
//
//    let hde: ColoredString = LH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
//    let hed: ColoredString = RH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
//    let vde: ColoredString = TH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
//    let ved: ColoredString = BH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
//
//    let hle: ColoredString = LH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
//    let hel: ColoredString = RH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
//    let vle: ColoredString = TH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
//    let vel: ColoredString = BH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
//
//    let d: ColoredString = " ".on_custom_color(BOARD_DARK);
//    let l: ColoredString = " ".on_custom_color(BOARD_LIGHT);
//    let s: ColoredString = " ".on_custom_color(BOARDER);
//}

fn print_colored_board(board: &Board) {
    const BH: &str = "\u{2584}";
    const TH: &str = "\u{2580}";
    const LH: &str = "\u{258C}";
    const RH: &str = "\u{2590}";
    const BR: &str = "\u{2597}";
    const UR: &str = "\u{259D}";
    const BL: &str = "\u{2596}";
    const UL: &str = "\u{2598}";

    const BOARD_LIGHT: CustomColor = CustomColor { r: 154, g: 175, b: 219 };
    const BOARD_DARK: CustomColor = CustomColor { r: 40, g: 54, b: 84 };
    const BOARDER: CustomColor = CustomColor { r: 0, g: 0, b: 0 };

    let vdl: ColoredString = TH.custom_color(BOARD_DARK).on_custom_color(BOARD_LIGHT);
    let vld: ColoredString = TH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);
    let hld: ColoredString = LH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);
    let hdl: ColoredString = RH.custom_color(BOARD_LIGHT).on_custom_color(BOARD_DARK);

    let bl: ColoredString = BL.custom_color(BOARDER);
    let br: ColoredString = BR.custom_color(BOARDER);
    let ul: ColoredString = UL.custom_color(BOARDER);
    let ur: ColoredString = UR.custom_color(BOARDER);
    let hse: ColoredString = LH.custom_color(BOARDER).on_custom_color(BOARDER);
    let hes: ColoredString = RH.custom_color(BOARDER).on_custom_color(BOARDER);
    let vse: ColoredString = TH.custom_color(BOARDER).on_custom_color(BOARDER);
    let ves: ColoredString = BH.custom_color(BOARDER).on_custom_color(BOARDER);

    let hde: ColoredString = LH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
    let hed: ColoredString = RH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
    let vde: ColoredString = TH.custom_color(BOARD_DARK).on_custom_color(BOARDER);
    let ved: ColoredString = BH.custom_color(BOARD_DARK).on_custom_color(BOARDER);

    let hle: ColoredString = LH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
    let hel: ColoredString = RH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
    let vle: ColoredString = TH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);
    let vel: ColoredString = BH.custom_color(BOARD_LIGHT).on_custom_color(BOARDER);

    let d: ColoredString = " ".on_custom_color(BOARD_DARK);
    let l: ColoredString = " ".on_custom_color(BOARD_LIGHT);
    let s: ColoredString = " ".on_custom_color(BOARDER);

    const FILE_LINE: &str = "     A   B   C   D   E   F   G   H";
    let top_line: String = format!(
        "  {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        hes, s, ved, ved, ved, ves, vel, vel, vel,
        ves, ved, ved, ved, ves, vel, vel, vel,
        ves, ved, ved, ved, ves, vel, vel, vel,
        ves, ved, ved, ved, ves, vel, vel, vel, s, hse
    );
    let top_line_inv: String = format!(
        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        ur, vse, vse, vse, vse, vse, vse, vse,
        vse, vse, vse, vse, vse, vse, vse, vse,
        vse, vse, vse, vse, vse, vse, vse, vse,
        vse, vse, vse, vse, vse, vse, vse, vse, ul
    );

    let piece_line: String = format!(
        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        hed, d, d, d, hdl, l, l, l,
        hld, d, d, d, hdl, l, l, l,
        hld, d, d, d, hdl, l, l, l,
        hld, d, d, d, hdl, l, l, l, hle
    );
    let piece_line_inv: String = format!(
        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        hel, l, l, l, hld, d, d, d,
        hdl, l, l, l, hld, d, d, d,
        hdl, l, l, l, hld, d, d, d,
        hdl, l, l, l, hld, d, d, d, hde
    );

    let bottom_line: String = format!(
        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        hes, vdl, vdl, vdl, s, vld, vld, vld,
        s, vdl, vdl, vdl, s, vld, vld, vld,
        s, vdl, vdl, vdl, s, vld, vld, vld,
        s, vdl, vdl, vdl, s, vld, vld, vld, hse
    );
    let bottom_line_inv: String = format!(
        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        hes, vld, vld, vld, s, vdl, vdl, vdl,
        s, vld, vld, vld, s, vdl, vdl, vdl,
        s, vld, vld, vld, s, vdl, vdl, vdl,
        s, vld, vld, vld, s, vdl, vdl, vdl, hse
    );

    println!("{}", FILE_LINE);
    println!("{}", top_line);
    println!("{}", piece_line.reversed());
    println!("{}", bottom_line);
    println!("{}", piece_line_inv);
    println!("{}", bottom_line_inv);
    println!("{}", piece_line);
    println!("{}", bottom_line);
    println!("{}", piece_line_inv);
    println!("{}", bottom_line_inv);

//    let board_str = board.str_rep_utf8();
//    for row in 0..7 as usize {
//        print!(" {} ", row + 1);
//        print!("{}", PIECE_LINE[0]);
//        for col in 0..7 as usize {
//            print!("{}{}", board_str[row][col], PIECE_LINE[1]);
//        }
//        print!("{}{}", board_str[row][7], PIECE_LINE[2]);
//        println!(" {}", row + 1);
//        println!("{}", INNER_LINE);
//    }
//    print!(" {} ", 8);
//    print!("{}", PIECE_LINE[0]);
//    for col in 0..7 as usize {
//        print!("{}{}", board_str[7][col], PIECE_LINE[1]);
//    }
//    print!("{}{}", board_str[7][7], PIECE_LINE[2]);
//    println!(" {}", 8);
//    println!("{}", BOTTOM_LINE);
//    println!("{}", FILE_LINE);
}

//fn print_board_pretty(board: &Board) {
//    const PIPE_H: char = '\u{2500}';
//    const PIPE_V: char = '\u{2502}';
//
//    const PIPE_DR: char = '\u{250C}';
//    const PIPE_DL: char = '\u{2510}';
//    const PIPE_UR: char = '\u{2514}';
//    const PIPE_UL: char = '\u{2519}';
//
//    const PIPE_VR: char = '\u{251C}';
//    const PIPE_VL: char = '\u{2524}';
//    const PIPE_HD: char = '\u{252C}';
//    const PIPE_HU: char = '\u{2534}';
//    const PIPE_HV: char = '\u{253C}';
//
//    const TOP_LINE: &str = formatcp!(
//        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//        PIPE_DR, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HD, PIPE_H, PIPE_H, PIPE_H, PIPE_DL
//    );
//
//    const PIECE_LINE: [&str; 3] = [
//        formatcp!("{} ", PIPE_V), formatcp!(" {} ", PIPE_V), formatcp!(" {}", PIPE_V)
//    ];
//
//    const INNER_LINE: &str = formatcp!(
//        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//        PIPE_VR, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HV, PIPE_H, PIPE_H, PIPE_H, PIPE_VL
//    );
//
//    const BOTTOM_LINE: &str = formatcp!(
//        "   {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//        PIPE_UR, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H,
//        PIPE_HU, PIPE_H, PIPE_H, PIPE_H, PIPE_UL
//    );
//
//    const FILE_LINE: &str = "     A   B   C   D   E   F   G   H";
//
//    println!("{}", FILE_LINE);
//    println!("{}", TOP_LINE);
//    let board_str = board.str_rep_utf8();
//    for row in 0..7 as usize {
//        print!(" {} ", row + 1);
//        print!("{}", PIECE_LINE[0]);
//        for col in 0..7 as usize {
//            print!("{}{}", board_str[row][col], PIECE_LINE[1]);
//        }
//        print!("{}{}", board_str[row][7], PIECE_LINE[2]);
//        println!(" {}", row + 1);
//        println!("{}", INNER_LINE);
//    }
//    print!(" {} ", 8);
//    print!("{}", PIECE_LINE[0]);
//    for col in 0..7 as usize {
//        print!("{}{}", board_str[7][col], PIECE_LINE[1]);
//    }
//    print!("{}{}", board_str[7][7], PIECE_LINE[2]);
//    println!(" {}", 8);
//    println!("{}", BOTTOM_LINE);
//    println!("{}", FILE_LINE);
//}

fn print_board_ascii(board: &Board) {
    const SEPARATOR_LINE: &str  = "   +---+---+---+---+---+---+---+---+";
    const PIECE_LINE: [&str; 3] = [ "| ", " | ", " |"];
    const FILE_LINE: &str = "     A   B   C   D   E   F   G   H";

    println!("{}", FILE_LINE);
    println!("{}", SEPARATOR_LINE);
    let board_str = board.str_rep();
    for row in 0..8 as usize {
        print!(" {} ", row + 1);
        print!("{}", PIECE_LINE[0]);
        for col in 0..7 as usize {
            print!("{}{}", board_str[row][col], PIECE_LINE[1]);
        }
        print!("{}{}", board_str[row][7], PIECE_LINE[2]);
        println!(" {}", row + 1);
        println!("{}", SEPARATOR_LINE);
    }
    println!("{}", FILE_LINE);
}

