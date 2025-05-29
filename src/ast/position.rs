use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, offset: Option<usize>) -> Self {
        match offset {
            Some(o) => Position {
                start: start + o,
                end: end + o,
            },
            None => Position { start, end },
        }
    }
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>, offset: Option<usize>) -> Self {
        let span = pair.as_span();
        Position::new(span.start(), span.end(), offset)
    }

    pub fn shift_start(&mut self, offset: usize) {
        self.start += offset;
    }
}
