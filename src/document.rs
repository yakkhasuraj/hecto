use crate::filetype::FileType;
use crate::Position;
use crate::Row;
use crate::SearchDirection;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    // pub fn open() -> Self {
    // pub fn open(filename: &str) -> Result<Self, std::io::Error> {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        // let mut start_with_comment = false;
        let mut rows = Vec::new();
        // rows.push(Row::from("Hello, World!"));
        // Self { rows }
        for value in contents.lines() {
            // rows.push(Row::from(value));
            // let mut row = Row::from(value);
            // row.highlight(None);
            // row.highlight(&file_type.highlighting_options(), None);
            // row.highlight(&file_type.highlighting_options(), None, false);
            rows.push(Row::from(value));
            // start_with_comment =
            //     row.highlight(&file_type.highlighting_options(), None, start_with_comment);
            // rows.push(row);
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            // file_type: FileType::from(filename),
            file_type,
        })
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        // if at.y > self.len() {
        if at.y > self.rows.len() {
            return;
        }

        // let new_row = Row::default();
        // if at.y == self.len() || at.y.saturating_add(1) == self.len() {
        //     self.rows.push(new_row);
        // } else {
        //     self.rows.insert(at.y + 1, new_row);
        // }

        // if at.y == self.len() {
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }

        // let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        #[allow(clippy::indexing_slicing)]
        // let new_row = self.rows[at.y].split(at.x);
        let current_row = &mut self.rows[at.y];
        // let mut new_row = current_row.split(at.x);
        let new_row = current_row.split(at.x);
        // current_row.highlight(None);
        // new_row.highlight(None);
        // current_row.highlight(&self.file_type.highlighting_options(), None, false);
        // new_row.highlight(&self.file_type.highlighting_options(), None, false);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        // if at.y > self.len() {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
            // return;
        } else if at.y == self.rows.len() {
            // if at.y == self.len() {
            // if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            // row.highlight(None);
            // row.highlight(&self.file_type.highlighting_options(), None, false);
            self.rows.push(row);
        // } else if at.y < self.len() {
        } else {
            // let row = self.rows.get_mut(at.y).unwrap();
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
            // row.highlight(None);
            // row.highlight(&self.file_type.highlighting_options(), None, false);
        }
        // self.highlight(None);
        self.unhighlight_rows(at.y);
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }

    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        // let len = self.len();
        let len = self.rows.len();
        // if at.y >= self.len() {
        if at.y >= len {
            return;
        }
        // let row = self.rows.get_mut(at.y).unwrap();
        // row.delete(at.x);

        self.dirty = true;
        // if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
        // if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y + 1 < len {
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            // let row = self.rows.get_mut(at.y).unwrap();
            let row = &mut self.rows[at.y];
            row.append(&next_row);
            // row.highlight(None);
            // row.highlight(&self.file_type.highlighting_options(), None, false);
        } else {
            // let row = self.rows.get_mut(at.y).unwrap();
            let row = &mut self.rows[at.y];
            row.delete(at.x);
            // row.highlight(None);
            // row.highlight(&self.file_type.highlighting_options(), None, false);
        }
        self.unhighlight_rows(at.y);
        // self.highlight(None);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            // for row in &self.rows {
            self.file_type = FileType::from(file_name);
            // let mut start_with_comment = false;
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
                // row.highlight(&self.file_type.highlighting_options(), None)
                // start_with_comment = row.highlight(
                //     &self.file_type.highlighting_options(),
                //     None,
                //     start_with_comment,
                // );
            }
            // self.file_type = FileType::from(file_name);
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    // pub fn find(&self, query: &str) -> Option<Position> {
    //     for (y, row) in self.rows.iter().enumerate() {
    //         if let Some(x) = row.find(query) {
    // pub fn find(&self, query: &str, after: &Position) -> Option<Position> {
    //     let mut x = after.x;
    //     for (y, row) in self.rows.iter().enumerate().skip(after.y) {
    //         if let Some(x) = row.find(query, x) {
    //             return Some(Position { x: x, y: y });
    //         }
    //         x = 0;
    //     }
    #[allow(clippy::indexing_slicing)]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }

                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }

        None
    }

    // pub fn highlight(&mut self, word: Option<&str>) {
    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;

        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };

        // for row in &mut self.rows {
        #[allow(clippy::indexing_slicing)]
        for row in &mut self.rows[..until] {
            // row.highlight(word);
            // row.highlight(&self.file_type.highlighting_options(), word);
            start_with_comment = row.highlight(
                &self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }
}
