extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

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
    
    for c in stdin.events() {
        let evt = c.unwrap();

        write!(stdout, "{}", "Test").unwrap();

        match evt {
            // Ctrl-cでアプリケーション終了
            Event::Key(Key::Ctrl('c')) => break,
            _ => {}
        }
    }

    write!(stdout, "{}", "Rust-Reversi suspended").unwrap();
}