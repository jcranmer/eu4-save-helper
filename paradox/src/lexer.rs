use byteorder::{ReadBytesExt, LittleEndian};
use crate::{FixedPoint, GameTrait, ParseError};
use derivative::Derivative;
use std::io::{BufReader, Bytes, Read};
use std::marker::PhantomData;
use string_cache::{Atom, StaticAtomSet};

type Result<T> = std::result::Result<T, ParseError>;

/// An individual toker from the lexer.
#[derive(Derivative)]
#[derivative(Debug(bound=""), Clone(bound=""), PartialEq(bound=""))]
pub enum Token<Static: StaticAtomSet> {
    /// The { token
    LBrace,
    /// The } token
    RBrace,
    /// The = token
    Eq,
    /// A quoted or unquoted string.
    String(String),
    /// A fixed atom (useful for faster parsing).
    Atom(Atom<Static>),
    // Special binary token types. We don't parse these in the text lexer.
    Bool(bool),
    Fixed(FixedPoint),
    Float(f64),
    Integer(i32),
    Unsigned(u32)
}

/// A trait for lexing the input files of Paradox games that use the Jomini or
/// Clausewitz engines.
pub trait Lexer<G: GameTrait> {
    /// Get the next token. If EOF has been reached, return None instead.
    fn get_token(&mut self) -> Result<Option<Token<G::Static>>>;

    /// Get a displayable name for the current location.
    fn get_location_info(&self) -> String;
}

pub struct TextLexer<R: Read> {
    reader: Bytes<BufReader<R>>,
    saved_char: Option<u8>,
    filename: String,
    line: u32,
    column: u32
}

impl <R: Read> TextLexer<R> {
    /// Create a lexer from the given input file. Pass a filename in as well, to
    /// give better error messages.
    pub fn new(reader: R, filename: String) -> Self {
        TextLexer {
            reader: BufReader::new(reader).bytes(),
            filename, line: 1, column: 1,
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

impl <G: GameTrait, R: Read> Lexer<G> for TextLexer<R> {
    fn get_token(&mut self) -> Result<Option<Token<G::Static>>> {
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
                    return Ok(Some(Token::Atom(self.read_unknown(ch)?.into())))
            }
        }
    }

    fn get_location_info(&self) -> String {
        format!("{}:{}:{}", self.filename, self.line, self.column)
    }
}

pub struct BinaryLexer<G: GameTrait, R: Read> {
    filename: String,
    offset: u32,
    reader: BufReader<R>,
    _trait: PhantomData<G>
}

impl <G: GameTrait, R: Read> BinaryLexer<G, R> {
    pub fn new(reader: R, filename: String) -> Self {
        BinaryLexer {
            reader: BufReader::new(reader),
            offset: 0,
            filename,
            _trait: PhantomData
        }
    }

    fn read_token(&mut self) -> Result<Token<G::Static>> {
        let code = self.reader.read_u16::<LittleEndian>()?;
        self.offset += 2;
        Ok(match code {
            0x0001 => Token::Eq,
            0x0003 => Token::LBrace,
            0x0004 => Token::RBrace,
            0x000b => Token::Atom("id".into()),
            0x000c => {
                let val = self.reader.read_i32::<LittleEndian>()?;
                self.offset += 4;
                Token::Integer(val)
            },
            0x000d => {
                // Fixed point notation.
                let val = self.reader.read_i32::<LittleEndian>()?;
                self.offset += 4;
                Token::Fixed(FixedPoint(val))
            },
            0x000e => {
                let val = self.reader.read_u8()?;
                self.offset += 1;
                match val {
                    0 => Token::Bool(false),
                    1 => Token::Bool(true),
                    _ => return Err(ParseError::Lexer(val.to_string()))
                }
            },
            0x000f | 0x0017 => {
                let len = self.reader.read_u16::<LittleEndian>()? as usize;
                self.offset += 2;
                let mut data = Vec::with_capacity(len);
                data.resize(len, 0);
                self.reader.read_exact(&mut data)?;
                self.offset += len as u32;
                let string = data.iter().map(|&ch| ch as char).collect();
                Token::String(string)
            },
            0x0014 => {
                let val = self.reader.read_u32::<LittleEndian>()?;
                self.offset += 4;
                Token::Unsigned(val)
            },
            0x001b => {
                Token::Atom("name".into())
            },
            0x0167 => {
                // A fixed point number, with a base of 1 << 16.
                let val = self.reader.read_i64::<LittleEndian>()?;
                self.offset += 8;
                // As long as the mantissa is small enough, we can represent
                // this number exactly in a double-precision floating-point
                // number.
                let mantissa_size = 64 - val.abs().leading_zeros();
                if mantissa_size > std::f64::MANTISSA_DIGITS + 1 {
                    return Err(ParseError::Lexer(format!("{:016x}", val)));
                }
                // Converting to double-precision and then doing a fdiv is the
                // easiest way to do the conversion. Both steps are exact, if we
                // do not fail the above test. We might get a slight speed boost
                // by doing the exponent/mantissa manipulation ourselves, but
                // it's not worth the code complexity.
                let val = (val as f64) / 65536.0;
                Token::Float(val)
            },
            0x0020..=0xffff => {
                G::get_binary_token(code)
                    .map(|s| Token::Atom(s.as_ref().into()))
                    .ok_or_else(|| ParseError::Lexer(
                            format!("Unknown code: {:04x}", code)))?
            },
            _ => panic!("Unknown code: {:4x}", code)
        })
    }
}

impl <G: GameTrait, R: Read> Lexer<G> for BinaryLexer<G, R> {
    fn get_token(&mut self) -> Result<Option<Token<G::Static>>> {
        match self.read_token() {
            Err(ParseError::Io(e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                Ok(None)
            },
            Err(e) => Err(e),
            Ok(t) => Ok(Some(t))
        }
    }

    fn get_location_info(&self) -> String {
        format!("{}:{:08x}", self.filename, self.offset)
    }
}

