use paradox::ParseError;
use paradox::ironmelt;

pub fn main() -> Result<(), ParseError> {
    let path = &std::path::Path::new("/tmp/TrailOfTears.eu4-37");
    let out = &std::path::Path::new("/tmp/melted.zip");
    ironmelt(path, out);
    Ok(())
}
