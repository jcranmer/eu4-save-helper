use std::borrow::Cow;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

const ERR_ON_INVALID_INPUT : bool = true;

type Result<T> = std::result::Result<T, ParseError>;

// There's no good documentation on Paradox's file format here. Most of this
// information is reverse-engineered from the existing files. In addition,
// there may be subtle differences between different engine versions, and hence
// between games (and since I don't own all of them, I can't test all of the
// issues here).

pub enum UnparsedValue {
    Complex {
        level: u32
    },
    Simple(String)
}

impl UnparsedValue {
    fn make_complex(parser: &Parser) -> Self {
        Self::Complex {
            level: parser.depth
        }
    }

    pub fn into_string(self) -> Result<String> {
        match self {
            Self::Complex{ level: _ } =>
                Err(ParseError::Parse(Token::LBrace)),
            Self::Simple(s) => Ok(s)
        }
    }

    pub fn next_key_value_pair<'a>(&mut self, parser: &'a mut Parser
                                ) -> Result<Option<ValuePair<'a>>> {
        let level = match self {
            Self::Complex { level } => *level,
            Self::Simple(s) =>
                return Err(ParseError::Parse(Token::String(s.clone())))
        };

        if level > parser.depth {
            Ok(None)
        } else {
            parser.get_value()
        }
    }

    pub fn drain(self, parser: &mut Parser) -> Result<()> {
        let mut discard = ();
        discard.read_from(parser, self)
    }

    pub fn validation_error(&self, class_name: &'static str, field: &str,
                            message: &str,
                            fatal: bool) -> Result<()> {
        let msg = format!("{}/{}: {}", class_name, field, message);
        if fatal || ERR_ON_INVALID_INPUT {
            Err(ParseError::Constraint(msg))
        } else {
            eprintln!("warning: {}", msg);
            Ok(())
        }
    }
}

pub type ValuePair<'a> = (Option<Cow<'a, str>>, UnparsedValue);

pub trait ParadoxParse {
    fn read_from(&mut self, parser: &mut Parser,
                 value: UnparsedValue) -> Result<()>;
}

pub trait FromParadoxKeyPair : Sized {
    fn try_from(parser: &mut Parser, key: &str,
                value: UnparsedValue) -> Result<Self>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    Eq,
    // XXX: simple value type is probably better
    String(String),
    Interned(&'static str)
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

pub struct Parser {
    lexer: Box<dyn Lexer>,
    depth: u32,
    saved_token: Option<Token>
}

impl Parser {
    pub fn new(lexer: Box<dyn Lexer>) -> Parser {
        Parser { lexer, depth: 0, saved_token: None }
    }

    pub fn parse(mut self, result: &mut dyn ParadoxParse) -> Result<()> {
        let unparsed = UnparsedValue::make_complex(&self);
        result.read_from(&mut self, unparsed)
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

    fn try_key_eq<'a>(&mut self, key: Cow<'a, str>) -> Result<ValuePair<'a>> {
        Ok(match self.get_token()? {
            // EOF: it's okay if we're at top depth.
            None if self.depth == 0 =>
                (None, UnparsedValue::Simple(key.into_owned())),
            None => return Err(ParseError::Eof),

            // Eq: we are to be followed by a value.
            Some(Token::Eq) => {
                (Some(key), match self.get_token()? {
                    Some(Token::String(value)) => UnparsedValue::Simple(value),
                    Some(Token::Interned(value)) =>
                        UnparsedValue::Simple(value.into()),
                    Some(Token::LBrace) => {
                        self.depth += 1;
                        UnparsedValue::make_complex(self)
                    },
                    None => return Err(ParseError::Eof),
                    Some(t) => return Err(ParseError::Parse(t))
                })
            },

            // LBrace: this happens in gamestate, and I assume an = should have
            // been present.
            Some(Token::LBrace) => {
                self.depth += 1;
                (Some(key), UnparsedValue::make_complex(self))
            },

            // For anything else, unget the character and return the
            // value as an untyped thing.
            Some(t) => {
                self.unget(t);
                (None, UnparsedValue::Simple(key.into_owned()))
            }
        })
    }

    fn get_value(&mut self) -> Result<Option<ValuePair>> {
        match self.get_token()? {
            None if self.depth == 0 => Ok(None),
            None => Err(ParseError::Eof),
            Some(Token::RBrace) if self.depth > 0 => {
                self.depth -= 1;
                Ok(None)
            },
            Some(Token::LBrace) => {
                self.depth += 1;
                Ok(Some((None, UnparsedValue::make_complex(self))))
            },
            Some(Token::String(key)) => {
                self.try_key_eq(key.into()).map(|val| Some(val))
            },
            Some(Token::Interned(key)) => {
                self.try_key_eq(key.into()).map(|val| Some(val))
            },
            Some(t) => Err(ParseError::Parse(t))
        }
    }
}

/// Load an entire directory of parseable files.
///
/// All of the entries will be loaded in alphabetical order.
pub fn load_directory(path: &Path, data: &mut dyn ParadoxParse) -> Result<()> {
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
        let lexer = TextLexer::new(file, filename);
        Parser::new(Box::new(lexer)).parse(data)?;
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
    fn test_parser() -> Result<(), ParseError> {
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
