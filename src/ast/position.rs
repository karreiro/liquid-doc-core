use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(start: usize, end: usize) -> Self {
        Position { start, end }
    }
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>) -> Self {
        let span = pair.as_span();
        Position {
            start: span.start(),
            end: span.end(),
        }
    }
}
