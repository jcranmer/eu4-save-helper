use paradox::{Date, FixedPoint};
use eu4::*;
use crate::lexer::{Lexer, parse_file};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::marker::PhantomData;
use zip::read::ZipArchive;

#[derive(Default, ParadoxScope, Debug)]
pub struct TypedId {
    pub id: i32,
    pub r#type: i32,
}

#[derive(Default, Clone, Debug)]
pub struct CacheKey<T> {
    key: String,
    phantom: PhantomData<T>
}

impl <T> From<i32> for CacheKey<T> {
    fn from(val: i32) -> CacheKey<T> {
        CacheKey {
            key: val.to_string(),
            phantom: Default::default()
        }
    }
}

impl <T> From<String> for CacheKey<T> {
    fn from(val: String) -> CacheKey<T> {
        CacheKey {
            key: val,
            phantom: Default::default()
        }
    }
}

#[derive(Default, ParadoxScope)]
pub struct Gamestate {
}

#[derive(Default, ParadoxScope)]
pub struct Country {
    pub authority: FixedPoint,
    pub coalition_date: Date,
    pub doom: FixedPoint,
    pub excommunicated: bool,
    pub harmony: FixedPoint,
    pub horde_unity: FixedPoint,
    pub is_elector: bool,
    pub is_great_power: bool,
    pub liberty_desire: FixedPoint,
    pub luck: bool,
    pub patriarch_authority: FixedPoint,
    pub rnw_generated: bool,
    pub overlord: CacheKey<Country>,
    pub coalition_target: CacheKey<Country>,
    pub colonial_parent: CacheKey<Country>,
    pub influenced_by: CacheKey<Country>,
    pub federation_leader: CacheKey<Country>,
    pub religion: CacheKey<Religion>,
    pub trade_port: CacheKey<Province>,
    pub capital: CacheKey<Province>,
    pub fixed_capital: CacheKey<Province>,
    pub religious_school: CacheKey<ReligiousSchool>,

    // Sliders & other UI changes
    pub last_conversion_secondary: Date,
    pub last_migration: Date,
    pub last_hre_vote: Date,
    pub preferred_emperor: CacheKey<Country>,

    // History
    pub inflation_history: Vec<FixedPoint>,
    pub score_place: i32,
    pub score_rank: Vec<i32>,
    pub age_score: Vec<FixedPoint>,
    pub vc_age_score: Vec<FixedPoint>,
    pub score_rating: Vec<FixedPoint>,
    pub friend_tags: Vec<CacheKey<Country>>,

    // Triggers?
    pub estimated_monthly_income: FixedPoint,
    pub num_of_banners: i32,
    pub num_of_independence_supporters: i32,
    pub num_uncontested_cores: i32,
    pub estimated_loan: FixedPoint,
    pub navy_strength: FixedPoint,
    pub great_power_score: FixedPoint,
    pub in_debt: bool,
    pub num_of_age_objectives: i32,
    pub num_of_rebel_armies: i32,
    pub num_of_rebel_controlled_provinces: i32,
    pub num_of_revolts: i32,
    pub num_of_subjects: i32,
    pub num_of_war_reparations: i32,
    pub blockaded_percent: FixedPoint,
    pub num_of_non_cores: i32,
    pub overextension_percentage: FixedPoint,
    pub num_of_heathen_provs: i32,
    pub num_of_heretic_provs: i32,
    //pub flags: HashMap<String, Date>,
    //pub hidden_flags: HashMap<String, Date>,
    //pub variables: HashMap<String, FixedPoint>,

    // Caches
    pub total_war_worth: i32,
    pub opinion_cache: Vec<i32>,
    pub last_war_ended: Date,
    pub custom_name: String,
    pub cached_colonies: i32,
    pub total_count: Vec<i32>,

    // Unknown
    pub rebel_threat: i32,
    pub goldtype: i32,
    pub wants_to_be_great_power: bool,
    pub wants_to_be_great_power_next: bool,
    pub convert: bool,
    pub last_bankrupt: Date,
    pub preferred_coalition_score: FixedPoint,
    pub debase_recharge_need: i32,

    // AI fields?
    pub human: bool,
    pub was_player: bool
}

#[derive(Default, ParadoxScope)]
pub struct Province {
}

fn load_gamestate(file: &mut dyn Read) -> Result<Gamestate> {
    let mut gamestate = Default::default();
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;
    if &magic == b"EU4txt" {
        let lexer = Lexer::new(file);
        parse_file(lexer, &mut gamestate)?;
        Ok(gamestate)
    } else if &magic == b"EU4bin" {
        panic!("Binary lexing not implemented yet");
    } else {
        Err(ErrorKind::InvalidData.into())
    }
}

pub fn load_savegame(mut file: File) -> Result<Gamestate> {
    let mut magic = [0u8; 4];
    file.read_exact(magic.as_mut())?;
    file.seek(SeekFrom::Start(0))?;
    if &magic == b"PK\x03\x04" {
        // Zip file
        let mut zip = ZipArchive::new(file)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let mut file = zip.by_name("gamestate")
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        load_gamestate(&mut file)
    } else {
        load_gamestate(&mut file)
    }
}
