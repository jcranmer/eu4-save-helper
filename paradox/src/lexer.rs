use crate::ParseError;
use std::io::Read;

type Result<T> = std::result::Result<T, ParseError>;

/// An individual toker from the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// The { token
    LBrace,
    /// The } token
    RBrace,
    /// The = token
    Eq,
    /// A quoted or unquoted string.
    String(String),
    /// XXX: legacy
    Interned(&'static str),
    // Special binary token types. We don't parse these in the text lexer.
    Bool(bool),
    Fixed(crate::FixedPoint),
    Float(f64),
    Integer(i32),
    Unsigned(u32)
}

/// A trait for lexing the input files of Paradox games that use the Jomini or
/// Clausewitz engines.
pub trait Lexer {
    /// Get the next token. If EOF has been reached, return None instead.
    fn get_token(&mut self) -> Result<Option<Token>>;

    /// Get a displayable name for the current location.
    fn get_location_info(&self) -> String;
}

pub struct TextLexer<R: Read> {
    reader: std::io::Bytes<R>,
    saved_char: Option<u8>,
    filename: String,
    line: u32,
    column: u32
}

impl <R: Read> TextLexer<R> {
    /// Create a lexer from the given input file. Pass a filename in as well, to
    /// give better error messages.
    pub fn new(reader: R, filename: String) -> Self {
        TextLexer { reader: reader.bytes(), filename, line: 1, column: 1,
            saved_char: None
        }
    }

    fn get_char(&mut self) -> std::io::Result<Option<u8>> {
        if let Some(ch) = self.saved_char {
            self.saved_char = None;
            Ok(Some(ch))
        } else {
            let ch = self.reader.next();
            match ch {
                Some(Ok(b'\n')) => {
                    self.line += 1; self.column = 1;
                    Ok(Some(b'\n'))
                },
                Some(Ok(ch)) => {
                    self.column += 1;
                    Ok(Some(ch))
                },
                Some(Err(err)) => {
                    Err(err)
                },
                None => {
                    Ok(None)
                },
            }
        }
    }

    fn unget(&mut self, ch: u8) {
        assert!(self.saved_char.is_none(), "Only one char can be ungotten");
        self.saved_char = Some(ch);
    }

    /// Read until the end of line of a comment.
    fn skip_comment(&mut self) -> std::io::Result<()> {
        loop {
            match self.get_char()? {
                Some(b'\n') | None => return Ok(()),
                _ => continue
            }
        }
    }

    /// Read the tail of a quoted string.
    fn read_qstring(&mut self) -> Result<String> {
        let mut s = String::new();
        loop {
            match self.get_char()? {
                Some(b'"') => return Ok(s),
                Some(ch) => s.push(ch as char),
                None => return Err(
                    ParseError::Lexer("could not find end of string".into()))
            }
        }
    }

    /// Read an unparsed full token.
    fn read_unknown(&mut self, init_char: u8) -> Result<String> {
        let mut s = String::new();
        s.push(init_char as char);
        loop {
            match self.get_char()? {
                Some(ch) if b"#{=}\"".contains(&ch) => {
                    self.unget(ch);
                    return Ok(s);
                },
                None => return Ok(s),
                Some(ch) if Self::is_whitespace(ch) => return Ok(s),
                Some(ch) => s.push(ch as char),
            }
        }
    }

    /// Check if the given character is whitespace, according to Paradox.
    fn is_whitespace(ch: u8) -> bool {
        ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n'
    }
}

impl <R: Read> Lexer for TextLexer<R> {
    fn get_token(&mut self) -> Result<Option<Token>> {
        loop {
            match self.get_char()? {
                None => return Ok(None),
                Some(ch) if Self::is_whitespace(ch) => continue,
                Some(b'#') => self.skip_comment()?,
                Some(b'{') => return Ok(Some(Token::LBrace)),
                Some(b'}') => return Ok(Some(Token::RBrace)),
                Some(b'=') => return Ok(Some(Token::Eq)),
                Some(b'"') =>
                    return Ok(Some(Token::String(self.read_qstring()?))),
                Some(ch) =>
                    return Ok(Some(Token::String(self.read_unknown(ch)?)))
            }
        }
    }

    fn get_location_info(&self) -> String {
        format!("{}:{}:{}", self.filename, self.line, self.column)
    }
}

