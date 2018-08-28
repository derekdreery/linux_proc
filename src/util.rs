use std::{self, io};
use Error; // todo use `!`.

/// A helper to facilitate paring line by line while reusing a string buffer.
pub struct LineParser<R> {
    reader: io::BufReader<R>,
    buffer: String,
}

impl<R> LineParser<R> where R: io::Read {

    pub fn new(reader: R) -> LineParser<R> {
        LineParser {
            reader: io::BufReader::new(reader),
            buffer: String::with_capacity(100),
        }
    }

    /// If the parse fails, the line is available for trying different parsers.
    pub fn parse_line<F, E, Val>(&mut self, parser: F) -> io::Result<Val>
        where F: Fn(&str) -> Result<Val, E>,
              E: std::error::Error + Send + Sync + 'static
    {
        // Only fetch next line if we consumed the previous
        if self.buffer.is_empty() {
            let read = io::BufRead::read_line(&mut self.reader, &mut self.buffer)?;
            if read == 0 {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }
        }
        let parsed = parser(&self.buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, Box::new(e)))?;
        // we've succeeded so clear the buffer.
        self.buffer.clear();
        Ok(parsed)
    }
}


pub fn parse_u64(input: &str) -> Option<(&str, u64)> {
    let input = consume_space(input);
    let mut chars = input.chars();
    let (mut next_idx, mut acc) = match chars.next() {
        Some(ch) => match ch.to_digit(10) {
            Some(val) => (ch.len_utf8(), val as u64),
            None => return None
        },
        None => return None
    };
    for ch in chars {
        match ch.to_digit(10) {
            Some(val) => {
                acc = acc * 10 + val as u64;
                next_idx += ch.len_utf8();
            },
            None => break
        }
    }
    Some((&input[next_idx..], acc))
}

#[test]
fn test_parse_u64() {
    assert_eq!(parse_u64(""), None);
    assert_eq!(parse_u64(" "), None);
    assert_eq!(parse_u64("12 "), Some((" ", 12)));
    assert_eq!(parse_u64("12"), Some(("", 12)));
    assert_eq!(parse_u64("a12"), None);
    assert_eq!(parse_u64(" 12"), Some(("", 12)));
    assert_eq!(parse_u64("a 12"), None);
}

pub fn consume_space(input: &str) -> &str {
    for (idx, ch) in input.char_indices() {
        if ! ch.is_whitespace() && ch != '\n' && ch != '\r' {
            return &input[idx..];
        }
    }
    return &input[input.len()..];
}

#[test]
fn test_consume_space() {
    assert_eq!(consume_space(""), "");
    assert_eq!(consume_space(" "), "");
    assert_eq!(consume_space(" a"), "a");
    assert_eq!(consume_space(" a "), "a ");
    assert_eq!(consume_space("a "), "a ");
}

/// Consumes any space before the token, but not after.
pub fn parse_token(input: &str) -> Option<(&str, &str)> {
    let token = consume_space(input);
    if token.is_empty() {
        return None;
    }
    let mut end = 0;
    for (idx, ch) in token.char_indices() {
        if ch.is_whitespace() {
            break;
        }
        end = idx + ch.len_utf8();
    }
    Some((&token[end..], &token[..end]))
}

#[test]
fn test_parse_token() {
    assert_eq!(parse_token(""), None);
    assert_eq!(parse_token(" "), None);
    assert_eq!(parse_token("token "), Some((" ", "token")));
    assert_eq!(parse_token("token"), Some(("", "token")));
    assert_eq!(parse_token(" token"), Some(("", "token")));
    assert_eq!(parse_token(" token "), Some((" ", "token")));
}


// todo should be ! not Error.
pub fn parse_dummy(_input: &str) -> Result<(), Error> {
    Ok(())
}
