extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::fmt;

const BOARD_SIZE: i32 = 64;

struct Board {
    white: u64,
    black: u64,
}

impl Board {
    fn new() -> Board {
        Board {
            white: 0x0000001008000000,
            black: 0x0000000810000000,
        }
    }
}

// println!マクロで指定フォーマットで出力できるように
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for i in 0..BOARD_SIZE {
            let bit_pos = i as u64;
            if get_bit(&self.white, &bit_pos)==1 as u64 {
                board.push_str("[@]");
            } else if get_bit(&self.black, &bit_pos)==1 as u64 {
                board.push_str("[O]");
            } else {
                board.push_str("[ ]");
            }

            if ((i+1) % 8) == 0 { 
                board.push_str("\r\n");
            }
        }
        write!(f, "{}", board)
    }
}

fn get_bit(bitboard: &u64, pos: &u64) -> u64 {
    return (bitboard >> pos) & (1 as u64);
}

fn print_test() {
    let board = Board::new();
    println!("{}", board);
}

fn main() {
    let stdin = stdin();

    // rawモードに移行
    // Alternate screenを使い、App終了後にもとの画面に戻るようにする。
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    //画面全体をクリアし、カーソルを先頭へ
    write!(stdout, "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1,1)).unwrap();

    // writeln使わない場合、stdoutはbufferingするらしいので、
    //　即座に反映させる
    stdout.flush().unwrap();

    print_test();
    
    for c in stdin.events() {
        let evt = c.unwrap();

        println!("{}", "test");

        match evt {
            // Ctrl-cでアプリケーション終了
            Event::Key(Key::Ctrl('c')) => break,
            _ => {}
        }
    }

    write!(stdout, "{}", "Rust-Reversi suspended").unwrap();
}