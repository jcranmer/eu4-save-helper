use crate::*;
use byteorder::{ReadBytesExt, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::{ZipArchive, result::ZipError};

impl From<ZipError> for ParseError {
    fn from(err: ZipError) -> Self {
        match err {
            ZipError::Io(err) => Self::Io(err),
            _ => Self::Conversion(Box::new(err))
        }
    }
}

struct BinaryLexer<R: Read> {
    offset: u32,
    reader: R,
}

impl <R: Read> BinaryLexer<R> {
    fn new(reader: R) -> Self {
        BinaryLexer { reader, offset: 0 }
    }

    fn read_token(&mut self) -> Result<Token, ParseError> {
        let code = self.reader.read_u16::<LittleEndian>()?;
        self.offset += 2;
        Ok(match code {
            0x0001 => Token::Eq,
            0x0003 => Token::LBrace,
            0x0004 => Token::RBrace,
            0x000b => Token::String("id".into()),
            0x000c => {
                let val = self.reader.read_i32::<LittleEndian>()?;
                self.offset += 4;
                Token::String(val.to_string())
            },
            0x000d => {
                // Fixed point notation.
                let val = self.reader.read_i32::<LittleEndian>()?;
                self.offset += 4;
                Token::String(format!("{}.{:03}", val / 1000, val % 1000))
            },
            0x000e => {
                let val = self.reader.read_u8()?;
                self.offset += 1;
                match val {
                    0 => Token::String("no".into()),
                    1 => Token::String("yes".into()),
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
                Token::String(val.to_string())
            },
            0x001b => {
                Token::String("name".into())
            },
            0x0167 => {
                // A fixed pointer number, with a base of 1 << 16.
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
                Token::String(val.to_string())
            },
            0x0020..=0xffff => Token::String(format!("{:04x}", code)),
            _ => panic!("Unknown code: {:4x}", code)
        })
    }
}

impl <R: Read> Lexer for BinaryLexer<R> {
    fn get_token(&mut self) -> Result<Option<Token>, ParseError> {
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
        format!("offset {:08x}", self.offset)
    }
}

pub fn load_savegame<T: ParadoxParse + Default>(path: &Path)
        -> Result<T, ParseError> {
    let mut archive = ZipArchive::new(File::open(path)?)?;
    // This is really bad behavior, but we require a 'static bounds on the
    // parse file, even though it's safe because we only need to borrow it for
    // as long as parse() is called.
    let laundered_archive : &'static mut _ = unsafe {
        (&mut archive as *mut ZipArchive<_>).as_mut().unwrap()
    };
    // Gamestate appears to always be the first entry in the index.
    let mut file = laundered_archive.by_index(0)?;
    let mut gamestate = T::default();

    // The first 6 bytes will tell us if it's a binary or save file.
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;
    let lexer = if &magic[3..] == b"txt" {
        Box::new(TextLexer::new(file, path.to_string_lossy().into()))
            as Box::<dyn Lexer>
    } else if &magic[3..] == b"bin" {
        Box::new(BinaryLexer::new(file))
            as Box::<dyn Lexer>
    } else {
        return Err(ParseError::Parse(Token::String(String::from_utf8_lossy(&magic).into())));
    };

    Parser::new(lexer).parse(&mut gamestate)?;

    Ok(gamestate)
}
