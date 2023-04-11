use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;
// use crate::document;
// use std::string;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(PartialEq, Clone, Copy)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
}

impl Editor {
    pub fn run(&mut self) {
        // let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }

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
        //             _ => println!("{key:?}\r",),
        //         },
        //         Err(err) => die(&err),
        //     }
        // }
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");
        // let document = if args.len() > 1 {
        //     let file_name = &args[1];
        //     // Document::open(&file_name).unwrap_or_default()
        //     let doc = Document::open(&file_name);
        //     if doc.is_ok() {
        //         doc.unwrap()

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            // cursor_position: Position { x: 0, y: 0 },
            cursor_position: Position::default(),
            // document: Document::default(),
            // document: Document::open(),
            document,
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
            highlighted_word: None,
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        // print!("\x1b[2j");
        // print!("{} {}", All, Goto(1, 1));
        Terminal::cursor_hide();
        // Terminal::clear_screen();
        // Terminal::cursor_position(0, 0);
        // Terminal::cursor_position(&Position { x: 0, y: 0 });
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            // print!("{}", Goto(1, 1));
            // Terminal::cursor_position(0, 0);
            // Terminal::cursor_position(&self.cursor_position);
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }

        // io::stdout().flush()
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            // let new_name = self.prompt("Save as: ").unwrap_or(None);
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }

    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        // if let Some(query) = self
        // .prompt("Search: ", |editor, _, query| {
        //     if let Some(position) = editor.document.find(&query) {
        //         editor.cursor_position = position;
        //         editor.scroll();
        //     }
        // })
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        }
                        // _ => (),
                        Key::Left | Key::Up => direction = SearchDirection::Backward,
                        _ => direction = SearchDirection::Forward,
                    }

                    // if let Some(position) = editor.document.find(&query, &editor.cursor_position) {
                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                    // editor.document.highlight(Some(query));
                    editor.highlighted_word = Some(query.to_string());
                },
            )
            .unwrap_or(None);
        // {
        //     if let Some(position) = self.document.find(&query[..], &old_position) {
        //         self.cursor_position = position;
        //     } else {
        //         self.status_message = StatusMessage::from(format!("Not found :{}.", query));
        //     }
        // } else {
        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        // self.document.highlight(None);
        self.highlighted_word = None;
        // }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // let pressed_key = read_key()?;

        let pressed_key = Terminal::read_key()?;

        match pressed_key {
            // Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('q') => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true;
            }
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('f') => self.search(),
            // Key::Ctrl('f') => {
            //     // if let Some(query) = self.prompt("Search: ").unwrap_or(None) {
            //     if let Some(query) = self
            //         .prompt("Search: ", |editor, _, query| {
            //             if let Some(position) = editor.document.find(&query) {
            //                 editor.cursor_position = position;
            //                 editor.scroll();
            //             }
            //         })
            //         .unwrap_or(None)
            //     {
            //         if let Some(position) = self.document.find(&query[..]) {
            //             self.cursor_position = position;
            //         } else {
            //             self.status_message = StatusMessage::from(format!("Not found :{}.", query));
            //         }
            //     }
            // }
            // Key::Ctrl('s') => {
            //     if self.document.file_name.is_none() {
            //         self.document.file_name = Some(self.prompt("Save as: ")?);
            //     }

            //     if self.document.save().is_ok() {
            //         self.status_message =
            //             StatusMessage::from("File saved successfully".to_string());
            //     } else {
            //         self.status_message = StatusMessage::from("Error writing file!".to_string());
            //     }
            // }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }

        self.scroll();
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;

        // let size = self.terminal.size();
        // let height = size.height.saturating_sub(1) as usize;
        let height = self.document.len();
        // let width = size.width.saturating_sub(1) as usize;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            // Key::Down => y = y.saturating_add(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            // Key::Left => x = x.saturating_sub(1),
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            // Key::Right => x = x.saturating_add(1),
            Key::Right => {
                if x < width {
                    // x = x.saturating_add(1)
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            // Key::PageUp => y = 0,
            // Key::PageDown => y = height,
            Key::PageUp => {
                y = if y > terminal_height {
                    // y - terminal_height
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    // y + terminal_height as usize
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        println!("{welcome_message}\r");
    }

    pub fn draw_row(&self, row: &Row) {
        // let start = 0;
        // let end = self.terminal.size().width as usize;
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        // let end = self.offset.x + width;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{row}\r");
    }

    #[allow(clippy::integer_division, clippy::integer_arithmetic)]
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        // for terminal_row in 0..height - 1 {
        for terminal_row in 0..height {
            Terminal::clear_current_line();

            // if let Some(row) = self.document.row(terminal_row as usize) {
            // if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
            // if let Some(row) = self
            //     .document
            //     .row(self.offset.y.saturating_add(terminal_row as usize))
            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row)
            } else if self.document.is_empty() && terminal_row == height / 3 {
                // println!("Hecto editor -- version {}\r", VERSION);
                // let welcome_message = format!("Hecto editor -- version {}", VERSION);
                // let width =
                //     std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
                // println!("{}\r", &welcome_message[..width]);
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        // let spaces = " ".repeat(self.terminal.size().width as usize);
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        // status = format!("{} - {} lines", file_name, self.document.len());
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );
        // if width > status.len() {
        //     status.push_str(&" ".repeat(width - status.len()));
        // }
        let line_indicator = format!(
            "{} | {}/{}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        #[allow(clippy::integer_arithmetic)]
        let len = status.len() + line_indicator.len();
        // if width > len {
        //     status.push_str(&"".repeat(width - len));
        // }
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{status}{line_indicator}");
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        // println!("{}\r", spaces);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{text}");
        }
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            // if let Key::Char(c) = Terminal::read_key()? {
            //     if c == '\n' {
            //         self.status_message = StatusMessage::from(String::new());
            //         break;
            //     }
            //     if !c.is_control() {
            //         result.push(c);
            //     }
            // }
            let key = Terminal::read_key()?;
            // match Terminal::read_key()? {
            match key {
                // Key::Backspace => {
                //     if !result.is_empty() {
                //         result.truncate(result.len() - 1);
                //     }
                // }
                Key::Backspace => result.truncate(result.len().saturating_sub(1)),
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result);
        }

        // Ok(result)
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

// fn read_key() -> Result<Key, std::io::Error> {
//     loop {
//         if let Some(key) = io::stdin().lock().keys().next() {
//             return key;
//         }
//     }
// }

fn die(e: std::io::Error) {
    // print!("{}", All);

    Terminal::clear_screen();

    panic!("{}", e);
}
