use crate::error::QueryError;

pub fn escape(src: &str, dst: &mut String) {
    for c in src.chars() {
        match c {
            '\\' => dst.push_str("\\\\"),
            '/' => dst.push_str("\\/"),
            ' ' => dst.push_str("\\s"),
            '|' => dst.push_str("\\p"),
            '\x07' => dst.push_str("\\a"),
            '\x08' => dst.push_str("\\b"),
            '\x0C' => dst.push_str("\\f"),
            '\n' => dst.push_str("\\n"),
            '\r' => dst.push_str("\\r"),
            '\t' => dst.push_str("\\t"),
            '\x0B' => dst.push_str("\\v"),
            _ => dst.push(c),
        }
    }
}

pub fn unescape(src: &str, dst: &mut String) -> Result<(), QueryError> {
    let mut escape = false;

    for c in src.chars() {
        if escape {
            match c {
                '\\' => dst.push('\\'),
                '/' => dst.push('/'),
                's' => dst.push(' '),
                'p' => dst.push('|'),
                'a' => dst.push('\x07'),
                'b' => dst.push('\x08'),
                'f' => dst.push('\x0C'),
                'n' => dst.push('\n'),
                'r' => dst.push('\r'),
                't' => dst.push('\t'),
                'v' => dst.push('\x0B'),
                _ => {
                    dst.push('\\');
                    dst.push(c);
                },
            }

            escape = false;
        } else if c == '\\' {
            escape = true;
        } else {
            dst.push(c);
        }
    }

    if escape {
        Err(QueryError::MalformedEscapeSequence { src: dst.to_string() })
    } else {
        Ok(())
    }
}
