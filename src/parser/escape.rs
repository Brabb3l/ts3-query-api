use crate::error::ParseError;

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

pub fn unescape(src: &[u8], dst: &mut String) -> Result<(), ParseError> {
    let src = std::str::from_utf8(src)
        .map_err(ParseError::Utf8)?;
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
        Err(ParseError::MalformedEscapeSequence { src: dst.to_string() })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_escape_none() {
        let mut dst = String::new();

        escape("test", &mut dst);
        assert_eq!(dst, "test");
    }

    #[test]
    fn test_escape() {
        let mut dst = String::new();

        escape("test\\test", &mut dst);
        assert_eq!(dst, "test\\\\test");

        let mut dst = String::new();

        escape("test/test", &mut dst);
        assert_eq!(dst, "test\\/test");

        let mut dst = String::new();

        escape("test test", &mut dst);
        assert_eq!(dst, "test\\stest");

        let mut dst = String::new();

        escape("test|test", &mut dst);
        assert_eq!(dst, "test\\ptest");

        let mut dst = String::new();

        escape("test\x07test", &mut dst);
        assert_eq!(dst, "test\\atest");

        let mut dst = String::new();

        escape("test\x08test", &mut dst);
        assert_eq!(dst, "test\\btest");

        let mut dst = String::new();

        escape("test\x0Ctest", &mut dst);
        assert_eq!(dst, "test\\ftest");

        let mut dst = String::new();

        escape("test\ntest", &mut dst);
        assert_eq!(dst, "test\\ntest");

        let mut dst = String::new();

        escape("test\rtest", &mut dst);
        assert_eq!(dst, "test\\rtest");

        let mut dst = String::new();

        escape("test\ttest", &mut dst);
        assert_eq!(dst, "test\\ttest");

        let mut dst = String::new();

        escape("test\x0Btest", &mut dst);
        assert_eq!(dst, "test\\vtest");
    }

    #[test]
    fn test_escape_mixed() {
        let mut dst = String::new();

        escape("test\\test/test test|test\x07test\x08test\x0Ctest\ntest\rtest\ttest\x0Btest", &mut dst);
        assert_eq!(dst, "test\\\\test\\/test\\stest\\ptest\\atest\\btest\\ftest\\ntest\\rtest\\ttest\\vtest");
    }

    #[test]
    fn test_escape_error() {
        let mut dst = String::new();

        assert!(matches!(unescape(b"test\\", &mut dst), Err(ParseError::MalformedEscapeSequence { .. })));
    }

    #[test]
    fn test_unescape_none() {
        let mut dst = String::new();

        unescape(b"test", &mut dst).unwrap();
        assert_eq!(dst, "test");
    }

    #[test]
    fn test_unescape() {
        let mut dst = String::new();

        unescape(b"test\\\\test", &mut dst).unwrap();
        assert_eq!(dst, "test\\test");

        let mut dst = String::new();

        unescape(b"test\\/test", &mut dst).unwrap();
        assert_eq!(dst, "test/test");

        let mut dst = String::new();

        unescape(b"test\\stest", &mut dst).unwrap();
        assert_eq!(dst, "test test");

        let mut dst = String::new();

        unescape(b"test\\ptest", &mut dst).unwrap();
        assert_eq!(dst, "test|test");

        let mut dst = String::new();

        unescape(b"test\\atest", &mut dst).unwrap();
        assert_eq!(dst, "test\x07test");

        let mut dst = String::new();

        unescape(b"test\\btest", &mut dst).unwrap();
        assert_eq!(dst, "test\x08test");

        let mut dst = String::new();

        unescape(b"test\\ftest", &mut dst).unwrap();
        assert_eq!(dst, "test\x0Ctest");

        let mut dst = String::new();

        unescape(b"test\\ntest", &mut dst).unwrap();
        assert_eq!(dst, "test\ntest");

        let mut dst = String::new();

        unescape(b"test\\rtest", &mut dst).unwrap();
        assert_eq!(dst, "test\rtest");

        let mut dst = String::new();

        unescape(b"test\\ttest", &mut dst).unwrap();
        assert_eq!(dst, "test\ttest");

        let mut dst = String::new();

        unescape(b"test\\vtest", &mut dst).unwrap();
        assert_eq!(dst, "test\x0Btest");
    }

    #[test]
    fn test_unescape_mixed() {
        let mut dst = String::new();

        unescape(b"test\\\\test\\/test\\stest\\ptest\\atest\\btest\\ftest\\ntest\\rtest\\ttest\\vtest", &mut dst).unwrap();
        assert_eq!(dst, "test\\test/test test|test\x07test\x08test\x0Ctest\ntest\rtest\ttest\x0Btest");
    }

    #[test]
    fn test_unescape_error() {
        let mut dst = String::new();

        assert!(matches!(unescape(b"test\\", &mut dst), Err(ParseError::MalformedEscapeSequence { .. })));
    }
}