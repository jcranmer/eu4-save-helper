pub struct GameData {
    game_path: PathBuf,
    loaded_data: HashMap<String, Box<dyn Any + 'static>>
}

impl GameData {
    pub fn load(game_dir: &Path) -> std::io::Result<GameData> {
        if !game_dir.is_dir() {
            return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Path {} is not a directory", game_dir.display())));
        }

        Ok(GameData {
            game_path: game_dir,
            loaded_data: Default::default()
        })
    }
}
