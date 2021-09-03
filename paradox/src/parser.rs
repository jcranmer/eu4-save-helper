use std::io::Read;
use std::fs::File;
use std::path::Path;
use thiserror::Error;
use string_cache::{Atom, StaticAtomSet};

const ERR_ON_INVALID_INPUT : bool = false;

type Result<T> = std::result::Result<T, ParseError>;

// There's no good documentation on Paradox's file format here. Most of this
// information is reverse-engineered from the existing files. In addition,
// there may be subtle differences between different engine versions, and hence
// between games (and since I don't own all of them, I can't test all of the
// issues here).

impl Token {
    /// Convert the token into a string if it can be done.
    pub fn try_to_string(&self) -> Result<&str> {
        match self {
            Self::String(s) => Ok(&s),
            Self::Interned(s) => Ok(&s),
            t => Err(ParseError::Parse(t.clone()))
        }
    }
}

impl <Static: StaticAtomSet> From<Token> for Atom<Static> {
    fn from(t: Token) -> Self {
        match t {
            Token::LBrace | Token::RBrace | Token::Eq =>
                panic!("Shouldn't call this method if it's not a simple value"),
            Token::String(s) => Self::from(s),
            Token::Interned(s) => Self::from(s),
            Token::Bool(b) => Self::from(if b { "yes" } else { "no" }),
            Token::Fixed(f) => f.to_string().into(),
            Token::Float(f) => f.to_string().into(),
            Token::Integer(i) => i.to_string().into(),
            Token::Unsigned(i) => i.to_string().into()
        }
    }
}

pub trait ParadoxParse {
    fn read(&mut self, parser: &mut Parser) -> Result<()>;
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
    game_data: &'a mut crate::GameData,
    scope: Vec<String>,
}

impl <'a> Parser<'a> {
    pub fn new(lexer: &'a mut dyn Lexer,
               game_data: &'a mut crate::GameData) -> Parser<'a> {
        Parser { lexer, depth: 0, saved_token: None, game_data, scope: Vec::new() }
    }

    pub fn parse_key_scope<F>(&mut self, mut func: F) -> Result<()>
        where F: FnMut(ParserAtom, &mut Self) -> Result<()>
    {
        let is_top = self.depth == 0;
        if !is_top {
            match self.get_token()? {
                Some(Token::LBrace) => {},
                None => return Err(ParseError::Eof),
                Some(t) => return Err(ParseError::Parse(t)),
            }
        }
        self.depth += 1;
        let hit_eof = loop {
            let key = match self.get_token()? {
                Some(Token::RBrace) => break false,
                None => break true,
                Some(t) => ParserAtom::from(t),
            };
            match self.get_token()? {
                Some(Token::Eq) => {},
                Some(Token::LBrace) => {
                    self.unget(Token::LBrace);
                },
                None => return Err(ParseError::Eof),
                Some(t) => return Err(ParseError::Parse(t)),
            }
            self.scope.push(key.as_ref().into());
            //println!("Parsing {}", self.scope.join("/"));
            func(key, self)?;
            self.scope.pop();
        };
        self.depth -= 1;
        match (hit_eof, is_top) {
            (true, true) | (false, false) => Ok(()),
            (true, false) => Err(ParseError::Eof),
            (false, true) => Err(ParseError::Parse(Token::RBrace)),
        }
    }

    pub fn with_scope<F>(&mut self, mut func: F) -> Result<()>
        where F: FnMut(&mut Self) -> Result<()>
    {
        match self.get_token()? {
            Some(Token::LBrace) => {},
            None => return Err(ParseError::Eof),
            Some(t) => return Err(ParseError::Parse(t)),
        }
        self.depth += 1;
        self.scope.push("(with_scope)".into());
        loop {
            match self.get_token()? {
                Some(Token::RBrace) => break,
                None => break,
                Some(t) => self.unget(t),
            };
            func(self)?;
        }
        self.scope.pop();
        self.depth -= 1;
        Ok(())
    }

    pub fn get_game_data(&mut self) -> &mut crate::GameData {
        self.game_data
    }

    pub fn parse(mut self, result: &mut dyn ParadoxParse) -> Result<()> {
        result.read(&mut self)
            .or_else(|err| {
                eprintln!("Error at {}", self.lexer.get_location_info());
                Err(err)
            })
    }

    pub fn get_token(&mut self) -> Result<Option<Token>> {
        if self.saved_token.is_some() {
            Ok(self.saved_token.take())
        } else {
            self.lexer.get_token()
        }
    }

    pub fn unget(&mut self, token: Token) {
        assert!(self.saved_token.is_none(), "Can only save one token");
        self.saved_token = Some(token);
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
                self.unget(value);
                let mut discard = ();
                discard.read(self)?;
            }
            Ok(())
        }
    }
}

pub type ParserAtom = string_cache::DefaultAtom;

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
