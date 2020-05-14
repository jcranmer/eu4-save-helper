use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Bytes, Read, Result};
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eq,
    LBrace,
    RBrace,
    Id(u16),
    String(String),
    Integer(i32),
    Float(f32)
}

impl Token {
    pub fn is(&self, s: &str) -> bool {
        match *self {
            Token::String(ref inner) => inner == s,
            _ => false
        }
    }
}

impl From<Token> for String {
    fn from(t: Token) -> String {
        match t {
            Token::String(s) => s,
            Token::Integer(v) => v.to_string(),
            Token::Float(v) => v.to_string(),
            _ => panic!("Tried to convert {:?} to string", t)
        }
    }
}

impl From<Token> for bool {
    fn from(t: Token) -> bool {
        if t.is("yes") {
            true
        } else if t.is("no") {
            false
        } else {
            panic!("Tried to convert {:?} to bool", t);
        }
    }
}

impl From<Token> for i32 {
    fn from(t: Token) -> i32 {
        match t {
            Token::Integer(i) => i,
            _ => panic!("Tried to convert {:?} to int", t)
        }
    }
}

impl From<Token> for crate::FixedPoint {
    fn from(t: Token) -> Self {
        match t {
            Token::Float(f) => Self::from(f),
            _ => panic!("Tried to convert {:?} to fixed-point", t)
        }
    }
}

impl From<Token> for crate::Date {
    fn from(t: Token) -> crate::Date {
        let s = match t {
            Token::String(s) => s,
            Token::Integer(v) => v.to_string(),
            Token::Float(v) => v.to_string(),
            _ => panic!("Tried to convert {:?} to string", t)
        };
        s.parse().unwrap()
    }
}

impl <T> From<Token> for crate::gamestate::CacheKey<T> {
    fn from(t: Token) -> Self {
        match t {
            Token::Integer(i) => i.into(),
            Token::String(s) => s.into(),
            _ => panic!("Tried to convert {:?} to cache token", t)
        }
    }
}

fn convert_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap_or_else(|err| {
        let bytes = err.into_bytes();
        bytes.into_iter().map(|ch| ch as char).collect()
    })
}

pub struct Lexer<R: Read> {
    file: Peekable<Bytes<R>>,
    saved_token: Option<Token>,
    line: u32,
}

impl <R:Read> Lexer<R> {
    pub fn new(read: R) -> Lexer<R> {
        Lexer { file: read.bytes().peekable(), saved_token: None, line: 1 }
    }

    pub fn unget(&mut self, token: Token) {
        assert!(self.saved_token.is_none(), "Cannot call unget more than once");
        self.saved_token = Some(token)
    }

    fn peek_byte(&mut self) -> Result<Option<u8>> {
        match self.file.peek() {
            None => Ok(None),
            Some(&Err(ref e)) => Err(e.kind().into()),
            Some(&Ok(ch)) => Ok(Some(ch))
        }
    }

    fn advance(&mut self) -> Result<u8> {
        let ch = self.file.next().unwrap()?;
        if ch == b'\n' { self.line += 1; }
        Ok(ch)
    }

    fn consume(&mut self, t: Token) -> Result<Option<Token>> {
        self.advance()?;
        Ok(Some(t))
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            match self.peek_byte()? {
                None => break,
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') =>
                    self.advance()?,
                Some(_) => break
            };
        }
        Ok(())
    }

    fn skip_line(&mut self) -> Result<()> {
        loop {
            match self.peek_byte()? {
                None => break,
                Some(b'\n') => break,
                Some(_) => self.advance()?
            };
        }
        Ok(())
    }

    fn read_word(&mut self) -> Result<String> {
        let mut word = Vec::with_capacity(40);
        loop {
            match self.peek_byte()? {
                None => break,
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => break,
                Some(b'=') | Some(b'{') | Some(b'}') => break,
                Some(_) => {
                    word.push(self.advance()?)
                }
            }
        }
        Ok(convert_string(word))
    }

    fn read_qstring(&mut self) -> Result<String> {
        self.advance()?; // Consume the opening "
        let mut word = Vec::with_capacity(40);
        loop {
            match self.peek_byte()? {
                None => break,
                Some(b'"') => {
                    self.advance()?;
                    break;
                },
                Some(_) => {
                    word.push(self.advance()?)
                }
            }
        }
        Ok(convert_string(word))
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        loop {
            let ch = match self.peek_byte()? {
                None => return Ok(None),
                Some(ch) => ch as char
            };
            let token = match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.skip_whitespace()?;
                    continue
                },
                '#' => {
                    self.skip_line()?;
                    continue
                },
                '=' => return self.consume(Token::Eq),
                '{' => return self.consume(Token::LBrace),
                '}' => return self.consume(Token::RBrace),
                '"' => {
                    let s = self.read_qstring()?;
                    return Ok(Some(Token::String(s)));
                },
                'a'..='z'|'A'..='Z'|'0'..='9'|'-' => {
                    self.read_word()?
                },
                ch => {
                    println!("Unknown token: {}", ch);
                    return Err(std::io::ErrorKind::InvalidData.into());
                }
            };

            // Work out what the token kind is based on whether or not it
            // parses correctly.
            return if let Ok(val) = i32::from_str(&token) {
                Ok(Some(Token::Integer(val)))
            } else if let Ok(val) = f32::from_str(&token) {
                Ok(Some(Token::Float(val)))
            } else {
                Ok(Some(Token::String(token)))
            };
        }
    }
}

impl <R: Read> Iterator for Lexer<R> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.saved_token.take() {
            Some(Ok(t))
        } else {
            self.next_token().transpose()
        }
    }
}


pub trait ParadoxScope {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope;
    fn set_property(&mut self, id: Option<Token>, value: Token);
}

pub struct NullScope {}

const NULL_SCOPE: RefCell<NullScope> = RefCell::new(NullScope {});

impl NullScope {
    pub fn instance() -> &'static mut NullScope {
        unsafe {
            NULL_SCOPE.as_ptr().as_mut().unwrap()
        }
    }
}

impl ParadoxScope for NullScope {
    fn start_scope(&mut self, _: Token) -> &mut ParadoxScope {
        self
    }

    fn set_property(&mut self, _: Option<Token>, _: Token) { }
}

impl <T: From<Token>> ParadoxScope for Vec<T> {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        panic!("Unexpected token {:?} in vector", id);
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        assert!(id.is_none(), "Vectors cannot have keys");
        self.push(T::from(value));
    }
}

impl <K: From<Token> + Eq + Hash, V: ParadoxScope + Default> ParadoxScope for HashMap<K, V> {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        self.entry(id.into()).or_default()
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        panic!("Cannot set a property on a map of scopes.");
    }
}

/*
impl <K: From<Token> + Eq + Hash, V: From<Token>> ParadoxScope for HashMap<K, V> {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        panic!("Map of tokens cannot have scopes.");
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        if let Some(id) = id {
            self.insert(id.into(), value);
        } else {
            panic!("Cannot have an empty id in a HashMap.");
        }
    }
}
*/


fn parse<R: Read>(lexer: &mut Lexer<R>, scope: &mut ParadoxScope, depth: usize) -> Result<()> {
    macro_rules! error {
        (eof) => {{
            eprintln!("Unexpected EOF at line {}", lexer.line);
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }};
        (unexpected $t:expr) => {{
            eprintln!("Unexpected token: {:?} on line {}", $t, lexer.line);
            return Err(std::io::ErrorKind::InvalidInput.into());
        }};
    };
    loop {
        // Find the identifier start of a property.
        let key = match lexer.next().transpose()? {
            None => {
                if depth != 0 {
                    eprintln!("Depth was {}", depth);
                    error!{eof};
                }
                return Ok(());
            },
            Some(Token::RBrace) => {
                if depth == 0 {
                    error!{unexpected Token::RBrace};
                }
                return Ok(());
            },
            Some(_t @ Token::LBrace) => {
                // Funky case. This appears to only occur for {id=#,type=#}
                // structs that are built in a list of some kind.
                parse(lexer, scope.start_scope(Token::Id(0)), depth + 1)?;
                continue;
            },
            Some(t @ Token::Float(_)) => {
                // Property with no key
                scope.set_property(None, t);
                continue;
            },
            Some(t @ Token::Eq) => {
                error!{unexpected t};
            },
            Some(t) => t
        };

        // Is this an A=B property, or just an A property?
        match lexer.next().transpose()? {
            // A = B
            Some(Token::Eq) => {
                match lexer.next().transpose()? {
                    Some(Token::LBrace) => {
                        parse(lexer, scope.start_scope(key), depth + 1)?;
                    },
                    Some(t @ Token::RBrace) |
                    Some(t @ Token::Eq) => error!{unexpected t},
                    Some(t) => {
                        scope.set_property(Some(key), t);
                    },
                    None => error!{eof}
                }
            },
            Some(Token::LBrace) => {
                parse(lexer, scope.start_scope(key), depth + 1)?;
            },

            // A
            Some(t @ Token::Id(_)) |
            Some(t @ Token::String(_)) |
            Some(t @ Token::Integer(_)) |
            Some(t @ Token::RBrace) => {
                lexer.unget(t);
                scope.set_property(None, key);
            },

            // Neither
            None => error!(eof),
            Some(t) => error!(unexpected t),
        }
    }
}

pub fn parse_file<R: Read>(mut lexer: Lexer<R>, scope: &mut ParadoxScope) -> Result<()> {
    parse(&mut lexer, scope, 0)
}

pub fn make_scope<R: Read, S: ParadoxScope + Default>(lexer: Lexer<R>) -> Result<S> {
    let mut scope : S = Default::default();
    parse_file(lexer, &mut scope).map(|_| scope)
}
