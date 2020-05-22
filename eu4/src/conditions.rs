use paradox::{FixedPoint, IdRef};
use crate::{Continent, Country, Religion};
paradox::condition_list!{
    condition(Country, absolutism, FixedPoint);
    condition(Country, employed_advisor, ()); // XXX -- complex clauses...
    condition(Country, exists, IdRef<Country>);
    condition(Country, government, String);
    condition(Country, has_country_flag, String);
    condition(Country, has_country_modifier, String);
    condition(Country, has_dlc, String);
    condition(Country, has_estate_influence_modifier, ()); // XXX -- complex clauses...
    condition(Country, has_idea_group, String);
    condition(Country, has_institution, String);
    condition(Country, has_parliament, bool);
    condition(Country, is_religion_enabled, IdRef<Religion>);
    condition(Country, mercantilism, FixedPoint);
    condition(Country, owns, i32);
    condition(Country, religion, IdRef<Religion>);
    condition(Country, religion_years, ()); // XXX -- complex clauses...

    condition(Province, continent, IdRef<Continent>);

    // XXX: these are really *
    condition(Country, current_age, String);
    condition(Country, is_year, u32);
}

impl paradox::Condition for CountryCondition { }
impl paradox::Condition for ProvinceCondition { }
