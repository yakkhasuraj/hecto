// use std::io::{self, stdout};
// use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
pub use terminal::Terminal;

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte & 0b0001_1111
// }

// fn die(e: std::io::Error) {
//     panic!("{}", e);
// }

fn main() {
    // let _stdout = stdout().into_raw_mode().unwrap();

    // let editor = Editor::default();
    // editor.run();
    Editor::default().run();

    // for b in io::stdin().bytes() {
    // let c = b.unwrap() as char;
    // println!("{}", c);

    // let b = b.unwrap();
    // let c = b as char;
    // if c.is_control() {
    //     println!("{:?} \r", b);
    // } else {
    //     println!("{:?} ({})\r", b, c);
    // }

    // if c == 'q' {
    //     break;
    // }

    // match b {
    //     Ok(b) => {
    //         let c = b as char;
    //         if c.is_control() {
    //             println!("{:?} \r", b);
    //         } else {
    //             println!("{:?} ({})\r", b, c);
    //         }
    //         if b == to_ctrl_byte('q') {
    //             break;
    //         }
    //     }
    //     Err(err) => die(err),
    // }
    // }

    // for key in io::stdin().keys() {
    //     match key {
    //         Ok(key) => match key {
    //             Key::Char(c) => {
    //                 if c.is_control() {
    //                     println!("{:?}\r", c as u8);
    //                 } else {
    //                     println!("{:?} ({})\r", c as u8, c);
    //                 }
    //             }
    //             Key::Ctrl('q') => break,
    //             _ => println!("{:?}\r", key),
    //         },
    //         Err(err) => die(err),
    //     }
    // }
}
