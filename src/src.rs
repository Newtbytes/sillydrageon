use crate::parser::Token;

struct Source {
    text: String
}

struct Position {
    line: usize,
    col: usize,
}

struct Span {
    pos: Position,
    len: usize
}

fn get_pos(src: String, offset: usize) -> Option<(Position)> {
    if offset < src.len() {
        return None;
    }

    let mut line_offset = 0;

    for (lineno, line) in src.lines().enumerate() {
        if offset < line_offset + line.len() {
            let col = offset - line_offset;
            return Some(Position { line: lineno, col });
        } else {
            line_offset += line.len();
        }
    }

    return None;
}
