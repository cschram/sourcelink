use crate::error::*;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Comment<'source> {
    content: &'source str,
    start: usize,
    end: usize,
}

impl<'source> Comment<'source> {
    pub fn new(content: &'source str, start: usize, end: usize) -> Self {
        Self {
            content,
            start,
            end,
        }
    }

    pub fn content(&self) -> &'source str {
        &self.content
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

pub trait Parser<'source> {
    fn parse(&self, content: &'source str) -> Result<Vec<Comment<'source>>>;
}

pub fn substr<'source>(s: &'source str, start: usize, end: usize) -> Result<&'source str> {
    if end > s.len() {
        Err(SourcelinkError::OutOfRange(end, 0, s.len()).into())
    } else {
        Ok(&s[start..end])
    }
}
