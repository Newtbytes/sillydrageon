use std::fmt;

use crate::parser::Token;

#[derive(Debug)]
pub struct Source {
    pub text: String,
}

impl Source {
    pub fn get_span(&self, tok: &Token) -> Option<Span> {
        if let Some(pos) = self.pos_of(tok.offset) {
            Some(Span {
                pos,
                len: tok.value.len(),
            })
        } else {
            None
        }
    }

    fn pos_of(&self, offset: usize) -> Option<Position> {
        get_pos(self, offset)
    }
}

impl From<&str> for Source {
    fn from(src: &str) -> Self {
        Source {
            text: src.to_owned(),
        }
    }
}

struct Position<'src> {
    src: &'src Source,
    line: usize,
    col: usize,
}

pub struct Span<'src> {
    pos: Position<'src>,
    len: usize,
}

impl<'src> Into<&'src Source> for Span<'src> {
    fn into(self) -> &'src Source {
        self.pos.src
    }
}

impl fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = self.pos.line;
        let line = self.pos.src.text.lines().nth(line).unwrap_or("<n/a>");

        let pointer = " ".repeat(self.pos.col) + &"^".repeat(self.len);

        write!(f, "{}", line.to_owned() + "\n" + &pointer)
    }
}

fn get_pos(src: &Source, offset: usize) -> Option<Position> {
    if offset >= src.text.len() {
        return None;
    }

    let mut line_offset = 0;

    for (lineno, line) in src.text.lines().enumerate() {
        if offset < line_offset + line.len() {
            let col = offset - line_offset;
            return Some(Position {
                line: lineno,
                col,
                src,
            });
        } else {
            line_offset += line.len();
        }
    }

    return None;
}
