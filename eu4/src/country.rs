use paradox::{IdKey, ParadoxParse, ParseError, Parser, ParserAtom};
use std::collections::HashMap;

type ParseResult = Result<(), ParseError>;

#[derive(Default)]
pub struct CountryMap(HashMap<ParserAtom, Country>);

impl ParadoxParse for CountryMap {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut filename = String::default();
            filename.read(parser)?;
            let path = format!("common/{}", filename);
            let mut result : Country = Default::default();
            let data = parser.get_game_data();
            data.parse_directory(&path, &mut result)?;
            let id = key.clone();
            if self.0.insert(id, result).is_some() {
                return Err(ParseError::Constraint(
                        format!("Duplicate tag {}", key)));
            }
            Ok(())
        })?;

        // Add dynamic tags
        const PAIRS: &[(char, u32)] = &[
            ('D', 75), // MAX_CUSTOM_COUNTRIES in defines.lua
            ('C', 75), // MAX_COLONIAL_NATIONS in defines.lua
            ('K',100), // MAX_CLIENT_STATES in defines.lua
            ('F',100), // MAX_FEDERATION_COUNTRIES in defines.lua
            ('E', 50), // MAX_ESTATE_COUNTRIES in defines.lua
            ('T', 75), // MAX_TRADING_CITIES in defines.lua
            ('O', 10), // MAX_OBSERVERS in defines.lua
        ];
        for &(start, count) in PAIRS {
            for num in 0..count {
                let tag = format!("{}{:02}", start, num);
                self.0.insert(tag.into(), Default::default());
            }
        }
        Ok(())
    }
}

#[derive(ParadoxParse, Default)]
pub struct Country {
    pub color: crate::RgbColor,
    pub graphical_culture: String,
    pub leader_names: Vec<String>,
    pub monarch_names: (),
    #[optional] pub army_names: Vec<String>,
    #[optional] pub colonial_parent: String,
    #[optional] pub fleet_names: Vec<String>,
    #[optional] pub historical_idea_groups: Vec<String>,
    #[optional] pub historical_score: i32,
    #[optional] pub historical_units: Vec<String>,
    #[optional] pub preferred_religion: String,
    #[optional] pub random_nation_chance: i32,
    #[optional] pub revolutionary_colors: crate::RgbColor,
    #[optional] pub ship_names: Vec<String>,
    #[optional] pub special_unit_culture: String,
    #[optional] pub right_to_bear_arms: bool,
    #[optional] pub all_your_core_are_belong_to_us: bool,
}

#[derive(Default)]
pub struct Area {}

#[derive(ParadoxParse, Default)]
pub struct Region {
    #[optional] pub areas: Vec<ParserAtom>,
    #[optional] pub monsoon: [(); 2]
}

#[derive(Default)]
pub struct Superregion {}

#[derive(Default)]
pub struct Continent {}

#[derive(Default)]
pub struct Climate {}

#[derive(ParadoxParse, Default)]
pub struct ClimateList {
    pub equator_y_on_province_image: u32,

    #[collect]
    pub climates: HashMap<IdKey<Climate>, Vec<u32>>,
}
