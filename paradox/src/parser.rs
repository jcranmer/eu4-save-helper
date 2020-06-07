use std::borrow::Cow;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

const ERR_ON_INVALID_INPUT : bool = false;

type Result<T> = std::result::Result<T, ParseError>;

// There's no good documentation on Paradox's file format here. Most of this
// information is reverse-engineered from the existing files. In addition,
// there may be subtle differences between different engine versions, and hence
// between games (and since I don't own all of them, I can't test all of the
// issues here).

pub type UnparsedValue = Token;
impl Token {
    /// Return true if this is a simple value (string, integer, etc.).
    fn is_simple_value(&self) -> bool {
        match self {
            Self::LBrace => false,
            Self::RBrace | Self::Eq => false,
            _ => true
        }
    }

    /// Convert the token into a string if it can be done.
    pub fn try_to_string(&self) -> Result<&str> {
        match self {
            Self::String(s) => Ok(&s),
            Self::Interned(s) => Ok(&s),
            t => Err(ParseError::Parse(t.clone()))
        }
    }

    /// Return an error if the underlying value is not a complex (bracketed)
    /// value.
    pub fn expect_complex(self) -> Result<()> {
        match self {
            Self::LBrace => Ok(()),
            t => Err(ParseError::Parse(t))
        }
    }
}

impl From<Token> for Cow<'static, str> {
    fn from(t:Token) -> Cow<'static, str> {
        match t {
            Token::LBrace | Token::RBrace | Token::Eq =>
                panic!("Shouldn't call this method if it's not a simple value"),
            Token::String(s) => s.into(),
            Token::Interned(s) => s.into(),
            Token::Bool(b) => (if b { "yes " } else { "no" }).into(),
            Token::Fixed(f) => f.to_string().into(),
            Token::Float(f) => f.to_string().into(),
            Token::Integer(i) => i.to_string().into(),
            Token::Unsigned(i) => i.to_string().into()
        }
    }

}

pub type ValuePair = (Option<Cow<'static, str>>, UnparsedValue);

pub trait ParadoxParse {
    fn read_from(&mut self, parser: &mut Parser,
                 value: UnparsedValue) -> Result<()>;
}

pub trait FromParadoxKeyPair {
    fn try_from(parser: &mut Parser, key: &str,
                value: UnparsedValue) -> Result<Self>
        where Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    Eq,
    String(String),
    Interned(&'static str),
    // Special binary token types. We don't parse these in the text lexer.
    Bool(bool),
    Fixed(crate::FixedPoint),
    Float(f64),
    Integer(i32),
    Unsigned(u32)
}

pub trait Lexer {
    fn get_token(&mut self) -> Result<Option<Token>>;
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

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("error reading file")]
    Io(#[from] std::io::Error),
    #[error("lexing error: {0}")]
    Lexer(String),
    #[error("unexpected token: {0:?}")]
    Parse(Token),
    #[error("unexpected eof")]
    Eof,
    #[error("error reading type")]
    Conversion(#[from] Box<dyn std::error::Error + 'static>),
    #[error("value error: {0}")]
    Constraint(String)
}

pub struct Parser<'a> {
    lexer: &'a mut dyn Lexer,
    depth: u32,
    saved_token: Option<Token>,
    game_data: &'a mut crate::GameData
}

impl <'a> Parser<'a> {
    pub fn new(lexer: &'a mut dyn Lexer,
               game_data: &'a mut crate::GameData) -> Parser<'a> {
        Parser { lexer, depth: 0, saved_token: None, game_data }
    }

    pub fn get_game_data(&mut self) -> &mut crate::GameData {
        self.game_data
    }

    pub fn parse(mut self, result: &mut dyn ParadoxParse) -> Result<()> {
        result.read_from(&mut self, Token::LBrace)
            .or_else(|err| {
                eprintln!("Error at {}", self.lexer.get_location_info());
                Err(err)
            })
    }

    pub fn try_parse<T: FromParadoxKeyPair>(&mut self, key: &str,
                                            value: UnparsedValue) -> Result<T> {
        T::try_from(self, key, value)
    }

    fn get_token(&mut self) -> Result<Option<Token>> {
        if self.saved_token.is_some() {
            Ok(self.saved_token.take())
        } else {
            self.lexer.get_token()
        }
    }

    fn unget(&mut self, token: Token) {
        assert!(self.saved_token.is_none(), "Can only save one token");
        self.saved_token = Some(token);
    }

    fn try_key_eq(&mut self, key: Token) -> Result<ValuePair> {
        Ok(match self.get_token()? {
            // EOF: it's okay if we're at top depth.
            None if self.depth == 0 => (None, key),
            None => return Err(ParseError::Eof),

            // Eq: we are to be followed by a value.
            Some(Token::Eq) => {
                (Some(key.into()), match self.get_token()? {
                    Some(t) if t.is_simple_value() => t,
                    Some(Token::LBrace) => {
                        self.depth += 1;
                        Token::LBrace
                    },
                    None => return Err(ParseError::Eof),
                    Some(t) => return Err(ParseError::Parse(t))
                })
            },

            // LBrace: this happens in gamestate, and I assume an = should have
            // been present.
            Some(Token::LBrace) => {
                self.depth += 1;
                (Some(key.into()), Token::LBrace)
            },

            // For anything else, unget the character and return the
            // value as an untyped thing.
            Some(t) => {
                self.unget(t);
                (None, key)
            }
        })
    }

    pub fn get_next_value(&mut self) -> Result<Option<ValuePair>> {
        match self.get_token()? {
            None if self.depth == 0 => Ok(None),
            None => Err(ParseError::Eof),
            Some(Token::RBrace) if self.depth > 0 => {
                self.depth -= 1;
                Ok(None)
            },
            Some(Token::LBrace) => {
                self.depth += 1;
                Ok(Some((None, Token::LBrace)))
            },
            Some(t) if t.is_simple_value() =>
                self.try_key_eq(t).map(|val| Some(val)),
            Some(t) => Err(ParseError::Parse(t))
        }
    }

    pub fn validation_error(&mut self, class_name: &'static str, field: &str,
                            message: &str, fatal: bool,
                            value: Option<Token>) -> Result<()> {
        let type_hint = match value {
            Some(Token::LBrace) => " (scope)",
            Some(Token::Integer(_)) => " (i32)",
            Some(Token::Unsigned(_)) => " (u32)",
            Some(Token::Float(_)) => " (f64)",
            Some(Token::Fixed(_)) => " (FixedPoint)",
            Some(Token::Bool(_)) => " (bool)",
            Some(Token::String(_)) => " (String)",
            Some(Token::Interned(_)) => " (String)",
            _ => "",
        };
        let msg = format!("{}/{}{}: {}", class_name, field, type_hint, message);
        if fatal || ERR_ON_INVALID_INPUT {
            Err(ParseError::Constraint(msg))
        } else {
            println!("warning: {}", msg);
            if let Some(value) = value {
                let mut discard = ();
                discard.read_from(self, value)?;
            }
            Ok(())
        }
    }

    pub fn drain(&mut self, value: Token) -> Result<()> {
        let mut discard = ();
        discard.read_from(self, value)
    }
}

/// Load an entire directory of parseable files.
///
/// All of the entries will be loaded in alphabetical order.
pub fn load_directory(path: &Path, data: &mut dyn ParadoxParse,
                      gamedata: &mut crate::GameData) -> Result<()> {
    let mut files : Vec<_> = Default::default();
    if path.is_dir() {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if !entry.metadata()?.is_file() {
                eprintln!("Unexpected non-file in directory: {}",
                          path.display());
                continue;
            } else if path.extension().is_none() {
                eprintln!("Unexpected non-txt file in directory: {}",
                          path.display());
                continue;
            } else if path.extension().unwrap() != "txt" {
                eprintln!("Unexpected non-txt file in directory: {}",
                          path.display());
                continue;
            }
            files.push(path);
        }
        files.sort();
    } else {
        files.push(path.to_path_buf());
    }
    for path in files {
        let filename = path.to_string_lossy().into();
        let file = File::open(path)?;
        let mut lexer = TextLexer::new(file, filename);
        Parser::new(&mut lexer, gamedata).parse(data)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_reader(input: &'static [u8]) -> TextLexer<&'static [u8]> {
        TextLexer::new(input, "input".into())
    }

    fn check_tokens(mut lexer: impl Lexer, vec: Vec<Token>) {
        for token in vec {
            assert_eq!(lexer.get_token().unwrap(), Some(token));
        }
        assert_eq!(lexer.get_token().unwrap(), None);
    }

    #[test]
    fn test_lexer() {
        let lexer = make_reader(b"1.0#");
        check_tokens(lexer, vec![Token::String("1.0".into())]);

        let lexer = make_reader(b"# This is a comment\n1.0");
        check_tokens(lexer, vec![Token::String("1.0".into())]);

        let lexer = make_reader(b"-5={ 1} \"inner\"");
        check_tokens(lexer, vec![
            Token::String("-5".into()),
            Token::Eq,
            Token::LBrace,
            Token::String("1".into()),
            Token::RBrace,
            Token::String("inner".into())
        ]);
    }

    #[test]
    fn test_parser() -> Result<()> {
        let mut res : HashMap<String, i32> = Default::default();
        let lexer = make_reader(b"a=1 b=2");
        Parser::new(Box::new(lexer))
            .parse(&mut res)?;
        assert_eq!(*res.get("a").unwrap(), 1);
        assert_eq!(*res.get("b").unwrap(), 2);
        assert_eq!(res.iter().len(), 2);

        let mut res : HashMap<String, Vec<i32>> = Default::default();
        let lexer = make_reader(b"a={1 2 3}");
        Parser::new(Box::new(lexer))
            .parse(&mut res)?;
        assert_eq!(*res.get("a").unwrap(), vec![1, 2, 3]);
        assert_eq!(res.iter().len(), 1);

        let mut res : Vec<Vec<Vec<i32>>> = Default::default();
        let lexer = make_reader(b"{{1 2} {3 4} {5 6}}");
        Parser::new(Box::new(lexer))
            .parse(&mut res)?;
        assert_eq!(res, vec![vec![vec![1, 2], vec![3, 4], vec![5, 6]]]);

        Ok(())
    }
}
