use line_span::LineSpans;

#[derive(Clone, Copy, Debug)]
pub struct Comment {
    content: String,
    start: usize,
    end: usize,
}

impl Comment {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

trait Parser {
    fn parse(&self, content: &str) -> Vec<Comment>;
}
