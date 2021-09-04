use crate::*;
use std::fs::File;
use std::io::{Read, Seek};
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

struct ZipLexer<'a, G: 'static + GameTrait, R: Read + Seek> {
    archive: &'a mut ZipArchive<R>,
    path: &'a Path,
    cur_index: usize,
    cur_lexer: Box<dyn Lexer<G>>
}

impl <'a, G: 'static + GameTrait, R: Read + Seek> ZipLexer<'a, G, R> {
    fn new(archive: &'a mut ZipArchive<R>,
           path: &'a Path) -> Result<Self, ParseError> {
        let entry = archive.by_index(0)?;
        let lexer = get_lexer(entry, path)?;
        Ok(Self { archive, cur_index: 0, cur_lexer: lexer, path })
    }

    fn advance_to_next_file(&mut self) -> Result<bool, ParseError> {
        self.cur_index += 1;
        if self.cur_index >= self.archive.len() {
            return Ok(false);
        }

        let entry = self.archive.by_index(self.cur_index);
        if entry?.name().ends_with(".zip") {
            return Ok(false);
        }

        Ok(true)
    }
}

impl <G: GameTrait, R: Read + Seek> Lexer<G> for ZipLexer<'_, G, R> {
    fn get_token(&mut self) -> Result<Option<Token>, ParseError> {
        loop {
            match (*self.cur_lexer).get_token()? {
                None => {
                    if !self.advance_to_next_file()? {
                        return Ok(None);
                    }
                    self.cur_lexer = get_lexer(
                        self.archive.by_index(self.cur_index)?,
                        self.path)?;
                    continue;
                },
                t => return Ok(t)
            }
        }
    }

    fn get_location_info(&self) -> String {
        (*self.cur_lexer).get_location_info()
    }
}

fn get_lexer<G: 'static + GameTrait>(mut entry: zip::read::ZipFile,
             path: &Path) -> Result<Box<dyn Lexer<G>>, ParseError> {
    let entry_name = format!("{}/{}", path.display(), entry.name());

    // Read the first 6 bytes, to determine if it's a text or binary file.
    let mut magic = [0u8; 6];
    entry.read_exact(&mut magic)?;

    // Read the rest of the file to a vector. It sucks that we have to do
    // this, but we can't make the lifetimes work out with entry.
    let mut data = Vec::with_capacity(entry.size() as usize - 6);
    entry.read_to_end(&mut data)?;
    let file = std::io::Cursor::new(data);

    // Use the magic bytes to choose a text or a binary lexer.
    if &magic[3..] == b"txt" {
        Ok(Box::new(TextLexer::new(file, entry_name)) as Box::<dyn Lexer<G>>)
    } else if &magic[3..] == b"bin" {
        Ok(Box::new(BinaryLexer::new(file, entry_name)) as Box::<dyn Lexer<G>>)
    } else {
        Err(ParseError::Parse(Token::String(String::from_utf8_lossy(&magic).into())))
    }
}

pub fn load_savegame<G: 'static + GameTrait, T: ParadoxParse<G> + Default>(
    path: &Path, game_data: &mut GameData)
        -> Result<T, ParseError> {
    let mut archive = ZipArchive::new(File::open(path)?)?;
    let mut lexer = ZipLexer::new(&mut archive, path)?;
    let mut gamestate = T::default();
    Parser::new(&mut lexer, game_data).parse(&mut gamestate)?;

    Ok(gamestate)
}

pub fn ironmelt(in_path: &Path, out_path: &Path) -> Result<(), ParseError> {
    use std::io::Write;
    use zip::ZipWriter;
    use zip::write::FileOptions;
    let mut archive = ZipArchive::new(File::open(in_path)?)?;
    let mut writer = ZipWriter::new(File::create(out_path)?);
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let name = entry.name();
        let file_opts = FileOptions::default()
            .compression_method(entry.compression())
            .last_modified_time(entry.last_modified())
            .unix_permissions(entry.unix_mode().unwrap_or(0o644));
        writer.start_file(name, file_opts)?;
        writeln!(writer, "EU4txt")?;

        let mut lexer = get_lexer::<DummyTrait>(entry, in_path)?;
        let mut is_array = false;
        let mut is_key = true;
        let mut is_array_known = true;
        let mut indent = String::new();
        let mut saved_token = None;
        let mut stack = Vec::new();
        loop {
            let token = match lexer.get_token()? {
                Some(t) => t,
                None => break
            };
            match token {
                Token::LBrace => {
                    assert!(saved_token.is_none(), "Fuuuckk");
                    write!(writer, " {{")?;
                    stack.push((is_array_known, is_array));
                    is_array_known = false;
                    is_array = true;
                },
                Token::RBrace => {
                    if let Some(t) = saved_token.take() {
                        write!(writer, "{}", t)?;
                        is_array = true;
                    }
                    if is_array_known && !is_array {
                        indent.pop();
                        indent.pop();
                    }
                    writeln!(writer, "{}}}", if is_array { " " } else { "" })?;
                    write!(writer, "{}", indent)?;
                    let entry = stack.pop().unwrap();
                    is_array_known = entry.0;
                    is_array = entry.1;
                    is_key = true;
                },
                Token::Eq => {
                    if let Some(t) = saved_token.take() {
                        is_array = false;
                        is_array_known = true;
                        indent.push_str("  ");
                        write!(writer, "\n{}{}", indent, t)?;
                    } else {
                        assert!(is_array_known && !is_array, "Fuck?");
                    }
                    write!(writer, " =")?;
                    is_key = false;
                },
                t => {
                    let as_string = ParserAtom::from(t);
                    if is_array_known {
                        if is_array {
                            write!(writer, " {}", as_string)?;
                        } else if is_key {
                            write!(writer, "{}", as_string)?;
                            is_key = false;
                        } else {
                            write!(writer, " {}\n{}", as_string, indent)?;
                            is_key = true;
                        }
                    } else if let Some(t) = saved_token.take() {
                        is_array_known = true;
                        is_array = true;
                        write!(writer, " {} {}", t, as_string)?;
                    } else {
                        saved_token = Some(as_string);
                    }
                },
            }
        }
        assert!(saved_token.is_none(), "Not going to happen");
    }
    writer.finish()?;
    Ok(())
}
