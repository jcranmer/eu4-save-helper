paradox::scope_list!{
    // XXX: Syntax?
    // <province id> -> Province
    // <area> -> Province
    // <region> -> Province
    // <superregion> -> Province
    // <continent> -> Province
    // <trade_company> -> Province
    // <colonial_region> -> Province
    // <event_target> -> ???
    scope(*, Country, emperor);
    scope(*, Country, revolution_target);
    scope(*, Country, crusade_target);
    scope(*, Country, <tag: Country>);
    scope(*, Province, <an_area: Area>);
    scope(*, Province, <a_region: Region>);
    scope(*, Province, <a_superregion: Superregion>);
    scope(*, Province, <a_continent: Continent>);
    scope(Country, Country, all_countries_including_self);
    scope(Country, Country, colonial_parent);
    scope(Country, Country, overlord);
    scope(Country, Province, home_trade_node);
    scope(Country, Province, capital_scope);
    scope(Province, Country, controller);
    scope(Province, Country, owner);
    scope(Province, Country, most_province_trade_power);
    scope(Province, Country, strongest_trade_power);
    scope(Province, Province, area);
    scope(Province, Province, sea_zone);
    scope(Province, Province, area_for_scope_province);

    scope_many(Country, Province, active_trade_node);
    scope_many(Country, Country, ally);
    scope_many(Country, Country, coalition_member);
    scope_many(Country, Province, core_province);
    scope_many(Country, Country, country);
    scope_many(Country, Country, elector);
    scope_many(Country, Country, enemy_country);
    scope_many(Country, Province, heretic_province);
    scope_many(Country, Country, known_country);
    scope_many(Country, Country, local_enemy);
    scope_many(Country, Country, neighbor_country);
    scope_many(Country, Province, owned_province);
    scope_many(*, Province, province);
    scope_many(Country, Country, rival_country);
    scope_many(Country, Province, state_province);
    scope_many(Country, Province, states);
    scope_many(Country, Country, subject_country);
    scope_many(Country, Province, trade_node);
    scope_many(Province, Province, empty_neighbor_province);
    scope_many(Province, Province, neighbor_province);
    scope_many(Province, Province, province_in_state);
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CountryScope {
    scope(Scope),
    any_owned_province,
}

impl CountryScope {
    pub fn get_scope(parser: &mut paradox::Parser, key: &str) -> Option<Self> {
        if let Some(scope) = Scope::get_scope(parser, key) {
            return Some(CountryScope::scope(scope));
        }
        // XXX: Fix province ref somehow...
        use std::str::FromStr;
        if let Ok(_) = u32::from_str(key) {
            return Some(Self::any_owned_province);
        }
        None
    }

    /*
    fn parse_country_list(parser: &mut paradox::Parser, value: paradox::Token
                          ) -> Result<Vec<Box<dyn paradox::Condition>>, paradox::ParseError> {
        let mut vec = Vec::new();
        value.expect_complex()?;
        while let Some((key, value)) = parser.get_next_value()? {
            match key {
                None => {
                    parser.validation_error("CountryCondition", "", "bad_key", false,
                                            Some(value))?;
                },
                Some(key) => {
                    let key = key.into_owned();
                    let condition = Box::new(
                        parser.try_parse::<crate::CountryCondition>(&key, value)?);
                    vec.push(condition);
                },
            }
        }
        Ok(vec)
    }*/
}

pub type ProvinceScope = CountryScope;
